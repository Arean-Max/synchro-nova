import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

function formatDate(seconds, currentLang) {
  if (!seconds) return "-";
  const locale = (currentLang || lang()) === "en" ? "en-US" : "ru-RU";
  return new Intl.DateTimeFormat(locale, {
    day: "2-digit",
    month: "2-digit",
    year: "numeric"
  }).format(new Date(seconds * 1000));
}

function formatColorSummary(color) {
  if (!color) return "-";
  return `S ${Math.round(color.saturation)} / H ${Math.round(color.hue)} / C ${Math.round(color.contrast)} / G ${Math.round(color.gamma)}`;
}

export function renderBackupNameModal(viewState, t) {
  const isRu = lang() === "ru";
  const now = new Date();
  const defaultPlaceholder = isRu
    ? `Бэкап ${now.toLocaleDateString("ru-RU")} ${now.toLocaleTimeString("ru-RU", { hour: "2-digit", minute: "2-digit" })}`
    : `Backup ${now.toLocaleDateString("en-US")} ${now.toLocaleTimeString("en-US", { hour: "2-digit", minute: "2-digit" })}`;

  return [
    '<div class="backup-modal-backdrop" data-action="dismiss-backup-modal">',
    '  <div class="backup-minimal-dialog" onclick="event.stopPropagation()">',
    `    <input id="custom-backup-name-input" class="backup-minimal-input" type="text" data-field="backupName" value="${escapeAttr(viewState.backupName || "")}" placeholder="${escapeAttr(defaultPlaceholder)}" maxlength="50" spellcheck="false" autocomplete="off" autofocus>`,
    '  </div>',
    '</div>'
  ].join("");
}

export function renderBackupsPage(viewState, t) {
  const isRu = lang() === "ru";
  const isSelected = Boolean(viewState.selectedBackup);
  const restoreBtn = isSelected
    ? button(t("restore"), "check", "primary restore-highlight-btn", "restore-backup")
    : `<button class="btn btn-outline restore-dimmed" type="button" data-action="restore-backup" disabled title="${escapeAttr(t("selectBackupFirst") || (isRu ? "Сначала выберите бэкап" : "Select a backup from the list first"))}">${icon("check")}<span>${escapeHtml(t("restore"))}</span></button>`;
