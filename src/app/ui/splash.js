import { translate } from "../core/i18n.js";

export function runIntroSplash(options = {}) {
  const { onFinish, skipAnimation = false } = options;

  let splash = document.getElementById("intro-splash-screen");

  if (skipAnimation || document.body.classList.contains("no-animations")) {
    if (splash) splash.remove();
    if (typeof onFinish === "function") onFinish();
    return;
  }

  const currentLang = document.documentElement.lang || localStorage.getItem("synchro_language") || "en";
  const isRu = currentLang === "ru";
  const skipText = translate(currentLang, "skipSplash") || (isRu ? "Нажмите Esc или кликните для пропуска" : "Press Esc or click to skip");

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
      `<div class="intro-splash-skip-hint">${skipText}</div>`
    ].join("");
    document.body.appendChild(splash);
  } else {
    const hint = splash.querySelector(".intro-splash-skip-hint");
    if (hint) hint.textContent = skipText;
    splash.classList.add("splash-animate");
  }

  let finished = false;
  let exitTimeout = null;
  let autoTimeout = null;

  function finishSplash() {
    if (finished) return;
    finished = true;

    if (autoTimeout) clearTimeout(autoTimeout);
    if (exitTimeout) clearTimeout(exitTimeout);

    window.removeEventListener("keydown", onKeyDown, true);
    splash.removeEventListener("click", onClick, true);

    splash.classList.add("splash-fade-out");
    exitTimeout = setTimeout(() => {
      splash.remove();
      if (typeof onFinish === "function") onFinish();
    }, 280);
  }

  function onClick(e) {
    e.preventDefault();
    e.stopPropagation();
    finishSplash();
  }

  function onKeyDown(e) {
    if (e.key === "Escape" || e.key === " " || e.key === "Enter") {
      e.preventDefault();
      e.stopPropagation();
      finishSplash();
    }
  }

  splash.addEventListener("click", onClick, true);
  window.addEventListener("keydown", onKeyDown, true);

  const img = splash.querySelector(".intro-splash-logo-img");
  const launchAnim = () => {
    if (finished) return;
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

  autoTimeout = setTimeout(() => {
    finishSplash();
  }, 2400);
}
