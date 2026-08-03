import { activePage, appState, pageDefs, pageOrder } from "../core/state.js";
import { escapeHtml } from "../core/html.js";
import { renderCharacteristicsPage } from "../features/characteristics/page.js";
import { renderColorPage, renderGameColorPage } from "../features/color/page.js";
import { renderSettingsPage } from "../features/settings/page.js";
import { renderBackupsPage, renderConfigsPage } from "../features/storage/page.js";
import { renderTweaksPage } from "../features/tweaks/page.js";
import { renderBindsPage } from "../features/binds/page.js";
import { logo, icon } from "./icons.js";

function renderTitlebar(t) {
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

export function renderSidebarUpdateWidget(t, viewState) {
  const status = viewState?.updateStatus || "hidden";
  if (status === "hidden") {
    return '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper hidden"></div>';
  }

  if (status === "available") {
    const versionText = viewState?.latestVersion ? `v${viewState.latestVersion}` : "";
    return [
      '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper available">',
      `  <button class="update-action-btn" type="button" data-action="start-update-download" title="${t("downloadUpdate")}">`,
      `    ${icon("download")}`,
      `    <span class="update-btn-label">${t("downloadUpdate")}</span>`,
      versionText ? `    <span class="update-version-tag">${versionText}</span>` : "",
      "  </button>",
      "</div>"
    ].join("");
  }

  if (status === "downloading") {
    const pct = viewState?.updatePercent || 0;
    return [
      '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper downloading">',
      '  <div class="update-progress-container">',
      '    <div class="update-progress-header">',
      `      <span class="update-progress-title">${t("downloading")}</span>`,
      `      <span class="update-progress-pct">${pct}%</span>`,
      `      <button class="manual-download-link" type="button" data-action="open-manual-download" title="${t("manualDownload")}">${t("manualDownload")}</button>`,
      '    </div>',
      '    <div class="update-progress-bar-track">',
      `      <div class="update-progress-bar-fill" style="width:${pct}%"></div>`,
      '    </div>',
      '  </div>',
      '</div>'
    ].join("");
  }

  if (status === "ready") {
    return [
      '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper ready">',
      `  <button class="update-action-btn install-ready-btn" type="button" data-action="apply-update-install" title="${t("installUpdate")}">`,
      `    ${icon("check")}`,
      `    <span class="update-btn-label">${t("installUpdate")}</span>`,
      "  </button>",
      "</div>"
    ].join("");
  }

  if (status === "error") {
    return [
      '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper error">',
      '  <div class="update-error-container">',
      `    <span class="update-error-text">${t("updateError")}</span>`,
      `    <button class="manual-download-link" type="button" data-action="open-manual-download" title="${t("manualDownload")}">${t("manualDownload")}</button>`,
      '  </div>',
      '</div>'
    ].join("");
  }

  return '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper hidden"></div>';
}
