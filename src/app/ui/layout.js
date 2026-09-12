import { activePage, appState, booting, pageDefs, pageOrder } from "../core/state.js";
import { renderCharacteristicsPage } from "../features/characteristics/page.js";
import { renderColorPage, renderGameColorPage } from "../features/color/page.js";
import { renderAgreementGate } from "../features/settings/agreement.js";
import { renderSettingsPage } from "../features/settings/page.js";
import { renderBackupsPage, renderConfigsPage } from "../features/storage/page.js";
import { renderTweaksPage } from "../features/tweaks/page.js";
import { logo } from "./icons.js";
import { icon } from "./icons.js";

export function renderTitlebar(t) {
  return [
    '<div class="titlebar" data-window-drag data-tauri-drag-region>',
    '<div class="titlebar-left" data-window-drag data-tauri-drag-region>',
    '<span class="titlebar-logo" data-window-drag data-tauri-drag-region>' + logo() + "</span>",
    '<span class="titlebar-title" data-window-drag data-tauri-drag-region>Synchro Nova</span>',
    "</div>",
    '<div class="window-controls">',
    `<button class="window-btn" type="button" data-action="window-minimize" title="${t("minimize")}">` + icon("minus") + "</button>",
    `<button class="window-btn" type="button" data-action="window-maximize" title="${t("maximize")}">` + icon("square") + "</button>",
    `<button class="window-btn close" type="button" data-action="window-close" title="${t("close")}">` + icon("x") + "</button>",
    "</div>",
    "</div>"
  ].join("");
}

export function renderIntro() {
  return `<div class="intro-screen" aria-hidden="true"><div class="intro-word"><span>Synchro Nova</span><span class="intro-logo">${logo()}</span></div></div>`;
}

export function renderSidebar(t) {
  const nav = pageOrder
    .map((key) => {
      const page = pageDefs[key];
      const active = key === activePage ? "active" : "";
      return `<button class="nav-item ${active}" type="button" data-page="${key}">${icon(page.icon)}<span>${t(page.nav)}</span></button>`;
    })
    .join("");

  return [
    '<aside class="sidebar">',
    '<div class="brand" data-window-drag data-tauri-drag-region>',
    logo("lockup"),
    "</div>",
    '<span class="nav-indicator" aria-hidden="true"></span>',
    `<div class="nav-label">${t("main")}</div>`,
    `<nav class="nav-list">${nav}</nav>`,
    '<div class="sidebar-spacer"></div>',
    `<button class="exit-button" type="button" data-action="exit-app">${icon("logOut")}<span>${t("exit")}</span></button>`,
    "</aside>"
  ].join("");
}

export function renderPageBody(viewState, t) {
  if (activePage === "color") return renderColorPage(appState, viewState, t);
  if (activePage === "gameColor") return renderGameColorPage(appState, viewState, t);
  if (activePage === "tweaks") return renderTweaksPage(viewState, t);
  if (activePage === "characteristics") return renderCharacteristicsPage(viewState, t);
  if (activePage === "backups") return renderBackupsPage(viewState, t);
  if (activePage === "configs") return renderConfigsPage(viewState, t);
  if (activePage === "settings") return renderSettingsPage(appState, t);
  return renderColorPage(appState, viewState, t);
}

export function renderMain(viewState, t) {
  const page = pageDefs[activePage];
  return `<main class="main-panel"><header class="page-header" data-window-drag data-tauri-drag-region><h1 data-window-drag data-tauri-drag-region>${t(page.title)}</h1><p data-window-drag data-tauri-drag-region>${t(page.subtitle)}</p></header><div class="page-body page-${activePage}">${renderPageBody(viewState, t)}</div></main>`;
}

export function renderShell(viewState, t) {
  const intro = booting && !appState.settings.lowSpecMode ? renderIntro() : "";
  const agreement = viewState.agreementVisible && !appState.settings.acceptedAgreement ? renderAgreementGate(t) : "";
  return `${intro}${renderTitlebar(t)}<div class="workspace">${renderSidebar(t)}${renderMain(viewState, t)}</div>${agreement}`;
}
