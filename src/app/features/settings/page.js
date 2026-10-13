import { card, checkbox } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import { escapeAttr, escapeHtml } from "../../core/html.js";
import { supportedLanguages } from "../../core/i18n.js";

const ACCENT_PRESETS = [
  { color: "#ffffff", name: "White" },
  { color: "#2563eb", name: "Royal Blue" },
  { color: "#38bdf8", name: "Cyan" },
  { color: "#a855f7", name: "Purple" },
  { color: "#ec4899", name: "Pink" },
  { color: "#22c55e", name: "Green" },
  { color: "#f59e0b", name: "Amber" },
  { color: "#ef4444", name: "Red" }
];

const SETTING_TOGGLE_KEYS = [
  { key: "autostartWindows", labelKey: "startWithWindows" },
  { key: "closeToTray", labelKey: "closeToTray" },
  { key: "startMinimized", labelKey: "startMinimized" },
  { key: "autoBackupOnStart", labelKey: "autoBackupOnStart" },
  { key: "showOnRecordings", labelKey: "showOnRecordings" },
  { key: "autoUpdate", labelKey: "autoUpdate" },
  { key: "disableSplash", labelKey: "disableSplash" },
  { key: "disableAnimations", labelKey: "disableAnimations" }
];

export function renderSettingsPage(appState, viewState, t) {
  const currentLangCode = appState?.settings?.language || "en";
  const currentLang = supportedLanguages.find((l) => l.code === currentLangCode) || supportedLanguages[0];
  const isDropdownOpen = Boolean(viewState?.languageDropdownOpen);
  const currentAccent = (appState?.settings?.accentColor || "#ffffff").toLowerCase();
  const isPresetActive = ACCENT_PRESETS.some((p) => p.color.toLowerCase() === currentAccent);

  const langItems = supportedLanguages.map((l) => {
    const isSelected = l.code === currentLangCode;
    return `
      <button class="language-dropdown-item ${isSelected ? "selected" : ""}" type="button" data-action="select-language" data-language="${escapeAttr(l.code)}">
        <span class="language-item-left">
          <span class="language-item-native">${escapeHtml(l.nativeName)}</span>
          <span class="language-item-name">${escapeHtml(l.name)}</span>
        </span>
        ${isSelected ? `<span class="language-item-check">${icon("check")}</span>` : ""}
      </button>
    `;
  }).join("");

  const languageDropdown = `
    <div class="form-label">${escapeHtml(t("language"))}</div>
    <div class="language-custom-dropdown ${isDropdownOpen ? "open" : ""}">
      <button class="language-dropdown-trigger" type="button" data-action="toggle-language-dropdown">
        <span class="language-dropdown-label">
          <span class="language-label-native">${escapeHtml(currentLang.nativeName)}</span>
          <span class="language-label-name">${escapeHtml(currentLang.name)}</span>
        </span>
        <span class="language-dropdown-arrow">${icon("chevronDown")}</span>
      </button>
      ${isDropdownOpen ? `<div class="language-dropdown-menu">${langItems}</div>` : ""}
    </div>
  `;

  const swatches = ACCENT_PRESETS.map((p) => {
    const active = currentAccent === p.color.toLowerCase() ? "active" : "";
    return `<button class="accent-swatch ${active}" type="button" data-action="set-accent-color" data-color="${p.color}" style="background-color:${p.color};--swatch-color:${p.color};" title="${p.name}"></button>`;
  }).join("");

  const accentControl = `
    <div class="form-label" style="margin-top:14px;">${escapeHtml(t("interfaceAccent"))}</div>
    <div class="accent-color-row">
      ${swatches}
      <label class="accent-picker-wrapper ${!isPresetActive ? "active" : ""}" title="${escapeAttr(t("interfaceAccent"))}">
        <input type="color" class="accent-color-input" data-action="pick-accent-color" value="${currentAccent}">
        <span class="accent-custom-icon"></span>
      </label>
    </div>
  `;

  const communityLinks = `
    <div class="form-label">${escapeHtml(t("community"))}</div>
    <div class="settings-community-list">
      <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://github.com/Arean-Max/synchro-nova">
        <div class="tall-card-icon-wrap">${icon("github")}</div>
        <div class="tall-card-content">
          <span class="tall-card-title">${escapeHtml(t("sourceCodeCaps"))}</span>
          <span class="tall-card-sub">github.com/Arean-Max</span>
        </div>
      </button>
      <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://t.me/synchronova">
        <div class="tall-card-icon-wrap">${icon("telegram")}</div>
        <div class="tall-card-content">
          <span class="tall-card-title">${escapeHtml(t("weOnTelegramCaps"))}</span>
          <span class="tall-card-sub">t.me/synchronova</span>
        </div>
      </button>
    </div>
  `;

  const appToggles = SETTING_TOGGLE_KEYS.map(({ key, labelKey }) => {
    const checked = Boolean(appState?.settings?.[key]);
    return checkbox(t(labelKey), checked, key);
  }).join("");

  const securitySection = `
    <div class="form-label">${escapeHtml(t("eacShieldTitle"))}</div>
    <div class="settings-actions-row">
      <button class="btn btn-outline" type="button" data-action="audit-anticheat">
        ${icon("shield")}
        <span>${escapeHtml(t("auditAntiCheat"))}</span>
      </button>
      <button class="btn btn-outline" type="button" data-action="fix-anticheat">
        ${icon("shieldCheck")}
        <span>${escapeHtml(t("fixAntiCheat"))}</span>
      </button>
    </div>
  `;

  const unifiedCardBody = `
    <div class="settings-top-row">
      <div class="settings-top-left">
        ${languageDropdown}
        ${accentControl}
      </div>
      <div class="settings-top-right">
        ${communityLinks}
      </div>
    </div>
    <div class="settings-section-divider"></div>
    <div class="settings-toggles-section">
      ${appToggles}
    </div>
    <div class="settings-section-divider"></div>
    ${securitySection}
  `;

  return `
    <div class="settings-page settings-unified-page">
      ${card(t("application"), unifiedCardBody, "settings-unified-card")}
    </div>
  `;
}

export const settingsFeature = {
  render: renderSettingsPage,
  mount() {},
  unmount() {}
};
