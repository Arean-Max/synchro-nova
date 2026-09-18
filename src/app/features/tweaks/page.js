import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import {
  allTweaks,
  tweakAppImpacts,
  tweakCategory,
  tweakCatalog,
  tweakDescription,
  tweakGroupTitle,
  tweakNote,
  tweakTitle
} from "./catalog.js";

function tweakTile(tweak, viewState, t) {
  const selected = viewState.selectedTweaks.has(tweak.id);
  const installed = viewState.installedTweaks?.has(tweak.id);
  const category = tweakCategory(tweak);
  const title = tweakTitle(tweak, t);
  const desc = tweakDescription(tweak, t);
  const noteText = tweakNote(tweak, t) || t("noKnownConflict");
  const note = `<div class="tweak-note"><span>!</span><em>${escapeHtml(noteText)}</em></div>`;

  return `<button class="tweak-tile ${selected ? "selected" : ""} ${installed ? "installed" : "not-installed"}" type="button" data-tweak-id="${escapeAttr(tweak.id)}" data-category="${category}"><div class="tweak-head"><span class="tweak-title">${escapeHtml(title)}</span></div><p>${escapeHtml(desc)}</p>${note}</button>`;
}

function tweakGroup(group, viewState, t) {
  const filter = viewState.tweakFilter || "all";
  let tweaks = group.tweaks;
  if (filter === "user") {
    tweaks = tweaks.filter((tw) => !tw.badges?.includes("ADMIN"));
  } else if (filter === "admin") {
    tweaks = tweaks.filter((tw) => tw.badges?.includes("ADMIN"));
  }
  if (!tweaks.length) return "";
  const tiles = tweaks.map((tweak) => tweakTile(tweak, viewState, t)).join("");
  const title = tweakGroupTitle(group, t);
  return `<section class="card tweak-group"><h2 class="group-title">${icon(group.icon)}<span>${escapeHtml(title)}</span></h2><div class="tweak-grid">${tiles}</div></section>`;
}

function tweakResults(viewState, t) {
  const results = Array.isArray(viewState.tweakResults) ? viewState.tweakResults : [];
  if (!results.length) return "";
  const rows = results
    .slice(0, 8)
    .map((item) => {
      const statusKey = `status${String(item.status || "").replace(/^./, (ch) => ch.toUpperCase())}`;
      return `<div class="tweak-result-row ${escapeAttr(item.status || "skipped")}"><span>${escapeHtml(item.id || "-")}</span><strong>${escapeHtml(t(statusKey))}</strong><em>${escapeHtml(item.message || "")}</em></div>`;
    })
    .join("");
  return `<section class="card tweak-results"><h2>${t("appliedTweaks")}</h2>${rows}</section>`;
}

function renderSmartTipsModal(viewState, t) {
  const isRu = lang() === "ru";
  const apps = Array.isArray(viewState.detectedApps) ? viewState.detectedApps : [];
  const appMap = new Map(apps.map((a) => [a.id, a.installed]));

  const appChips = apps.length
    ? apps
        .map((app) => {
          const stateClass = app.installed ? "present" : "absent";
          const stateLabel = app.installed ? t("detectedOnPc") : t("notDetectedOnPc");
          return `<div class="app-chip ${stateClass}"><span class="chip-status-icon">${app.installed ? icon("check") : icon("minus")}</span><div class="chip-info"><strong>${escapeHtml(app.name)}</strong><small>${escapeHtml(stateLabel)}</small></div></div>`;
        })
        .join("")
    : `<div class="empty-state">${t("loading")}</div>`;

  const tweaks = allTweaks();
  const tweakMap = new Map(tweaks.map((tw) => [tw.id, tw]));

  const impactRows = Object.entries(tweakAppImpacts)
    .map(([tweakId, info]) => {
      const tweak = tweakMap.get(tweakId);
      const title = tweak ? tweakTitle(tweak, t) : tweakId;
      const appInstalled = appMap.get(info.appId) ?? false;
      const impactText = isRu ? info.impactRu : info.impactEn;
      const statusBadge = appInstalled
        ? `<span class="app-installed-badge">${t("detectedOnPc")}</span>`
        : "";

      return [
        '<div class="impact-card">',
        '<div class="impact-head">',
        `<span class="impact-tweak-name">${escapeHtml(title)}</span>`,
        `<span class="impact-app-tag">${escapeHtml(info.appName)}</span>`,
        statusBadge,
        '</div>',
        `<div class="impact-body">${escapeHtml(impactText)}</div>`,
        '</div>'
      ].join("");
    })
    .join("");

  return [
    '<div class="smart-tips-backdrop" data-action="close-smart-tips">',
    '<div class="smart-tips-dialog" onclick="event.stopPropagation()">',
    '<div class="smart-tips-dialog-header">',
    '<div class="smart-tips-dialog-title">',
    `<span class="smart-tips-glyph big">?</span>`,
    `<div><h3>${escapeHtml(t("smartTipsTitle"))}</h3><p>${escapeHtml(t("smartTipsDesc"))}</p></div>`,
    '</div>',
    `<button class="dialog-close-btn" type="button" data-action="close-smart-tips" title="${escapeAttr(t("close"))}">${icon("x")}</button>`,
    '</div>',
    '<div class="smart-tips-dialog-body">',
    '<div class="smart-tips-section">',
    `<h4 class="smart-tips-section-title">${icon("monitor")}<span>${escapeHtml(t("detectedAppsSummary"))}</span></h4>`,
    `<div class="app-chips-grid">${appChips}</div>`,
    '</div>',
    '<div class="smart-tips-section">',
    `<h4 class="smart-tips-section-title">${icon("shield")}<span>${escapeHtml(t("whatWillStopWorking"))}</span></h4>`,
    `<div class="impact-cards-list">${impactRows}</div>`,
    '</div>',
    '</div>',
    '<div class="smart-tips-dialog-footer">',
    `<button class="btn btn-primary" type="button" data-action="close-smart-tips"><span>${escapeHtml(t("closeTips"))}</span></button>`,
    '</div>',
    '</div>',
    '</div>'
  ].join("");
}

function renderAdminElevationModal(t) {
  return [
    '<div class="smart-tips-backdrop" data-action="dismiss-admin-prompt">',
    '<div class="admin-prompt-dialog" onclick="event.stopPropagation()">',
    '<div class="admin-prompt-header">',
    icon("shield", "admin-shield-icon"),
    `<h3>${escapeHtml(t("adminPromptTitle"))}</h3>`,
    '</div>',
    `<p class="admin-prompt-text">${escapeHtml(t("adminPromptDesc"))}</p>`,
    '<div class="admin-prompt-actions">',
    `<button class="btn btn-primary" type="button" data-action="restart-as-admin">${icon("shield")}<span>${escapeHtml(t("restartAsAdmin"))}</span></button>`,
    `<button class="btn btn-outline" type="button" data-action="dismiss-admin-prompt"><span>${escapeHtml(t("continueWithoutAdmin"))}</span></button>`,
    '</div>',
    '</div>',
    '</div>'
  ].join("");
}

export function renderTweaksPage(viewState, t) {
  const groups = tweakCatalog.map((group) => tweakGroup(group, viewState, t)).join("");
  const applyLabel = viewState.applyingTweaks ? t("loading") : t("applySelected");

  const filter = viewState.tweakFilter || "all";

  const topToolbar = [
    '<div class="tweaks-top-toolbar">',
    '  <div class="tweaks-filter-bar">',
    `    <button class="tweak-filter-btn ${filter === "all" ? "active" : ""}" type="button" data-action="filter-tweaks-all"><span>${escapeHtml(t("tweakFilterAll"))}</span></button>`,
    `    <button class="tweak-filter-btn ${filter === "user" ? "active" : ""}" type="button" data-action="filter-tweaks-user">${icon("user")}<span>${escapeHtml(t("tweakFilterUser"))}</span></button>`,
    `    <button class="tweak-filter-btn ${filter === "admin" ? "active" : ""}" type="button" data-action="filter-tweaks-admin">${icon("shield")}<span>${escapeHtml(t("tweakFilterAdmin"))}</span></button>`,
    '  </div>',
    '  <div class="tweaks-toolbar-actions">',
    `    <button class="tweak-action-pill" type="button" data-action="restart-explorer" title="${escapeAttr(t("restartExplorer"))}">${icon("rotate")}<span>${escapeHtml(t("restartExplorer"))}</span></button>`,
    `    <button class="tweak-action-pill" type="button" data-action="restart-graphics-driver" title="${escapeAttr(t("restartGpuDriver"))}">${icon("monitor")}<span>${escapeHtml(t("restartGpuDriver"))}</span></button>`,
    `    <button class="smart-tips-trigger" type="button" data-action="toggle-smart-tips" title="${escapeAttr(t("smartTipsTitle"))}" aria-label="${escapeAttr(t("smartTipsTitle"))}">`,
    '      <span class="smart-tips-badge-icon">?</span>',
    `      <span>${escapeHtml(t("smartTips"))}</span>`,
    '    </button>',
    '  </div>',
    '</div>'
  ].join("");

  return [
    '<div class="tweaks-page">',
    topToolbar,
    `<div class="scroll-panel">${tweakResults(viewState, t)}${groups}</div>`,
    `<div class="actions tweaks-actions">${button(applyLabel, "check", "primary", "apply-tweaks")}${button(t("rollbackTweaks"), "rotate", "outline", "rollback-tweaks")}</div>`,
    viewState.smartTipsOpen ? renderSmartTipsModal(viewState, t) : "",
    viewState.showAdminPrompt ? renderAdminElevationModal(t) : "",
    '</div>'
  ].join("");
}
