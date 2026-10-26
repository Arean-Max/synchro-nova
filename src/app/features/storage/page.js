import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

const LOCALE_MAP = {
  en: "en-US",
  ru: "ru-RU",
  zh: "zh-CN",
  zht: "zh-TW",
  ko: "ko-KR",
  de: "de-DE",
  es: "es-ES",
  fr: "fr-FR",
  it: "it-IT",
  da: "da-DK",
  ja: "ja-JP",
  pl: "pl-PL",
  bs: "bs-BA",
  ar: "ar-SA",
  no: "nb-NO",
  br: "pt-BR",
  th: "th-TH",
  tr: "tr-TR",
  uk: "uk-UA",
  bn: "bn-BD",
  gr: "el-GR",
  vi: "vi-VN"
};

function formatDate(seconds, currentLang) {
  if (!seconds) return "-";
  const code = currentLang || lang();
  const locale = LOCALE_MAP[code] || "en-US";
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
  const defaultPlaceholder = `${t("backupPrefix") || (isRu ? "Бэкап" : "Backup")} ${now.toLocaleDateString(isRu ? "ru-RU" : undefined)} ${now.toLocaleTimeString(isRu ? "ru-RU" : undefined, { hour: "2-digit", minute: "2-digit" })}`;

  return `
    <div class="backup-modal-backdrop" data-action="dismiss-backup-modal">
      <div class="template-config-dialog backup-name-dialog" role="dialog" aria-modal="true" aria-labelledby="backup-dialog-title">
        <div class="template-config-header">
          <div class="template-config-title">
            <h3 id="backup-dialog-title">${escapeHtml(t("backupNotificationTitle") || (isRu ? "Снимок системы" : "Safety Snapshot"))}</h3>
            <p>${escapeHtml(t("backupsSubtitle") || (isRu ? "Создание точки восстановления перед настройкой системы" : "Create a system settings snapshot before tweaking"))}</p>
          </div>
          <button class="dialog-close-btn" type="button" data-action="dismiss-backup-modal" title="${escapeAttr(t("close"))}">${icon("x")}</button>
        </div>
        <div class="template-config-body">
          <div class="template-name-group">
            <label class="template-name-label" for="custom-backup-name-input">${escapeHtml(t("backupName") || (isRu ? "Имя снимка" : "Snapshot Name"))}</label>
            <input id="custom-backup-name-input" class="template-name-input" type="text" data-field="backupName" value="${escapeAttr(viewState.backupName || "")}" placeholder="${escapeAttr(defaultPlaceholder)}" maxlength="50" spellcheck="false" autocomplete="off" autofocus>
          </div>
        </div>
        <div class="template-config-footer">
          <button class="btn btn-outline template-reset-btn" type="button" data-action="dismiss-backup-modal"><span>${escapeHtml(t("cancel"))}</span></button>
          <button class="btn btn-primary template-done-btn" type="button" data-action="confirm-create-backup"><span>${escapeHtml(t("createBackup"))}</span></button>
        </div>
      </div>
    </div>
  `;
}

export function renderBackupsPage(viewState, t) {
  const isSelected = Boolean(viewState.selectedBackup);
  const restoreBtn = isSelected
    ? button(t("restore"), "check", "primary restore-highlight-btn", "restore-backup")
    : "";

  const rows = viewState.backups.length
    ? viewState.backups.map((item) => `<button class="config-row backup-row ${viewState.selectedBackup === item.id ? "selected" : ""}" type="button" data-backup-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedBackup === item.id ? icon("check") : ""}</span><span class="backup-name"><strong>${escapeHtml(item.name)}</strong><small>${formatDate(item.createdAt)}</small></span></button>`).join("")
    : `<div class="empty-state">${escapeHtml(t("noBackups"))}</div>`;

  return `
    <div class="storage-page-container">
      <section class="card backup-card">
        <h2>${escapeHtml(t("backupManager"))}</h2>
        <div class="toolbar-row backup-tools">
          ${button(t("createBackup"), "save", "outline", "create-backup")}
          ${restoreBtn}
          ${button(t("deleteBackup"), "trash", "outline", "delete-backup")}
          ${button(t("openFolder"), "folder", "outline", "open-backups-folder")}
        </div>
        <div class="config-list backup-list">${rows}</div>
      </section>
    </div>
  `;
}

export function renderConfigsPage(viewState, t) {
  const rows = viewState.configs?.length
    ? viewState.configs.map((item) => `<button class="config-row ${viewState.selectedConfig === item.id ? "selected" : ""}" type="button" data-config-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedConfig === item.id ? icon("check") : ""}</span><span>${escapeHtml(item.name)}</span><em>${escapeHtml(formatColorSummary(item.color))}</em></button>`).join("")
    : `<div class="empty-state">${escapeHtml(t("noConfigs"))}</div>`;
  return `<section class="card configs-card"><h2>${escapeHtml(t("savedConfigs"))}</h2><div class="toolbar-row config-name-row"><input class="text-input wide" data-field="configName" value="${escapeAttr(viewState.configName)}" placeholder="${escapeAttr(t("configName"))}" aria-label="${escapeAttr(t("configName"))}">${button(t("saveConfig"), "save", "primary", "save-config")}</div><div class="toolbar-row config-tools">${button(t("loadConfig"), "folder", "outline", "load-config")}${button(t("applyConfig"), "check", "outline", "apply-config")}${button(t("deleteConfig"), "trash", "outline", "delete-config")}${button(t("openFolder"), "folder", "outline", "open-configs-folder")}</div><div class="config-list">${rows}</div></section>`;
}

export const backupsFeature = {
  render: renderBackupsPage,
  mount(container, context) {
    if (context?.loadLists) {
      context.loadLists();
    }
  },
  unmount() {}
};
