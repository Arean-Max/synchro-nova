import { escapeHtml, escapeAttr } from "../../core/html.js";
import { icon } from "../../ui/icons.js";
import { lang } from "../../core/state.js";
import { allTweaks, tweakTitle } from "./catalog.js";

export function renderApplyModal(modalState, t) {
  if (!modalState) return "";

  const phase = modalState.phase || "loading";
  const progress = Math.min(100, Math.max(0, Math.round(modalState.progress || 0)));
  const isComplete = phase === "complete";
  const isRollingBack = phase === "rollingBack";

  if (!isComplete) {
    const title = isRollingBack ? t("rollingBackTitle") : t("applyingTweaksTitle");
    const sub = isRollingBack ? t("rollingBackSubtitle") : t("applyingTweaksSubtitle");

    return [
      '<div class="apply-modal-backdrop" id="apply-modal-backdrop">',
      '  <div class="apply-modal-loading">',
      '    <div class="apply-spinner-wrapper">',
      '      <div class="apply-spinner-ring"></div>',
      '      <div class="apply-spinner-glow"></div>',
      `      <div class="apply-spinner-icon">${icon("zap")}</div>`,
      '    </div>',
      `    <h3 class="apply-modal-title">${escapeHtml(title)}</h3>`,
      `    <p class="apply-modal-sub">${escapeHtml(sub)}</p>`,
      '    <div class="apply-progress-track">',
      `      <div class="apply-progress-fill" style="width: ${progress}%;"></div>`,
      '    </div>',
      `    <div class="apply-progress-pct">${progress}%</div>`,
      '  </div>',
      '</div>'
    ].join("");
  }

  // Complete phase
  const isRu = lang() === "ru";
  const results = Array.isArray(modalState.results) ? modalState.results : [];
  const tweaksMap = new Map(allTweaks().map((tw) => [tw.id, tw]));

  const appliedCount = results.filter((r) => r.status === "applied").length;
  const adminCount = results.filter((r) => r.status === "requiresAdmin").length;

  const rows = results.map((item) => {
    const tweak = tweaksMap.get(item.id);
    const title = tweak ? tweakTitle(tweak, t) : (item.id || "-");

    let statusClass = "applied";
    let iconContent = icon("check");
    let statusTag = isRu ? "Применено" : "Applied";
    let desc = item.message || (isRu ? "Оптимизация активна" : "Tweak enabled");

    if (item.status === "requiresAdmin") {
      statusClass = "requires-admin";
      iconContent = icon("shield");
      statusTag = isRu ? "Нужен админ" : "Requires Admin";
      desc = isRu ? "Требуются права администратора" : "Requires administrator privileges";
    } else if (item.status === "skipped") {
      statusClass = "skipped";
      iconContent = icon("minus");
      statusTag = isRu ? "Пропущено" : "Skipped";
    } else if (item.status === "failed") {
      statusClass = "failed";
      iconContent = icon("x");
      statusTag = isRu ? "Ошибка" : "Failed";
    }

    return [
      `<div class="apply-result-item ${escapeAttr(statusClass)}">`,
      '  <div class="apply-result-left">',
      `    <button class="tweak-help-btn apply-result-help-btn" type="button" data-action="show-tweak-impact" data-tweak-id="${escapeAttr(item.id)}" title="${escapeAttr(t("whatBreaks") || "?")}">?</button>`,
      '    <div class="apply-result-info">',
      `      <strong class="apply-result-title">${escapeHtml(title)}</strong>`,
      `      <span class="apply-result-desc">${escapeHtml(desc)}</span>`,
      '    </div>',
      '  </div>',
      `  <span class="apply-result-badge ${escapeAttr(statusClass)}">${escapeHtml(statusTag)}</span>`,
      '</div>'
    ].join("");
  }).join("");

  const headerTitle = adminCount > 0
    ? (isRu ? "Применение завершено" : "Execution Finished")
    : (isRu ? "Настройки успешно применены" : "Settings Successfully Applied");

  const statBadges = [
    `<span class="apply-stat-pill applied">${appliedCount} ${isRu ? "применено" : "applied"}</span>`,
    adminCount > 0
      ? `<span class="apply-stat-pill admin">${icon("shield")} ${adminCount} ${isRu ? "требуют прав админа" : "requires admin"}</span>`
      : ""
  ].filter(Boolean).join("");

  const adminNotice = adminCount > 0 ? [
    '<div class="apply-admin-notice">',
    '  <div class="apply-admin-notice-content">',
    `    <span class="apply-admin-notice-icon">${icon("shield")}</span>`,
    `    <span>${isRu ? "Для системных твиков требуются права администратора" : "Administrator rights required for system tweaks"}</span>`,
    '  </div>',
    `  <button class="apply-admin-elevate-btn" type="button" data-action="restart-as-admin">${isRu ? "Перезапустить как админ" : "Restart as admin"}</button>`,
    '</div>'
  ].join("") : "";

  return [
    '<div class="apply-modal-backdrop" id="apply-modal-backdrop" data-action="dismiss-apply-modal">',
    '  <div class="apply-modal-complete" onclick="event.stopPropagation()">',
    '    <div class="apply-complete-head">',
    '      <div class="apply-complete-titles">',
    `        <h2>${escapeHtml(headerTitle)}</h2>`,
    `        <div class="apply-stat-pills">${statBadges}</div>`,
    '      </div>',
    '    </div>',
    '    <div class="apply-complete-list">',
    rows || `<div class="apply-result-empty">${escapeHtml(t("noSelection"))}</div>`,
    '    </div>',
    adminNotice,
    '    <div class="apply-complete-actions">',
    `      <button class="btn-apply-ok" type="button" data-action="dismiss-apply-modal">${escapeHtml(t("okButton") || "OK")}</button>`,
    `      <button class="btn-apply-rollback" type="button" data-action="rollback-from-apply-modal">${escapeHtml(t("rollbackButton") || (isRu ? "Откатить" : "Rollback"))}</button>`,
    '    </div>',
    '  </div>',
    '</div>'
  ].join("");
}
