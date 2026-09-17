import { invokeCommand, nativeInvoke } from "./core/bridge.js";
import {
  activePage,
  appState,
  cloneState,
  defaultState,
  lang,
  mergeState,
  pageDefs,
  setActivePage,
  sliderDefs,
  viewState
} from "./core/state.js";
import { translate } from "./core/i18n.js";
import {
  ensureColorGameSelection,
  fallbackColorGames,
  renderGameColorPage,
  renderColorPage,
  selectColorGame,
  selectedColorGame,
  stepColorGame,
  syncAllSliders,
  updateSliderDom
} from "./features/color/page.js";
import { updateCharacteristicsLiveDom } from "./features/characteristics/page.js";
import { allTweaks, isSafeTweak } from "./features/tweaks/catalog.js";
import { icon } from "./ui/icons.js";
import { renderMain, renderShell, renderPageBody } from "./ui/layout.js";

let applyTimer = 0;
let characteristicsTimer = 0;
let characteristicsBusy = false;
let carouselStepAt = 0;
let trimTimer = 0;

function scheduleTrimMemory(delay = 400) {
  window.clearTimeout(trimTimer);
  trimTimer = window.setTimeout(() => {
    invokeCommand("trim_memory");
  }, delay);
}

function t(key) {
  return translate(lang(), key);
}

function syncNavIndicator() {
  const sidebar = document.querySelector(".sidebar");
  const activeItem = document.querySelector(".nav-item.active");
  if (!sidebar || !activeItem) return;
  const indicatorHeight = Math.min(24, activeItem.offsetHeight);
  const y = activeItem.offsetTop + (activeItem.offsetHeight - indicatorHeight) / 2;
  sidebar.style.setProperty("--nav-indicator-y", `${Math.round(y)}px`);
  sidebar.style.setProperty("--nav-indicator-height", `${indicatorHeight}px`);
}

function updateNavState() {
  document.querySelectorAll(".nav-item").forEach((item) => {
    item.classList.toggle("active", item.getAttribute("data-page") === activePage);
  });
  syncNavIndicator();
}

function updateMain() {
  const page = pageDefs[activePage];
  const title = document.querySelector(".page-header h1");
  const subtitle = document.querySelector(".page-header p");
  const body = document.querySelector(".page-body");
  if (title && subtitle && body) {
    title.textContent = t(page.title);
    subtitle.textContent = t(page.subtitle);
    body.className = `page-body page-${activePage}`;
    body.innerHTML = renderPageBody(viewState, t);
  } else {
    const main = document.querySelector(".main-panel");
    if (main) main.outerHTML = renderMain(viewState, t);
  }
  syncAllSliders(appState);
  updateNavState();
  window.requestAnimationFrame(syncNavIndicator);
  scheduleTrimMemory(350);
}

function updateColorPage(options = {}) {
  const isColorSurface = activePage === "color" || activePage === "gameColor";
  const body = document.querySelector(activePage === "gameColor" ? ".page-gameColor" : ".page-color");
  if (!body || !isColorSurface) {
    updateMain();
    return;
  }
  const app = document.getElementById("app");
  if (options.stableDrawer && app) app.classList.add("stable-drawer");
  body.innerHTML = activePage === "gameColor"
    ? renderGameColorPage(appState, viewState, t)
    : renderColorPage(appState, viewState, t);
  syncAllSliders(appState);
  if (options.stableDrawer && app) {
    window.requestAnimationFrame(() => app.classList.remove("stable-drawer"));
  }
}

function updateMainKeepingScroll(selector = ".scroll-panel") {
  const panel = document.querySelector(selector);
  const scrollTop = panel?.scrollTop || 0;
  updateMain();
  const nextPanel = document.querySelector(selector);
  if (!nextPanel) return;
  nextPanel.scrollTop = scrollTop;
  window.requestAnimationFrame(() => {
    nextPanel.scrollTop = scrollTop;
  });
}

function syncTweakTile(id) {
  const tile = Array.from(document.querySelectorAll("[data-tweak-id]")).find(
    (element) => element.getAttribute("data-tweak-id") === id
  );
  if (!tile) {
    updateMainKeepingScroll(".tweaks-page .scroll-panel");
    return;
  }
  const selected = viewState.selectedTweaks.has(id);
  const installed = Boolean(viewState.installedTweaks?.has(id));
  const checked = selected || installed;
  tile.classList.toggle("selected", selected);
  tile.classList.toggle("installed", installed);
  tile.setAttribute("aria-pressed", selected ? "true" : "false");
  const box = tile.querySelector(".box");
  if (box) {
    box.classList.toggle("checked", checked);
    box.innerHTML = checked ? icon("check", "box-check") : "";
  }
}

async function applyColor() {
  const result = await invokeCommand("apply_color_settings", { color: appState.color });
  if (result) mergeState(result);
}

function scheduleApplyColor() {
  if (!appState.settings.applyInstantly) return;
  window.clearTimeout(applyTimer);
  applyTimer = window.setTimeout(applyColor, 80);
}

async function saveSettings() {
  const result = await invokeCommand("update_app_settings", { settings: appState.settings });
  if (result) mergeState(result);
}

async function loadLists() {
  const [backups, configs] = await Promise.all([
    invokeCommand("list_backups"),
    invokeCommand("list_configs")
  ]);

  if (Array.isArray(backups)) {
    viewState.backups = backups;
    if (!backups.some((item) => item.id === viewState.selectedBackup)) {
      viewState.selectedBackup = backups[0]?.id || "";
    }
  }

  if (Array.isArray(configs)) {
    viewState.configs = configs;
    if (!configs.some((item) => item.id === viewState.selectedConfig)) {
      viewState.selectedConfig = configs[0]?.id || "";
    }
  }
}

async function loadColorGames() {
  const games = await invokeCommand("list_installed_games");
  if (Array.isArray(games)) {
    viewState.colorGames = games;
    viewState.colorGamesDemo = false;
  } else if (!nativeInvoke) {
    viewState.colorGames = fallbackColorGames();
    viewState.colorGamesDemo = true;
  } else {
    viewState.colorGames = [];
    viewState.colorGamesDemo = false;
  }
  viewState.colorGamesLoaded = true;
  ensureColorGameSelection(viewState);
  if (activePage === "gameColor") updateMain();
}

async function loadTweakStatuses() {
  const statuses = await invokeCommand("get_tweak_statuses");
  viewState.installedTweaks.clear();
  if (!Array.isArray(statuses)) return;
  statuses.forEach((item) => {
    if (item?.installed && item.id) viewState.installedTweaks.add(item.id);
  });
}

function mergeCharacteristics(info) {
  if (!info) return false;
  viewState.system = { ...(viewState.system || {}), ...info };
  const cpu = Number.parseFloat(String(info.cpuUsagePercent ?? "").replace(",", "."));
  if (Number.isFinite(cpu)) {
    viewState.cpuHistory.push(Math.max(0, Math.min(100, cpu)));
    if (viewState.cpuHistory.length > 42) viewState.cpuHistory.splice(0, viewState.cpuHistory.length - 42);
  }
  return true;
}

async function refreshCharacteristics() {
  if (characteristicsBusy) return;
  characteristicsBusy = true;
  const info = await invokeCommand("get_system_characteristics");
  characteristicsBusy = false;
  if (mergeCharacteristics(info) && activePage === "characteristics") updateMain();
}

async function refreshLiveCharacteristics() {
  if (activePage !== "characteristics" || characteristicsBusy || document.hidden) return;
  characteristicsBusy = true;
  const info = await invokeCommand("get_system_live_metrics");
  characteristicsBusy = false;
  if (mergeCharacteristics(info) && activePage === "characteristics") updateCharacteristicsLiveDom(viewState);
}

function syncCharacteristicsMonitor() {
  window.clearInterval(characteristicsTimer);
  characteristicsTimer = 0;
  if (activePage !== "characteristics" || document.hidden) return;
  characteristicsTimer = window.setInterval(refreshLiveCharacteristics, 2000);
}

function applyPreset(name) {
  const presets = {
    balanced: { saturation: 160, hue: -5, contrast: 97, gamma: 118, enabled: true },
    vibrant: { saturation: 200, hue: -5, contrast: 95, gamma: 105, enabled: true },
    soft: { saturation: 150, hue: -5, contrast: 85, gamma: 115, enabled: true },
    night: { saturation: 120, hue: -5, contrast: 90, gamma: 150, enabled: true }
  };
  appState.color = { ...appState.color, ...presets[name] };
  updateColorPage();
  scheduleApplyColor();
}

function stepColorSelection(step) {
  const now = performance.now();
  if (now - carouselStepAt < 150) return false;
  if (!stepColorGame(viewState, step)) return false;
  carouselStepAt = now;
  updateColorPage({ stableDrawer: true });
  return true;
}

async function handleAction(action) {
  if (action === "window-minimize") return invokeCommand("minimize_window");
  if (action === "window-maximize") return invokeCommand("toggle_window_maximize");
  if (action === "window-close") return invokeCommand("close_window");
  if (action === "exit-app") return invokeCommand("exit_app");
  if (action === "restart-as-admin") return invokeCommand("restart_as_admin");
  if (action === "apply-color") return applyColor();
  if (action === "launch-color-game") {
    const game = selectedColorGame(viewState);
    if (game?.id && game.launchable) return invokeCommand("launch_installed_game", { id: game.id });
    return null;
  }
  if (action === "save-color-template") {
    const name = (viewState.colorTemplateName || "").trim() || "Default";
    const configs = await invokeCommand("save_config", { name });
    if (Array.isArray(configs)) {
      viewState.configs = configs;
      viewState.selectedConfig = configs[0]?.id || "";
      viewState.colorTemplateName = "";
      updateColorPage();
    }
    return null;
  }
  if (action === "select-safe-tweaks") {
    viewState.selectedTweaks.clear();
    allTweaks().filter(isSafeTweak).forEach((tweak) => viewState.selectedTweaks.add(tweak.id));
    return updateMain();
  }
  if (action === "apply-tweaks") {
    if (viewState.applyingTweaks) return null;
    const ids = Array.from(viewState.selectedTweaks);
    if (!ids.length) {
      viewState.tweakResults = [{ id: "-", status: "skipped", message: t("noSelection") }];
      return updateMain();
    }
    viewState.applyingTweaks = true;
    updateMain();
    const results = await invokeCommand("apply_tweaks", { ids });
    viewState.tweakResults = Array.isArray(results) ? results : [];
    await loadTweakStatuses();
    viewState.applyingTweaks = false;
    return updateMain();
  }
  if (action === "rollback-tweaks") {
    if (viewState.applyingTweaks) return null;
    viewState.applyingTweaks = true;
    updateMain();
    const restored = await invokeCommand("rollback_last_tweaks");
    if (restored) {
      mergeState(restored);
      await loadTweakStatuses();
      viewState.tweakResults = [{ id: "rollback", status: "applied", message: t("rollbackSuccess") }];
    } else {
      viewState.tweakResults = [{ id: "rollback", status: "skipped", message: t("noRollbackFound") }];
    }
    viewState.applyingTweaks = false;
    return updateMain();
  }
  if (action === "reset-color") {
    appState.color = cloneState(defaultState).color;
    updateMain();
    return applyColor();
  }
  if (action === "refresh-characteristics") return refreshCharacteristics();
  if (action === "toggle-all-drivers") {
    viewState.showAllDrivers = !viewState.showAllDrivers;
    return updateMain();
  }
  if (action === "create-backup") {
    const backups = await invokeCommand("create_backup", { name: viewState.backupName });
    if (Array.isArray(backups)) {
      viewState.backups = backups;
      viewState.selectedBackup = backups[0]?.id || "";
      updateMain();
    }
  }
  if (action === "restore-backup" && viewState.selectedBackup) {
    const restored = await invokeCommand("restore_backup", { id: viewState.selectedBackup });
    if (restored) {
      mergeState(restored);
      render();
    }
  }
  if (action === "delete-backup" && viewState.selectedBackup) {
    const backups = await invokeCommand("delete_backup", { id: viewState.selectedBackup });
    if (Array.isArray(backups)) {
      viewState.backups = backups;
      viewState.selectedBackup = backups[0]?.id || "";
      updateMain();
    }
  }
  if (action === "open-backups-folder") return invokeCommand("open_storage_folder", { kind: "backups" });
  if (action === "save-config") {
    const configs = await invokeCommand("save_config", { name: viewState.configName });
    if (Array.isArray(configs)) {
      viewState.configs = configs;
      viewState.selectedConfig = configs[0]?.id || "";
      updateMain();
    }
  }
  if (action === "load-config" && viewState.selectedConfig) {
    const loaded = await invokeCommand("load_config", { id: viewState.selectedConfig });
    if (loaded) {
      mergeState(loaded);
      syncCharacteristicsMonitor();
      render();
      scheduleApplyColor();
    }
  }
  if (action === "apply-config" && viewState.selectedConfig) {
    const next = await invokeCommand("apply_config", { id: viewState.selectedConfig });
    if (next) {
      mergeState(next);
      render();
    }
  }
  if (action === "delete-config" && viewState.selectedConfig) {
    const configs = await invokeCommand("delete_config", { id: viewState.selectedConfig });
    if (Array.isArray(configs)) {
      viewState.configs = configs;
      viewState.selectedConfig = configs[0]?.id || "";
      updateMain();
    }
  }
  if (action === "open-configs-folder") return invokeCommand("open_storage_folder", { kind: "configs" });
  return null;
}

function handleInput(event) {
  const input = event.target;
  if (!(input instanceof HTMLInputElement)) return;

  const colorField = input.getAttribute("data-color-field");
  if (colorField && sliderDefs[colorField]) {
    appState.color[colorField] = Number(input.value);
    updateSliderDom(colorField, appState);
    scheduleApplyColor();
    return;
  }

  const field = input.getAttribute("data-field");
  if (field && Object.hasOwn(viewState, field)) {
    viewState[field] = input.value;
  }
}

async function handleClick(event) {
  const target = event.target instanceof Element ? event.target : null;
  if (!target) return;

  const navItem = target.closest("[data-page]");
  if (navItem) {
    const nextPage = navItem.getAttribute("data-page");
    if (!nextPage || !pageDefs[nextPage] || nextPage === activePage) return;
    setActivePage(nextPage);
    updateMain();
    syncCharacteristicsMonitor();
    if (nextPage === "gameColor" && !viewState.colorGamesLoaded) {
      loadColorGames();
    } else if (nextPage === "tweaks" && viewState.installedTweaks.size === 0) {
      loadTweakStatuses().then(updateMain);
    } else if (nextPage === "characteristics") {
      if (!viewState.system) refreshCharacteristics();
      else refreshLiveCharacteristics();
    } else if ((nextPage === "color" || nextPage === "gameColor" || nextPage === "backups" || nextPage === "configs" || nextPage === "storage") && (!viewState.configs.length || !viewState.backups.length)) {
      loadLists().then(() => {
        if (nextPage === "color" || nextPage === "gameColor") updateColorPage();
        else updateMain();
      });
    }
    return;
  }

  const setting = target.closest("[data-setting]");
  if (setting) {
    const key = setting.getAttribute("data-setting");
    if (!Object.hasOwn(appState.settings, key)) return;
    appState.settings[key] = !appState.settings[key];
    await saveSettings();
    syncCharacteristicsMonitor();
    updateMain();
    if (key === "applyInstantly" && appState.settings.applyInstantly) await applyColor();
    return;
  }

  const language = target.closest("[data-language]");
  if (language) {
    appState.settings.language = language.getAttribute("data-language");
    await saveSettings();
    render();
    return;
  }

  const preset = target.closest("[data-preset]");
  if (preset) {
    applyPreset(preset.getAttribute("data-preset"));
    return;
  }

  const deleteTemplate = target.closest("[data-action='delete-color-template']");
  if (deleteTemplate) {
    event.preventDefault();
    event.stopPropagation();
    const id = deleteTemplate.getAttribute("data-delete-config-id");
    if (id) {
      const configs = await invokeCommand("delete_config", { id });
      if (Array.isArray(configs)) {
        viewState.configs = configs;
        if (viewState.selectedConfig === id) {
          viewState.selectedConfig = configs[0]?.id || "";
        }
        updateColorPage();
      }
    }
    return;
  }

  const colorTemplate = target.closest("[data-color-template-id]");
  if (colorTemplate) {
    const id = colorTemplate.getAttribute("data-color-template-id");
    if (!id) return;
    const loaded = await invokeCommand("load_config", { id });
    if (loaded) {
      mergeState(loaded);
      updateColorPage();
      scheduleApplyColor();
    }
    return;
  }

  const colorGame = target.closest("[data-game-id]");
  if (colorGame) {
    const id = colorGame.getAttribute("data-game-id");
    if (id && selectColorGame(viewState, id, true)) updateColorPage();
    return;
  }

  const tweak = target.closest("[data-tweak-id]");
  if (tweak) {
    event.preventDefault();
    const id = tweak.getAttribute("data-tweak-id");
    if (!id) return;
    if (viewState.selectedTweaks.has(id)) {
      viewState.selectedTweaks.delete(id);
    } else {
      viewState.selectedTweaks.add(id);
    }
    if (tweak instanceof HTMLElement) tweak.blur();
    syncTweakTile(id);
    return;
  }

  const driverSearch = target.closest("[data-driver-search]");
  if (driverSearch) {
    const query = driverSearch.getAttribute("data-driver-search");
    if (query) await invokeCommand("open_driver_search", { query });
    return;
  }

  const backup = target.closest("[data-backup-id]");
  if (backup) {
    viewState.selectedBackup = backup.getAttribute("data-backup-id");
    updateMain();
    return;
  }

  const config = target.closest("[data-config-id]");
  if (config) {
    viewState.selectedConfig = config.getAttribute("data-config-id");
    updateMain();
    return;
  }

  const action = target.closest("[data-action]");
  if (action) await handleAction(action.getAttribute("data-action"));
}

function handleWheel(event) {
  const target = event.target instanceof Element ? event.target : null;
  if (!target?.closest(".game-carousel")) return;
  const step = event.deltaY > 0 || event.deltaX > 0 ? 1 : -1;
  if (!stepColorSelection(step)) return;
  event.preventDefault();
}

function handleKeydown(event) {
  if (activePage !== "gameColor") return;
  const target = event.target;
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement || target instanceof HTMLSelectElement) return;
  if (event.key === "ArrowRight" || event.key === "ArrowDown") {
    if (stepColorSelection(1)) {
      event.preventDefault();
    }
    return;
  }
  if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
    if (stepColorSelection(-1)) {
      event.preventDefault();
    }
    return;
  }
  if (event.key === "Enter") {
    const game = selectedColorGame(viewState);
    if (!game) return;
    viewState.colorGamePanelOpen = true;
    event.preventDefault();
    updateColorPage();
    return;
  }
  if (event.key === "Escape" && viewState.colorGamePanelOpen) {
    viewState.colorGamePanelOpen = false;
    event.preventDefault();
    updateColorPage();
  }
}

function handlePointerDown(event) {
  if (event.button !== 0) return;
  const target = event.target instanceof Element ? event.target : null;
  if (!target?.closest("[data-window-drag]")) return;
  if (target.closest("button, input, textarea, select, a, [data-action], [data-page], [data-setting], [data-game-id], .window-controls")) return;
  event.preventDefault();
  invokeCommand("start_window_drag");
}

function handleImageError(event) {
  const image = event.target;
  if (!(image instanceof HTMLImageElement)) return;
  if (
    image.classList.contains("game-card-image") ||
    image.classList.contains("game-card-logo-image") ||
    image.classList.contains("game-card-watermark") ||
    image.classList.contains("game-title-watermark") ||
    image.classList.contains("color-game-head-mark")
  ) {
    if (image.classList.contains("game-card-logo-image")) {
      image.closest(".game-card")?.setAttribute("data-logo-loaded", "false");
    }
    image.remove();
  }
}

function render() {
  const app = document.getElementById("app");
  document.documentElement.lang = lang();
  app.className = "app-shell";
  app.innerHTML = renderShell(viewState, t);
  syncAllSliders(appState);
  updateNavState();
  window.requestAnimationFrame(syncNavIndicator);
  syncCharacteristicsMonitor();
}

async function boot() {
  const state = await invokeCommand("get_app_state");
  if (state) mergeState(state);
  await loadLists();
  render();
  loadColorGames();
  scheduleTrimMemory(1200);
}

document.addEventListener("input", handleInput);
document.addEventListener("click", handleClick);
document.addEventListener("pointerdown", handlePointerDown);
document.addEventListener("error", handleImageError, true);
document.addEventListener("wheel", handleWheel, { passive: false });
document.addEventListener("keydown", handleKeydown);
window.addEventListener("resize", syncNavIndicator);
window.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    window.clearInterval(characteristicsTimer);
    characteristicsTimer = 0;
  } else {
    syncCharacteristicsMonitor();
    if (activePage === "characteristics") refreshLiveCharacteristics();
  }
});
boot();
