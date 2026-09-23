import { card, checkbox } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

export function renderSettingsPage(appState, _viewState, t) {
  const language = [
    `<div class="form-label">${t("language")}</div>`,
    `<div class="segmented-control">`,
    `  <button class="seg-option ${appState.settings.language === "en" ? "active" : ""}" type="button" data-language="en">${t("english")}</button>`,
    `  <button class="seg-option ${appState.settings.language === "ru" ? "active" : ""}" type="button" data-language="ru">${t("russian")}</button>`,
    `</div>`
  ].join("");

  const currentAccent = (appState.settings.accentColor || "#ffffff").toLowerCase();
  const presets = [
    { color: "#ffffff", name: "White" },
    { color: "#2563eb", name: "Royal Blue" },
    { color: "#38bdf8", name: "Cyan" },
    { color: "#a855f7", name: "Purple" },
    { color: "#ec4899", name: "Pink" },
    { color: "#22c55e", name: "Green" },
    { color: "#f59e0b", name: "Amber" },
    { color: "#ef4444", name: "Red" }
  ];

  const isPresetActive = presets.some((p) => p.color.toLowerCase() === currentAccent);

  const swatches = presets.map((p) => {
    const active = currentAccent === p.color.toLowerCase() ? "active" : "";
    return `<button class="accent-swatch ${active}" type="button" data-action="set-accent-color" data-color="${p.color}" style="background-color:${p.color};--swatch-color:${p.color};" title="${p.name}"></button>`;
  }).join("");

  const accentControl = [
    `<div class="form-label" style="margin-top:14px;">${t("interfaceAccent")}</div>`,
    '<div class="accent-color-row">',
    swatches,
    `<label class="accent-picker-wrapper ${!isPresetActive ? "active" : ""}" title="${t("interfaceAccent")}">`,
    `<input type="color" class="accent-color-input" data-action="pick-accent-color" value="${currentAccent}">`,
    `<span class="accent-custom-icon"></span>`,
    '</label>',
    '</div>',
    `<p class="settings-hint-text accent-hint-text">${t("interfaceAccentDesc")}</p>`
  ].join("");

  const communityList = [
    `<div class="form-label">${t("community")}</div>`,
    '<div class="settings-community-list">',
    '  <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://github.com/Arean-Max/synchro-nova">',
    '    <div class="tall-card-icon-wrap">',
    icon("github"),
    '    </div>',
    '    <div class="tall-card-content">',
    `      <span class="tall-card-title">${t("weOnGithubCaps")}</span>`,
    '      <span class="tall-card-sub">github.com/Arean-Max</span>',
    '    </div>',
    '  </button>',
    '  <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://t.me/synchronova">',
    '    <div class="tall-card-icon-wrap">',
    icon("telegram"),
    '    </div>',
    '    <div class="tall-card-content">',
    `      <span class="tall-card-title">${t("weOnTelegramCaps")}</span>`,
    '      <span class="tall-card-sub">t.me/synchronova</span>',
    '    </div>',
    '  </button>',
    '</div>'
  ].join("");

  const appToggles = [
    checkbox(t("startWithWindows"), appState.settings.autostartWindows, "autostartWindows"),
    checkbox(t("closeToTray"), appState.settings.closeToTray, "closeToTray"),
    checkbox(t("startMinimized"), appState.settings.startMinimized, "startMinimized"),
    checkbox(t("autoBackupOnStart"), appState.settings.autoBackupOnStart, "autoBackupOnStart"),
    checkbox(t("showOnRecordings"), Boolean(appState.settings.showOnRecordings), "showOnRecordings")
  ].join("");

  const unifiedCardBody = [
    '<div class="settings-top-row">',
    '  <div class="settings-top-left">',
    language,
    accentControl,
    '  </div>',
    '  <div class="settings-top-right">',
    communityList,
    '  </div>',
    '</div>',
    '<div class="settings-section-divider"></div>',
    '<div class="settings-toggles-section">',
    appToggles,
    '</div>'
  ].join("");

  return [
    '<div class="settings-page settings-unified-page">',
    card(t("application"), unifiedCardBody, "settings-unified-card"),
    '</div>'
  ].join("");
}

export const settingsFeature = {
  render: renderSettingsPage,
  mount(container, context) {},
  unmount() {}
};

