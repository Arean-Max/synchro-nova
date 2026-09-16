export const defaultState = {
  color: { saturation: 100, hue: 0, contrast: 100, gamma: 100, blackHolo: 0, enabled: true },
  settings: {
    applyInstantly: true,
    saveColorCorrection: true,
    autostartWindows: false,
    closeToTray: true,
    startMinimized: false,
    autoBackupOnStart: true,
    acceptedAgreement: true,
    language: "en",
    showOnRecordings: true,
    accentColor: "#2563eb",
    templateOverrides: {}
  },
  isAdmin: false
};

export const defaultPresets = {
  balanced: { saturation: 160, hue: -5, contrast: 97, gamma: 118, enabled: true },
  vibrant: { saturation: 200, hue: -5, contrast: 95, gamma: 105, enabled: true },
  soft: { saturation: 150, hue: -5, contrast: 85, gamma: 115, enabled: true },
  night: { saturation: 120, hue: -5, contrast: 90, gamma: 150, enabled: true }
};

export const viewState = {
  backups: [],
  configs: [],
  colorGames: [],
  colorGamesLoaded: false,
  colorGamesDemo: false,
  selectedColorGame: "",
  colorGamePanelOpen: false,
  colorTemplateName: "",
  system: null,
  selectedBackup: "",
  selectedConfig: "",
  backupName: "",
  configName: "",
  cpuHistory: [],
  showAllDrivers: false,
  drivers: [],
  driversLoading: false,
  driverFilter: "all",
  driverSearch: "",
  selectedTweaks: new Set(),
  installedTweaks: new Set(),
  tweakResults: [],
  applyingTweaks: false,
  sidebarCollapsed: false,
  detectedApps: [],
  activeImpactBanner: null,
  editingTemplate: null,
  editingTemplateName: "",
  editingTemplateColor: null,
  showAdminPrompt: false,
  dismissedAdminPrompt: false,
  showBackupNameModal: false,
  applyModal: null,
  colorPreviewMode: "day"
};


export const sliderDefs = {
  saturation: { label: "Saturation", min: 0, max: 200, step: 1, suffix: "%" },
  hue: { label: "Hue", min: -180, max: 180, step: 1, suffix: " deg", zero: true },
  contrast: { label: "Contrast", min: 50, max: 150, step: 1, suffix: "%" },
  gamma: { label: "Gamma", min: 50, max: 150, step: 1, suffix: "%" }
};

export const pageDefs = {
  color: { title: "colorTitle", subtitle: "colorSubtitle", nav: "colorTitle", icon: "layers" },
  tweaks: { title: "tweaksTitle", subtitle: "tweaksSubtitle", nav: "tweaksTitle", icon: "fileText" },
  characteristics: { title: "characteristicsTitle", subtitle: "characteristicsSubtitle", nav: "characteristicsTitle", icon: "cpu" },
  backups: { title: "backupsTitle", subtitle: "backupsSubtitle", nav: "backupsTitle", icon: "archive" },
  settings: { title: "settingsTitle", subtitle: "settingsSubtitle", nav: "settingsTitle", icon: "settings" }
};

export const pageOrder = ["color", "tweaks", "characteristics", "backups", "settings"];

export let activePage = "color";
export let booting = false;
export let appState = cloneState(defaultState);

export function setActivePage(page) {
  activePage = pageDefs[page] ? page : "color";
}

export function setBooting(value) {
  booting = Boolean(value);
}

export function cloneState(state) {
  return JSON.parse(JSON.stringify(state));
}

export function lang() {
  return appState.settings.language === "ru" ? "ru" : "en";
}

export function applyInterfaceAccent(color) {
  const accent = color || "#2563eb";
  if (typeof document === "undefined" || !document.documentElement) return;
  document.documentElement.style.setProperty("--ui-accent", accent);
  const hex = accent.replace("#", "");
  let r = 255, g = 255, b = 255;
  if (hex.length === 6) {
    r = parseInt(hex.slice(0, 2), 16) || 255;
    g = parseInt(hex.slice(2, 4), 16) || 255;
    b = parseInt(hex.slice(4, 6), 16) || 255;
  } else if (hex.length === 3) {
    r = parseInt(hex[0] + hex[0], 16) || 255;
    g = parseInt(hex[1] + hex[1], 16) || 255;
    b = parseInt(hex[2] + hex[2], 16) || 255;
  }
  const brightness = (r * 299 + g * 587 + b * 114) / 1000;
  const textColor = brightness > 155 ? "#111113" : "#ffffff";
  const thumbColor = brightness > 200 ? "#141416" : "#ffffff";
  document.documentElement.style.setProperty("--ui-accent-text", textColor);
  document.documentElement.style.setProperty("--ui-accent-thumb", thumbColor);
  document.documentElement.style.setProperty("--ui-accent-glow", `rgba(${r}, ${g}, ${b}, 0.28)`);
}

export function mergeState(state) {
  appState = {
    color: { ...defaultState.color, ...(state?.color || {}) },
    settings: { ...defaultState.settings, ...(state?.settings || {}) },
    isAdmin: Boolean(state?.isAdmin ?? state?.is_admin ?? defaultState.isAdmin)
  };
  document.documentElement.lang = lang();
  applyInterfaceAccent(appState.settings.accentColor);
  return appState;
}
