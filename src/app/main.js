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
import { renderMain, renderShell, renderPageBody, renderSidebarUpdateWidget } from "./ui/layout.js";
import { escapeHtml } from "./core/html.js";
import { runIntroSplash } from "./ui/splash.js";


router.register("color", colorFeature);
router.register("gameColor", colorFeature);
router.register("tweaks", tweaksFeature);
router.register("binds", bindsFeature);
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
    appName: isRu ? "СБОЙ ТВИКА" : "TWEAK ERROR",
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
    appName: isRu ? "СНИМОК СИСТЕМЫ" : "SAFETY BACKUP",
    impactText: isRu
      ? "Перед жестким твиком сделай быстрый снимок системы, чтобы не сломать винду."
      : "Take a quick safety snapshot before applying system-level tweaks."
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
    appName: isRu ? "СНИМОК СИСТЕМЫ" : "SAFETY SNAPSHOT",
    title: t("backupNotificationTitle") || (isRu ? "Снимок Системы" : "Safety Snapshot"),
    impactText: t("backupNotificationMsg") || (isRu ? "Кликни здесь, чтобы задать имя снимка" : "Click here to set custom snapshot name")
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
    appName: isRu ? "НУЖЕН АДМИН" : "ADMIN REQUIRED",
    title: t("adminNotificationTitle") || (isRu ? "Требуются права администратора" : "Administrator Rights Required"),
    impactText: t("adminNotificationMsg") || (isRu ? "Кликни здесь, чтобы перезапустить Synchro с правами админа" : "Click here to restart Synchro with admin rights")
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

function getWin32Vk(event) {
  if (typeof event.keyCode === "number" && event.keyCode > 0 && event.keyCode !== 229) {
    return event.keyCode;
  }
  const code = event.code || "";
  if (/^Key[A-Z]$/.test(code)) return code.charCodeAt(3);
  if (/^Digit[0-9]$/.test(code)) return code.charCodeAt(5);
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(code)) return 111 + parseInt(code.slice(1), 10);
  if (/^Numpad[0-9]$/.test(code)) return 96 + parseInt(code.slice(6), 10);
  const vkMap = {
    Space: 32,
    Enter: 13,
    NumpadEnter: 13,
    Tab: 9,
    Escape: 27,
    Backspace: 8,
    Insert: 45,
    Delete: 46,
    Home: 36,
    End: 35,
    PageUp: 33,
    PageDown: 34,
    ArrowUp: 38,
    ArrowDown: 40,
    ArrowLeft: 37,
    ArrowRight: 39,
    NumpadMultiply: 106,
    NumpadAdd: 107,
    NumpadSubtract: 109,
    NumpadDecimal: 110,
    NumpadDivide: 111,
    Minus: 189,
    Equal: 187,
    BracketLeft: 219,
    BracketRight: 221,
    Backslash: 220,
    Semicolon: 186,
    Quote: 222,
    Backquote: 192,
    Comma: 188,
    Period: 190,
    Slash: 191
  };
  return vkMap[code] || 0;
}

function formatKeyLabel(event) {
  const parts = [];
  if (event.ctrlKey) parts.push("Ctrl");
  if (event.altKey) parts.push("Alt");
  if (event.shiftKey) parts.push("Shift");
  if (event.metaKey) parts.push("Win");

  let base = "";
  const code = event.code || "";
  if (code.startsWith("Key")) {
    base = code.slice(3).toUpperCase();
  } else if (code.startsWith("Digit")) {
    base = code.slice(5);
  } else if (code.startsWith("Numpad") && /^Numpad\d$/.test(code)) {
    base = `Num ${code.slice(6)}`;
  } else if (code === "NumpadEnter") {
    base = "Num Enter";
  } else if (code === "NumpadAdd") {
    base = "Num +";
  } else if (code === "NumpadSubtract") {
    base = "Num -";
  } else if (code === "NumpadMultiply") {
    base = "Num *";
  } else if (code === "NumpadDivide") {
    base = "Num /";
  } else if (code === "NumpadDecimal") {
    base = "Num .";
  } else if (/^F\d+$/.test(code)) {
    base = code;
  } else {
    const specialMap = {
      Space: "Space",
      Enter: "Enter",
      Tab: "Tab",
      Backspace: "Backspace",
      Delete: "Delete",
      Insert: "Insert",
      Home: "Home",
      End: "End",
      PageUp: "PageUp",
      PageDown: "PageDown",
      ArrowUp: "Up",
      ArrowDown: "Down",
      ArrowLeft: "Left",
      ArrowRight: "Right",
      Minus: "-",
      Equal: "=",
      BracketLeft: "[",
      BracketRight: "]",
      Backslash: "\\",
      Semicolon: ";",
      Quote: "'",
      Backquote: "`",
      Comma: ",",
      Period: ".",
      Slash: "/"
    };
    base = specialMap[code] || event.key?.toUpperCase() || code;
  }

  parts.push(base);
  return parts.join(" + ");
}

async function syncKeybinds() {
  try {
    await invokeCommand("sync_keybinds", { keybinds: appState.keybinds || [] });
  } catch (err) {
    console.error("Failed to sync keybinds:", err);
  }
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