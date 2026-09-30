import { invokeCommand } from "./core/bridge.js";
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
  formatSliderValue,
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
import { button } from "./ui/components.js";
import { renderMain, renderShell, renderPageBody, renderSidebarUpdateWidget } from "./ui/layout.js";
import { escapeHtml } from "./core/html.js";


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
  if (appState.isAdmin && viewState.activeImpactBanner?.isAdminPrompt) {
    viewState.activeImpactBanner = null;
  }
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

function copyTextToClipboard(text) {
  if (!text) return;
  if (navigator.clipboard?.writeText) {
    navigator.clipboard.writeText(text).catch(() => {
      fallbackCopyText(text);
    });
  } else {
    fallbackCopyText(text);
  }
}

function fallbackCopyText(text) {
  try {
    const ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.left = "-9999px";
    ta.style.top = "-9999px";
    ta.setAttribute("readonly", "");
    document.body.appendChild(ta);
    ta.focus();
    ta.select();
    document.execCommand("copy");
    ta.remove();
  } catch (e) {
    console.error("Fallback clipboard copy failed:", e);
  }
}

let pillToastTimer = null;
function showPillToast(message) {
  if (pillToastTimer) {
    clearTimeout(pillToastTimer);
    pillToastTimer = null;
  }
  let pill = document.querySelector(".clipboard-pill-toast");
  if (!pill) {
    pill = document.createElement("div");
    pill.className = "clipboard-pill-toast";
    document.body.appendChild(pill);
  }
  pill.innerHTML = `
    <span class="clipboard-pill-icon"><svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"></polyline></svg></span>
    <span class="clipboard-pill-text">${escapeHtml(message)}</span>
  `;
  pill.classList.remove("hiding");
  void pill.offsetWidth;
  pill.classList.add("visible");

  pillToastTimer = setTimeout(() => {
    pill.classList.add("hiding");
    setTimeout(() => {
      pill.classList.remove("visible", "hiding");
      pill.remove();
    }, 280);
  }, 2200);
}

function showErrorToast(title, err) {
  const isRu = lang() === "ru";
  const msg = (err && typeof err === "object" && err.message)
    ? err.message
    : (typeof err === "string" ? err : (err ? JSON.stringify(err) : "Unknown error"));
  showToastBanner(title || (isRu ? "Ошибка" : "Error"), msg, "danger");
}

function showToastBanner(title, impactText, category = "safe") {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  const defaultAppName = category === "danger"
    ? (isRu ? "ОШИБКА" : "ERROR")
    : (isRu ? "УВЕДОМЛЕНИЕ" : "NOTIFICATION");
  viewState.activeImpactBanner = {
    title,
    category,
    appName: defaultAppName,
    impactText: String(impactText || "")
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(category === "danger" ? 6000 : 3000);
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

function showTweakErrorBanner(tweakId, errorMsg) {
  clearImpactBannerTimer();
  const tweaks = allTweaks();
  const tweak = tweaks.find((t) => t.id === tweakId);
  const isRu = lang() === "ru";
  const title = tweak ? tweakTitle(tweak, t) : (tweakId || (isRu ? "Ошибка твика" : "Tweak Error"));
  const impactText = errorMsg || (isRu ? "Не удалось применить данный твик в системе." : "Failed to apply this tweak to the system.");

  viewState.activeImpactBanner = {
    tweakId,
    title: isRu ? `Сбой: ${title}` : `Failed: ${title}`,
    category: "danger",
    appName: isRu ? "ПОЧЕМУ НЕ ПОСТАВИЛСЯ" : "INSTALLATION ERROR",
    impactText
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(8000);
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
  if (appState.isAdmin) {
    viewState.activeImpactBanner = null;
    document.querySelectorAll(".ios-banner-container").forEach((c) => c.remove());
    return;
  }
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
  syncAllSliders(appState, viewState);
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

  document.querySelectorAll("[data-language]").forEach((btn) => {
    btn.classList.toggle("active", btn.getAttribute("data-language") === nextLang);
  });

  if (workspace) {
    workspace.classList.remove("lang-fade-in");
    workspace.classList.add("lang-fade-out");
  }

  await new Promise((resolve) => window.setTimeout(resolve, 100));

  appState.settings.language = nextLang;
  document.documentElement.lang = nextLang;

  const minBtn = document.querySelector('.window-btn[data-action="window-minimize"]');
  if (minBtn) minBtn.title = t("minimize");
  const closeBtn = document.querySelector('.window-btn[data-action="window-close"]');
  if (closeBtn) closeBtn.title = t("close");

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

  const page = pageDefs[activePage];
  const title = document.querySelector(".page-header h1");
  const subtitle = document.querySelector(".page-header p");
  if (title) title.textContent = t(page.title);
  if (subtitle) subtitle.textContent = t(page.subtitle);

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

  syncAllSliders(appState);
  syncAllTemplateSliders(appState);
  renderBackupModalDom();
  renderApplyModalDom();
  if (viewState.activeImpactBanner) {
    if (appState.isAdmin && viewState.activeImpactBanner.isAdminPrompt) {
      viewState.activeImpactBanner = null;
      document.querySelectorAll(".ios-banner-container").forEach((c) => c.remove());
    } else {
      renderNotificationBannerDom();
    }
  }
  syncNavIndicator();

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
  syncAllSliders(appState, viewState);
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

function syncTweaksActionButtons() {
  const actionsEl = document.querySelector(".tweaks-actions");
  if (!actionsEl) return;
  const hasSelected = viewState.selectedTweaks && viewState.selectedTweaks.size > 0;
  let applyBtn = actionsEl.querySelector("[data-action='apply-tweaks']");
  if (hasSelected) {
    if (!applyBtn) {
      const applyLabel = viewState.applyingTweaks ? t("loading") : t("applySelected");
      const btnHtml = button(applyLabel, "check", "primary", "apply-tweaks");
      actionsEl.insertAdjacentHTML("afterbegin", btnHtml);
    }
  } else {
    if (applyBtn) {
      applyBtn.remove();
    }
  }
}

function syncTweakTile(id) {
  const tile = Array.from(document.querySelectorAll("[data-tweak-id]")).find(
    (element) => element.getAttribute("data-tweak-id") === id && element.classList.contains("tweak-tile")
  );
  if (!tile) {
    updateMainKeepingScroll(".tweaks-page .scroll-panel");
    return;
  }
  const installed = Boolean(viewState.installedTweaks?.has(id));
  const selected = Boolean(viewState.selectedTweaks?.has(id));
  tile.classList.toggle("installed", installed);
  tile.classList.toggle("selected", selected);
  tile.classList.toggle("not-installed", !installed && !selected);
  tile.setAttribute("aria-pressed", (installed || selected) ? "true" : "false");
  syncTweaksActionButtons();
}

async function applyColor() {
  const result = await invokeCommand("apply_color_settings", { color: appState.color });
  if (result) mergeState(result);
}

function scheduleApplyColor() {
  window.clearTimeout(applyTimer);
  applyTimer = window.setTimeout(applyColor, 50);
}


async function loadBlackHoloStatus() {
  try {
    const status = await invokeCommand("get_black_holo_status");
    if (status) {
      viewState.blackHoloStatus = status;
      appState.color.blackHolo = status.active ? 100 : 0;
      updateBlackHoloDom(status.active, status);
    }
  } catch (err) {
    console.warn("Failed to load black holo status:", err);
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

function applyPreset(name) {
  const custom = appState.settings?.templateOverrides?.[name];
  const preset = custom || defaultPresets[name];
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
  switch (action) {
    case "window-minimize":
      return invokeCommand("minimize_window");
    case "window-maximize":
      return invokeCommand("toggle_window_maximize");
    case "window-close":
      return invokeCommand("close_window");
    case "exit-app":
      return invokeCommand("exit_app");

    case "restart-as-admin": {
      try {
        localStorage.setItem("synchro_pending_nav", "tweaks");
      } catch {}
      const banner = document.querySelector(".ios-banner");
      if (banner) banner.style.pointerEvents = "none";
      try {
        const res = await invokeCommand("restart_as_admin");
        if (res === null && !appState.isAdmin) {
          if (banner) banner.style.pointerEvents = "auto";
          if (activePage === "tweaks") {
            showAdminNotificationBanner();
          }
        }
        return res;
      } catch (err) {
        console.error("restart_as_admin failed or cancelled:", err);
        if (banner) banner.style.pointerEvents = "auto";
        if (!appState.isAdmin && activePage === "tweaks") {
          showAdminNotificationBanner();
        }
        return null;
      }
    }

    case "apply-color":
      return applyColor();

    case "toggle-sidebar": {
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

    case "close-color-game-panel":
      viewState.colorGamePanelOpen = false;
      updateColorPage({ stableDrawer: true });
      return null;

    case "launch-color-game": {
      const game = selectedColorGame(viewState);
      if (game?.id && game.launchable) return invokeCommand("launch_installed_game", { id: game.id });
      return null;
    }

    case "save-color-template": {
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

    case "close-template-config": {
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

    case "reset-template-preset": {
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

    case "click-impact-banner": {
      const banner = viewState.activeImpactBanner;
      if (banner) {
        const textToCopy = [banner.title, banner.impactText].filter(Boolean).join(": ");
        if (textToCopy) {
          copyTextToClipboard(textToCopy);
          showPillToast(lang() === "ru" ? "Скопировано в буфер обмена" : "Copied to clipboard");
        }
      }
      dismissNotificationBanner();
      return null;
    }

    case "dismiss-impact-banner":
      if (viewState.activeImpactBanner?.isAdminPrompt) return null;
      dismissNotificationBanner();
      return null;

    case "dismiss-admin-prompt":
      viewState.showAdminPrompt = false;
      viewState.dismissedAdminPrompt = true;
      return updateMain();

    case "restart-explorer":
      await invokeCommand("restart_explorer");
      return null;

    case "dismiss-tweak-results":
      viewState.tweakResults = [];
      return updateMain();

    case "dismiss-apply-modal":
      viewState.applyModal = null;
      renderApplyModalDom();
      updateMain();
      return null;

    case "rollback-from-apply-modal": {
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
      const progressInterval = setInterval(() => {
        if (currentProgress < 85) {
          currentProgress += Math.max(1, (88 - currentProgress) * 0.12);
          updateApplyProgress(currentProgress);
        }
      }, 40);

      try {
        const restored = await invokeCommand("rollback_last_tweaks");
        clearInterval(progressInterval);

        await new Promise((resolve) => {
          const finishInterval = setInterval(() => {
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

    case "apply-tweaks": {
      if (!appState.isAdmin) {
        highlightAdminBanner();
        return null;
      }
      if (viewState.applyingTweaks) return null;
      const ids = Array.from(viewState.selectedTweaks).filter(
        (id) => !viewState.installedTweaks.has(id) || id === "clean-temp-junk"
      );
      if (!ids.length) {
        const isRu = lang() === "ru";
        showToastBanner(
          t("noSelection"),
          viewState.selectedTweaks.size > 0
            ? (isRu ? "Все выбранные твики уже установлены в системе" : "All selected tweaks are already active in the system")
            : t("noSelection"),
          "safe"
        );
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
      const progressInterval = setInterval(() => {
        if (currentProgress < 85) {
          currentProgress += Math.max(0.5, (88 - currentProgress) * 0.08);
          updateApplyProgress(currentProgress);
        }
      }, 45);

      try {
        const results = await invokeCommand("apply_tweaks", { ids });
        clearInterval(progressInterval);

        await new Promise((resolve) => {
          const finishInterval = setInterval(() => {
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
        viewState.selectedTweaks.clear();
        syncTweaksActionButtons();
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

    case "rollback-tweaks": {
      if (!appState.isAdmin) {
        highlightAdminBanner();
        return null;
      }
      return handleAction("rollback-from-apply-modal");
    }

    case "reset-color":
      appState.color = cloneState(defaultState).color;
      updateMain();
      return applyColor();

    case "refresh-characteristics":
    case "scan-drivers":
      return triggerScanDrivers();

    case "open-windows-update-drivers":
      try {
        await invokeCommand("open_windows_driver_updates");
      } catch (e) {
        console.error(e);
      }
      return null;

    case "toggle-all-drivers":
      viewState.showAllDrivers = !viewState.showAllDrivers;
      return updateMain();

    case "create-backup":
      showBackupNotificationBanner();
      return null;

    case "open-backup-name-modal":
      clearImpactBannerTimer();
      viewState.activeImpactBanner = null;
      renderNotificationBannerDom();
      viewState.showBackupNameModal = true;
      updateMain();
      return null;

    case "confirm-create-backup": {
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

    case "dismiss-backup-modal":
      viewState.showBackupNameModal = false;
      viewState.backupName = "";
      updateMain();
      return null;

    case "restore-backup":
      if (viewState.selectedBackup) {
        const restored = await invokeCommand("restore_backup", { id: viewState.selectedBackup });
        if (restored) {
          mergeState(restored);
          render();
        }
      }
      return null;

    case "delete-backup":
      if (viewState.selectedBackup) {
        const backups = await invokeCommand("delete_backup", { id: viewState.selectedBackup });
        if (Array.isArray(backups)) {
          viewState.backups = backups;
          viewState.selectedBackup = "";
          updateMain();
        }
      }
      return null;

    case "open-backups-folder":
      return invokeCommand("open_storage_folder", { kind: "backups" });

    case "save-config": {
      const configs = await invokeCommand("save_config", { name: viewState.configName });
      if (Array.isArray(configs)) {
        viewState.configs = configs;
        viewState.selectedConfig = configs[0]?.id || "";
        viewState.configName = "";
        updateMain();
      }
      return null;
    }

    case "load-config":
      if (viewState.selectedConfig) {
        const loaded = await invokeCommand("load_config", { id: viewState.selectedConfig });
        if (loaded) {
          mergeState(loaded);
          render();
          scheduleApplyColor();
        }
      }
      return null;

    case "apply-config":
      if (viewState.selectedConfig) {
        const next = await invokeCommand("apply_config", { id: viewState.selectedConfig });
        if (next) {
          mergeState(next);
          render();
        }
      }
      return null;

    case "delete-config":
      if (viewState.selectedConfig) {
        const configs = await invokeCommand("delete_config", { id: viewState.selectedConfig });
        if (Array.isArray(configs)) {
          viewState.configs = configs;
          viewState.selectedConfig = configs[0]?.id || "";
          updateMain();
        }
      }
      return null;

    case "open-configs-folder":
      return invokeCommand("open_storage_folder", { kind: "configs" });

    default:
      return null;
  }
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

  const valBadge = target.closest("[data-slider-value]");
  if (valBadge && !valBadge.querySelector("input")) {
    const field = valBadge.getAttribute("data-slider-value");
    const def = sliderDefs[field];
    if (def) {
      event.preventDefault();
      event.stopPropagation();
      const curVal = Math.round(Number(appState.color[field] ?? 100));
      valBadge.innerHTML = `<input class="slider-inline-input" type="number" min="${def.min}" max="${def.max}" step="${def.step}" value="${curVal}">`;
      const numInput = valBadge.querySelector("input");
      if (numInput) {
        numInput.focus();
        numInput.select();
        let committed = false;
        const commit = () => {
          if (committed) return;
          committed = true;
          const raw = Number(numInput.value);
          const finalVal = Number.isFinite(raw) ? Math.min(def.max, Math.max(def.min, raw)) : curVal;
          appState.color[field] = finalVal;
          valBadge.textContent = formatSliderValue(field, finalVal);
          updateSliderDom(field, appState);
          scheduleApplyColor();
          saveSettings();
        };
        numInput.addEventListener("blur", commit, { once: true });
        numInput.addEventListener("keydown", (e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            commit();
          } else if (e.key === "Escape") {
            e.preventDefault();
            committed = true;
            valBadge.textContent = formatSliderValue(field, curVal);
          }
        });
        numInput.addEventListener("click", (e) => e.stopPropagation());
      }
      return;
    }
  }

  const tplValBadge = target.closest("[data-template-slider-value]");
  if (tplValBadge && !tplValBadge.querySelector("input")) {
    const field = tplValBadge.getAttribute("data-template-slider-value");
    const def = sliderDefs[field];
    if (def && viewState.editingTemplateColor) {
      event.preventDefault();
      event.stopPropagation();
      const curVal = Math.round(Number(viewState.editingTemplateColor[field] ?? 100));
      tplValBadge.innerHTML = `<input class="slider-inline-input" type="number" min="${def.min}" max="${def.max}" step="${def.step}" value="${curVal}">`;
      const numInput = tplValBadge.querySelector("input");
      if (numInput) {
        numInput.focus();
        numInput.select();
        let committed = false;
        const commit = () => {
          if (committed) return;
          committed = true;
          const raw = Number(numInput.value);
          const finalVal = Number.isFinite(raw) ? Math.min(def.max, Math.max(def.min, raw)) : curVal;
          viewState.editingTemplateColor[field] = finalVal;
          tplValBadge.textContent = formatSliderValue(field, finalVal);
          updateTemplateSliderDom(field, viewState.editingTemplateColor);
        };
        numInput.addEventListener("blur", commit, { once: true });
        numInput.addEventListener("keydown", (e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            commit();
          } else if (e.key === "Escape") {
            e.preventDefault();
            committed = true;
            tplValBadge.textContent = formatSliderValue(field, curVal);
          }
        });
        numInput.addEventListener("click", (e) => e.stopPropagation());
      }
      return;
    }
  }

  const impactBannerClose = target.closest(".ios-banner-close");
  if (impactBannerClose) {
    event.preventDefault();
    event.stopPropagation();
    dismissNotificationBanner();
    return;
  }

  const clickBanner = target.closest(".ios-banner");
  if (clickBanner) {
    event.preventDefault();
    event.stopPropagation();
    const banner = viewState.activeImpactBanner;
    if (banner?.isAdminPrompt) {
      await handleAction("restart-as-admin");
      return;
    }
    if (banner?.isBackupPrompt) {
      await handleAction("open-backup-name-modal");
      return;
    }
    if (banner) {
      const textToCopy = [banner.title, banner.impactText].filter(Boolean).join(": ");
      if (textToCopy) {
        copyTextToClipboard(textToCopy);
        showPillToast(lang() === "ru" ? "Скопировано в буфер обмена" : "Copied to clipboard");
      }
    }
    dismissNotificationBanner();
    return;
  }

  const backdropDismiss = target.closest("[data-action='dismiss-backup-modal']");
  if (backdropDismiss && !target.closest(".template-config-dialog") && !target.closest(".backup-minimal-dialog")) {
    await handleAction("dismiss-backup-modal");
    return;
  }

  if (target.id === "apply-modal-backdrop") {
    await handleAction("dismiss-apply-modal");
    return;
  }

  if (target.closest(".apply-modal-complete")) {
    const actionBtn = target.closest("[data-action]");
    if (actionBtn && actionBtn.closest(".apply-modal-complete")) {
      const actionName = actionBtn.getAttribute("data-action");
      if (actionName === "show-tweak-error") {
        event.preventDefault();
        event.stopPropagation();
        const id = actionBtn.getAttribute("data-tweak-id");
        const errorMsg = actionBtn.getAttribute("data-error-msg") || "";
        showTweakErrorBanner(id, errorMsg);
        return;
      }
      if (actionName === "show-tweak-impact") {
        event.preventDefault();
        event.stopPropagation();
        const id = actionBtn.getAttribute("data-tweak-id");
        if (id) showTweakImpactBanner(id);
        return;
      }
      if (actionName) await handleAction(actionName);
    }
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

    saveSettings();

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
    const isCurrentlyActive = Boolean(
      viewState.blackHoloStatus?.active ||
      toggleBlackHolo.classList.contains("active") ||
      Number(appState?.color?.blackHolo || 0) > 0
    );
    const nextActive = !isCurrentlyActive;
    const nextVal = nextActive ? 100 : 0;
    appState.color.blackHolo = nextVal;

    toggleBlackHolo.classList.toggle("active", nextActive);
    toggleBlackHolo.setAttribute("aria-checked", String(nextActive));
    toggleBlackHolo.setAttribute("aria-pressed", String(nextActive));
    const sw = toggleBlackHolo.querySelector(".ios-switch");
    if (sw) sw.classList.toggle("active", nextActive);

    if (nextActive && Math.abs(Number(appState?.color?.hue || 0)) > 0.1) {
      appState.color.hue = 0;
      updateSliderDom("hue", appState);
    }

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

    try {
      const status = await invokeCommand("toggle_hardware_black_holo", { enabled: nextActive });
      if (status) {
        viewState.blackHoloStatus = status;
        appState.color.blackHolo = status.active ? 100 : 0;
        updateBlackHoloDom(status.active, status);
        const isRu = lang() === "ru";
        if (status.error) {
          showToastBanner(t("blackHolo"), status.error, "danger");
        } else if (status.active) {
          showToastBanner(
            t("blackHolo"),
            isRu
              ? "Black Holosight включен. В консоли Rust (F1): accessibility.holosightcolour 2"
              : "Black Holosight enabled. In Rust F1 console: accessibility.holosightcolour 2",
            "safe"
          );
        } else {
          showToastBanner(
            t("blackHolo"),
            isRu ? "Black Holosight выключен" : "Black Holosight disabled",
            "safe"
          );
        }
      } else {
        updateBlackHoloDom(nextVal > 0, viewState.blackHoloStatus);
      }
    } catch (err) {
      console.error("Hardware black holo toggle error:", err);
      showErrorToast(t("blackHolo"), err);
      updateBlackHoloDom(nextVal > 0, viewState.blackHoloStatus);
    }

    scheduleApplyColor();
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
      } else if (res?.message) {
        showErrorToast(t("screenshot"), res.message);
      }
    } catch (err) {
      console.error("Screenshot error:", err);
      showErrorToast(t("screenshot"), err);
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

  const tweakError = target.closest("[data-action='show-tweak-error']");
  if (tweakError) {
    event.preventDefault();
    event.stopPropagation();
    const id = tweakError.getAttribute("data-tweak-id");
    const errorMsg = tweakError.getAttribute("data-error-msg") || "";
    showTweakErrorBanner(id, errorMsg);
    return;
  }

  const openBackupModal = target.closest("[data-action='open-backup-name-modal']");
  if (openBackupModal) {
    event.preventDefault();
    await handleAction("open-backup-name-modal");
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

  const tweak = target.closest(".tweak-tile[data-tweak-id]");
  if (tweak) {
    event.preventDefault();
    const id = tweak.getAttribute("data-tweak-id");
    if (!id) return;

    const isInstalled = viewState.installedTweaks.has(id);
    const isMaintenance = id === "clean-temp-junk";

    if (isInstalled && !isMaintenance) {
      const isRu = lang() === "ru";
      showPillToast(isRu ? "Этот твик уже активен в вашей системе" : "This tweak is already active in your system");
      return;
    }

    if (viewState.selectedTweaks.has(id)) {
      viewState.selectedTweaks.delete(id);
    } else {
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
        await invokeCommand("open_external_url", { url });
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
  syncAllSliders(appState, viewState);
  updateNavState();
  renderBackupModalDom();
  renderApplyModalDom();
  if (activePage === "tweaks" && !appState.isAdmin) {
    showAdminNotificationBanner();
  }
  window.requestAnimationFrame(syncNavIndicator);
}

async function boot() {
  const statePromise = invokeCommand("get_app_state");
  const holoPromise = loadBlackHoloStatus();
  try {
    const savedCollapsed = localStorage.getItem("synchro_sidebar_collapsed");
    if (savedCollapsed !== null) {
      viewState.sidebarCollapsed = savedCollapsed === "true";
    }
  } catch {}
  try {
    localStorage.removeItem("synchro_pending_nav");
  } catch {}
  setActivePage("color");

  const state = await statePromise;
  if (state) mergeState(state);
  render();

  Promise.all([holoPromise, loadLists(), loadColorGames()]).then(() => {
    updateMainKeepingScroll();
  });

  if (appState.settings && appState.settings.autoUpdate) {
    checkForUpdates();
  }
  loadTweakStatuses();
  scheduleTrimMemory(600);
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
    scheduleTrimMemory(150);
  }
});
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

if (window.__TAURI__?.event?.listen) {
  window.__TAURI__.event.listen("black-holo-toggled", (e) => {
    const status = e?.payload;
    if (!status) return;
    viewState.blackHoloStatus = status;
    const isAct = Boolean(status.active);
    appState.color.blackHolo = isAct ? 100 : 0;
    if (isAct && Math.abs(Number(appState?.color?.hue || 0)) > 0.1) {
      appState.color.hue = 0;
      updateSliderDom("hue", appState);
    }
    updateBlackHoloDom(isAct, status);
    const isRu = lang() === "ru";
    if (status.error) {
      showToastBanner(t("blackHolo"), status.error, "danger");
    } else if (isAct) {
      showToastBanner(
        t("blackHolo"),
        isRu
          ? "Black Holosight включен. В консоли Rust (F1): accessibility.holosightcolour 2"
          : "Black Holosight enabled. In Rust F1 console: accessibility.holosightcolour 2",
        "safe"
      );
    } else {
      showToastBanner(
        t("blackHolo"),
        isRu ? "Black Holosight выключен" : "Black Holosight disabled",
        "safe"
      );
    }
    scheduleApplyColor();
  });
}

window.addEventListener("error", (event) => {
  const msg = event.error?.message || event.message;
  if (msg) {
    showErrorToast(lang() === "ru" ? "Системная ошибка" : "System Error", msg);
  }
});

window.addEventListener("unhandledrejection", (event) => {
  const reason = event.reason;
  const msg = reason?.message || (typeof reason === "string" ? reason : "");
  if (msg && !msg.includes("canceled") && !msg.includes("aborted")) {
    showErrorToast(lang() === "ru" ? "Ошибка выполнения" : "Execution Error", msg);
  }
});

boot();
