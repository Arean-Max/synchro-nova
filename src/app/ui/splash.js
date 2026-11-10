import { translate } from "../core/i18n.js";
import { escapeHtml } from "../core/html.js";

let splashFinished = false;
let isDownloadingUpdate = false;
let autoTimeout = null;
let exitTimeout = null;
let finishCallback = null;
let keyListener = null;
let clickListener = null;
let manualCallback = null;
let skipCallback = null;

export function setSplashDownloading(downloading) {
  isDownloadingUpdate = Boolean(downloading);
  if (isDownloadingUpdate && autoTimeout) {
    clearTimeout(autoTimeout);
    autoTimeout = null;
  }
}

export function isSplashDownloading() {
  return isDownloadingUpdate;
}

export function isSplashActive() {
  return Boolean(document.getElementById("intro-splash-screen")) && !splashFinished;
}

export function showSplashChecking(text, isUpToDate = false) {
  const splash = document.getElementById("intro-splash-screen");
  if (!splash) return;

  const checkTextEl = document.getElementById("splash-check-text");
  const cardEl = document.getElementById("splash-updater-card");

  if (cardEl) {
    cardEl.style.display = "none";
  }

  if (checkTextEl) {
    checkTextEl.style.display = "";
    if (text !== undefined) checkTextEl.textContent = text;
    if (isUpToDate) {
      checkTextEl.style.color = "rgba(255, 255, 255, 0.95)";
      checkTextEl.style.fontWeight = "600";
    } else {
      checkTextEl.style.color = "rgba(255, 255, 255, 0.65)";
      checkTextEl.style.fontWeight = "500";
    }
  }
}

export function showSplashDownloadCard(options = {}) {
  const { title, pct = 0, onManual, onSkip } = options;
  const splash = document.getElementById("intro-splash-screen");
  if (!splash) return;

  manualCallback = onManual || null;
  skipCallback = onSkip || null;

  const checkTextEl = document.getElementById("splash-check-text");
  if (checkTextEl) {
    checkTextEl.style.display = "none";
  }

  const hintEl = document.getElementById("splash-skip-hint");
  if (hintEl) {
    hintEl.style.display = "none";
  }

  const cardEl = document.getElementById("splash-updater-card");
  if (cardEl) {
    cardEl.style.display = "flex";
  }

  const statusEl = document.getElementById("splash-updater-status");
  if (statusEl && title !== undefined) {
    statusEl.textContent = title;
  }

  const pctEl = document.getElementById("splash-updater-pct");
  if (pctEl) {
    pctEl.textContent = `${Math.max(0, Math.min(100, Number(pct) || 0))}%`;
  }

  const fillEl = document.getElementById("splash-updater-fill");
  if (fillEl) {
    fillEl.style.width = `${Math.max(0, Math.min(100, Number(pct) || 0))}%`;
  }

  const manualBtn = document.getElementById("splash-btn-manual");
  if (manualBtn) {
    manualBtn.style.display = "";
    manualBtn.onclick = (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (typeof manualCallback === "function") {
        manualCallback();
      }
    };
  }

  const skipBtn = document.getElementById("splash-btn-skip");
  if (skipBtn) {
    skipBtn.style.display = "";
    skipBtn.onclick = (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (typeof skipCallback === "function") {
        skipCallback();
      }
    };
  }
}

export function updateSplashDownloadCard({ title, pct = null, isInstalling = false, isError = false }) {
  const splash = document.getElementById("intro-splash-screen");
  if (!splash) return;

  const statusEl = document.getElementById("splash-updater-status");
  if (statusEl && title !== undefined) {
    statusEl.textContent = title;
  }

  const pctEl = document.getElementById("splash-updater-pct");
  const fillEl = document.getElementById("splash-updater-fill");

  if (pct !== null && pct !== undefined) {
    const p = Math.max(0, Math.min(100, Number(pct) || 0));
    if (pctEl) pctEl.textContent = `${p}%`;
    if (fillEl) fillEl.style.width = `${p}%`;
  }

  if (isInstalling) {
    if (pctEl) pctEl.textContent = "100%";
    if (fillEl) fillEl.style.width = "100%";
    const actionsEl = splash.querySelector(".splash-card-actions");
    if (actionsEl) actionsEl.style.display = "none";
  }

  if (isError) {
    if (fillEl) {
      fillEl.style.background = "#ef4444";
      fillEl.style.boxShadow = "0 0 10px rgba(239, 68, 68, 0.5)";
    }
  }
}

// Backward compatibility helper
export function updateSplashUpdater(options = {}) {
  const { text, percent = null, showManual = false } = options;
  const cardEl = document.getElementById("splash-updater-card");
  if (!cardEl || cardEl.style.display === "none") {
    showSplashChecking(text);
  } else {
    updateSplashDownloadCard({
      title: text,
      pct: percent
    });
  }
}

function doFinishSplash() {
  if (splashFinished) return;
  splashFinished = true;

  if (autoTimeout) {
    clearTimeout(autoTimeout);
    autoTimeout = null;
  }
  if (exitTimeout) {
    clearTimeout(exitTimeout);
    exitTimeout = null;
  }

  const splash = document.getElementById("intro-splash-screen");
  if (!splash) {
    if (typeof finishCallback === "function") {
      finishCallback();
      finishCallback = null;
    }
    return;
  }

  if (keyListener) {
    window.removeEventListener("keydown", keyListener, true);
    keyListener = null;
  }
  if (clickListener) {
    splash.removeEventListener("click", clickListener, true);
    clickListener = null;
  }

  splash.classList.add("splash-fade-out");
  exitTimeout = setTimeout(() => {
    splash.remove();
    if (typeof finishCallback === "function") {
      finishCallback();
      finishCallback = null;
    }
  }, 280);
}

export function finishIntroSplash(delay = 0) {
  if (splashFinished) return;
  if (delay > 0) {
    setTimeout(() => doFinishSplash(), delay);
  } else {
    doFinishSplash();
  }
}

export function runIntroSplash(options = {}) {
  const { onFinish, onManualDownload, skipAnimation = false } = options;
  finishCallback = onFinish;
  manualCallback = onManualDownload || null;
  splashFinished = false;
  isDownloadingUpdate = false;

  let splash = document.getElementById("intro-splash-screen");

  if (skipAnimation || document.body.classList.contains("no-animations")) {
    if (splash) splash.remove();
    if (typeof onFinish === "function") onFinish();
    return;
  }

  const currentLang = document.documentElement.lang || localStorage.getItem("synchro_language") || "en";
  const isRu = currentLang === "ru";
  const skipText = translate(currentLang, "skipSplash") || (isRu ? "Нажмите Esc или кликните для пропуска" : "Press Esc or click to skip");
  const checkingText = translate(currentLang, "checkingUpdates") || (isRu ? "Проверка обновлений..." : "Checking for updates...");
  const manualDownloadText = translate(currentLang, "manualDownload") || (isRu ? "Вручную" : "Manual Download");
  const skipBtnText = translate(currentLang, "skip") || (isRu ? "Пропустить" : "Skip");

  if (!splash) {
    const lettersSynchro = ["S", "y", "n", "c", "h", "r", "o"];
    const lettersNova = ["N", "o", "v", "a"];

    const synchroHtml = lettersSynchro
      .map((ch, i) => `<span class="splash-char" style="--char-idx: ${i}">${ch}</span>`)
      .join("");

    const novaHtml = lettersNova
      .map((ch, i) => `<span class="splash-char" style="--char-idx: ${i + 7}">${ch}</span>`)
      .join("");

    splash = document.createElement("div");
    splash.id = "intro-splash-screen";
    splash.className = "intro-splash-screen splash-animate";
    splash.innerHTML = [
      '<div class="intro-splash-backdrop"></div>',
      '<div class="intro-splash-stage">',
      '  <div class="intro-splash-logo-wrap">',
      '    <img class="intro-splash-logo-img" src="./assets/synchro-nova-icon.png" alt="Synchro Nova" decoding="sync" loading="eager" />',
      '  </div>',
      '  <div class="intro-splash-title">',
      `    <span class="splash-word-synchro">${synchroHtml}</span>`,
      '    <span class="splash-space">&nbsp;</span>',
      `    <span class="splash-word-nova">${novaHtml}</span>`,
      '  </div>',
      '</div>',
      `<div class="intro-splash-check-text" id="splash-check-text">${escapeHtml(checkingText)}</div>`,
      '<div class="intro-splash-updater-card" id="splash-updater-card" style="display: none;">',
      '  <div class="splash-card-row">',
      '    <span class="splash-card-title" id="splash-updater-status">Downloading update...</span>',
      '    <span class="splash-card-pct" id="splash-updater-pct">0%</span>',
      '  </div>',
      '  <div class="splash-card-track" id="splash-updater-track">',
      '    <div class="splash-card-fill" id="splash-updater-fill" style="width: 0%"></div>',
      '  </div>',
      '  <div class="splash-card-actions">',
      '    <button class="splash-btn splash-btn-manual" id="splash-btn-manual" type="button">',
      '      <svg class="splash-btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">',
      '        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>',
      '        <polyline points="7 10 12 15 17 10"/>',
      '        <line x1="12" y1="15" x2="12" y2="3"/>',
      '      </svg>',
      `      <span id="splash-btn-manual-label">${escapeHtml(manualDownloadText)}</span>`,
      '    </button>',
      '    <button class="splash-btn splash-btn-skip" id="splash-btn-skip" type="button">',
      `      <span id="splash-btn-skip-label">${escapeHtml(skipBtnText)}</span>`,
      '    </button>',
      '  </div>',
      '</div>',
      `<div class="intro-splash-skip-hint" id="splash-skip-hint">${escapeHtml(skipText)}</div>`
    ].join("");
    document.body.appendChild(splash);
  } else {
    const hint = splash.querySelector("#splash-skip-hint, .intro-splash-skip-hint");
    if (hint) hint.textContent = skipText;
    splash.classList.add("splash-animate");

    const checkTextEl = splash.querySelector("#splash-check-text");
    if (checkTextEl) checkTextEl.textContent = checkingText;

    const manualLabel = splash.querySelector("#splash-btn-manual-label");
    if (manualLabel) manualLabel.textContent = manualDownloadText;

    const skipLabel = splash.querySelector("#splash-btn-skip-label");
    if (skipLabel) skipLabel.textContent = skipBtnText;
  }

  clickListener = function onClick(e) {
    if (e.target && e.target.closest("#splash-updater-card")) {
      return;
    }
    // Prevent accidental dismiss while downloading
    if (isDownloadingUpdate) {
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    doFinishSplash();
  };

  keyListener = function onKeyDown(e) {
    if (e.key === "Escape" || e.key === " " || e.key === "Enter") {
      // Prevent accidental key dismiss while downloading
      if (isDownloadingUpdate) return;
      e.preventDefault();
      e.stopPropagation();
      doFinishSplash();
    }
  };

  splash.addEventListener("click", clickListener, true);
  window.addEventListener("keydown", keyListener, true);

  const img = splash.querySelector(".intro-splash-logo-img");
  const launchAnim = () => {
    if (splashFinished) return;
    requestAnimationFrame(() => {
      splash.classList.add("splash-animate");
    });
  };

  launchAnim();
  if (img && typeof img.decode === "function") {
    img.decode().then(launchAnim).catch(launchAnim);
  } else if (img && !img.complete) {
    img.onload = launchAnim;
    img.onerror = launchAnim;
  }

  // Safety fallback ONLY if no download ever starts: after 7s finish smoothly
  autoTimeout = setTimeout(() => {
    if (!isDownloadingUpdate) {
      doFinishSplash();
    }
  }, 7000);
}
