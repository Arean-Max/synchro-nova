import { escapeAttr, escapeHtml } from "../../core/html.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import { tweakBadges, tweakCatalog } from "./catalog.js";

const badgeKeys = {
  SAFE: "badgeSafe",
  ADMIN: "badgeAdmin",
  REBOOT: "badgeReboot",
  ADVANCED: "badgeAdvanced",
  AGGRESSIVE: "badgeAggressive",
  PRIVACY: "badgePrivacy",
  EXPERIMENTAL: "badgeExperimental",
  VERIFIED: "badgeVerified"
};

function badgeLabel(badge, t) {
  return t(badgeKeys[badge] || badge);
}

function tweakTile(tweak, viewState, t) {
  const badgeList = tweakBadges(tweak);
  const aggressive = badgeList.includes("AGGRESSIVE");
  const selected = viewState.selectedTweaks.has(tweak.id);
  const installed = viewState.installedTweaks?.has(tweak.id);
  const checked = selected || installed;
  const safe = !badgeList.some((badge) => ["ADMIN", "REBOOT", "ADVANCED", "AGGRESSIVE", "EXPERIMENTAL"].includes(badge));
  const badgeHtml = badgeList
    .map((badge) => `<span class="badge badge-${escapeAttr(badge.toLowerCase())}">${escapeHtml(badgeLabel(badge, t))}</span>`)
    .join("");
  const noteText = tweak.note || t("noKnownConflict");
  const note = `<div class="tweak-note"><span>!</span><em>${escapeHtml(noteText)}</em></div>`;
  return `<button class="tweak-tile ${selected ? "selected" : ""} ${installed ? "installed" : ""} ${aggressive ? "aggressive" : ""}" type="button" data-tweak-id="${escapeAttr(tweak.id)}" data-safe="${safe ? "true" : "false"}"><div class="tweak-head"><span class="box ${checked ? "checked" : ""}">${checked ? icon("check", "box-check") : ""}</span><span class="tweak-title">${escapeHtml(tweak.title)}</span></div><p>${escapeHtml(tweak.description)}</p><div class="tweak-badges">${badgeHtml}</div>${note}</button>`;
}

function tweakGroup(group, viewState, t) {
  const tiles = group.tweaks.map((tweak) => tweakTile(tweak, viewState, t)).join("");
  return `<section class="card tweak-group"><h2 class="group-title">${icon(group.icon)}<span>${escapeHtml(group.title)}</span></h2><div class="tweak-grid">${tiles}</div></section>`;
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

export function renderTweaksPage(viewState, t) {
  const groups = tweakCatalog.map((group) => tweakGroup(group, viewState, t)).join("");
  const applyLabel = viewState.applyingTweaks ? t("loading") : t("applySelected");
  return `<div class="tweaks-page"><div class="scroll-panel">${tweakResults(viewState, t)}${groups}</div><div class="actions tweaks-actions">${button(applyLabel, "check", "primary", "apply-tweaks")}${button(t("rollbackTweaks"), "history", "outline", "rollback-tweaks")}${button(t("selectSafe"), "settings", "outline", "select-safe-tweaks")}${button(t("createBackup"), "save", "outline", "create-backup")}${button(t("runAsAdmin"), "shield", "outline", "restart-as-admin")}</div></div>`;
}
