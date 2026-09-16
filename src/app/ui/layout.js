import { activePage, appState, pageDefs, pageOrder } from "../core/state.js";
import { escapeHtml } from "../core/html.js";
import { renderCharacteristicsPage } from "../features/characteristics/page.js";
import { renderColorPage, renderGameColorPage } from "../features/color/page.js";
import { renderSettingsPage } from "../features/settings/page.js";
import { renderBackupsPage, renderConfigsPage } from "../features/storage/page.js";
import { renderTweaksPage } from "../features/tweaks/page.js";
import { logo, icon } from "./icons.js";

export function renderTitlebar(t) {
  return [
    '<div class="titlebar" data-window-drag data-tauri-drag-region>',
    '<div class="titlebar-left" data-window-drag data-tauri-drag-region>',
    '<span class="titlebar-logo" data-window-drag data-tauri-drag-region>' + logo() + "</span>",
    '<span class="titlebar-title" data-window-drag data-tauri-drag-region>Synchro Nova</span>',
    "</div>",
    '<div class="window-controls">',
    `<button class="window-btn" type="button" data-action="window-minimize" title="${t("minimize")}">` + icon("minus") + "</button>",
    `<button class="window-btn close" type="button" data-action="window-close" title="${t("close")}">` + icon("x") + "</button>",
    "</div>",
    "</div>"
  ].join("");
}

export function renderSidebar(t, viewState) {
  const isCollapsed = Boolean(viewState?.sidebarCollapsed);
  const toggleTitle = isCollapsed ? t("expandSidebar") : t("collapseSidebar");
  const nav = pageOrder
    .map((key) => {
      const page = pageDefs[key];
      const active = key === activePage ? "active" : "";
      const label = t(page.nav);
      return `<button class="nav-item ${active}" type="button" data-page="${key}" title="${escapeHtml(label)}">${icon(page.icon)}<span>${escapeHtml(label)}</span></button>`;
    })
    .join("");

  return [
    '<aside class="sidebar">',
    '<div class="brand" data-window-drag data-tauri-drag-region>',
    logo("lockup"),
    logo("mark"),
    "</div>",
    '<span class="nav-indicator" aria-hidden="true"></span>',
    '<div class="nav-section-head">',
    `<span class="nav-label">${t("main")}</span>`,
    `<button class="sidebar-toggle-btn" type="button" data-action="toggle-sidebar" title="${escapeHtml(toggleTitle)}" aria-label="${escapeHtml(toggleTitle)}">${icon("sidebar")}</button>`,
    "</div>",
    `<nav class="nav-list">${nav}</nav>`,
    '<div class="sidebar-spacer"></div>',
    `<button class="exit-button" type="button" data-action="exit-app" title="${escapeHtml(t("exit"))}">${icon("logOut")}<span>${t("exit")}</span></button>`,
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
  if (activePage === "settings") return renderSettingsPage(appState, viewState, t);
  return renderColorPage(appState, viewState, t);
}

export function renderMain(viewState, t) {
  const page = pageDefs[activePage];
  return `<main class="main-panel"><header class="page-header" data-window-drag data-tauri-drag-region><h1 data-window-drag data-tauri-drag-region>${t(page.title)}</h1><p data-window-drag data-tauri-drag-region>${t(page.subtitle)}</p></header><div class="page-body page-${activePage}">${renderPageBody(viewState, t)}</div></main>`;
}

export function renderShell(viewState, t) {
  const collapsed = viewState?.sidebarCollapsed ? " sidebar-collapsed" : "";
  return `${renderTitlebar(t)}<div class="workspace${collapsed}">${renderSidebar(t, viewState)}${renderMain(viewState, t)}</div>`;
}
