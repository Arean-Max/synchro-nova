/*
   ,--,                                            
,---.'|                                            
|   | :                                            
:   : |                        ,--,                
|   ' :                      ,--.'|                
;   ; '                 .---.|  |,      .--.--.    
'   | |__   ,---.     /.  ./|`--'_     /  /    '   
|   | :.'| /       .-' . ' |,' ,'|   |  :  /`./   
'   :    ;/    /  |/___/ \: |'  | |   |  :  ;_     
|   |  ./.    ' / |.   \  ' .|  | :    \  \    `.  
;   : ;  '   ;   /| \   \   ''  : |__   `----.   \ 
|   ,/   '   |  / |  \   \   |  | '.'| /  /`--'  / 
'---'    |   :    |   \   \ |;  :    ;'--'.     /  
          \   \  /     '---" |  ,   /   `--'---'   
           `----'             ---`-'               
                                                   

author - vkmef.t,me
*/
import { invokeCommand } from "./core/bridge.js";
import {
  activePage,
  appState,
  applyInterfaceAccent,
  cloneState,
  defaultPresets,
  defaultState,
  filterPresets,
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
  renderColorPage,
  standardTemplateNames,
  formatSliderValue,
  syncAllSliders,
  syncAllTemplateSliders,
  updateBlackHoloDom,
  updateSliderDom,
  updateTemplateSliderDom,
  isTemplateActive,
  colorFeature
} from "./features/color/page.js";
import { allTweaks, tweakCategory, tweakTitle } from "./features/tweaks/catalog.js";
import { getTweakImpactDetails, renderIosNotification, tweaksFeature } from "./features/tweaks/page.js";
import { renderBackupNameModal, backupsFeature } from "./features/storage/page.js";
import { characteristicsFeature } from "./features/characteristics/page.js";
import { bindsFeature } from "./features/binds/page.js";
import { settingsFeature } from "./features/settings/page.js";
import { router } from "./core/router.js";
import { renderApplyModal } from "./features/tweaks/applyModal.js";
import { icon } from "./ui/icons.js";
import { button } from "./ui/components.js";
import { renderMain, renderShell, renderPageBody, renderSidebarUpdateWidget, renderNavList, resolveActiveSubSection } from "./ui/layout.js";
import { escapeHtml } from "./core/html.js";
import { runIntroSplash, showSplashChecking, showSplashDownloadCard, updateSplashDownloadCard, setSplashDownloading, finishIntroSplash, isSplashActive } from "./ui/splash.js";
import { applyTheme, resolveSeasonalTheme, themeFx } from "./ui/themes.js";
import { registerLifecycleHooks, withErrorBoundary } from "./core/lifecycle.js";
import { copyTextToClipboard } from "./core/clipboard.js";
import { formatKeyLabel, getWin32Vk } from "./features/binds/keyUtils.js";
let impactBannerTimer = 0;
let pillToastTimer = null;

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
      if (appEl) {
        appEl.textContent = "Synchro Notification";
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

function showToastBanner(title, impactText, category = "safe") {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  const defaultAppName = category === "danger"
    ? (t("notification_error") || (isRu ? "ОШИБКА" : "ERROR"))
    : (t("notification_info") || (isRu ? "УВЕДОМЛЕНИЕ" : "NOTIFICATION"));
  viewState.activeImpactBanner = {
    title,
    category,
    appName: defaultAppName,
    impactText: String(impactText || "")
  };
  renderNotificationBannerDom();
  startImpactBannerTimer(category === "danger" ? 6000 : 3000);
}

function showErrorToast(title, err) {
  const isRu = lang() === "ru";
  const msg = (err && typeof err === "object" && err.message)
    ? err.message
    : (typeof err === "string" ? err : (err ? JSON.stringify(err) : (t("unknownError") || "Unknown error")));
  showToastBanner(title || t("notification_error") || (isRu ? "Ошибка" : "Error"), msg, "danger");
}

function showSettingHelpBanner(key) {
  const titles = {
    macTrafficLights: t("macTrafficLights"),
    eacAutoProtect: t("eacAutoProtect"),
    autostartWindows: t("startWithWindows"),
    closeToTray: t("closeToTray"),
    zeroDaemonMode: t("zeroDaemonMode"),
    startMinimized: t("startMinimized"),
    autoBackupOnStart: t("autoBackupOnStart"),
    showOnRecordings: t("showOnRecordings"),
    autoUpdate: t("autoUpdate"),
    disableSplash: t("disableSplash"),
    disableAnimations: t("disableAnimations")
  };
  const title = titles[key] || t(key) || key;
  const helpText = t(`help_${key}`);
  showToastBanner(title, helpText, "safe");
}

function showTweakImpactBanner(tweakId) {
  clearImpactBannerTimer();
  const tweaks = allTweaks();
  const tweak = tweaks.find((item) => item.id === tweakId);
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
  const tweak = tweaks.find((item) => item.id === tweakId);
  const isRu = lang() === "ru";
  const title = tweak ? tweakTitle(tweak, t) : (tweakId || t("tweakErrorTitle") || (isRu ? "Ошибка твика" : "Tweak Error"));
  const impactText = errorMsg || t("tweakErrorMsg") || (isRu ? "Не удалось применить данный твик в системе." : "Failed to apply this tweak to the system.");

  viewState.activeImpactBanner = {
    tweakId,
    title: `${t("statusFailed") || (isRu ? "Сбой" : "Failed")}: ${title}`,
    category: "danger",
    appName: t("notification_error") || (isRu ? "СБОЙ ТВИКА" : "TWEAK ERROR"),
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
    title: `${t("warning") || (isRu ? "Внимание" : "Warning")}: ${title}`,
    category: "risk",
    appName: t("safetyBackup") || (isRu ? "СНИМОК СИСТЕМЫ" : "SAFETY BACKUP"),
    impactText: t("riskWarningMsg") || (isRu
      ? "Перед применением системных настроек рекомендуется создать резервную копию параметров Windows."
      : "Take a quick safety snapshot before applying system-level tweaks.")
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
    appName: t("safetySnapshot") || (isRu ? "СНИМОК СИСТЕМЫ" : "SAFETY SNAPSHOT"),
    title: t("backupNotificationTitle") || (isRu ? "Снимок Системы" : "Safety Snapshot"),
    impactText: t("backupNotificationMsg") || (isRu ? "Нажмите здесь, чтобы задать имя резервной копии" : "Click here to set custom snapshot name")
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
    appName: t("adminRequired") || (isRu ? "НУЖЕН АДМИН" : "ADMIN REQUIRED"),
    title: t("adminNotificationTitle") || (isRu ? "Требуются права администратора" : "Administrator Rights Required"),
    impactText: t("adminNotificationMsg") || (isRu ? "Нажмите для перезапуска Synchro с правами администратора" : "Click here to restart Synchro with admin rights")
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

router.register("color", colorFeature);
router.register("tweaks", tweaksFeature);
router.register("binds", bindsFeature);
router.register("characteristics", characteristicsFeature);
router.register("backups", backupsFeature);
router.register("settings", settingsFeature);

let applyTimer = 0;
let trimTimer = 0;
if (typeof window !== "undefined") {
  window.__synchroShowErrorToast = showErrorToast;
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

function scheduleTrimMemory(delay = 400) {
  window.clearTimeout(trimTimer);
  trimTimer = window.setTimeout(() => {
    invokeCommand("trim_memory");
  }, delay);
}



function syncKeybinds() {
  try {
    return invokeCommand("sync_keybinds", { keybinds: appState.keybinds || [] }).catch((err) => {
      console.error("Failed to sync keybinds:", err);
      return null;
    });
  } catch (err) {
    console.error("Failed to sync keybinds:", err);
    return Promise.resolve(null);
  }
}


function t(key) {
  return translate(lang(), key);
}

function syncNavIndicator() {
  const sidebar = document.querySelector(".sidebar");
  const activeItem = document.querySelector(".nav-item.active");
  if (!sidebar || !activeItem) return;
  const sidebarRect = sidebar.getBoundingClientRect();
  const itemRect = activeItem.getBoundingClientRect();
  const itemHeight = itemRect.height || 40;
  const indicatorHeight = Math.min(24, itemHeight);
  const y = (itemRect.top - sidebarRect.top) + (itemHeight - indicatorHeight) / 2;
  sidebar.style.setProperty("--nav-indicator-y", `${Math.round(y)}px`);
  sidebar.style.setProperty("--nav-indicator-height", `${indicatorHeight}px`);
}

function updateNavState() {
  const navList = document.querySelector(".nav-list");
  if (navList) {
    navList.innerHTML = renderNavList(t, viewState);
    navList.scrollTop = 0;
    navList.onscroll = syncNavIndicator;
  }
  syncNavIndicator();
}

let scrollSpyRafId = null;
function bindScrollSpy(scrollPanelEl) {
  if (!scrollPanelEl) return;
  scrollPanelEl.onscroll = () => {
    if (viewState.scrollLockUntil && Date.now() < viewState.scrollLockUntil) {
      return;
    }
    if (scrollSpyRafId) return;
    scrollSpyRafId = window.requestAnimationFrame(() => {
      scrollSpyRafId = null;
      if (!scrollPanelEl.isConnected) return;
      const sections = scrollPanelEl.querySelectorAll(".tweak-group, .driver-group, .settings-group");
      if (!sections.length) return;
      const currentId = resolveActiveSubSection(scrollPanelEl, sections);
      if (currentId && currentId !== viewState.activeSubSection) {
        viewState.activeSubSection = currentId;
        document.querySelectorAll(".nav-sub-item").forEach((btn) => {
          btn.classList.toggle("active", btn.getAttribute("data-section") === currentId);
        });
      }
    });
  };
}

function updateMain() {
  router.clearPageCache(activePage);
  const page = pageDefs[activePage];
  const title = document.querySelector(".page-header h1");
  const subtitle = document.querySelector(".page-header p");
  const body = document.querySelector(".page-body");
  const mountContext = {
    appState,
    viewState,
    showAdminBanner: showAdminNotificationBanner
  };
  if (title && subtitle && body) {
    title.textContent = t(page.title);
    subtitle.textContent = t(page.subtitle);
    body.className = `page-body page-${activePage}`;
    body.innerHTML = renderPageBody(viewState, t, true);
    router.mount(activePage, body, mountContext);
  } else {
    const main = document.querySelector(".main-panel");
    if (main) {
      main.outerHTML = renderMain(viewState, t, true);
      const nextBody = document.querySelector(".page-body");
      if (nextBody) router.mount(activePage, nextBody, mountContext);
    }
  }
  if (activePage === "color") {
    syncAllSliders(appState, viewState);
  }
  updateNavState();
  const scrollPanelEl = document.querySelector(".scroll-panel");
  if (scrollPanelEl) {
    bindScrollSpy(scrollPanelEl);
  }
  renderBackupModalDom();
  renderApplyModalDom();
  if (activePage === "tweaks" && !appState.isAdmin) {
    showAdminNotificationBanner();
  }
  window.requestAnimationFrame(syncNavIndicator);
}

let isLanguageSwitching = false;

async function switchLanguage(nextLang) {
  if (isLanguageSwitching || !nextLang || nextLang === appState.settings.language) {
    return;
  }
  isLanguageSwitching = true;
  viewState.languageDropdownOpen = false;

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
  document.documentElement.dir = nextLang === "ar" ? "rtl" : "ltr";

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
  updateNavState();

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
  if (viewState.editingTemplate && viewState.editingTemplateColor) {
    syncAllTemplateSliders(viewState.editingTemplateColor);
  }
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
  updateNavState();
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

function stopGlobalUpdatePoller() {
  if (updatePollTimer) {
    clearInterval(updatePollTimer);
    updatePollTimer = 0;
  }
}

function syncUpdateStatus(progress) {
  if (!progress) return;
  const oldStatus = viewState.updateStatus;
  const newStatus = progress.status;

  const latest = progress.latestVersion || progress.latest_version;
  if (latest) {
    viewState.latestVersion = latest;
  }
  const current = progress.currentVersion || progress.current_version;
  if (current) {
    viewState.appVersion = current;
  }

  if (newStatus === "downloading" || newStatus === "verifying") {
    viewState.updateStatus = "downloading";
    viewState.updatePercent = progress.percent || 0;

    const pctEl = document.querySelector("#sidebar-update-wrapper .update-progress-pct");
    if (pctEl) pctEl.textContent = `${progress.percent}%`;
    const barFill = document.querySelector("#sidebar-update-wrapper .update-progress-bar-fill");
    if (barFill) barFill.style.width = `${progress.percent}%`;

    if (oldStatus !== "downloading") {
      updateSidebarUpdateWidgetDom();
    }
  } else if (newStatus === "ready") {
    viewState.updateStatus = "ready";
    viewState.updatePercent = 100;
    stopGlobalUpdatePoller();
    if (oldStatus !== "ready") {
      updateSidebarUpdateWidgetDom();
    }
  } else if (newStatus === "available") {
    viewState.updateStatus = "available";
    if (oldStatus !== "available") {
      updateSidebarUpdateWidgetDom();
    }
  } else if (newStatus === "error") {
    viewState.updateStatus = "error";
    stopGlobalUpdatePoller();
    if (oldStatus !== "error") {
      updateSidebarUpdateWidgetDom();
    }
  } else if (newStatus === "idle") {
    stopGlobalUpdatePoller();
    if (oldStatus && oldStatus !== "hidden" && oldStatus !== "available" && oldStatus !== "ready") {
      viewState.updateStatus = "hidden";
      updateSidebarUpdateWidgetDom();
    }
  }
}

function startGlobalUpdatePoller() {
  if (updatePollTimer) return;
  updatePollTimer = window.setInterval(async () => {
    if (viewState.updateStatus !== "downloading" && viewState.updateStatus !== "installing") {
      stopGlobalUpdatePoller();
      return;
    }
    try {
      const progress = await invokeCommand("get_update_progress");
      syncUpdateStatus(progress);
    } catch (_) {}
  }, 1000);
}

async function checkForUpdates(manual = false) {
  if (manual) {
    viewState.updateCheckMessage = t("checkingUpdates");
    if (activePage === "settings") updateMain();
  }

  try {
    const res = await invokeCommand("check_for_updates");
    if (!res) {
      throw new Error("No response from update service");
    }

    viewState.lastUpdateCheck = new Date().toLocaleTimeString();
    if (res.currentVersion) {
      viewState.appVersion = res.currentVersion;
    }

    if (res.updateAvailable) {
      viewState.updateStatus = "available";
      viewState.latestVersion = res.latestVersion || "";
      viewState.downloadUrl = res.downloadUrl || "";
      viewState.assetSize = res.assetSize || 0;
      viewState.shaManifestUrl = res.shaManifestUrl || "";
      viewState.updateCheckMessage = "";
      updateSidebarUpdateWidgetDom();

      if (appState.settings && appState.settings.autoUpdate) {
        startUpdateDownload();
      }
    } else {
      if (viewState.updateStatus === "available") {
        viewState.updateStatus = "hidden";
        viewState.latestVersion = "";
        viewState.downloadUrl = "";
        updateSidebarUpdateWidgetDom();
      }
      if (manual) {
        viewState.updateCheckMessage = t("latestVersionInstalled");
      }
    }
  } catch (err) {
    console.warn("Check for updates failed:", err);
    if (manual) {
      viewState.updateCheckMessage = t("updateError");
    }
  }

  if (manual && activePage === "settings") {
    updateMain();
  }
}

async function startUpdateDownload() {
  if (viewState.updateStatus === "downloading") return;
  viewState.updateStatus = "downloading";
  viewState.updatePercent = 0;
  updateSidebarUpdateWidgetDom();
  startGlobalUpdatePoller();

  try {
    await invokeCommand("download_update", {
      url: viewState.downloadUrl || null,
      size: viewState.assetSize || null,
      version: viewState.latestVersion || null,
      manifestUrl: viewState.shaManifestUrl || null
    });
  } catch (err) {
    console.error("Start download failed:", err);
    viewState.updateStatus = "error";
    stopGlobalUpdatePoller();
    updateSidebarUpdateWidgetDom();
  }
}

async function applyUpdateInstall() {
  try {
    viewState.updateStatus = "installing";
    updateSidebarUpdateWidgetDom();
    startGlobalUpdatePoller();
    await invokeCommand("install_update", { noRelaunch: false });
  } catch (err) {
    console.error("Apply update failed:", err);
    viewState.updateStatus = "error";
    stopGlobalUpdatePoller();
    updateSidebarUpdateWidgetDom();
  }
}

async function handleStartupUpdateCheck() {
  // Initial State: minimal text, NO module card, NO taskbar progress
  showSplashChecking(t("checkingUpdates") || "Checking for updates...");

  let checkSkipped = false;

  try {
    const res = await invokeCommand("check_for_updates");
    if (checkSkipped) return;
    if (!res) {
      throw new Error("No response from update service");
    }

    if (res.currentVersion) {
      viewState.appVersion = res.currentVersion;
    }

    if (res.updateAvailable) {
      viewState.updateStatus = "available";
      viewState.latestVersion = res.latestVersion || "";
      viewState.downloadUrl = res.downloadUrl || "";
      viewState.assetSize = res.assetSize || 0;
      viewState.shaManifestUrl = res.shaManifestUrl || "";

      // Signal downloading: prevents auto-timeout and window closing
      setSplashDownloading(true);

      // Show elegant download card with separate Manual and Skip buttons
      showSplashDownloadCard({
        title: t("downloading") || "Downloading update...",
        pct: 0,
        onManual: () => {
          invokeCommand("open_external_url", {
            url: "https://github.com/Arean-Max/synchro-nova/releases/latest"
          });
        },
        onSkip: () => {
          checkSkipped = true;
          setSplashDownloading(false);
          invokeCommand("set_taskbar_progress", { progress: 0, isError: false }).catch(() => {});
          finishIntroSplash(0, true);
        }
      });

      // Taskbar progress starts ONLY when download begins
      await invokeCommand("set_taskbar_progress", { progress: 0, isError: false }).catch(() => {});

      await invokeCommand("download_update", {
        url: viewState.downloadUrl || null,
        size: viewState.assetSize || null,
        version: viewState.latestVersion || null,
        manifestUrl: viewState.shaManifestUrl || null
      });

      let installTriggered = false;

      const pollStartupProgress = async () => {
        if (checkSkipped) return;
        try {
          const progress = await invokeCommand("get_update_progress");
          if (!progress || checkSkipped) return;
          const pct = Math.max(0, Math.min(100, Number(progress.percent) || 0));
          const status = progress.status;

          if (status === "downloading" || status === "verifying") {
            updateSplashDownloadCard({
              title: t("downloading") || "Downloading update...",
              pct
            });
            await invokeCommand("set_taskbar_progress", { progress: pct, isError: false }).catch(() => {});
            setTimeout(pollStartupProgress, 100);
          } else if (status === "ready") {
            if (installTriggered || checkSkipped) return;
            installTriggered = true;

            updateSplashDownloadCard({
              title: t("installingRestarting") || t("installing") || "Installing update... Restarting",
              pct: 100,
              isInstalling: true
            });
            await invokeCommand("set_taskbar_progress", { progress: 100, isError: false }).catch(() => {});

            // Keep status on screen for 800ms so user clearly sees the install & restart status
            setTimeout(async () => {
              if (checkSkipped) return;
              try {
                await invokeCommand("install_update", { noRelaunch: false });
              } catch (err) {
                console.error("Install update failed:", err);
                updateSplashDownloadCard({
                  title: t("updateError") || "Download Failed",
                  isError: true
                });
                await invokeCommand("set_taskbar_progress", { progress: 0, isError: true }).catch(() => {});
                setSplashDownloading(false);
              }
            }, 800);
          } else if (status === "error") {
            updateSplashDownloadCard({
              title: t("updateError") || "Download Failed",
              isError: true
            });
            await invokeCommand("set_taskbar_progress", { progress: 0, isError: true }).catch(() => {});
            setSplashDownloading(false);
          }
        } catch (_) {
          if (!checkSkipped) {
            setTimeout(pollStartupProgress, 200);
          }
        }
      };

      setTimeout(pollStartupProgress, 100);
    } else {
      // Up to date: clean text feedback, NO taskbar progress
      showSplashChecking(t("statusUpToDate") || t("latestVersionInstalled") || "Latest version installed", true);
      setTimeout(() => {
        if (!checkSkipped) finishIntroSplash();
      }, 650);
    }
  } catch (err) {
    console.warn("Startup update check failed:", err);
    if (!checkSkipped) {
      finishIntroSplash(400);
    }
  }
}

function updateColorPage() {
  const activePage = router.activePageName();
  if (activePage !== "color") {
    updateMain();
    return;
  }
  const body = document.querySelector(".page-color");
  if (!body) {
    updateMain();
    return;
  }
  body.innerHTML = renderColorPage(appState, viewState, t);
  syncAllSliders(appState, viewState);
  if (viewState.editingTemplate && viewState.editingTemplateColor) {
    syncAllTemplateSliders(viewState.editingTemplateColor);
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
  const hasSelected = (viewState.selectedTweaks && viewState.selectedTweaks.size > 0) ||
    (viewState.pendingRevertTweaks && viewState.pendingRevertTweaks.size > 0);
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
  const isInstalled = Boolean(viewState.installedTweaks?.has(id));
  const isPendingRevert = Boolean(viewState.pendingRevertTweaks?.has(id));
  const installed = isInstalled && !isPendingRevert;
  const selected = Boolean(viewState.selectedTweaks?.has(id));
  tile.classList.toggle("installed", installed);
  tile.classList.toggle("pending-revert", isPendingRevert);
  tile.classList.toggle("selected", selected);
  tile.classList.toggle("not-installed", !installed && !selected && !isPendingRevert);
  tile.setAttribute("aria-pressed", (installed || selected || isPendingRevert) ? "true" : "false");
  syncTweaksActionButtons();
}

let isApplyingColor = false;
let pendingColorApply = false;

async function applyColor() {
  if (isApplyingColor) {
    pendingColorApply = true;
    return;
  }
  isApplyingColor = true;
  pendingColorApply = false;
  if (!appState.color?.activeFilter) {
    saveNormalColorState();
  }
  try {
    await invokeCommand("apply_color_settings", { color: appState.color });
  } catch (err) {
    console.error("Color apply error:", err);
  } finally {
    isApplyingColor = false;
    if (pendingColorApply) {
      pendingColorApply = false;
      scheduleApplyColor(16);
    }
  }
}

function scheduleApplyColor(delay = 40) {
  window.clearTimeout(applyTimer);
  applyTimer = window.setTimeout(applyColor, delay);
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
  const backups = await invokeCommand("list_backups");
  if (Array.isArray(backups)) {
    viewState.backups = backups;
    if (viewState.selectedBackup && !backups.some((item) => item.id === viewState.selectedBackup)) {
      viewState.selectedBackup = "";
    }
  }
}


function syncAllTweakTilesDom(installedSet) {
  const tiles = document.querySelectorAll(".tweak-tile[data-tweak-id]");
  tiles.forEach((tile) => {
    const id = tile.getAttribute("data-tweak-id");
    if (!id) return;
    const isInstalled = Boolean(installedSet?.has(id));
    const isPendingRevert = Boolean(viewState.pendingRevertTweaks?.has(id));
    const installed = isInstalled && !isPendingRevert;
    const selected = Boolean(viewState.selectedTweaks?.has(id));
    tile.classList.toggle("installed", installed);
    tile.classList.toggle("pending-revert", isPendingRevert);
    tile.classList.toggle("selected", selected);
    tile.classList.toggle("not-installed", !installed && !selected && !isPendingRevert);
    tile.setAttribute("aria-pressed", (installed || selected || isPendingRevert) ? "true" : "false");
  });
  syncTweaksActionButtons();
}

let isLoadingTweakStatuses = false;
let isRestartingAsAdmin = false;

async function loadTweakStatuses() {
  if (isLoadingTweakStatuses) return;
  isLoadingTweakStatuses = true;
  try {
    const statuses = await invokeCommand("get_tweak_statuses");
    if (!Array.isArray(statuses)) return;
    const nextInstalled = new Set();
    statuses.forEach((item) => {
      if (item?.installed && item.id) nextInstalled.add(item.id);
    });
    const prevInstalled = viewState.installedTweaks;
    let changed = false;
    if (!prevInstalled || prevInstalled.size !== nextInstalled.size) {
      changed = true;
    } else {
      for (const id of nextInstalled) {
        if (!prevInstalled.has(id)) {
          changed = true;
          break;
        }
      }
    }
    viewState.installedTweaks = nextInstalled;
    if (activePage === "tweaks" && changed) {
      syncAllTweakTilesDom(nextInstalled);
    }
  } catch (err) {
    console.warn("Failed to load tweak statuses:", err);
  } finally {
    isLoadingTweakStatuses = false;
  }
}

async function triggerScanDrivers() {
  if (viewState.driversLoading) return;
  viewState.driversLoading = true;
  if (activePage === "characteristics") {
    updateMain();
  }
  try {
    const list = await invokeCommand("scan_drivers");
    if (Array.isArray(list)) {
      viewState.drivers = list;
    }
  } catch (err) {
    console.error("Failed to scan drivers:", err);
  } finally {
    viewState.driversLoading = false;
    if (activePage === "characteristics") {
      updateMain();
    }
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

function saveNormalColorState() {
  if (!appState?.color || appState.color.activeFilter) return;
  const snapshot = {
    saturation: Number(appState.color.saturation ?? 100),
    hue: Number(appState.color.hue ?? 0),
    contrast: Number(appState.color.contrast ?? 100),
    gamma: Number(appState.color.gamma ?? 100)
  };
  try {
    localStorage.setItem("synchro_normal_color", JSON.stringify(snapshot));
  } catch (e) {}
}

function getSavedNormalColor() {
  try {
    const raw = localStorage.getItem("synchro_normal_color");
    if (raw) {
      const parsed = JSON.parse(raw);
      if (parsed && typeof parsed.saturation === "number") return parsed;
    }
  } catch (e) {}
  return { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
}

async function applyPreset(name) {
  if (isTemplateActive(name, appState)) {
    appState.color = {
      ...appState.color,
      saturation: 100,
      hue: 0,
      contrast: 100,
      gamma: 100,
      activeFilter: ""
    };
    saveNormalColorState();
    updateColorPage();
    await applyColor();
    return;
  }

  const custom = appState.settings?.templateOverrides?.[name];
  const preset = custom || defaultPresets[name];
  if (!preset) return;
  appState.color = {
    ...appState.color,
    saturation: preset.saturation,
    hue: preset.hue,
    contrast: preset.contrast,
    gamma: preset.gamma,
    activeFilter: ""
  };
  saveNormalColorState();
  updateColorPage();
  await applyColor();
}


let isTogglingBlackHolo = false;

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
      if (isRestartingAsAdmin) return null;
      isRestartingAsAdmin = true;
      try {
        localStorage.setItem("synchro_pending_nav", "tweaks");
      } catch {}
      const banner = document.querySelector(".ios-banner");
      const titleEl = banner?.querySelector(".ios-banner-title");
      const origTitle = titleEl ? titleEl.textContent : "";
      if (banner) banner.style.pointerEvents = "none";
      if (titleEl) {
        titleEl.textContent = lang() === "ru" ? "Запрос прав администратора..." : "Requesting administrator rights...";
      }
      try {
        const res = await invokeCommand("restart_as_admin");
        return res;
      } catch (err) {
        console.error("restart_as_admin failed or cancelled:", err);
        isRestartingAsAdmin = false;
        if (banner) banner.style.pointerEvents = "auto";
        if (titleEl && origTitle) titleEl.textContent = origTitle;
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
        updateNavState();
        window.requestAnimationFrame(syncNavIndicator);
      } else {
        render();
      }
      return null;
    }

    

    case "fix-anticheat": {
      try {
        const res = await invokeCommand("fix_anticheat_environment");
        if (res && res.success) {
          showToastBanner(
            t("eacFixedTitle"),
            res.message || t("eacFixedMsg"),
            "safe"
          );
        } else {
          showToastBanner(
            t("eacShieldTitle"),
            res?.message || t("unknownError"),
            "danger"
          );
        }
      } catch (err) {
        console.error("fix_anticheat_environment failed:", err);
        showErrorToast(t("eacShieldTitle"), err);
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
        const defaultPreset = defaultPresets[presetKey] || filterPresets[presetKey] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
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
        if (appState.color.activeFilter === presetKey) {
          appState.color.saturation = colors.saturation;
          appState.color.hue = colors.hue;
          appState.color.contrast = colors.contrast;
          appState.color.gamma = colors.gamma;
          scheduleApplyColor();
        }
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
          night: t("night"),
          rust_cold_tactical: t("rustColdTactical"),
          rust_midnight_neon: t("rustMidnightNeon"),
          clear_sight: t("clearSight")
        };
        viewState.editingTemplateName = presetLabels[presetKey] || presetKey;
        const defaultPreset = defaultPresets[presetKey] || filterPresets[presetKey] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
        viewState.editingTemplateColor = { ...defaultPreset };
        if (appState.color.activeFilter === presetKey) {
          appState.color.saturation = defaultPreset.saturation;
          appState.color.hue = defaultPreset.hue;
          appState.color.contrast = defaultPreset.contrast;
          appState.color.gamma = defaultPreset.gamma;
          scheduleApplyColor();
        }
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
          showPillToast(t("copiedToClipboard") || (lang() === "ru" ? "Скопировано в буфер обмена" : "Copied to clipboard"));
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
      if (activePage === "tweaks") {
        syncTweaksActionButtons();
      }
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

        if (!restored) {
          throw new Error("No backup available to restore");
        }

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

        mergeState(restored);
        await Promise.all([loadTweakStatuses(), loadLists()]);

        viewState.applyModal = null;
        renderApplyModalDom();
        updateMain();
        showToastBanner(t("rollbackSuccess"), t("rollbackSuccess"), "safe");
      } catch (err) {
        clearInterval(progressInterval);
        viewState.applyModal = null;
        renderApplyModalDom();
        updateMain();
        showToastBanner(
          t("rollbackFailed") || "Rollback failed",
          err?.message || (typeof err === "string" ? err : t("unknownError")),
          "danger"
        );
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
      const idsToApply = Array.from(viewState.selectedTweaks).filter(
        (id) => !viewState.installedTweaks.has(id) || id === "clean-temp-junk"
      );
      const idsToRevert = Array.from(viewState.pendingRevertTweaks || []);
      if (!idsToApply.length && !idsToRevert.length) {
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
      const progressInterval = setInterval(() => {
        if (currentProgress < 85) {
          currentProgress += Math.max(0.5, (88 - currentProgress) * 0.08);
          updateApplyProgress(currentProgress);
        }
      }, 45);

      try {
        const results = [];
        if (idsToRevert.length > 0) {
          const revertResults = await invokeCommand("revert_tweaks", { ids: idsToRevert });
          if (Array.isArray(revertResults)) {
            results.push(...revertResults);
          }
        }
        if (idsToApply.length > 0) {
          const applyResults = await invokeCommand("apply_tweaks", { ids: idsToApply });
          if (Array.isArray(applyResults)) {
            results.push(...applyResults);
          }
        }
        clearInterval(progressInterval);

        const hasApplied = results.some((r) => r.status === "applied" || r.status === "reverted" || r.status === "restored");
        const hasFailed = results.some((r) => r.status === "failed");
        const statusText = hasFailed
          ? (hasApplied ? (t("appliedWithErrors") || "Applied with errors") : (t("applyFailed") || "Failed"))
          : t("ready");

        await new Promise((resolve) => {
          const finishInterval = setInterval(() => {
            currentProgress += Math.max(2, (100 - currentProgress) * 0.35);
            if (currentProgress >= 99.5) {
              currentProgress = 100;
              updateApplyProgress(100, statusText);
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
          results
        };
        renderApplyModalDom();

        if (results?.some((r) => r.status === "requiresAdmin") && !appState.isAdmin) {
          viewState.showAdminPrompt = true;
        }
        viewState.selectedTweaks.clear();
        viewState.pendingRevertTweaks.clear();
        await Promise.all([loadTweakStatuses(), loadLists()]);
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
      const name = input?.value?.trim() || viewState.backupName?.trim() || input?.getAttribute("placeholder") || "";
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
        showToastBanner(t("backupNotificationTitle"), t("backupSuccess") || (lang() === "ru" ? "Снимок системы успешно создан" : "System snapshot created successfully"), "safe");
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
          await loadTweakStatuses();
          render();
          const isRu = lang() === "ru";
          showToastBanner(
            t("backupNotificationTitle") || (isRu ? "Резервная копия" : "Backup"),
            t("backupRestoredToast") || (isRu ? "Бэкап успешно применён, твики и сеть возвращены к стандарту" : "Backup restored, tweaks and network reset to defaults"),
            "safe"
          );
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

  const clearSearch = target.closest("[data-action='clear-driver-search']");
  if (clearSearch) {
    viewState.driverSearch = "";
    updateDriversPageContent();
    return;
  }

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
        showPillToast(t("copiedToClipboard") || (lang() === "ru" ? "Скопировано в буфер обмена" : "Copied to clipboard"));
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

  const scrollSectionBtn = target.closest('[data-action="scroll-to-section"]');
  if (scrollSectionBtn) {
    const sectionId = scrollSectionBtn.getAttribute("data-section");
    if (sectionId) {
      viewState.activeSubSection = sectionId;
      document.querySelectorAll(".nav-sub-item").forEach((btn) => {
        btn.classList.toggle("active", btn.getAttribute("data-section") === sectionId);
      });
      const targetEl = document.getElementById(sectionId);
      if (targetEl) {
        viewState.scrollLockUntil = Date.now() + 500;
        const scrollPanel = targetEl.closest(".scroll-panel") || document.querySelector(".scroll-panel");
        if (scrollPanel) {
          const panelRect = scrollPanel.getBoundingClientRect();
          const targetRect = targetEl.getBoundingClientRect();
          const targetTop = scrollPanel.scrollTop + (targetRect.top - panelRect.top);
          scrollPanel.scrollTo({
            top: Math.max(0, targetTop),
            behavior: appState.settings?.disableAnimations ? "auto" : "smooth"
          });
        }
      }
    }
    return;
  }

  const navItem = target.closest("[data-page]");
  if (navItem) {
    const nextPage = navItem.getAttribute("data-page");
    if (!nextPage || !pageDefs[nextPage]) return;
    if (nextPage === activePage) {
      const hasSubNav = nextPage === "tweaks" || nextPage === "characteristics";
      if (hasSubNav) {
        if (!viewState.collapsedSubNav) viewState.collapsedSubNav = {};
        viewState.collapsedSubNav[nextPage] = !viewState.collapsedSubNav[nextPage];
        updateNavState();
        return;
      }
      viewState.activeSubSection = null;
      const scrollPanel = document.querySelector(".scroll-panel") || document.querySelector(".page-body");
      if (scrollPanel) {
        scrollPanel.scrollTo({ top: 0, behavior: "smooth" });
      }
      return;
    }
    if (viewState.collapsedSubNav) {
      viewState.collapsedSubNav[nextPage] = false;
    }
    viewState.activeSubSection = null;
    viewState.scrollLockUntil = 0;
    viewState.languageDropdownOpen = false;
    viewState.openDropdownBindId = null;
    const ws = document.querySelector(".workspace");
    if (ws && ws.scrollTop > 0) ws.scrollTop = 0;
    const mp = document.querySelector(".main-panel");
    if (mp && mp.scrollTop > 0) mp.scrollTop = 0;
    const pb = document.querySelector(".page-body");
    if (pb && pb.scrollTop > 0) pb.scrollTop = 0;
    if (nextPage !== "tweaks" && viewState.activeImpactBanner) {
      dismissNotificationBanner(true);
    }
    router.navigate(nextPage, {
      onNavigate: () => {
        updateMain();
      },
      appState,
      viewState,
      showAdminBanner: showAdminNotificationBanner
    });
    if (nextPage === "tweaks") {
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
      loadTweakStatuses();
    } else if (nextPage === "characteristics") {
      if (!viewState.drivers || !viewState.drivers.length) {
        triggerScanDrivers();
      }
    } else if (nextPage === "backups" && !viewState.backups.length) {
      loadLists().then(() => {
        if (activePage === "backups") {
          updateMain();
        }
      });
    }
    return;
  }

  const helpBtn = target.closest("[data-action='setting-help']");
  if (helpBtn) {
    event.preventDefault();
    event.stopPropagation();
    const key = helpBtn.getAttribute("data-setting-key");
    if (key) showSettingHelpBanner(key);
    return;
  }

  const setting = target.closest("[data-setting]");
  if (setting) {
    const key = setting.getAttribute("data-setting");
    if (!key) return;
    if (!appState.settings) appState.settings = {};
    const nextVal = !Boolean(appState.settings[key]);
    appState.settings[key] = nextVal;

    setting.classList.toggle("active", nextVal);
    setting.setAttribute("aria-pressed", nextVal ? "true" : "false");
    setting.setAttribute("aria-checked", nextVal ? "true" : "false");
    const sw = setting.querySelector(".ios-switch");
    if (sw) sw.classList.toggle("active", nextVal);

    saveSettings();

    if (key === "applyInstantly" && appState.settings.applyInstantly) await applyColor();
    if (key === "showOnRecordings") await applyColor();
    if (key === "disableAnimations") {
      document.body.classList.toggle("no-animations", Boolean(appState.settings.disableAnimations));
      applyTheme(appState.settings?.theme || "default", { disableAnimations: appState.settings.disableAnimations });
    }
    if (key === "macTrafficLights") {
      render();
    }
    if (key === "showSidebarSubnav") {
      updateNavState();
    }
    if (key === "eacAutoProtect" && nextVal) {
      try {
        await invokeCommand("fix_anticheat_environment");
      } catch (err) {
        console.error("eac auto-protect fix failed:", err);
      }
    }
    return;
  }

  const toggleLanguageDropdown = target.closest("[data-action='toggle-language-dropdown']");
  if (toggleLanguageDropdown) {
    event.preventDefault();
    viewState.languageDropdownOpen = !viewState.languageDropdownOpen;
    viewState.fontDropdownOpen = false;
    updateMain();
    return;
  }

  const toggleFontDropdown = target.closest("[data-action='toggle-font-dropdown']");
  if (toggleFontDropdown) {
    event.preventDefault();
    viewState.fontDropdownOpen = !viewState.fontDropdownOpen;
    viewState.languageDropdownOpen = false;
    updateMain();
    return;
  }

  const language = target.closest("[data-action='select-language']") || target.closest("[data-language]");
  if (language) {
    event.preventDefault();
    viewState.languageDropdownOpen = false;
    const nextLang = language.getAttribute("data-language");
    if (nextLang && nextLang !== appState.settings.language) {
      await switchLanguage(nextLang);
    } else {
      updateMain();
    }
    return;
  }

  const accentSwatch = target.closest("[data-action='set-accent-color']");
  const themeCard = target.closest("[data-action='select-theme']");
  if (themeCard) {
    const nextTheme = themeCard.getAttribute("data-theme") || "default";
    appState.settings.theme = nextTheme;
    applyTheme(nextTheme, { updateAccent: true, disableAnimations: appState.settings?.disableAnimations });
    document.querySelectorAll(".theme-card").forEach((c) => {
      c.classList.toggle("active", c.getAttribute("data-theme") === nextTheme);
    });
    await saveSettings();
    return;
  }

  const fontBtn = target.closest("[data-action='select-font']");
  if (fontBtn) {
    const nextFont = fontBtn.getAttribute("data-font") || "Inter";
    appState.settings.fontFamily = nextFont;
    viewState.fontDropdownOpen = false;
    document.documentElement.style.setProperty("--app-font-family", `'${nextFont}', sans-serif`);
    try {
      localStorage.setItem("synchro_font", nextFont);
    } catch (_) {}
    updateMain();
    await saveSettings();
    return;
  }

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
    document.querySelectorAll(".preview-mode-btn[data-mode]").forEach((btn) => {
      btn.classList.toggle("active", btn.getAttribute("data-mode") === nextMode);
    });
    return;
  }

  const fineTuningTabBtn = target.closest("[data-action='set-fine-tuning-tab']");
  if (fineTuningTabBtn) {
    const tab = fineTuningTabBtn.getAttribute("data-tab");
    if (tab && viewState.fineTuningTab !== tab) {
      viewState.fineTuningTab = tab;
      updateColorPage();
    }
    return;
  }

  const configureFilter = target.closest("[data-action='configure-filter']");
  if (configureFilter) {
    event.preventDefault();
    event.stopPropagation();
    const filterId = configureFilter.getAttribute("data-filter");
    if (filterId) {
      viewState.editingTemplate = filterId;
      const filterNames = {
        rust_cold_tactical: t("rustColdTactical"),
        rust_midnight_neon: t("rustMidnightNeon"),
        clear_sight: t("clearSight")
      };
      const custom = appState.settings?.templateOverrides?.[filterId];
      const defaultFilter = filterPresets[filterId] || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
      viewState.editingTemplateName = custom?.name || filterNames[filterId] || filterId;
      viewState.editingTemplateColor = {
        saturation: custom?.saturation ?? defaultFilter.saturation,
        hue: custom?.hue ?? defaultFilter.hue,
        contrast: custom?.contrast ?? defaultFilter.contrast,
        gamma: custom?.gamma ?? defaultFilter.gamma
      };
      updateColorPage();
      syncAllTemplateSliders(viewState.editingTemplateColor);
    }
    return;
  }

  const toggleFilter = target.closest("[data-action='toggle-color-filter']");
  if (toggleFilter && !target.closest("[data-action='configure-filter']")) {
    const filterId = toggleFilter.getAttribute("data-filter-id");
    if (filterId) {
      const isAlreadyActive = appState.color.activeFilter === filterId;
      if (isAlreadyActive) {
        appState.color.activeFilter = "";
        const normal = getSavedNormalColor();
        appState.color.saturation = normal.saturation;
        appState.color.hue = normal.hue;
        appState.color.contrast = normal.contrast;
        appState.color.gamma = normal.gamma;
      } else {
        if (!appState.color.activeFilter) {
          saveNormalColorState();
        }
        appState.color.activeFilter = filterId;
        const custom = appState.settings?.templateOverrides?.[filterId];
        const defaultFilter = filterPresets[filterId];
        const cfg = custom || defaultFilter;
        if (cfg) {
          if (cfg.saturation !== undefined) appState.color.saturation = cfg.saturation;
          if (cfg.contrast !== undefined) appState.color.contrast = cfg.contrast;
          if (cfg.gamma !== undefined) appState.color.gamma = cfg.gamma;
          if (cfg.hue !== undefined) appState.color.hue = cfg.hue;
        }
      }
      updateColorPage();
      await applyColor();
      saveSettings();
    }
    return;
  }

  const disableFilter = target.closest("[data-action='disable-color-filter']");
  if (disableFilter) {
    appState.color.activeFilter = "";
    const normal = getSavedNormalColor();
    appState.color.saturation = normal.saturation;
    appState.color.hue = normal.hue;
    appState.color.contrast = normal.contrast;
    appState.color.gamma = normal.gamma;
    updateColorPage();
    await applyColor();
    saveSettings();
    return;
  }

  const toggleBlackHolo = target.closest("[data-action='toggle-black-holo']");
  if (toggleBlackHolo) {
    if (isTogglingBlackHolo) return;
    isTogglingBlackHolo = true;
    const isCurrentlyActive = Boolean(
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
            t("blackHoloEnabled") || (isRu ? "Режим Black Holo включен" : "Black Holo mode enabled"),
            "safe"
          );
        } else {
          showToastBanner(
            t("blackHolo"),
            t("blackHoloDisabled") || (isRu ? "Черный Холик отключен" : "Black Holosight disabled"),
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
    } finally {
      isTogglingBlackHolo = false;
    }

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
          t("screenshotCopiedToast") || (lang() === "ru"
            ? "Снимок экрана скопирован в буфер обмена (Ctrl+V)"
            : "Juicy screenshot copied to clipboard (Ctrl+V)!"),
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

  const checkUpdatesBtn = target.closest("[data-action='check-updates-manually']");
  if (checkUpdatesBtn) {
    checkForUpdates(true);
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

  if (!appState.isAdmin && activePage === "tweaks" && target.closest(".page-body .tweaks-page")) {
    if (target.closest("[data-tweak-id]") || target.closest("[data-action='apply-tweaks']") || target.closest("[data-action='rollback-tweaks']")) {
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
      if (viewState.pendingRevertTweaks.has(id)) {
        viewState.pendingRevertTweaks.delete(id);
      } else {
        viewState.pendingRevertTweaks.add(id);
      }
      if (tweak instanceof HTMLElement) tweak.blur();
      syncTweakTile(id);
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

  const addBind = target.closest("[data-action='add-bind']");
  if (addBind) {
    event.preventDefault();
    const newId = `bind_${Date.now()}_${Math.random().toString(36).slice(2, 6)}`;
    if (!Array.isArray(appState.keybinds)) appState.keybinds = [];
    appState.keybinds.push({
      id: newId,
      key: "",
      modifiers: 0,
      vk: 0,
      action: "template:vibrant",
      enabled: true
    });
    viewState.recordingBindId = newId;
    await syncKeybinds();
    updateMain();
    return;
  }

  const toggleDropdown = target.closest("[data-action='toggle-bind-dropdown']");
  if (toggleDropdown) {
    event.preventDefault();
    const bindId = toggleDropdown.getAttribute("data-bind-id");
    viewState.openDropdownBindId = viewState.openDropdownBindId === bindId ? null : bindId;
    updateMain();
    return;
  }

  const selectAction = target.closest("[data-action='select-bind-action']");
  if (selectAction) {
    event.preventDefault();
    const bindId = selectAction.getAttribute("data-bind-id");
    const val = selectAction.getAttribute("data-value");
    const bind = appState.keybinds?.find((b) => b.id === bindId);
    if (bind && val) {
      bind.action = val;
      viewState.openDropdownBindId = null;
      await syncKeybinds();
      updateMain();
    }
    return;
  }

  if (viewState.openDropdownBindId && !target.closest(".bind-custom-dropdown")) {
    viewState.openDropdownBindId = null;
    updateMain();
  }

  if (viewState.languageDropdownOpen && !target.closest(".language-custom-dropdown")) {
    viewState.languageDropdownOpen = false;
    updateMain();
  }

  if (viewState.fontDropdownOpen && !target.closest(".font-custom-dropdown")) {
    viewState.fontDropdownOpen = false;
    updateMain();
  }

  const recordBind = target.closest("[data-action='record-bind']");
  if (recordBind) {
    event.preventDefault();
    const bindId = recordBind.getAttribute("data-bind-id");
    if (bindId) {
      viewState.recordingBindId = viewState.recordingBindId === bindId ? null : bindId;
      updateMain();
    }
    return;
  }

  const toggleBind = target.closest("[data-action='toggle-bind']");
  if (toggleBind) {
    event.preventDefault();
    const bindId = toggleBind.getAttribute("data-bind-id");
    const bind = appState.keybinds?.find((b) => b.id === bindId);
    if (bind) {
      bind.enabled = !bind.enabled;
      await syncKeybinds();
      updateMain();
    }
    return;
  }

  const deleteBind = target.closest("[data-action='delete-bind']");
  if (deleteBind) {
    event.preventDefault();
    const bindId = deleteBind.getAttribute("data-bind-id");
    if (bindId) {
      appState.keybinds = (appState.keybinds || []).filter((b) => b.id !== bindId);
      if (viewState.recordingBindId === bindId) viewState.recordingBindId = null;
      if (viewState.openDropdownBindId === bindId) viewState.openDropdownBindId = null;
      await syncKeybinds();
      updateMain();
    }
    return;
  }

  const clearAllBinds = target.closest("[data-action='clear-all-binds']");
  if (clearAllBinds) {
    event.preventDefault();
    appState.keybinds = [];
    viewState.recordingBindId = null;
    viewState.openDropdownBindId = null;
    await syncKeybinds();
    updateMain();
    return;
  }

  if (viewState.recordingBindId && !target.closest("[data-action='record-bind']")) {
    viewState.recordingBindId = null;
    updateMain();
  }

  const backup = target.closest("[data-backup-id]");
  if (backup) {
    const clickedId = backup.getAttribute("data-backup-id");
    viewState.selectedBackup = viewState.selectedBackup === clickedId ? null : clickedId;
    updateMain();
    return;
  }

  

  const action = target.closest("[data-action]");
  if (action) await handleAction(action.getAttribute("data-action"));
}


async function handleKeydown(event) {
  const k = event.key;
  const isCtrl = event.ctrlKey || event.metaKey;
  const isAlt = event.altKey;

  const isBrowserShortcut =
    k === "F1" ||
    k === "F3" ||
    k === "F5" ||
    k === "F6" ||
    k === "F7" ||
    k === "F11" ||
    k === "F12" ||
    (isCtrl && ["f", "g", "p", "u", "s", "h", "j", "o", "n", "t", "w", "r", "d"].includes(k?.toLowerCase())) ||
    (isAlt && (k === "ArrowLeft" || k === "ArrowRight"));

  if (isBrowserShortcut) {
    const targetTag = (event.target && event.target.tagName) ? event.target.tagName.toLowerCase() : "";
    const isEditingText = targetTag === "input" || targetTag === "textarea";
    const isTextEditCommand = isCtrl && ["c", "v", "x", "a", "z", "y"].includes(k?.toLowerCase());

    if (!isEditingText || !isTextEditCommand) {
      event.preventDefault();
      event.stopPropagation();
      if (!viewState.recordingBindId) return;
    }
  }

  if (viewState.recordingBindId) {
    if (event.key === "Escape" && !event.ctrlKey && !event.shiftKey && !event.altKey) {
      event.preventDefault();
      viewState.recordingBindId = null;
      updateMain();
      return;
    }
    if (["Control", "Shift", "Alt", "Meta"].includes(event.key)) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const bind = appState.keybinds?.find((b) => b.id === viewState.recordingBindId);
    if (bind) {
      let modifiers = 0;
      if (event.altKey) modifiers |= 1;
      if (event.ctrlKey) modifiers |= 2;
      if (event.shiftKey) modifiers |= 4;
      if (event.metaKey) modifiers |= 8;
      bind.modifiers = modifiers;
      bind.vk = getWin32Vk(event);
      bind.key = formatKeyLabel(event);
    }
    viewState.recordingBindId = null;
    await syncKeybinds();
    updateMain();
    return;
  }

  if (viewState.openDropdownBindId && event.key === "Escape") {
    event.preventDefault();
    viewState.openDropdownBindId = null;
    updateMain();
    return;
  }

  if (viewState.languageDropdownOpen && event.key === "Escape") {
    event.preventDefault();
    viewState.languageDropdownOpen = false;
    updateMain();
    return;
  }

  if (viewState.fontDropdownOpen && event.key === "Escape") {
    event.preventDefault();
    viewState.fontDropdownOpen = false;
    updateMain();
    return;
  }

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
  const activeEl = document.activeElement;
  if ((event.key === "Enter" || event.key === " ") && activeEl) {
    if (
      activeEl.classList.contains("check-row") ||
      activeEl.classList.contains("tweak-tile") ||
      activeEl.classList.contains("color-filter-tile") ||
      activeEl.classList.contains("template-tile")
    ) {
      event.preventDefault();
      activeEl.click();
      return;
    }
  }
  
}

function handlePointerDown(event) {
  if (event.button !== 0) return;
  const target = event.target instanceof Element ? event.target : null;
  if (!target?.closest("[data-window-drag]")) return;
  if (target.closest("button, input, textarea, select, a, [data-action], [data-page], [data-setting], .window-controls")) return;
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

function render(bypassCache = true) {
  const app = document.getElementById("app");
  if (!app) return;
  document.documentElement.lang = lang();
  const curTheme = appState.settings?.theme || "default";
  applyTheme(curTheme, { disableAnimations: appState.settings?.disableAnimations });
  if (curTheme === "default" && appState.settings.accentColor) {
    applyInterfaceAccent(appState.settings.accentColor);
  }
  app.className = "app-shell";
  app.innerHTML = renderShell(viewState, t, bypassCache);
  syncAllSliders(appState, viewState);
  updateNavState();
  const scrollPanelEl = document.querySelector(".scroll-panel");
  if (scrollPanelEl) {
    bindScrollSpy(scrollPanelEl);
  }
  renderBackupModalDom();
  renderApplyModalDom();
  if (activePage === "tweaks" && !appState.isAdmin) {
    showAdminNotificationBanner();
  }
  window.requestAnimationFrame(syncNavIndicator);
}

async function checkSecurityIncidents() {
  if (typeof document !== "undefined" && document.hidden) return;
  try {
    const status = await invokeCommand("get_security_guard_status");
    if (status && Array.isArray(status.recentIncidents) && status.recentIncidents.length > 0) {
      const latest = status.recentIncidents[0];
      const lastReported = Number(localStorage.getItem("synchro_last_sec_incident") || 0);
      if (latest.timestamp > lastReported) {
        localStorage.setItem("synchro_last_sec_incident", String(latest.timestamp));
        const isRu = lang() === "ru";
        const shieldBlockedMsg = t("securityShieldBlocked");
        const blockedText = shieldBlockedMsg
          ? (shieldBlockedMsg.includes("{target}")
              ? shieldBlockedMsg.replace("{target}", latest.target || "")
              : (latest.target ? `${shieldBlockedMsg} (${latest.target})` : shieldBlockedMsg))
          : (isRu
              ? `Заблокировано небезопасное обращение к ${latest.target}. Ваш игровой аккаунт находится в полной безопасности.`
              : `Blocked unsafe access to ${latest.target}. Your gaming account is completely safe.`);
        showToastBanner(
          t("securityShieldTitle") || (isRu ? "Защита аккаунта Synchro Nova" : "Synchro Nova Security Shield"),
          blockedText,
          "safe"
        );
      }
    }
  } catch (_) {}
}

async function boot() {
  registerLifecycleHooks();
  let cachedTheme = "default";
  let cachedFont = "Inter";
  try {
    cachedTheme = localStorage.getItem("synchro_theme") || "default";
    cachedFont = localStorage.getItem("synchro_font") || "Inter";
  } catch {}
  applyTheme(cachedTheme, { disableAnimations: true });
  document.documentElement.style.setProperty("--app-font-family", `'${cachedFont}', sans-serif`);

  try {
    const savedCollapsed = localStorage.getItem("synchro_sidebar_collapsed");
    if (savedCollapsed !== null) {
      viewState.sidebarCollapsed = savedCollapsed === "true";
    }
  } catch {}

  let initialPage = "color";
  try {
    const pendingStorage = localStorage.getItem("synchro_pending_nav");
    if (pendingStorage && pageDefs[pendingStorage]) {
      initialPage = pendingStorage;
      localStorage.removeItem("synchro_pending_nav");
    }
  } catch {}
  if (!pageDefs[initialPage]) {
    initialPage = "color";
  }

  const statePromise = invokeCommand("get_app_state").catch(() => null);
  const holoPromise = loadBlackHoloStatus().catch(() => null);
  const pendingNavPromise = invokeCommand("get_pending_navigation").catch(() => null);

  const [state, pendingNav] = await Promise.all([statePromise, pendingNavPromise]);
  if (state) {
    mergeState(state);
  }

  const targetPage = (pendingNav && typeof pendingNav === "string" && pageDefs[pendingNav])
    ? pendingNav
    : initialPage;

  setActivePage(targetPage);
  router.clearPageCache();

  applyTheme(appState.settings?.theme || "default", {
    disableAnimations: Boolean(appState.settings?.disableAnimations)
  });
  if (!appState.color?.activeFilter) {
    saveNormalColorState();
  }

  const shouldDisableAnim = Boolean(appState.settings?.disableAnimations);
  document.body.classList.toggle("no-animations", shouldDisableAnim);

  invokeCommand("consume_intro_splash").catch(() => false);
  const showSplash = !shouldDisableAnim && !appState.settings?.disableSplash;
  if (showSplash) {
    runIntroSplash({
      onManualDownload: () => {
        invokeCommand("open_external_url", { url: "https://github.com/Arean-Max/synchro-nova/releases/latest" });
      }
    });
    handleStartupUpdateCheck();
  } else {
    checkForUpdates(false);
  }

  if (Array.isArray(appState.keybinds) && appState.keybinds.length) {
    syncKeybinds();
  }

  try {
    render(true);
  } catch (err) {
    console.error("Critical error during initial render:", err);
  }

  Promise.all([holoPromise, loadLists()]).then(() => {
    if (activePage === "backups") {
      updateMainKeepingScroll();
    }
  });

  triggerScanDrivers();

  window.setInterval(() => checkForUpdates(false), 4 * 60 * 60 * 1000);
  loadTweakStatuses();
  scheduleTrimMemory(600);
  checkSecurityIncidents();
}

document.addEventListener("input", withErrorBoundary(handleInput, "Input"));
document.addEventListener("click", withErrorBoundary(handleClick, "Click"));
document.addEventListener("contextmenu", (e) => e.preventDefault(), { capture: true });
document.addEventListener("pointerdown", handlePointerDown);
document.addEventListener("error", handleImageError, true);
document.addEventListener("keydown", withErrorBoundary(handleKeydown, "Keydown"));

window.addEventListener("wheel", (event) => {
  if (event.ctrlKey || event.altKey) return;
  const target = event.target instanceof Element ? event.target : null;
  if (!target) return;

  if (target.closest(".modal-card, .apply-modal, .dropdown-menu, .template-config-dialog, .backup-name-dialog")) {
    return;
  }

  const mainPanel = document.querySelector(".main-panel");
  if (!mainPanel || !mainPanel.contains(target)) return;

  const activeContainer = target.closest(".scroll-panel, .settings-page, .storage-page-container, .page-binds, .global-color-page");
  if (activeContainer) {
    return;
  }

  const primary = mainPanel.querySelector(
    ".scroll-panel, .settings-page, .storage-page-container, .page-binds, .global-color-page"
  );
  if (primary && primary.scrollHeight > primary.clientHeight) {
    primary.scrollTop += event.deltaY;
  }
}, { passive: true });
window.addEventListener(
  "keydown",
  (e) => {
    const k = e.key;
    const isCtrl = e.ctrlKey || e.metaKey;
    const isAlt = e.altKey;
    const isBrowserShortcut =
      k === "F1" ||
      k === "F3" ||
      k === "F5" ||
      k === "F6" ||
      k === "F7" ||
      k === "F11" ||
      k === "F12" ||
      (isCtrl && ["f", "g", "p", "u", "s", "h", "j", "o", "n", "t", "w", "r", "d"].includes(k?.toLowerCase())) ||
      (isAlt && (k === "ArrowLeft" || k === "ArrowRight"));

    if (isBrowserShortcut) {
      const targetTag = (e.target && e.target.tagName) ? e.target.tagName.toLowerCase() : "";
      const isEditingText = targetTag === "input" || targetTag === "textarea";
      const isTextEditCommand = isCtrl && ["c", "v", "x", "a", "z", "y"].includes(k?.toLowerCase());

      if (!isEditingText || !isTextEditCommand) {
        if (!viewState.recordingBindId) {
          e.preventDefault();
          e.stopPropagation();
        }
      }
    }
  },
  { capture: true }
);
function handleWindowRestored() {
  document.body.style.pointerEvents = "";
  document.documentElement.style.pointerEvents = "";
  document.body.classList.remove("paused-animations");
  window.dispatchEvent(new Event("resize"));
  syncNavIndicator();
  checkSecurityIncidents();
  if (typeof themeFx !== "undefined" && themeFx && typeof themeFx.resume === "function") {
    themeFx.resume();
  }
}

window.addEventListener("resize", syncNavIndicator);
window.addEventListener("focus", handleWindowRestored);
window.addEventListener("pageshow", handleWindowRestored);
window.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    clearImpactBannerTimer();
    document.body.classList.add("paused-animations");
  } else {
    handleWindowRestored();
  }
});
window.addEventListener("keydown", async (event) => {
  if (event.key === "PrintScreen" || (event.ctrlKey && event.shiftKey && event.key?.toLowerCase() === "s")) {
    try {
      const res = await invokeCommand("take_juicy_screenshot");
      if (res?.success) {
        showToastBanner(
          t("screenshot"),
          t("screenshotCopiedToast") || (lang() === "ru"
            ? "Снимок экрана скопирован в буфер обмена (Ctrl+V)"
            : "Juicy screenshot copied to clipboard (Ctrl+V)!"),
          "safe"
        );
      }
    } catch (err) {
      console.error("Screenshot hotkey failed:", err);
    }
  }
});

if (window.__TAURI__?.event?.listen) {
  window.__TAURI__.event.listen("window-restored", handleWindowRestored);
  window.__TAURI__.event.listen("hotkey-triggered", (e) => {
    const payload = e?.payload;
    if (!payload) return;
    const isRu = lang() === "ru";
    if (payload.action === "trim_memory") {
      showToastBanner(t("action_trim_memory"), t("ramPurgedToast") || (isRu ? "Оперативная память успешно очищена" : "RAM trimmed"), "safe");
    } else if (payload.action === "toggle_black_holo") {
      const active = Boolean(payload.active);
      appState.color.blackHolo = active ? 100 : 0;
      updateBlackHoloDom(active, viewState.blackHoloStatus);
      showToastBanner(t("blackHolo"), active ? (t("blackHoloEnabled") || (isRu ? "Черный Холик активен" : "Black Holosight enabled")) : (t("blackHoloDisabled") || (isRu ? "Черный Холик отключен" : "Black Holosight disabled")), "safe");
    } else if (payload.color) {
      appState.color = { ...appState.color, ...payload.color };
      syncAllSliders(appState);
      updateColorPage();
      const label = payload.name ? `${t("templateAppliedToast") || (isRu ? "Применён шаблон" : "Template applied")}: ${payload.name}` : (t("colorsUpdatedToast") || (isRu ? "Цвета обновлены" : "Colors updated"));
      showToastBanner(t("colorTitle"), label, "safe");
    }
    if (activePage === "color") {
      updateColorPage();
    }
  });

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
        t("blackHoloEnabled") || (isRu ? "Режим Black Holo включен" : "Black Holo mode enabled"),
        "safe"
      );
    } else {
      showToastBanner(
        t("blackHolo"),
        t("blackHoloDisabled") || (isRu ? "Черный Холик отключен" : "Black Holosight disabled"),
        "safe"
      );
    }
  });
}

window.addEventListener("error", (event) => {
  const msg = event.error?.message || event.message;
  if (msg) {
    showErrorToast(t("systemError") || (lang() === "ru" ? "Системная ошибка" : "System Error"), msg);
  }
});

window.addEventListener("unhandledrejection", (event) => {
  const reason = event.reason;
  const msg = reason?.message || (typeof reason === "string" ? reason : "");
  if (msg && !msg.includes("canceled") && !msg.includes("aborted")) {
    showErrorToast(t("executionError") || (lang() === "ru" ? "Ошибка выполнения" : "Execution Error"), msg);
  }
});

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", () => {
    boot();
  });
} else {
  boot();
}
