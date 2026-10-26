import { appState, lang, viewState } from "../core/state.js";
import { escapeHtml } from "../core/html.js";
import { renderIosNotification, getTweakImpactDetails } from "../features/tweaks/page.js";
import { allTweaks, tweakCategory, tweakTitle } from "../features/tweaks/catalog.js";

let impactBannerTimer = 0;
let pillToastTimer = null;

export function clearImpactBannerTimer() {
  if (impactBannerTimer) {
    window.clearTimeout(impactBannerTimer);
    impactBannerTimer = 0;
  }
}

export function startImpactBannerTimer(delay = 3000) {
  if (viewState.activeImpactBanner?.isAdminPrompt) return;
  clearImpactBannerTimer();
  impactBannerTimer = window.setTimeout(() => {
    dismissNotificationBanner();
  }, delay);
}

export function attachBannerHoverListeners() {
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

export function dismissNotificationBanner(immediate = false) {
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

export function renderNotificationBannerDom(t = (k) => k) {
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

export function showPillToast(message) {
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

export function showErrorToast(title, err, t = (k) => k) {
  const isRu = lang() === "ru";
  const msg = (err && typeof err === "object" && err.message)
    ? err.message
    : (typeof err === "string" ? err : (err ? JSON.stringify(err) : (t("unknownError") || "Unknown error")));
  showToastBanner(title || t("notification_error") || (isRu ? "Ошибка" : "Error"), msg, "danger", t);
}

export function showToastBanner(title, impactText, category = "safe", t = (k) => k) {
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
  renderNotificationBannerDom(t);
  startImpactBannerTimer(category === "danger" ? 6000 : 3000);
}

export function showSettingHelpBanner(key, t = (k) => k) {
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
  showToastBanner(title, helpText, "safe", t);
}

export function showTweakImpactBanner(tweakId, t = (k) => k) {
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
  renderNotificationBannerDom(t);
  startImpactBannerTimer(3000);
}

export function showTweakErrorBanner(tweakId, errorMsg, t = (k) => k) {
  clearImpactBannerTimer();
  const tweaks = allTweaks();
  const tweak = tweaks.find((t) => t.id === tweakId);
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
  renderNotificationBannerDom(t);
  startImpactBannerTimer(8000);
}

export function showRiskWarningBanner(tweak, t = (k) => k) {
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
  renderNotificationBannerDom(t);
  startImpactBannerTimer(3000);
}

export function showBackupNotificationBanner(t = (k) => k) {
  clearImpactBannerTimer();
  const isRu = lang() === "ru";
  viewState.activeImpactBanner = {
    isBackupPrompt: true,
    category: "safe",
    appName: t("safetySnapshot") || (isRu ? "СНИМОК СИСТЕМЫ" : "SAFETY SNAPSHOT"),
    title: t("backupNotificationTitle") || (isRu ? "Снимок Системы" : "Safety Snapshot"),
    impactText: t("backupNotificationMsg") || (isRu ? "Нажмите здесь, чтобы задать имя резервной копии" : "Click here to set custom snapshot name")
  };
  renderNotificationBannerDom(t);
  startImpactBannerTimer(3000);
}

export function showAdminNotificationBanner(t = (k) => k) {
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
  renderNotificationBannerDom(t);
}

export function highlightAdminBanner(t = (k) => k) {
  const container = document.querySelector(".ios-banner-container");
  if (container) {
    container.classList.remove("pulse-hint");
    void container.offsetWidth;
    container.classList.add("pulse-hint");
  } else {
    showAdminNotificationBanner(t);
  }
}
