import { translate } from "../core/i18n.js";
import { escapeHtml } from "../core/html.js";

let splashFinished = false;
let isDownloadingUpdate = false;
let autoTimeout = null;
let exitTimeout = null;
let finishCallback = null;
let keyListener = null;
let clickListener = null;

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

export function updateSplashUpdater({ text, percent = null, showManual = false, isIndeterminate = false }) {
  const splash = document.getElementById("intro-splash-screen");
  if (!splash) return;

  const statusEl = document.getElementById("splash-updater-status");
  const pctEl = document.getElementById("splash-updater-pct");
  const manualBtn = document.getElementById("splash-updater-manual");
  const fillEl = document.getElementById("splash-updater-fill");

  if (statusEl && text !== undefined) {
    statusEl.textContent = text;
  }

  if (pctEl) {
    if (percent !== null && percent !== undefined && !isIndeterminate) {
      pctEl.textContent = `${percent}%`;
      pctEl.style.display = "";
    } else {
      pctEl.textContent = "";
      pctEl.style.display = "none";
    }
  }

  if (manualBtn) {
    manualBtn.style.display = showManual ? "" : "none";
  }

  if (fillEl) {
    if (isIndeterminate) {
      fillEl.classList.add("indeterminate");
      fillEl.style.width = "100%";
    } else {
      fillEl.classList.remove("indeterminate");
      const p = Math.max(0, Math.min(100, Number(percent) || 0));
      fillEl.style.width = `${p}%`;
    }
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
  const manualDownloadText = translate(currentLang, "manualDownload") || (isRu ? "Скачать вручную" : "Manual Download");

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
      '<div class="intro-splash-updater" id="splash-updater">',
      '  <div class="splash-updater-row">',
      '    <div class="splash-updater-left">',
      `      <span class="splash-updater-status" id="splash-updater-status">${escapeHtml(checkingText)}</span>`,
      '    </div>',
      '    <div class="splash-updater-right">',
      '      <span class="splash-updater-pct" id="splash-updater-pct" style="display:none"></span>',
      `      <button class="splash-updater-manual" id="splash-updater-manual" type="button" style="display:none">${escapeHtml(manualDownloadText)}</button>`,
      '    </div>',
      '  </div>',
      '  <div class="splash-updater-track" id="splash-updater-track">',
      '    <div class="splash-updater-fill indeterminate" id="splash-updater-fill" style="width: 100%"></div>',
      '  </div>',
      '</div>',
      `<div class="intro-splash-skip-hint">${escapeHtml(skipText)}</div>`
    ].join("");
    document.body.appendChild(splash);
  } else {
    const hint = splash.querySelector(".intro-splash-skip-hint");
    if (hint) hint.textContent = skipText;
    splash.classList.add("splash-animate");

    let updater = splash.querySelector("#splash-updater");
    if (!updater) {
      updater = document.createElement("div");
      updater.className = "intro-splash-updater";
      updater.id = "splash-updater";
      updater.innerHTML = [
        '<div class="splash-updater-row">',
        '  <div class="splash-updater-left">',
        `    <span class="splash-updater-status" id="splash-updater-status">${escapeHtml(checkingText)}</span>`,
        '  </div>',
        '  <div class="splash-updater-right">',
        '    <span class="splash-updater-pct" id="splash-updater-pct" style="display:none"></span>',
        `    <button class="splash-updater-manual" id="splash-updater-manual" type="button" style="display:none">${escapeHtml(manualDownloadText)}</button>`,
        '  </div>',
        '</div>',
        '<div class="splash-updater-track" id="splash-updater-track">',
        '  <div class="splash-updater-fill indeterminate" id="splash-updater-fill" style="width: 100%"></div>',
        '</div>'
      ].join("");
      if (hint) {
        splash.insertBefore(updater, hint);
      } else {
        splash.appendChild(updater);
      }
    } else {
      const statusEl = updater.querySelector("#splash-updater-status");
      if (statusEl) statusEl.textContent = checkingText;
      const manualEl = updater.querySelector("#splash-updater-manual");
      if (manualEl) manualEl.textContent = manualDownloadText;
    }
  }

  const manualBtn = splash.querySelector("#splash-updater-manual");
  if (manualBtn) {
    manualBtn.onclick = (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (typeof onManualDownload === "function") {
        onManualDownload();
      }
    };
  }

  clickListener = function onClick(e) {
    if (e.target && e.target.closest("#splash-updater-manual")) {
      return;
    }
    if (isDownloadingUpdate) {
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    doFinishSplash();
  };

  keyListener = function onKeyDown(e) {
    if (e.key === "Escape" || e.key === " " || e.key === "Enter") {
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

  // Safety fallback: if no update is checking/downloading after 4.5s, finish splash smoothly
  autoTimeout = setTimeout(() => {
    if (!isDownloadingUpdate) {
      doFinishSplash();
    }
  }, 4500);
}
