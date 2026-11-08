import { safeRender } from "../core/errorBoundary.js";
import { activePage, appState, lang, pageDefs, pageOrder } from "../core/state.js";
import { router } from "../core/router.js";
import { escapeAttr, escapeHtml } from "../core/html.js";
import { renderCharacteristicsPage, DRIVER_CATEGORIES } from "../features/characteristics/page.js";
import { renderColorPage } from "../features/color/page.js";
import { renderSettingsPage } from "../features/settings/page.js";
import { renderBackupsPage } from "../features/storage/page.js";
import { renderTweaksPage } from "../features/tweaks/page.js";
import { tweakCatalog, tweakGroupTitle } from "../features/tweaks/catalog.js";
import { renderBindsPage } from "../features/binds/page.js";
import { logo, icon } from "./icons.js";

function renderTitlebar(t) {
  const isMacLights = Boolean(appState?.settings?.macTrafficLights);

  const controls = isMacLights
    ? `
      <div class="window-controls mac-traffic-lights">
        <button class="mac-light minimize" type="button" data-action="window-minimize" title="${escapeHtml(t("minimize"))}" aria-label="${escapeHtml(t("minimize"))}">
          <span class="mac-light-symbol">−</span>
        </button>
        <button class="mac-light close" type="button" data-action="window-close" title="${escapeHtml(t("close"))}" aria-label="${escapeHtml(t("close"))}">
          <span class="mac-light-symbol">×</span>
        </button>
      </div>
    `
    : `
      <div class="window-controls">
        <button class="window-btn" type="button" data-action="window-minimize" title="${escapeHtml(t("minimize"))}">${icon("minus")}</button>
        <button class="window-btn close" type="button" data-action="window-close" title="${escapeHtml(t("close"))}">${icon("x")}</button>
      </div>
    `;

  return `
    <div class="titlebar ${isMacLights ? "has-mac-lights" : ""}" data-window-drag data-tauri-drag-region>
      <div class="titlebar-left" data-window-drag data-tauri-drag-region></div>
      ${controls}
    </div>
  `;
}

export function renderSidebarUpdateWidget(t, viewState) {
  const status = viewState?.updateStatus || "hidden";
  if (status === "hidden") {
    return '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper hidden"></div>';
  }

  if (status === "available") {
    const versionText = viewState?.latestVersion ? `v${viewState.latestVersion}` : "";
    return `
      <div id="sidebar-update-wrapper" class="sidebar-update-wrapper available">
        <button class="update-action-btn" type="button" data-action="start-update-download" title="${escapeHtml(t("downloadUpdate"))}">
          ${icon("download")}
          <span class="update-btn-label">${escapeHtml(t("downloadUpdate"))}</span>
          ${versionText ? `<span class="update-version-tag">${escapeHtml(versionText)}</span>` : ""}
        </button>
      </div>
    `;
  }

  if (status === "downloading") {
    const pct = viewState?.updatePercent || 0;
    return `
      <div id="sidebar-update-wrapper" class="sidebar-update-wrapper downloading">
        <div class="update-progress-container">
          <div class="update-progress-header">
            <span class="update-progress-title">${escapeHtml(t("downloading"))}</span>
            <span class="update-progress-pct">${pct}%</span>
            <button class="manual-download-link" type="button" data-action="open-manual-download" title="${escapeHtml(t("manualDownload"))}">${escapeHtml(t("manualDownload"))}</button>
          </div>
          <div class="update-progress-bar-track">
            <div class="update-progress-bar-fill" style="width:${pct}%"></div>
          </div>
        </div>
      </div>
    `;
  }

  if (status === "ready") {
    return `
      <div id="sidebar-update-wrapper" class="sidebar-update-wrapper ready">
        <button class="update-action-btn install-ready-btn" type="button" data-action="apply-update-install" title="${escapeHtml(t("installUpdate"))}">
          ${icon("check")}
          <span class="update-btn-label">${escapeHtml(t("installUpdate"))}</span>
        </button>
      </div>
    `;
  }

  if (status === "installing") {
    return `
      <div id="sidebar-update-wrapper" class="sidebar-update-wrapper installing">
        <div class="update-progress-container">
          <div class="update-progress-header">
            <span class="update-progress-title">${escapeHtml(t("installing"))}</span>
          </div>
        </div>
      </div>
    `;
  }

  if (status === "error") {
    return `
      <div id="sidebar-update-wrapper" class="sidebar-update-wrapper error">
        <div class="update-error-container">
          <span class="update-error-text">${escapeHtml(t("updateError"))}</span>
          <button class="manual-download-link" type="button" data-action="open-manual-download" title="${escapeHtml(t("manualDownload"))}">${escapeHtml(t("manualDownload"))}</button>
        </div>
      </div>
    `;
  }

  return '<div id="sidebar-update-wrapper" class="sidebar-update-wrapper hidden"></div>';
}

export function renderNavList(t, viewState) {
  const isRu = lang() === "ru";
  const isCollapsed = Boolean(viewState?.sidebarCollapsed);

  return pageOrder
    .map((key) => {
      const page = pageDefs[key];
      const active = key === activePage ? "active" : "";
      const label = t(page.nav);
      const hasSubNav = key === "tweaks" || key === "characteristics";
      const topBtn = `<button class="nav-item ${active}" type="button" data-page="${key}" title="${escapeHtml(label)}">${icon(page.icon)}<span>${escapeHtml(label)}</span></button>`;

      const isSubNavCollapsed = Boolean(viewState?.collapsedSubNav?.[key]);
      if (isCollapsed || key !== activePage || isSubNavCollapsed) {
        return topBtn;
      }

      let subItems = "";
      if (key === "tweaks") {
        subItems = tweakCatalog
          .map((group, index) => {
            const title = tweakGroupTitle(group, t);
            const sectionId = `tweak-section-${group.id}`;
            const isSubActive = viewState?.activeSubSection ? viewState.activeSubSection === sectionId : index === 0;
            return `<button class="nav-sub-item ${isSubActive ? "active" : ""}" type="button" data-action="scroll-to-section" data-section="${escapeAttr(sectionId)}" title="${escapeAttr(title)}"><span>${escapeHtml(title)}</span></button>`;
          })
          .join("");
      } else if (key === "characteristics") {
        subItems = DRIVER_CATEGORIES
          .map((cat, index) => {
            const title = t("driverGroup_" + cat.id) || (isRu ? cat.titleRu : cat.titleEn);
            const sectionId = `driver-section-${cat.id}`;
            const isSubActive = viewState?.activeSubSection ? viewState.activeSubSection === sectionId : index === 0;
            return `<button class="nav-sub-item ${isSubActive ? "active" : ""}" type="button" data-action="scroll-to-section" data-section="${escapeAttr(sectionId)}" title="${escapeAttr(title)}"><span>${escapeHtml(title)}</span></button>`;
          })
          .join("");
      }

      if (!subItems) {
        return topBtn;
      }

      return `${topBtn}<div class="nav-sub-list">${subItems}</div>`;
    })
    .join("");
}

function renderSidebar(t, viewState) {
  const isCollapsed = Boolean(viewState?.sidebarCollapsed);
  const toggleTitle = isCollapsed ? t("expandSidebar") : t("collapseSidebar");
  const nav = renderNavList(t, viewState);

  return `
    <aside class="sidebar">
      <div class="brand" data-window-drag data-tauri-drag-region>
        ${logo("lockup")}
        ${logo("mark")}
      </div>
      <span class="nav-indicator" aria-hidden="true"></span>
      <div class="nav-section-head">
        <span class="nav-label">${escapeHtml(t("main"))}</span>
        <button class="sidebar-toggle-btn" type="button" data-action="toggle-sidebar" title="${escapeHtml(toggleTitle)}" aria-label="${escapeHtml(toggleTitle)}">${icon("sidebar")}</button>
      </div>
      <nav class="nav-list">${nav}</nav>
      <div class="sidebar-divider"></div>
      <div class="sidebar-secondary-nav">
        <button class="nav-item nav-item-disabled" type="button" disabled aria-disabled="true" title="${escapeAttr(t("rustBetaTitle") && t("rustBetaTitle") !== "rustBetaTitle" ? t("rustBetaTitle") : "Rust (Beta) — Coming Soon")}">
          ${icon("crosshair")}
          <span>Rust <span class="nav-beta-badge">${escapeHtml(t("scriptsBeta") && t("scriptsBeta") !== "scriptsBeta" ? t("scriptsBeta") : "Beta")}</span></span>
        </button>
      </div>
      <div class="sidebar-spacer"></div>
      ${renderSidebarUpdateWidget(t, viewState)}
      <button class="exit-button" type="button" data-action="exit-app" title="${escapeHtml(t("exit"))}">${icon("logOut")}<span>${escapeHtml(t("exit"))}</span></button>
    </aside>
  `;
}

const safeColor = safeRender(renderColorPage, "Color Calibration");
const safeTweaks = safeRender(renderTweaksPage, "System Tweaks");
const safeBinds = safeRender(renderBindsPage, "Keybinds");
const safeCharacteristics = safeRender(renderCharacteristicsPage, "Hardware Drivers");
const safeBackups = safeRender(renderBackupsPage, "System Backups");
const safeSettings = safeRender(renderSettingsPage, "Preferences");

export function renderPageBody(viewState, t, bypassCache = false) {
  if (!bypassCache) {
    const cached = router.getCachedPageHtml(activePage);
    if (cached) return cached;
  }
  let html = "";
  if (activePage === "color") html = safeColor(appState, viewState, t);
  else if (activePage === "tweaks") html = safeTweaks(viewState, t);
  else if (activePage === "binds") html = safeBinds(appState, viewState, t);
  else if (activePage === "characteristics") html = safeCharacteristics(viewState, t);
  else if (activePage === "backups") html = safeBackups(viewState, t);
  else if (activePage === "settings") html = safeSettings(appState, viewState, t);
  else html = safeColor(appState, viewState, t);
  router.cachePageHtml(activePage, html);
  return html;
}

export function renderMain(viewState, t, bypassCache = false) {
  const page = pageDefs[activePage];
  return `<main class="main-panel"><header class="page-header" data-window-drag data-tauri-drag-region><h1 data-window-drag data-tauri-drag-region>${escapeHtml(t(page.title))}</h1><p data-window-drag data-tauri-drag-region>${escapeHtml(t(page.subtitle))}</p></header><div class="page-body page-${activePage}">${renderPageBody(viewState, t, bypassCache)}</div></main>`;
}

export function renderShell(viewState, t) {
  const collapsed = viewState?.sidebarCollapsed ? " sidebar-collapsed" : "";
  return `${renderTitlebar(t)}<div class="workspace${collapsed}">${renderSidebar(t, viewState)}${renderMain(viewState, t)}</div>`;
}

export function resolveActiveSubSection(scrollPanelEl, sections) {
  if (!scrollPanelEl || !sections || !sections.length) return null;

  const scrollHeight = Number(scrollPanelEl.scrollHeight) || 0;
  const clientHeight = Number(scrollPanelEl.clientHeight) || 0;
  const scrollTop = Number(scrollPanelEl.scrollTop) || 0;

  const hasOverflow = scrollHeight > clientHeight + 10;
  const isAtBottom = hasOverflow && (scrollHeight - scrollTop - clientHeight <= 48);

  if (scrollTop <= 10) {
    return sections[0].id || null;
  }
  if (isAtBottom) {
    return sections[sections.length - 1].id || null;
  }

  const panelTop = typeof scrollPanelEl.getBoundingClientRect === "function"
    ? scrollPanelEl.getBoundingClientRect().top
    : 0;

  let currentId = null;
  for (let i = 0; i < sections.length; i++) {
    const sec = sections[i];
    const top = typeof sec.getBoundingClientRect === "function"
      ? sec.getBoundingClientRect().top
      : (sec.offsetTop ?? 0);
    if (top - panelTop <= 100) {
      currentId = sec.id;
    }
  }

  return currentId || sections[0].id || null;
}

