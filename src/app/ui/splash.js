export function runIntroSplash(options = {}) {
  const { onFinish, skipAnimation = false } = options;

  if (skipAnimation || document.body.classList.contains("no-animations")) {
    if (typeof onFinish === "function") onFinish();
    return;
  }

  const existing = document.getElementById("intro-splash-screen");
  if (existing) existing.remove();

  const isRu = document.documentElement.lang === "ru" || localStorage.getItem("synchro_language") === "ru";
  const skipText = isRu ? "Нажмите Esc или кликните для пропуска" : "Press Esc or click to skip";

  const lettersSynchro = ["S", "y", "n", "c", "h", "r", "o"];
  const lettersNova = ["N", "o", "v", "a"];

  const synchroHtml = lettersSynchro
    .map((ch, i) => `<span class="splash-char" style="--char-idx: ${i}">${ch}</span>`)
    .join("");

  const novaHtml = lettersNova
    .map((ch, i) => `<span class="splash-char" style="--char-idx: ${i + 7}">${ch}</span>`)
    .join("");

  const splash = document.createElement("div");
  splash.id = "intro-splash-screen";
  splash.className = "intro-splash-screen";
  splash.innerHTML = [
    '<div class="intro-splash-backdrop"></div>',
    '<div class="intro-splash-stage">',
    '  <div class="intro-splash-logo-wrap">',
    '    <img class="intro-splash-logo-img" src="./assets/synchro-nova-splash.png" alt="Synchro Nova" />',
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
    }, 260);
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

  requestAnimationFrame(() => {
    splash.classList.add("splash-animate");
  });

  autoTimeout = setTimeout(() => {
    finishSplash();
  }, 2950);
}
