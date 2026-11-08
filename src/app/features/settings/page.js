/*
   ,--,                                            
,---.'|                                            
|   | :                                            
:   : |                        ,--,                
|   ' :                      ,--.'|                
;   ; '                 .---.|  |,      .--.--.    
'   | |__   ,---.     /.  ./|`--'_     /  /    '   
|   | :.'| /       .-' . ' |,' ,'|   |  :  /`./   
'   :    ;/    /  |/___/ \: |'  | |   |  :  ;_     
|   |  ./.    ' / |.   \  ' .|  | :    \  \    `.  
;   : ;  '   ;   /| \   \   ''  : |__   `----.   \ 
|   ,/   '   |  / |  \   \   |  | '.'| /  /`--'  / 
'---'    |   :    |   \   \ |;  :    ;'--'.     /  
          \   \  /     '---" |  ,   /   `--'---'   
           `----'             ---`-'               
                                                   

author - vkmef.t,me
*/
import { card, checkbox } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import { escapeAttr, escapeHtml } from "../../core/html.js";
import { supportedLanguages } from "../../core/i18n.js";
import { lang } from "../../core/state.js";
import { THEMES } from "../../ui/themes.js";



export const AVAILABLE_FONTS = [
  { id: "Inter", name: "Inter (Modern UI)" },
  { id: "Segoe UI", name: "Segoe UI (Windows Default)" },
  { id: "Segoe UI Variable", name: "Segoe UI Variable (Windows 11)" },
  { id: "Bahnschrift", name: "Bahnschrift (DIN Variable)" },
  { id: "Cascadia Code", name: "Cascadia Code (Monospace)" },
  { id: "Consolas", name: "Consolas (Code Mono)" },
  { id: "Trebuchet MS", name: "Trebuchet MS (Humanist)" },
  { id: "Verdana", name: "Verdana (Wide Screen)" },
  { id: "Arial", name: "Arial (Standard Clean)" },
  { id: "Georgia", name: "Georgia (Serif)" },
  { id: "Century Gothic", name: "Century Gothic (Geometric)" },
  { id: "Tahoma", name: "Tahoma (Compact UI)" }
];

const SETTING_TOGGLE_KEYS = [
  { key: "macTrafficLights", labelKey: "macTrafficLights" },
  { key: "eacAutoProtect", labelKey: "eacAutoProtect" },
  { key: "autostartWindows", labelKey: "startWithWindows" },
  { key: "closeToTray", labelKey: "closeToTray" },
  { key: "zeroDaemonMode", labelKey: "zeroDaemonMode" },
  { key: "startMinimized", labelKey: "startMinimized" },
  { key: "autoBackupOnStart", labelKey: "autoBackupOnStart" },
  { key: "showOnRecordings", labelKey: "showOnRecordings" },
  { key: "autoUpdate", labelKey: "autoUpdate" },
  { key: "disableSplash", labelKey: "disableSplash" },
  { key: "disableAnimations", labelKey: "disableAnimations" }
];

export function renderSettingsPage(appState, viewState, t) {
  const isRu = lang() === "ru";
  const currentLangCode = appState?.settings?.language || "en";
  const currentLang = supportedLanguages.find((l) => l.code === currentLangCode) || supportedLanguages[0];
  const isDropdownOpen = Boolean(viewState?.languageDropdownOpen);


  const langItems = supportedLanguages.map((l) => {
    const isSelected = l.code === currentLangCode;
    return `
      <button class="language-dropdown-item ${isSelected ? "selected" : ""}" type="button" data-action="select-language" data-language="${escapeAttr(l.code)}">
        <span class="language-item-left">
          <img class="language-item-flag" src="./assets/flags/${escapeAttr(l.code)}.png" alt="${escapeAttr(l.name)}" loading="lazy" decoding="async" />
          <span class="language-item-native">${escapeHtml(l.nativeName)}</span>
          <span class="language-item-name">${escapeHtml(l.name)}</span>
        </span>
        ${isSelected ? `<span class="language-item-check">${icon("check")}</span>` : ""}
      </button>
    `;
  }).join("");

  const langSecondary = currentLang.nativeName !== currentLang.name ? `<span class="language-label-name">${escapeHtml(currentLang.name)}</span>` : "";

  const languageDropdown = `
    <div class="form-label">${escapeHtml(t("language") || (isRu ? "Язык интерфейса" : "Interface Language"))}</div>
    <div class="language-custom-dropdown ${isDropdownOpen ? "open" : ""}">
      <button class="language-dropdown-trigger" type="button" data-action="toggle-language-dropdown">
        <span class="language-dropdown-label">
          <img class="language-dropdown-flag" src="./assets/flags/${escapeAttr(currentLang.code)}.png" alt="${escapeAttr(currentLang.name)}" />
          <span class="language-label-native">${escapeHtml(currentLang.nativeName)}</span>
          ${langSecondary}
        </span>
        <span class="language-dropdown-arrow">${icon("chevronDown")}</span>
      </button>
      ${isDropdownOpen ? `<div class="language-dropdown-menu">${langItems}</div>` : ""}
    </div>
  `;

  const madeWithLove = `
    <div class="settings-made-with-love">
      <span class="made-with-love-text">${escapeHtml(t("madeWithLove") || (isRu ? "Сделано с любовью" : "Made with love"))}</span>
      <img class="made-with-love-heart" src="./assets/heart.png" alt="❤️" />
    </div>
  `;

  const communityLinks = `
    <div class="form-label">${escapeHtml(t("community") || (isRu ? "Сообщество и ссылки" : "Community & Links"))}</div>
    <div class="settings-community-list">
      <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://github.com/Arean-Max/synchro-nova">
        <div class="tall-card-icon-wrap">${icon("github")}</div>
        <div class="tall-card-content">
          <span class="tall-card-title">${escapeHtml(t("sourceCodeCaps") || (isRu ? "ИСХОДНЫЙ КОД" : "SOURCE CODE"))}</span>
          <span class="tall-card-sub">github.com/Arean-Max</span>
        </div>
      </button>
      <button class="settings-tall-card-btn compact" type="button" data-action="open-external-url" data-url="https://t.me/synchronova">
        <div class="tall-card-icon-wrap">${icon("telegram")}</div>
        <div class="tall-card-content">
          <span class="tall-card-title">${escapeHtml(t("weOnTelegramCaps") || (isRu ? "МЫ В TELEGRAM" : "TELEGRAM CHANNEL"))}</span>
          <span class="tall-card-sub">t.me/synchronova</span>
        </div>
      </button>
    </div>
  `;

  const currentTheme = appState?.settings?.theme || (typeof localStorage !== "undefined" && localStorage.getItem("synchro_theme")) || "default";

  const themeCards = THEMES.map((theme) => {
    const isSelected = currentTheme === theme.id;
    const name = t("theme_" + theme.id) || (isRu ? theme.nameRu : theme.nameEn);
    const gradient = `linear-gradient(135deg, ${theme.previewColors[0]} 0%, ${theme.previewColors[1]} 55%, ${theme.previewColors[2]} 100%)`;

    return `
      <button class="theme-card ${isSelected ? "active" : ""}" type="button" data-action="select-theme" data-theme="${escapeAttr(theme.id)}" title="${escapeAttr(name)}">
        <div class="theme-card-preview" style="background:${gradient};">
          <span class="theme-card-icon">${icon(theme.icon)}</span>
        </div>
        <div class="theme-card-info">
          <span class="theme-card-name">${escapeHtml(name)}</span>
        </div>
      </button>
    `;
  }).join("");

  const themeSection = `
    <div class="form-label">${escapeHtml(t("themeTitle") || (isRu ? "Тема оформления" : "Interface Theme"))}</div>
    <div class="theme-cards-grid">
      ${themeCards}
    </div>
  `;

  const isFontDropdownOpen = Boolean(viewState?.fontDropdownOpen);
  const currentFontId = appState?.settings?.fontFamily || (typeof localStorage !== "undefined" && localStorage.getItem("synchro_font")) || "Inter";
  const currentFont = AVAILABLE_FONTS.find((f) => f.id === currentFontId) || AVAILABLE_FONTS[0];

  const fontItems = AVAILABLE_FONTS.map((f) => {
    const isSelected = f.id === currentFont.id;
    return `
      <button class="font-dropdown-item ${isSelected ? "selected" : ""}" type="button" data-action="select-font" data-font="${escapeAttr(f.id)}">
        <span class="font-dropdown-preview" style="font-family: '${escapeAttr(f.id)}', sans-serif;">${escapeHtml(f.name)}</span>
        ${isSelected ? `<span class="language-item-check">${icon("check")}</span>` : ""}
      </button>
    `;
  }).join("");

  const fontDropdown = `
    <div class="form-label">${escapeHtml(t("interfaceFont") || t("fontTitle") || (isRu ? "Шрифт интерфейса" : "Interface Font"))}</div>
    <div class="font-custom-dropdown ${isFontDropdownOpen ? "open" : ""}">
      <button class="font-dropdown-trigger" type="button" data-action="toggle-font-dropdown">
        <span class="font-dropdown-label" style="font-family: '${escapeAttr(currentFont.id)}', sans-serif;">
          <span class="font-icon-badge">Aa</span>
          <span class="font-label-name">${escapeHtml(currentFont.name)}</span>
        </span>
        <span class="language-dropdown-arrow">${icon("chevronDown")}</span>
      </button>
      ${isFontDropdownOpen ? `<div class="font-dropdown-menu">${fontItems}</div>` : ""}
    </div>
  `;

  const appToggles = SETTING_TOGGLE_KEYS.map(({ key, labelKey }) => {
    const checked = Boolean(appState?.settings?.[key]);
    return checkbox(t(labelKey), checked, key);
  }).join("");

  const unifiedCardBody = `
    <div class="settings-theme-section">
      ${themeSection}
    </div>
    <div class="settings-section-divider"></div>
    <div class="settings-top-row">
      <div class="settings-top-left">
        ${languageDropdown}
        ${fontDropdown}
      </div>
      <div class="settings-top-right">
        ${communityLinks}
        ${madeWithLove}
      </div>
    </div>
    <div class="settings-section-divider"></div>
    <div class="settings-toggles-section">
      ${appToggles}
    </div>
  `;

  return `
    <div class="settings-page settings-unified-page">
      ${card(t("application") || (isRu ? "Параметры приложения" : "Application"), unifiedCardBody, "settings-unified-card")}
    </div>
  `;
}

export const settingsFeature = {
  render: renderSettingsPage,
  mount() {},
  unmount() {}
};
