import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

export function formatDate(seconds, currentLang) {
  if (!seconds) return "-";
  const locale = (currentLang || lang()) === "en" ? "en-US" : "ru-RU";
  return new Intl.DateTimeFormat(locale, {
    day: "2-digit",
    month: "2-digit",
    year: "numeric"
  }).format(new Date(seconds * 1000));
}

export function formatColorSummary(color) {
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

  const rows = viewState.backups.length
    ? viewState.backups.map((item) => `<button class="config-row backup-row ${viewState.selectedBackup === item.id ? "selected" : ""}" type="button" data-backup-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedBackup === item.id ? icon("check") : ""}</span><span class="backup-name"><strong>${escapeHtml(item.name)}</strong><small>${formatDate(item.createdAt)}</small></span></button>`).join("")
    : `<div class="empty-state">${t("noBackups")}</div>`;

  return [
    '<section class="card backup-card">',
    `<h2>${t("backupManager")}</h2>`,
    '<div class="toolbar-row backup-tools">',
    button(t("createBackup"), "save", "outline", "create-backup"),
    restoreBtn,
    button(t("deleteBackup"), "trash", "outline", "delete-backup"),
    button(t("openFolder"), "folder", "outline", "open-backups-folder"),
    '</div>',
    `<div class="config-list backup-list">${rows}</div>`,
    '</section>'
  ].join("");
}

export function renderConfigsPage(viewState, t) {
  const rows = viewState.configs.length
    ? viewState.configs.map((item) => `<button class="config-row ${viewState.selectedConfig === item.id ? "selected" : ""}" type="button" data-config-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedConfig === item.id ? icon("check") : ""}</span><span>${escapeHtml(item.name)}</span><em>${escapeHtml(formatColorSummary(item.color))}</em></button>`).join("")
    : `<div class="empty-state">${t("noConfigs")}</div>`;
  return `<section class="card configs-card"><h2>${t("savedConfigs")}</h2><div class="toolbar-row config-name-row"><input class="text-input wide" data-field="configName" value="${escapeAttr(viewState.configName)}" placeholder="${escapeAttr(t("configName"))}" aria-label="${escapeAttr(t("configName"))}">${button(t("saveConfig"), "save", "primary", "save-config")}</div><div class="toolbar-row config-tools">${button(t("loadConfig"), "folder", "outline", "load-config")}${button(t("applyConfig"), "check", "outline", "apply-config")}${button(t("deleteConfig"), "trash", "outline", "delete-config")}${button(t("openFolder"), "folder", "outline", "open-configs-folder")}</div><div class="config-list">${rows}</div></section>`;
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
