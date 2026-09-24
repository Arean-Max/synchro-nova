import { invokeCommand, nativeInvoke } from "./core/bridge.js";
import {
  activePage,
  appState,
  applyInterfaceAccent,
  cloneState,
  defaultPresets,
  defaultState,
  lang,
  mergeState,
  pageDefs,
  pageOrder,
  setActivePage,
  sliderDefs,
  viewState
} from "./core/state.js";
import { translate } from "./core/i18n.js";
import {
  ensureColorGameSelection,
  renderGameColorPage,
  renderColorPage,
  selectColorGame,
  selectedColorGame,
  standardTemplateNames,
  stepColorGame,
  syncAllSliders,
  syncAllTemplateSliders,
  updateBlackHoloDom,
  updateSliderDom,
  updateTemplateSliderDom,
  colorFeature
} from "./features/color/page.js";
import { allTweaks, tweakCategory, tweakTitle } from "./features/tweaks/catalog.js";
import { getTweakImpactDetails, renderIosNotification, tweaksFeature } from "./features/tweaks/page.js";
import { renderBackupNameModal, backupsFeature } from "./features/storage/page.js";
import { characteristicsFeature } from "./features/characteristics/page.js";
import { settingsFeature } from "./features/settings/page.js";
import { router } from "./core/router.js";
import { renderApplyModal } from "./features/tweaks/applyModal.js";
import { icon } from "./ui/icons.js";
import { renderMain, renderShell, renderPageBody, renderSidebarUpdateWidget } from "./ui/layout.js";


router.register("color", colorFeature);
router.register("gameColor", colorFeature);
router.register("tweaks", tweaksFeature);
router.register("characteristics", characteristicsFeature);
router.register("backups", backupsFeature);
router.register("configs", backupsFeature);
router.register("settings", settingsFeature);

let applyTimer = 0;
let carouselStepAt = 0;
let trimTimer = 0;
let impactBannerTimer = 0;

function clearImpactBannerTimer() {
  if (impactBannerTimer) {
    window.clearTimeout(impactBannerTimer);
    impactBannerTimer = 0;
  }
}

function startImpactBannerTimer(delay = 3000) {
  if (viewState.activeImpactBanner?.isAdminPrompt) return;
  clearImpactBannerTimer();
  impactBannerTimer = window.setTimeout(() => {
    dismissNotificationBanner();
  }, delay);
}

function attachBannerHoverListeners() {
  const bannerEl = document.querySelector(".ios-banner");
  if (!bannerEl) return;
  bannerEl.onmouseenter = () => {
    clearImpactBannerTimer();
  };
  bannerEl.onmouseleave = () => {
    if (viewState.activeImpactBanner?.isAdminPrompt) return;
    clearImpactBannerTimer();
    impactBannerTimer = window.setTimeout(() => {
      dismissNotificationBanner();
    }, 600);
  };
}

function dismissNotificationBanner(immediate = false) {
  clearImpactBannerTimer();
  const banners = document.querySelectorAll(".ios-banner");
  if (!banners.length || immediate) {
    viewState.activeImpactBanner = null;
    renderNotificationBannerDom();
    return;
  }
  banners.forEach((banner) => {
    if (!banner.classList.contains("dismissing")) {
      banner.classList.add("dismissing");
    }
  });
  impactBannerTimer = window.setTimeout(() => {
    viewState.activeImpactBanner = null;
    impactBannerTimer = 0;
    renderNotificationBannerDom();
  }, 270);
}

function renderNotificationBannerDom() {
  const containers = document.querySelectorAll(".ios-banner-container");
  const html = renderIosNotification(viewState, t);
  if (!html) {
    containers.forEach((c) => c.remove());
    return;
  }
  if (!containers.length) {
    const parent = document.getElementById("app") || document.body;
    const temp = document.createElement("div");
    temp.innerHTML = html;
    if (temp.firstElementChild) {
      parent.appendChild(temp.firstElementChild);
    }
  } else {
    const existingBanner = containers[0].querySelector(".ios-banner");
    const isSameAdmin =
      existingBanner?.getAttribute("data-action") === "restart-as-admin" &&
      viewState.activeImpactBanner?.isAdminPrompt;

    if (isSameAdmin) {
      const appEl = existingBanner.querySelector(".ios-banner-app");
      const titleEl = existingBanner.querySelector(".ios-banner-title");
      const msgEl = existingBanner.querySelector(".ios-banner-message");
      if (appEl && viewState.activeImpactBanner?.appName) {
        appEl.textContent = viewState.activeImpactBanner.appName;
      }
      if (titleEl && viewState.activeImpactBanner?.title) {
        titleEl.textContent = viewState.activeImpactBanner.title;
      }
      if (msgEl && viewState.activeImpactBanner?.impactText) {
        msgEl.textContent = viewState.activeImpactBanner.impactText;
      }
    } else {
      containers[0].outerHTML = html;
    }
    for (let i = 1; i < containers.length; i++) {
      containers[i].remove();
    }
  }
  attachBannerHoverListeners();
}

function renderBackupModalDom() {
  let backdrop = document.querySelector(".backup-modal-backdrop");
  if (!viewState.showBackupNameModal) {
    if (backdrop) backdrop.remove();
    return;
  }
  const html = renderBackupNameModal(viewState, t);
  if (!backdrop) {
    const parent = document.getElementById("app") || document.body;
    const temp = document.createElement("div");
    temp.innerHTML = html;
    if (temp.firstElementChild) {
      parent.appendChild(temp.firstElementChild);
      const input = document.getElementById("custom-backup-name-input");
      if (input instanceof HTMLInputElement) {
        window.requestAnimationFrame(() => {
          input.focus();
          input.select();
        });
      }
    }
  } else {
    backdrop.outerHTML = html;
    const input = document.getElementById("custom-backup-name-input");
    if (input instanceof HTMLInputElement) {
      window.requestAnimationFrame(() => {
        input.focus();
      });
    }
  }
}

function renderApplyModalDom() {
  let backdrop = document.getElementById("apply-modal-backdrop");
  if (!viewState.applyModal) {
    if (backdrop) backdrop.remove();
    return;
  }
  const html = renderApplyModal(viewState.applyModal, t);
  if (!backdrop) {
    const parent = document.getElementById("app") || document.body;
    const temp = document.createElement("div");
    temp.innerHTML = html;
    if (temp.firstElementChild) {
      parent.appendChild(temp.firstElementChild);
    }
  } else {
    backdrop.outerHTML = html;
  }
}

function updateApplyProgress(progress, statusText) {
  if (!viewState.applyModal) return;
  viewState.applyModal.progress = progress;
  if (statusText) viewState.applyModal.statusText = statusText;
  const pctEl = document.querySelector(".apply-progress-pct");
  const fillEl = document.querySelector(".apply-progress-fill");
  const subEl = document.querySelector(".apply-modal-sub");
  const rounded = Math.min(100, Math.max(0, Math.round(progress)));
  if (fillEl instanceof HTMLElement) fillEl.style.width = `${rounded}%`;
  if (pctEl instanceof HTMLElement) pctEl.textContent = `${rounded}%`;
  if (subEl instanceof HTMLElement && statusText) subEl.textContent = statusText;
}

function showToastBanner(title, impactText, category = "safe") {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  viewState.activeImpactBanner = {
    title,
    category,
    appName: isRu ? "УВЕДОМЛЕНИЕ" : "NOTIFICATION",
    impactText
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(3000);
}

function showTweakImpactBanner(tweakId) {
  clearImpactBannerTimer();
  const tweaks = allTweaks();
  const tweak = tweaks.find((t) => t.id === tweakId);
  if (!tweak) return;

  const isRu = lang() === "ru";
  const details = getTweakImpactDetails(tweak, t, isRu);
  const title = tweakTitle(tweak, t);
  const category = tweakCategory(tweak);

  viewState.activeImpactBanner = {
    tweakId: tweak.id,
    title,
    category,
    appName: details.appName,
    impactText: details.impactText
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(3000);
}

function showRiskWarningBanner(tweak) {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  const title = tweakTitle(tweak, t);
  viewState.activeImpactBanner = {
    tweakId: tweak.id,
    title: isRu ? `Внимание: ${title}` : `Warning: ${title}`,
    category: "risk",
    appName: isRu ? "ПРЕДУПРЕЖДЕНИЕ" : "SYSTEM WARNING",
    impactText: isRu
      ? "Перед применением данного твика рекомендуется создать бэкап."
      : "It is recommended to create a backup before applying this tweak."
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(3000);
}

function showBackupNotificationBanner() {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  viewState.activeImpactBanner = {
    isBackupPrompt: true,
    category: "safe",
    appName: isRu ? "РЕЗЕРВНАЯ КОПИЯ" : "SYSTEM BACKUP",
    title: t("backupNotificationTitle") || (isRu ? "Создание бэкапа" : "Create Backup"),
    impactText: t("backupNotificationMsg") || (isRu ? "Нажмите здесь, чтобы задать имя бэкапа" : "Click here to set custom backup name")
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(3000);
}

function showAdminNotificationBanner() {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  viewState.activeImpactBanner = {
    isAdminPrompt: true,
    category: "admin",
    appName: isRu ? "ПРАВА АДМИНИСТРАТОРА" : "ADMINISTRATOR RIGHTS",
    title: t("adminNotificationTitle") || (isRu ? "Требуются права администратора" : "Administrator Rights Required"),
    impactText: t("adminNotificationMsg") || (isRu ? "Нажмите здесь для перезапуска Synchro с правами админа" : "Click here to restart Synchro as administrator")
  };
  renderNotificationBannerDom();
}

function highlightAdminBanner() {
  const container = document.querySelector(".ios-banner-container");
  if (container) {
    container.classList.remove("pulse-hint");
    void container.offsetWidth;
    container.classList.add("pulse-hint");
  } else {
    showAdminNotificationBanner();
  }
}

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
  const mountContext = {
    appState,
    viewState,
    showAdminBanner: showAdminNotificationBanner,
    loadTweakStatuses
  };
  if (title && subtitle && body) {
    title.textContent = t(page.title);
    subtitle.textContent = t(page.subtitle);
    body.className = `page-body page-${activePage}`;
    body.innerHTML = renderPageBody(viewState, t);
    router.mount(activePage, body, mountContext);
  } else {
    const main = document.querySelector(".main-panel");
    if (main) {
      main.outerHTML = renderMain(viewState, t);
      const nextBody = document.querySelector(".page-body");
      if (nextBody) router.mount(activePage, nextBody, mountContext);
    }
  }
  syncAllSliders(appState);
  updateNavState();
  renderBackupModalDom();
  renderApplyModalDom();
  if (activePage === "tweaks" && !appState.isAdmin) {
    showAdminNotificationBanner();
  }
  window.requestAnimationFrame(syncNavIndicator);
  scheduleTrimMemory(350);
}

let isLanguageSwitching = false;

async function switchLanguage(nextLang) {
  if (isLanguageSwitching || !nextLang || nextLang === appState.settings.language) {
    return;
  }
  isLanguageSwitching = true;

  const workspace = document.querySelector(".workspace") || document.querySelector(".app-shell");
  const scrollPanel = document.querySelector(".scroll-panel");
  const pageBody = document.querySelector(".page-body");
  const savedScrollTop = scrollPanel ? scrollPanel.scrollTop : (pageBody ? pageBody.scrollTop : 0);

  // Instantly toggle active class on segmented control button for immediate visual tactile feedback
  document.querySelectorAll("[data-language]").forEach((btn) => {
    btn.classList.toggle("active", btn.getAttribute("data-language") === nextLang);
  });

  // Start subtle soft dip animation (fade + micro-blur)
  if (workspace) {
    workspace.classList.remove("lang-fade-in");
    workspace.classList.add("lang-fade-out");
  }

  // Wait 100ms for smooth transition
  await new Promise((resolve) => window.setTimeout(resolve, 100));

  // Change language in state and root attribute
  appState.settings.language = nextLang;
  document.documentElement.lang = nextLang;

  // 1. Update Titlebar tooltips
  const minBtn = document.querySelector('.window-btn[data-action="window-minimize"]');
  if (minBtn) minBtn.title = t("minimize");
  const closeBtn = document.querySelector('.window-btn[data-action="window-close"]');
  if (closeBtn) closeBtn.title = t("close");

  // 2. Update Sidebar labels in place (zero layout jump, indicator locked)
  const navLabel = document.querySelector(".nav-label");
  if (navLabel) navLabel.textContent = t("main");

  const toggleBtn = document.querySelector(".sidebar-toggle-btn");
  if (toggleBtn) {
    const isCollapsed = Boolean(viewState?.sidebarCollapsed);
    const toggleTitle = isCollapsed ? t("expandSidebar") : t("collapseSidebar");
    toggleBtn.title = toggleTitle;
    toggleBtn.setAttribute("aria-label", toggleTitle);
  }

  pageOrder.forEach((key) => {
    const page = pageDefs[key];
    const navItem = document.querySelector(`.nav-item[data-page="${key}"]`);
    if (navItem && page) {
      const label = t(page.nav);
      navItem.setAttribute("title", label);
      const span = navItem.querySelector("span");
      if (span) span.textContent = label;
    }
  });

  const exitBtn = document.querySelector(".exit-button");
  if (exitBtn) {
    exitBtn.setAttribute("title", t("exit"));
    const span = exitBtn.querySelector("span");
    if (span) span.textContent = t("exit");
  }
  updateSidebarUpdateWidgetDom();

  // 3. Update Page Header
  const page = pageDefs[activePage];
  const title = document.querySelector(".page-header h1");
  const subtitle = document.querySelector(".page-header p");
  if (title) title.textContent = t(page.title);
  if (subtitle) subtitle.textContent = t(page.subtitle);

  // 4. Update Page Body & restore scroll
  const body = document.querySelector(".page-body");
  if (body) {
    body.className = `page-body page-${activePage}`;
    body.innerHTML = renderPageBody(viewState, t);
    const newScrollPanel = document.querySelector(".scroll-panel");
    if (newScrollPanel) {
      newScrollPanel.scrollTop = savedScrollTop;
    } else {
      body.scrollTop = savedScrollTop;
    }
  }

  // 5. Sync sliders, modals, notification
  syncAllSliders(appState);
  syncAllTemplateSliders(appState);
  renderBackupModalDom();
  renderApplyModalDom();
  if (viewState.activeImpactBanner) {
    renderNotificationBannerDom();
  }
  syncNavIndicator();

  // 6. Complete transition: smooth fade-in
  if (workspace) {
    workspace.classList.remove("lang-fade-out");
    workspace.classList.add("lang-fade-in");
    window.setTimeout(() => {
      workspace.classList.remove("lang-fade-in");
      isLanguageSwitching = false;
    }, 220);
  } else {
    isLanguageSwitching = false;
  }

  // Save setting to backend
  await saveSettings();
}

function updateSidebarUpdateWidgetDom() {
  const container = document.getElementById("sidebar-update-wrapper");
  if (!container) return;
  const temp = document.createElement("div");
  temp.innerHTML = renderSidebarUpdateWidget(t, viewState);
  const newEl = temp.firstElementChild;
  if (newEl) {
    container.className = newEl.className;
    container.innerHTML = newEl.innerHTML;
  }
}

let updatePollTimer = 0;

async function checkForUpdates() {
  try {
    const res = await invokeCommand("check_for_updates");
    if (res && res.updateAvailable) {
      viewState.updateStatus = "available";
      viewState.latestVersion = res.latestVersion;
      viewState.downloadUrl = res.downloadUrl;
      viewState.assetSize = res.assetSize;
      updateSidebarUpdateWidgetDom();

      if (appState.settings.autoUpdate) {
        startUpdateDownload();
      }
    }
  } catch (err) {
    console.warn("Check for updates failed:", err);
  }
}

async function startUpdateDownload() {
  if (viewState.updateStatus === "downloading") return;
  viewState.updateStatus = "downloading";
  viewState.updatePercent = 0;
  updateSidebarUpdateWidgetDom();

  try {
    await invokeCommand("download_update", {
      url: viewState.downloadUrl || null,
      size: viewState.assetSize || null
    });

    if (updatePollTimer) window.clearInterval(updatePollTimer);
    updatePollTimer = window.setInterval(async () => {
      try {
        const progress = await invokeCommand("get_update_progress");
        if (!progress) return;

        viewState.updatePercent = progress.percent || 0;

        const pctEl = document.querySelector("#sidebar-update-wrapper .update-progress-pct");
        if (pctEl) pctEl.textContent = `${progress.percent}%`;
        const barFill = document.querySelector("#sidebar-update-wrapper .update-progress-bar-fill");
        if (barFill) barFill.style.width = `${progress.percent}%`;

        if (progress.status === "ready") {
          window.clearInterval(updatePollTimer);
          updatePollTimer = 0;
          viewState.updateStatus = "ready";
          viewState.updatePercent = 100;
          updateSidebarUpdateWidgetDom();
        } else if (progress.status === "error") {
          window.clearInterval(updatePollTimer);
          updatePollTimer = 0;
          viewState.updateStatus = "error";
          updateSidebarUpdateWidgetDom();
        }
      } catch (err) {
        console.warn("Error polling update progress:", err);
      }
    }, 100);
  } catch (err) {
    console.error("Start download failed:", err);
    viewState.updateStatus = "error";
    updateSidebarUpdateWidgetDom();
  }
}

async function applyUpdateInstall() {
  try {
    await invokeCommand("install_update");
  } catch (err) {
    console.error("Apply update failed:", err);
  }
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
  if (viewState.editingTemplate && viewState.editingTemplateColor) {
    syncAllTemplateSliders(viewState.editingTemplateColor);
  }
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
  const installed = Boolean(viewState.installedTweaks?.has(id));
  tile.classList.toggle("installed", installed);
  tile.classList.toggle("not-installed", !installed);
  tile.classList.remove("selected");
  tile.setAttribute("aria-pressed", installed ? "true" : "false");
  const box = tile.querySelector(".box");
  if (box) {
    box.classList.toggle("checked", installed);
    box.innerHTML = installed ? icon("check", "box-check") : "";
  }
}

async function applyColor() {
  const result = await invokeCommand("apply_color_settings", { color: appState.color });
  if (result) mergeState(result);
}

function scheduleApplyColor() {
  window.clearTimeout(applyTimer);
  applyTimer = window.setTimeout(applyColor, 50);
}

let lastBlackHoloSynced = null;
async function syncRustBlackHolo(enabled) {
  if (lastBlackHoloSynced === enabled) return;
  lastBlackHoloSynced = enabled;
  try {
    const res = await invokeCommand("set_rust_black_holo", { enabled });
    if (res?.success) {
      const isRu = lang() === "ru";
      if (enabled) {
        if (res.modified) {
          showToastBanner(
            t("blackHolo"),
            isRu
              ? 'Rust client.cfg: holosightcolour изменён на "2"'
              : 'Rust client.cfg: holosightcolour set to "2"',
            "safe"
          );
        } else {
          showToastBanner(
            t("blackHolo"),
            isRu
              ? 'Rust client.cfg: уже установлено значение "2"'
              : 'Rust client.cfg: holosightcolour is already "2"',
            "safe"
          );
        }
      } else {
        if (res.modified) {
          const orig = res.currentValue || "0";
          showToastBanner(
            t("blackHolo"),
            isRu
              ? `Rust client.cfg: исходный цвет (${orig}) восстановлен`
              : `Rust client.cfg: restored original colour (${orig})`,
            "safe"
          );
        }
      }
      if (res.rustRunning) {
        setTimeout(() => {
          showToastBanner(
            t("blackHolo"),
            isRu
              ? "Rust запущен! Введите в консоли (F1): readcfg"
              : "Rust is running! In console (F1) type: readcfg",
            "safe"
          );
        }, 1800);
      }
    } else if (res?.message) {
      console.warn("Rust client.cfg:", res.message);
    }
  } catch (err) {
    console.error("Failed to sync Rust black holo cfg:", err);
  }
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
    if (viewState.selectedBackup && !backups.some((item) => item.id === viewState.selectedBackup)) {
      viewState.selectedBackup = "";
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
  viewState.colorGames = Array.isArray(games) ? games : [];
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

const MAX_CPU_HISTORY = 40;

function mergeCharacteristics(info) {
  if (!info) return false;
  viewState.system = { ...(viewState.system || {}), ...info };
  const cpu = Number.parseFloat(String(info.cpuUsagePercent ?? "").replace(",", "."));
  if (Number.isFinite(cpu)) {
    viewState.cpuHistory.push(Math.max(0, Math.min(100, cpu)));
    if (viewState.cpuHistory.length > MAX_CPU_HISTORY) {
      viewState.cpuHistory.splice(0, viewState.cpuHistory.length - MAX_CPU_HISTORY);
    }
  }
  return true;
}

async function triggerScanDrivers() {
  if (viewState.driversLoading) return;
  viewState.driversLoading = true;
  updateMain();
  try {
    const list = await invokeCommand("scan_drivers");
    if (Array.isArray(list) && list.length) {
      viewState.drivers = list;
    }
  } catch (err) {
    console.error("Failed to scan drivers:", err);
  } finally {
    viewState.driversLoading = false;
    updateMain();
  }
}

function updateDriversPageContent() {
  const pageBody = document.querySelector(".page-characteristics");
  if (!pageBody) return;
  const input = document.querySelector(".driver-search-input");
  const selStart = input instanceof HTMLInputElement ? input.selectionStart : null;
  const selEnd = input instanceof HTMLInputElement ? input.selectionEnd : null;
  const isFocused = input === document.activeElement;

  pageBody.innerHTML = renderPageBody(viewState, t);

  if (isFocused) {
    const newInput = document.querySelector(".driver-search-input");
    if (newInput instanceof HTMLInputElement) {
      newInput.focus();
      if (selStart !== null && selEnd !== null) {
        newInput.setSelectionRange(selStart, selEnd);
      }
    }
  }
}

async function refreshCharacteristics() {
  await triggerScanDrivers();
}


function applyPreset(name) {
  const custom = appState.settings?.templateOverrides?.[name];
  const preset = custom || defaultPresets[name];
  if (!preset) return;
  appState.color = { ...appState.color, ...preset };
  updateColorPage();
  scheduleApplyColor();
}

function applyDefaultPreset(name) {
  const preset = defaultPresets[name];
  if (!preset) return;
  appState.color = { ...appState.color, ...preset };
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
  if (action === "restart-as-admin") {
    try {
      localStorage.setItem("synchro_pending_nav", "tweaks");
    } catch {}
    const banner = document.querySelector(".ios-banner");
    if (banner) banner.style.pointerEvents = "none";
    try {
      return await invokeCommand("restart_as_admin");
    } catch (err) {
      console.error("restart_as_admin failed or cancelled:", err);
      if (banner) banner.style.pointerEvents = "auto";
      if (!appState.isAdmin && activePage === "tweaks") {
        showAdminNotificationBanner();
      }
      return null;
    }
  }
  if (action === "apply-color") return applyColor();
  if (action === "toggle-sidebar") {
    viewState.sidebarCollapsed = !viewState.sidebarCollapsed;
    try {
      localStorage.setItem("synchro_sidebar_collapsed", String(viewState.sidebarCollapsed));
    } catch {}
    const ws = document.querySelector(".workspace");
    if (ws) {
      ws.classList.toggle("sidebar-collapsed", viewState.sidebarCollapsed);
      const toggleBtn = document.querySelector(".sidebar-toggle-btn");
      if (toggleBtn) {
        const title = viewState.sidebarCollapsed ? t("expandSidebar") : t("collapseSidebar");
        toggleBtn.setAttribute("title", title);
        toggleBtn.setAttribute("aria-label", title);
      }
      syncNavIndicator();
      setTimeout(syncNavIndicator, 140);
      setTimeout(syncNavIndicator, 280);
    } else {
      render();
    }
    return null;
  }
  if (action === "close-color-game-panel") {
    viewState.colorGamePanelOpen = false;
    updateColorPage({ stableDrawer: true });
    return null;
  }
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
  if (action === "close-template-config") {
    if (viewState.editingTemplate) {
      if (!appState.settings.templateOverrides) appState.settings.templateOverrides = {};
      const presetKey = viewState.editingTemplate;
      const nameInput = document.getElementById("template-name-input");
      const inputName = nameInput instanceof HTMLInputElement ? nameInput.value.trim() : "";
      const customName = inputName || (typeof viewState.editingTemplateName === "string" ? viewState.editingTemplateName.trim() : "");
      const defaultPreset = defaultPresets[presetKey] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
      const currentOverride = appState.settings.templateOverrides[presetKey] || defaultPreset;
      const colors = {
        saturation: Number(viewState.editingTemplateColor?.saturation ?? currentOverride.saturation),
        hue: Number(viewState.editingTemplateColor?.hue ?? currentOverride.hue),
        contrast: Number(viewState.editingTemplateColor?.contrast ?? currentOverride.contrast),
        gamma: Number(viewState.editingTemplateColor?.gamma ?? currentOverride.gamma)
      };
      appState.settings.templateOverrides[presetKey] = {
        name: customName || appState.settings.templateOverrides[presetKey]?.name || undefined,
        saturation: colors.saturation,
        hue: colors.hue,
        contrast: colors.contrast,
        gamma: colors.gamma,
        enabled: true
      };
      saveSettings();
      viewState.editingTemplate = null;
      viewState.editingTemplateName = "";
      viewState.editingTemplateColor = null;
      updateColorPage();
    }
    return null;
  }
  if (action === "reset-template-preset") {
    const presetKey = viewState.editingTemplate;
    if (presetKey) {
      if (appState.settings?.templateOverrides) {
        delete appState.settings.templateOverrides[presetKey];
        saveSettings();
      }
      const presetLabels = {
        balanced: t("balanced"),
        vibrant: t("vibrant"),
        soft: t("soft"),
        night: t("night")
      };
      viewState.editingTemplateName = presetLabels[presetKey] || presetKey;
      const defaultPreset = defaultPresets[presetKey] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
      viewState.editingTemplateColor = { ...defaultPreset };
      updateColorPage();
      syncAllTemplateSliders(viewState.editingTemplateColor);
    }
    return null;
  }
  if (action === "dismiss-impact-banner") {
    if (viewState.activeImpactBanner?.isAdminPrompt) {
      return null;
    }
    dismissNotificationBanner();
    return null;
  }
  if (action === "dismiss-admin-prompt") {
    viewState.showAdminPrompt = false;
    viewState.dismissedAdminPrompt = true;
    return updateMain();
  }
  if (action === "restart-explorer") {
    await invokeCommand("restart_explorer");
    return null;
  }
  if (action === "dismiss-tweak-results") {
    viewState.tweakResults = [];
    return updateMain();
  }
  if (action === "dismiss-apply-modal") {
    viewState.applyModal = null;
    renderApplyModalDom();
    updateMain();
    return null;
  }
  if (action === "rollback-from-apply-modal") {
    if (!appState.isAdmin) {
      highlightAdminBanner();
      return null;
    }
    if (viewState.applyingTweaks) return null;
    viewState.applyingTweaks = true;
    viewState.applyModal = {
      phase: "rollingBack",
      progress: 0,
      statusText: t("rollingBackSubtitle"),
      results: []
    };
    renderApplyModalDom();

    let currentProgress = 0;
    let progressInterval = setInterval(() => {
      if (currentProgress < 85) {
        currentProgress += Math.max(1, (88 - currentProgress) * 0.12);
        updateApplyProgress(currentProgress);
      }
    }, 40);

    try {
      const restored = await invokeCommand("rollback_last_tweaks");
      clearInterval(progressInterval);

      await new Promise((resolve) => {
        let finishInterval = setInterval(() => {
          currentProgress += Math.max(3, (100 - currentProgress) * 0.4);
          if (currentProgress >= 99.5) {
            currentProgress = 100;
            updateApplyProgress(100, t("rollbackSuccess"));
            clearInterval(finishInterval);
            setTimeout(resolve, 450);
          } else {
            updateApplyProgress(currentProgress);
          }
        }, 20);
      });

      if (restored) {
        mergeState(restored);
        await Promise.all([loadTweakStatuses(), loadLists()]);
      }

      viewState.applyModal = null;
      renderApplyModalDom();
      updateMain();
      showToastBanner(t("rollbackSuccess"), t("rollbackSuccess"), "safe");
    } catch (err) {
      clearInterval(progressInterval);
      viewState.applyModal = null;
      renderApplyModalDom();
      updateMain();
    } finally {
      viewState.applyingTweaks = false;
    }
    return null;
  }
  if (action === "apply-tweaks") {
    if (!appState.isAdmin) {
      highlightAdminBanner();
      return null;
    }
    if (viewState.applyingTweaks) return null;
    const ids = Array.from(viewState.installedTweaks);
    if (!ids.length) {
      showToastBanner(t("noSelection"), t("noSelection"), "safe");
      return null;
    }
    viewState.applyingTweaks = true;
    viewState.applyModal = {
      phase: "loading",
      progress: 0,
      statusText: t("applyingTweaksSubtitle"),
      results: []
    };
    renderApplyModalDom();

    let currentProgress = 0;
    let progressInterval = setInterval(() => {
      if (currentProgress < 85) {
        currentProgress += Math.max(0.5, (88 - currentProgress) * 0.08);
        updateApplyProgress(currentProgress);
      }
    }, 45);

    try {
      const results = await invokeCommand("apply_tweaks", { ids });
      clearInterval(progressInterval);

      await new Promise((resolve) => {
        let finishInterval = setInterval(() => {
          currentProgress += Math.max(2, (100 - currentProgress) * 0.35);
          if (currentProgress >= 99.5) {
            currentProgress = 100;
            updateApplyProgress(100, t("ready"));
            clearInterval(finishInterval);
            setTimeout(resolve, 360);
          } else {
            updateApplyProgress(currentProgress);
          }
        }, 25);
      });

      viewState.applyModal = {
        phase: "complete",
        progress: 100,
        results: Array.isArray(results) ? results : []
      };
      renderApplyModalDom();

      if (results?.some((r) => r.status === "requiresAdmin") && !appState.isAdmin) {
        viewState.showAdminPrompt = true;
      }
      await Promise.all([loadTweakStatuses(), loadLists()]);
    } catch (err) {
      clearInterval(progressInterval);
      viewState.applyModal = {
        phase: "complete",
        progress: 100,
        results: [{ id: "error", status: "failed", message: String(err) }]
      };
      renderApplyModalDom();
    } finally {
      viewState.applyingTweaks = false;
    }
    return null;
  }
  if (action === "rollback-tweaks") {
    if (!appState.isAdmin) {
      highlightAdminBanner();
      return null;
    }
    return handleAction("rollback-from-apply-modal");
  }
  if (action === "reset-color") {
    appState.color = cloneState(defaultState).color;
    updateMain();
    return applyColor();
  }
  if (action === "refresh-characteristics" || action === "scan-drivers") return triggerScanDrivers();
  if (action === "open-windows-update-drivers") {
    try {
      await invokeCommand("open_windows_driver_updates");
    } catch (e) {
      console.error(e);
    }
    return null;
  }
  if (action === "toggle-all-drivers") {
    viewState.showAllDrivers = !viewState.showAllDrivers;
    return updateMain();
  }
  if (action === "create-backup") {
    showBackupNotificationBanner();
    return null;
  }
  if (action === "open-backup-name-modal") {
    clearImpactBannerTimer();
    viewState.activeImpactBanner = null;
    renderNotificationBannerDom();
    viewState.showBackupNameModal = true;
    updateMain();
    return null;
  }
  if (action === "confirm-create-backup") {
    const input = document.getElementById("custom-backup-name-input");
    const name = input?.value?.trim() || viewState.backupName?.trim() || "";
    viewState.showBackupNameModal = false;
    viewState.backupName = "";
    clearImpactBannerTimer();
    viewState.activeImpactBanner = null;
    renderNotificationBannerDom();
    updateMain();
    const backups = await invokeCommand("create_backup", { name });
    if (Array.isArray(backups)) {
      viewState.backups = backups;
      viewState.selectedBackup = backups[0]?.id || "";
      updateMain();
    }
    return null;
  }
  if (action === "dismiss-backup-modal") {
    viewState.showBackupNameModal = false;
    viewState.backupName = "";
    updateMain();
    return null;
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
      viewState.selectedBackup = "";
      updateMain();
    }
  }
  if (action === "open-backups-folder") return invokeCommand("open_storage_folder", { kind: "backups" });
  if (action === "save-config") {
    const configs = await invokeCommand("save_config", { name: viewState.configName });
    if (Array.isArray(configs)) {
      viewState.configs = configs;
      viewState.selectedConfig = configs[0]?.id || "";
      viewState.configName = "";
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

  const templateColorField = input.getAttribute("data-template-color-field");
  if (templateColorField && sliderDefs[templateColorField]) {
    if (!viewState.editingTemplateColor) {
      viewState.editingTemplateColor = { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
    }
    viewState.editingTemplateColor[templateColorField] = Number(input.value);
    updateTemplateSliderDom(templateColorField, viewState.editingTemplateColor);
    return;
  }

  const colorField = input.getAttribute("data-color-field");
  if (colorField && sliderDefs[colorField]) {
    appState.color[colorField] = Number(input.value);
    updateSliderDom(colorField, appState);
    scheduleApplyColor();
    return;
  }

  if (input.getAttribute("data-action") === "pick-accent-color") {
    const color = input.value;
    appState.settings.accentColor = color;
    applyInterfaceAccent(color);
    const pickerWrapper = document.querySelector(".accent-picker-wrapper");
    let matchesPreset = false;
    document.querySelectorAll(".accent-swatch").forEach((swatch) => {
      const match = swatch.getAttribute("data-color")?.toLowerCase() === color.toLowerCase();
      swatch.classList.toggle("active", match);
      if (match) matchesPreset = true;
    });
    if (pickerWrapper) {
      pickerWrapper.classList.toggle("active", !matchesPreset);
    }
    saveSettings();
    return;
  }

  if (input.getAttribute("data-action") === "input-driver-search") {
    viewState.driverSearch = input.value;
    updateDriversPageContent();
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

  const backdropDismiss = target.closest("[data-action='dismiss-backup-modal']");
  if (backdropDismiss && !target.closest(".template-config-dialog") && !target.closest(".backup-minimal-dialog")) {
    await handleAction("dismiss-backup-modal");
    return;
  }

  const applyBackdropDismiss = target.closest("[data-action='dismiss-apply-modal']");
  if (applyBackdropDismiss && !target.closest(".apply-modal-complete")) {
    await handleAction("dismiss-apply-modal");
    return;
  }

  if (target.closest(".template-config-dialog")) {
    const dialogAction = target.closest("[data-action]");
    if (dialogAction && dialogAction.closest(".template-config-dialog")) {
      const actionName = dialogAction.getAttribute("data-action");
      if (actionName) await handleAction(actionName);
    }
    return;
  }

  const navItem = target.closest("[data-page]");
  if (navItem) {
    const nextPage = navItem.getAttribute("data-page");
    if (!nextPage || !pageDefs[nextPage] || nextPage === activePage) return;
    if (nextPage !== "tweaks" && viewState.activeImpactBanner?.isAdminPrompt) {
      dismissNotificationBanner(true);
    }
    router.navigate(nextPage, {
      onNavigate: () => {
        updateMain();
        syncCharacteristicsMonitor();
      },
      appState,
      viewState,
      showAdminBanner: showAdminNotificationBanner,
      loadTweakStatuses
    });
    if (nextPage === "gameColor" && !viewState.colorGamesLoaded) {
      loadColorGames();
    } else if (nextPage === "tweaks") {
      if (!appState.isAdmin) {
        showAdminNotificationBanner();
      }
      if (!viewState.detectedApps.length) {
        invokeCommand("detect_installed_apps").then((apps) => {
          if (Array.isArray(apps)) {
            viewState.detectedApps = apps;
            if (activePage === "tweaks" && viewState.smartTipsOpen) updateMain();
          }
        });
      }
      if (viewState.installedTweaks.size === 0) {
        loadTweakStatuses().then(updateMain);
      }
    } else if (nextPage === "characteristics") {
      if (!viewState.drivers || !viewState.drivers.length) {
        triggerScanDrivers();
      }
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
    const nextVal = !appState.settings[key];
    appState.settings[key] = nextVal;

    setting.classList.toggle("active", nextVal);
    setting.setAttribute("aria-pressed", nextVal ? "true" : "false");
    setting.setAttribute("aria-checked", nextVal ? "true" : "false");
    const sw = setting.querySelector(".ios-switch");
    if (sw) sw.classList.toggle("active", nextVal);

    saveSettings().then(() => {
      syncCharacteristicsMonitor();
    });

    if (key === "applyInstantly" && appState.settings.applyInstantly) await applyColor();
    if (key === "showOnRecordings") await applyColor();
    return;
  }

  const language = target.closest("[data-language]");
  if (language) {
    const nextLang = language.getAttribute("data-language");
    if (nextLang && nextLang !== appState.settings.language) {
      await switchLanguage(nextLang);
    }
    return;
  }

  const accentSwatch = target.closest("[data-action='set-accent-color']");
  if (accentSwatch) {
    const color = accentSwatch.getAttribute("data-color");
    if (color) {
      appState.settings.accentColor = color;
      applyInterfaceAccent(color);
      document.querySelectorAll(".accent-swatch").forEach((swatch) => {
        swatch.classList.toggle("active", swatch.getAttribute("data-color")?.toLowerCase() === color.toLowerCase());
      });
      const pickerWrapper = document.querySelector(".accent-picker-wrapper");
      if (pickerWrapper) pickerWrapper.classList.remove("active");
      const colorInput = document.querySelector(".accent-color-input");
      if (colorInput instanceof HTMLInputElement) colorInput.value = color;
      await saveSettings();
    }
    return;
  }

  const previewModeBtn = target.closest("[data-action='set-color-preview-mode']");
  if (previewModeBtn) {
    const nextMode = previewModeBtn.getAttribute("data-mode") || "day";
    viewState.colorPreviewMode = nextMode;
    const img = document.getElementById("color-preview-image");
    if (img instanceof HTMLImageElement) {
      img.src = nextMode === "night" ? "./assets/night.png" : (nextMode === "holo" ? "./assets/blackholo.png" : "./assets/preview.png");
    }
    document.querySelectorAll(".preview-mode-btn").forEach((btn) => {
      btn.classList.toggle("active", btn.getAttribute("data-mode") === nextMode);
    });
    return;
  }

  const toggleBlackHolo = target.closest("[data-action='toggle-black-holo']");
  if (toggleBlackHolo) {
    const isCurrentlyActive = Number(appState.color.blackHolo || 0) > 0;
    const nextVal = isCurrentlyActive ? 0 : 100;
    appState.color.blackHolo = nextVal;
    updateBlackHoloDom(nextVal > 0);
    if (nextVal > 0) {
      viewState.colorPreviewMode = "holo";
      const img = document.getElementById("color-preview-image");
      if (img instanceof HTMLImageElement) {
        img.src = "./assets/blackholo.png";
      }
      document.querySelectorAll(".preview-mode-btn").forEach((btn) => {
        btn.classList.toggle("active", btn.getAttribute("data-mode") === "holo");
      });
    } else if (viewState.colorPreviewMode === "holo") {
      viewState.colorPreviewMode = "day";
      const img = document.getElementById("color-preview-image");
      if (img instanceof HTMLImageElement) {
        img.src = "./assets/preview.png";
      }
      document.querySelectorAll(".preview-mode-btn").forEach((btn) => {
        btn.classList.toggle("active", btn.getAttribute("data-mode") === "day");
      });
    }
    scheduleApplyColor();
    syncRustBlackHolo(nextVal > 0);
    return;
  }

  const screenshotBtn = target.closest("[data-action='take-juicy-screenshot']");
  if (screenshotBtn) {
    try {
      screenshotBtn.classList.add("taking");
      const res = await invokeCommand("take_juicy_screenshot");
      setTimeout(() => screenshotBtn.classList.remove("taking"), 600);
      if (res?.success) {
        showToastBanner(
          t("screenshot"),
          lang() === "ru"
            ? "Сочный скриншот скопирован в буфер обмена (Ctrl+V)!"
            : "Juicy screenshot copied to clipboard (Ctrl+V)!",
          "safe"
        );
      }
    } catch (err) {
      console.error("Screenshot error:", err);
    }
    return;
  }

  const updateDownloadBtn = target.closest("[data-action='start-update-download']");
  if (updateDownloadBtn) {
    startUpdateDownload();
    return;
  }

  const updateInstallBtn = target.closest("[data-action='apply-update-install']");
  if (updateInstallBtn) {
    applyUpdateInstall();
    return;
  }

  const manualDownloadBtn = target.closest("[data-action='open-manual-download']");
  if (manualDownloadBtn) {
    invokeCommand("open_external_url", { url: "https://github.com/Arean-Max/synchro-nova/releases" });
    return;
  }

  const scopeApplyBtn = target.closest(".scope-apply-btn");
  if (scopeApplyBtn) {
    scopeApplyBtn.classList.add("applied");
    window.setTimeout(() => scopeApplyBtn.classList.remove("applied"), 1200);
  }

  const configureTemplate = target.closest("[data-action='configure-template']");
  if (configureTemplate) {
    event.preventDefault();
    event.stopPropagation();
    const presetKey = configureTemplate.getAttribute("data-preset");
    if (presetKey) {
      viewState.editingTemplate = presetKey;
      const presetLabels = {
        balanced: t("balanced"),
        vibrant: t("vibrant"),
        soft: t("soft"),
        night: t("night")
      };
      const custom = appState.settings?.templateOverrides?.[presetKey];
      const defaultPreset = defaultPresets[presetKey] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
      let templateDisplayName = custom?.name || presetLabels[presetKey] || presetKey;
      if (templateDisplayName && standardTemplateNames.has(templateDisplayName)) {
        templateDisplayName = presetLabels[presetKey] || presetKey;
      }
      viewState.editingTemplateName = templateDisplayName;
      viewState.editingTemplateColor = {
        saturation: custom?.saturation ?? defaultPreset.saturation,
        hue: custom?.hue ?? defaultPreset.hue,
        contrast: custom?.contrast ?? defaultPreset.contrast,
        gamma: custom?.gamma ?? defaultPreset.gamma
      };
      updateColorPage();
      syncAllTemplateSliders(viewState.editingTemplateColor);
    }
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

  const tweakHelp = target.closest("[data-action='show-tweak-impact']");
  if (tweakHelp) {
    event.preventDefault();
    event.stopPropagation();
    const id = tweakHelp.getAttribute("data-tweak-id");
    if (id) showTweakImpactBanner(id);
    return;
  }

  const openBackupModal = target.closest("[data-action='open-backup-name-modal']");
  if (openBackupModal) {
    event.preventDefault();
    await handleAction("open-backup-name-modal");
    return;
  }

  const impactBanner = target.closest("[data-action='dismiss-impact-banner']");
  if (impactBanner) {
    event.preventDefault();
    dismissNotificationBanner();
    return;
  }

  if (!appState.isAdmin && (activePage === "tweaks" || target.closest(".tweaks-page"))) {
    if (target.closest("[data-tweak-id]") || target.closest("[data-action='apply-tweaks']") || target.closest("[data-action='rollback-tweaks']") || target.closest(".tweaks-page.admin-locked")) {
      event.preventDefault();
      event.stopPropagation();
      highlightAdminBanner();
      return;
    }
  }

  const tweak = target.closest("[data-tweak-id]");
  if (tweak) {
    event.preventDefault();
    const id = tweak.getAttribute("data-tweak-id");
    if (!id) return;
    if (viewState.installedTweaks.has(id)) {
      viewState.installedTweaks.delete(id);
      viewState.selectedTweaks.delete(id);
    } else {
      viewState.installedTweaks.add(id);
      viewState.selectedTweaks.add(id);
      const tweakItem = allTweaks().find((item) => item.id === id);
      if (tweakItem && tweakCategory(tweakItem) === "risk") {
        showRiskWarningBanner(tweakItem);
      }
    }
    if (tweak instanceof HTMLElement) tweak.blur();
    syncTweakTile(id);
    return;
  }

  const setDriverFilter = target.closest("[data-action='set-driver-filter']");
  if (setDriverFilter) {
    event.preventDefault();
    const filter = setDriverFilter.getAttribute("data-filter") || "all";
    viewState.driverFilter = filter;
    updateMain();
    return;
  }

  const openDriverUrl = target.closest("[data-action='open-driver-url']");
  if (openDriverUrl) {
    event.preventDefault();
    event.stopPropagation();
    const url = openDriverUrl.getAttribute("data-url");
    if (url) {
      try {
        await invokeCommand("open_official_driver_url", { url });
      } catch (e) {
        console.error("Failed to open driver URL:", e);
      }
    }
    return;
  }

  const openExtUrl = target.closest("[data-action='open-external-url']");
  if (openExtUrl) {
    event.preventDefault();
    event.stopPropagation();
    const url = openExtUrl.getAttribute("data-url");
    if (url) {
      try {
        await invokeCommand("open_external_url", { url });
      } catch (e) {
        console.error("Failed to open external URL:", e);
      }
    }
    return;
  }

  const clearDriverSearch = target.closest("[data-action='clear-driver-search']");
  if (clearDriverSearch) {
    event.preventDefault();
    viewState.driverSearch = "";
    updateMain();
    return;
  }

  const scanDriversBtn = target.closest("[data-action='scan-drivers']");
  if (scanDriversBtn) {
    event.preventDefault();
    await triggerScanDrivers();
    return;
  }

  const winUpdateBtn = target.closest("[data-action='open-windows-update-drivers']");
  if (winUpdateBtn) {
    event.preventDefault();
    try {
      await invokeCommand("open_windows_driver_updates");
    } catch (e) {
      console.error("Failed to open Windows Update:", e);
    }
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
  if (viewState.applyModal && viewState.applyModal.phase === "complete") {
    if (event.key === "Escape" || event.key === "Enter") {
      event.preventDefault();
      handleAction("dismiss-apply-modal");
      return;
    }
  }
  if (viewState.editingTemplate) {
    if (event.key === "Escape") {
      event.preventDefault();
      handleAction("close-template-config");
      return;
    }
    if (event.key === "Enter" && event.target?.getAttribute?.("data-field") === "editingTemplateName") {
      event.preventDefault();
      handleAction("close-template-config");
      return;
    }
  }
  if (viewState.showBackupNameModal) {
    if (event.key === "Escape") {
      event.preventDefault();
      handleAction("dismiss-backup-modal");
      return;
    }
    if (event.key === "Enter") {
      event.preventDefault();
      handleAction("confirm-create-backup");
      return;
    }
  }
  if (viewState.activeImpactBanner && event.key === "Escape") {
    if (viewState.activeImpactBanner.isAdminPrompt) return;
    event.preventDefault();
    dismissNotificationBanner();
    return;
  }
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
  if (event.key === "Escape") {
    if (viewState.colorGamePanelOpen) {
      viewState.colorGamePanelOpen = false;
      event.preventDefault();
      updateColorPage();
    }
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
  if (image.classList.contains("color-preview-img")) return;
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
  applyInterfaceAccent(appState.settings.accentColor);
  app.className = "app-shell";
  app.innerHTML = renderShell(viewState, t);
  syncAllSliders(appState);
  updateNavState();
  renderBackupModalDom();
  renderApplyModalDom();
  if (activePage === "tweaks" && !appState.isAdmin) {
    showAdminNotificationBanner();
  }
  window.requestAnimationFrame(syncNavIndicator);
}

async function boot() {
  const state = await invokeCommand("get_app_state");
  if (state) mergeState(state);
  try {
    const savedCollapsed = localStorage.getItem("synchro_sidebar_collapsed");
    if (savedCollapsed !== null) {
      viewState.sidebarCollapsed = savedCollapsed === "true";
    }
  } catch {}
  try {
    const pendingNav = localStorage.getItem("synchro_pending_nav");
    if (pendingNav && pageDefs[pendingNav]) {
      localStorage.removeItem("synchro_pending_nav");
      setActivePage(pendingNav);
    }
  } catch {}
  await loadLists();
  render();
  loadColorGames();
  checkForUpdates();
  if (activePage === "tweaks") {
    if (!appState.isAdmin) {
      showAdminNotificationBanner();
    }
    loadTweakStatuses().then(updateMain);
  }
  if (activePage === "characteristics") {
    triggerScanDrivers();
  }
  scheduleTrimMemory(1200);
}

document.addEventListener("input", handleInput);
document.addEventListener("click", handleClick);
document.addEventListener("pointerdown", handlePointerDown);
document.addEventListener("error", handleImageError, true);
document.addEventListener("wheel", handleWheel, { passive: false });
document.addEventListener("keydown", handleKeydown);
window.addEventListener("resize", syncNavIndicator);
window.addEventListener("blur", () => scheduleTrimMemory(2000));
window.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    scheduleTrimMemory(100);
  }
});

let idleTrimTimer = 0;
function resetIdleTrimTimer() {
  window.clearTimeout(idleTrimTimer);
  idleTrimTimer = window.setTimeout(() => scheduleTrimMemory(100), 20000);
}
document.addEventListener("pointerdown", resetIdleTrimTimer, { passive: true });
document.addEventListener("keydown", resetIdleTrimTimer, { passive: true });
window.addEventListener("keydown", async (event) => {
  if (event.key === "PrintScreen" || (event.ctrlKey && event.shiftKey && event.key?.toLowerCase() === "s")) {
    try {
      const res = await invokeCommand("take_juicy_screenshot");
      if (res?.success) {
        showToastBanner(
          t("screenshot"),
          lang() === "ru"
            ? "Сочный скриншот скопирован в буфер обмена (Ctrl+V)!"
            : "Juicy screenshot copied to clipboard (Ctrl+V)!",
          "safe"
        );
      }
    } catch (err) {
      console.error("Screenshot hotkey failed:", err);
    }
  }
});

boot();
