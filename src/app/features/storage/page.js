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

export function renderBackupsPage(viewState, t) {
  const rows = viewState.backups.length
    ? viewState.backups.map((item) => `<button class="config-row backup-row ${viewState.selectedBackup === item.id ? "selected" : ""}" type="button" data-backup-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedBackup === item.id ? icon("check") : ""}</span><span class="backup-name"><strong>${escapeHtml(item.name)}</strong><small>${formatDate(item.createdAt)}</small></span></button>`).join("")
    : `<div class="empty-state">${t("noBackups")}</div>`;
  return `<section class="card backup-card"><h2>${t("backupManager")}</h2><div class="toolbar-row backup-tools"><input class="text-input wide" data-field="backupName" value="${escapeAttr(viewState.backupName)}" placeholder="${escapeAttr(t("backupName"))}" aria-label="${escapeAttr(t("backupName"))}">${button(t("createBackup"), "save", "primary", "create-backup")}${button(t("restore"), "check", "outline", "restore-backup")}${button(t("deleteBackup"), "trash", "outline", "delete-backup")}${button(t("openFolder"), "folder", "outline", "open-backups-folder")}</div><div class="config-list backup-list">${rows}</div></section>`;
}

export function renderConfigsPage(viewState, t) {
  const rows = viewState.configs.length
    ? viewState.configs.map((item) => `<button class="config-row ${viewState.selectedConfig === item.id ? "selected" : ""}" type="button" data-config-id="${escapeAttr(item.id)}"><span class="config-check">${viewState.selectedConfig === item.id ? icon("check") : ""}</span><span>${escapeHtml(item.name)}</span><em>${escapeHtml(formatColorSummary(item.color))}</em></button>`).join("")
    : `<div class="empty-state">${t("noConfigs")}</div>`;
  return `<section class="card configs-card"><h2>${t("savedConfigs")}</h2><div class="toolbar-row config-name-row"><input class="text-input wide" data-field="configName" value="${escapeAttr(viewState.configName)}" placeholder="${escapeAttr(t("configName"))}" aria-label="${escapeAttr(t("configName"))}">${button(t("saveConfig"), "save", "primary", "save-config")}</div><div class="toolbar-row config-tools">${button(t("loadConfig"), "folder", "outline", "load-config")}${button(t("applyConfig"), "check", "outline", "apply-config")}${button(t("deleteConfig"), "trash", "outline", "delete-config")}${button(t("openFolder"), "folder", "outline", "open-configs-folder")}</div><div class="config-list">${rows}</div></section>`;
}
