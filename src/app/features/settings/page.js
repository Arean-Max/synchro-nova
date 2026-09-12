import { card, checkbox } from "../../ui/components.js";

function betaLabel(label) {
  return `<span>${label}</span><span class="beta-badge">BETA</span>`;
}

export function renderSettingsPage(appState, t) {
  const language = `<div class="form-label">${t("language")}</div><div class="segmented-control"><button class="seg-option ${appState.settings.language === "en" ? "active" : ""}" type="button" data-language="en">${t("english")}</button><button class="seg-option ${appState.settings.language === "ru" ? "active" : ""}" type="button" data-language="ru">${t("russian")}</button></div>`;
  const appCard = [
    language,
    checkbox(t("startWithWindows"), appState.settings.autostartWindows, "autostartWindows"),
    checkbox(t("closeToTray"), appState.settings.closeToTray, "closeToTray"),
    checkbox(t("startMinimized"), appState.settings.startMinimized, "startMinimized"),
    checkbox(t("autoBackupOnStart"), appState.settings.autoBackupOnStart, "autoBackupOnStart")
  ].join("");
  const performanceCard = [
    `<div class="settings-mode-row">${checkbox(t("lowSpecMode"), appState.settings.lowSpecMode, "lowSpecMode")}<span>${t("lowSpecMeta")}</span></div>`,
    `<div class="settings-metrics"><span>${t("compactEffects")}</span><span>${t("cachedInfo")}</span><span>${t("leanApply")}</span></div>`
  ].join("");
  const privacyCard = [
    checkbox(betaLabel(t("sendDailyPing")), appState.settings.sendDailyPing, "sendDailyPing"),
    checkbox(betaLabel(t("sendCrashTelemetry")), appState.settings.sendCrashTelemetry, "sendCrashTelemetry")
  ].join("");
  return `<div class="settings-page">${card(t("application"), appCard)}${card(t("performance"), performanceCard)}${card(t("privacy"), privacyCard)}</div>`;
}
