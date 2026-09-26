export const defaultState = {
  color: { saturation: 100, hue: 0, contrast: 100, gamma: 100, blackHolo: 0, enabled: true },
  settings: {
    applyInstantly: true,
    saveColorCorrection: true,
    autostartWindows: false,
    closeToTray: true,
    startMinimized: false,
    autoBackupOnStart: false,
    acceptedAgreement: true,
    language: "en",
    showOnRecordings: true,
    accentColor: "#ffffff",
    autoUpdate: false,
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
  selectedColorGame: "",
  colorGamePanelOpen: false,
  colorTemplateName: "",
  system: null,
  selectedBackup: "",
  selectedConfig: "",
  backupName: "",
  configName: "",
  cpuHistory: [],
  updateStatus: "hidden",
  updatePercent: 0,
  latestVersion: "",
  downloadUrl: "",
  assetSize: 0,
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
  colorPreviewMode: "day",
  blackHoloStatus: {
    active: false,
    gpuVendor: "unknown",
    gpuName: "",
    curveProfile: "",
    hotkey: "F11",
    rustSynced: true,
    error: null
  }
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
export let appState = cloneState(defaultState);

export function setActivePage(page) {
  activePage = pageDefs[page] ? page : "color";
}

export function cloneState(state) {
  return JSON.parse(JSON.stringify(state));
}

export function lang() {
  return appState.settings.language === "ru" ? "ru" : "en";
}

export function applyInterfaceAccent(color) {
  const accent = color || "#ffffff";
  if (typeof document === "undefined" || !document.documentElement) return;

  const hex = accent.replace("#", "");
  let r = 255, g = 255, b = 255;
  if (hex.length === 6) {
    const pr = parseInt(hex.slice(0, 2), 16);
    const pg = parseInt(hex.slice(2, 4), 16);
    const pb = parseInt(hex.slice(4, 6), 16);
    r = Number.isFinite(pr) ? pr : 255;
    g = Number.isFinite(pg) ? pg : 255;
    b = Number.isFinite(pb) ? pb : 255;
  } else if (hex.length === 3) {
    const pr = parseInt(hex[0] + hex[0], 16);
    const pg = parseInt(hex[1] + hex[1], 16);
    const pb = parseInt(hex[2] + hex[2], 16);
    r = Number.isFinite(pr) ? pr : 255;
    g = Number.isFinite(pg) ? pg : 255;
    b = Number.isFinite(pb) ? pb : 255;
  }

  // Rec. 601 perceived luminance
  const brightness = (r * 299 + g * 587 + b * 114) / 1000;

  // If accent is light (e.g. #ffffff), text on accent background is dark (#111113).
  // If accent is dark (e.g. #000000), text on accent background is bright white (#ffffff).
  const textColor = brightness > 140 ? "#111113" : "#ffffff";
  const thumbColor = brightness > 190 ? "#141416" : "#ffffff";

  // For text/icons tinted with the accent against the dark app background (#141414),
  // if the accent is very dark (e.g. #000000), ensure it never becomes invisible.
  const fgAccent = brightness < 65 ? "#ffffff" : accent;

  // Subtle border outline for dark accents on dark backgrounds
  const accentBorder = brightness < 65 ? "rgba(255, 255, 255, 0.28)" : "transparent";

  // Glow definition
  const glowR = brightness < 65 ? 255 : r;
  const glowG = brightness < 65 ? 255 : g;
  const glowB = brightness < 65 ? 255 : b;
  const glowAlpha = brightness < 65 ? 0.18 : 0.32;

  document.documentElement.style.setProperty("--ui-accent", accent);
  document.documentElement.style.setProperty("--ui-accent-fg", fgAccent);
  document.documentElement.style.setProperty("--ui-accent-text", textColor);
  document.documentElement.style.setProperty("--ui-accent-thumb", thumbColor);
  document.documentElement.style.setProperty("--ui-accent-glow", `rgba(${glowR}, ${glowG}, ${glowB}, ${glowAlpha})`);
  document.documentElement.style.setProperty("--ui-accent-border", accentBorder);
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
