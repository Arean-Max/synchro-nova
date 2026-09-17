import { button, card, checkbox } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

export function renderSettingsPage(appState, viewState, t) {
  const language = `<div class="form-label">${t("language")}</div><div class="segmented-control"><button class="seg-option ${appState.settings.language === "en" ? "active" : ""}" type="button" data-language="en">${t("english")}</button><button class="seg-option ${appState.settings.language === "ru" ? "active" : ""}" type="button" data-language="ru">${t("russian")}</button></div>`;
  
  const appCard = [
    language,
    checkbox(t("startWithWindows"), appState.settings.autostartWindows, "autostartWindows"),
    checkbox(t("closeToTray"), appState.settings.closeToTray, "closeToTray"),
    checkbox(t("startMinimized"), appState.settings.startMinimized, "startMinimized"),
    checkbox(t("autoBackupOnStart"), appState.settings.autoBackupOnStart, "autoBackupOnStart")
  ].join("");

  const isAdmin = Boolean(appState.isAdmin);
  const adminBadge = isAdmin
    ? `<strong style="color: #22c55e; display: inline-flex; align-items: center; gap: 4px;">${icon("shield")} ${t("adminMode")}</strong>`
    : `<strong style="color: var(--text-secondary);">${t("userMode")}</strong>`;

  const restartBtn = !isAdmin
    ? `<div style="margin-top: 14px;">${button(t("runAsAdmin"), "shield", "outline", "restart-as-admin")}</div>`
    : "";

  const aboutCard = [
    `<div class="settings-info-row"><span>${t("appVersion")}</span><strong>v0.1.0 (Release)</strong></div>`,
    `<div class="settings-info-row"><span>${t("engine")}</span><strong>${t("engineValue")}</strong></div>`,
    `<div class="settings-info-row"><span>${t("runMode")}</span>${adminBadge}</div>`,
    `<div class="settings-info-row"><span>${t("security")}</span><strong>${t("securityValue")}</strong></div>`,
    `<div class="settings-info-row"><span>${t("telemetryStatus")}</span><strong>${t("telemetryValue")}</strong></div>`,
    restartBtn
  ].join("");

  const backupsCount = Array.isArray(viewState?.backups) ? viewState.backups.length : 0;
  const configsCount = Array.isArray(viewState?.configs) ? viewState.configs.length : 0;

  const storageCard = [
    `<div class="settings-info-row"><span>${t("backupsTitle")}</span><strong>${backupsCount}</strong></div>`,
    `<div class="settings-info-row"><span>${t("configsTitle")}</span><strong>${configsCount}</strong></div>`,
    '<div class="settings-storage-actions">',
    `<button class="btn btn-outline" type="button" data-action="open-backups-folder">${icon("folder")}<span>${t("openBackups")}</span></button>`,
    `<button class="btn btn-outline" type="button" data-action="open-configs-folder">${icon("folder")}<span>${t("openConfigs")}</span></button>`,
    "</div>"
  ].join("");

  return [
    '<div class="settings-page">',
    '<div class="stack">',
    card(t("application"), appCard),
    "</div>",
    '<div class="stack">',
    card(t("aboutApp"), aboutCard),
    card(t("storageLocations"), storageCard),
    "</div>",
    "</div>"
  ].join("");
}
