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