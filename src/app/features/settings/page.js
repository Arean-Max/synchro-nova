import { button, card, checkbox } from "../../ui/components.js";

export function renderSettingsPage(appState, _viewState, t) {
  const language = `<div class="form-label">${t("language")}</div><div class="segmented-control"><button class="seg-option ${appState.settings.language === "en" ? "active" : ""}" type="button" data-language="en">${t("english")}</button><button class="seg-option ${appState.settings.language === "ru" ? "active" : ""}" type="button" data-language="ru">${t("russian")}</button></div>`;

  const isAdmin = Boolean(appState.isAdmin);
  const restartBtn = !isAdmin
    ? `<div style="margin-top: 14px;">${button(t("runAsAdmin"), "shield", "outline", "restart-as-admin")}</div>`
    : "";

  const appCard = [
    language,
    checkbox(t("startWithWindows"), appState.settings.autostartWindows, "autostartWindows"),
    checkbox(t("closeToTray"), appState.settings.closeToTray, "closeToTray"),
    checkbox(t("startMinimized"), appState.settings.startMinimized, "startMinimized"),
    checkbox(t("autoBackupOnStart"), appState.settings.autoBackupOnStart, "autoBackupOnStart"),
    restartBtn
  ].join("");

  return [
    '<div class="settings-page">',
    '<div class="stack">',
    card(t("application"), appCard),
    "</div>",
    "</div>"
  ].join("");
}
