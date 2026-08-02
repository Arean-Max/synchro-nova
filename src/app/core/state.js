export function detectDefaultLanguage() {
  try {
    const sys = (
      (typeof navigator !== "undefined" && (navigator.language || navigator.userLanguage)) ||
      ""
    ).toLowerCase();
    if (sys.startsWith("ru")) return "ru";
  } catch (_) {}
  return "en";
}

export const defaultState = {
  color: { saturation: 100, hue: 0, contrast: 100, gamma: 100, blackHolo: 0, enabled: true, activeFilter: "" },
  settings: {
    applyInstantly: true,
    saveColorCorrection: true,
    autostartWindows: false,
    closeToTray: true,
    startMinimized: false,
    autoBackupOnStart: false,
    acceptedAgreement: true,
    language: detectDefaultLanguage(),
    showOnRecordings: true,
    accentColor: "#ffffff",
    autoUpdate: false,
    disableSplash: false,
    disableAnimations: false,
    templateOverrides: {}
  },
  keybinds: [],
  isAdmin: false
};

export const defaultPresets = {
  balanced: { saturation: 160, hue: -5, contrast: 97, gamma: 118, enabled: true },
  vibrant: { saturation: 200, hue: -5, contrast: 95, gamma: 105, enabled: true },
  soft: { saturation: 150, hue: -5, contrast: 85, gamma: 115, enabled: true },
  night: { saturation: 120, hue: -5, contrast: 90, gamma: 150, enabled: true }
};

export const filterPresets = {
  rust_cold_tactical: { saturation: 78, hue: 0, contrast: 116, gamma: 105, enabled: true },
  rust_midnight_neon: { saturation: 140, hue: 0, contrast: 122, gamma: 100, enabled: true },
  clear_sight: { saturation: 125, hue: 0, contrast: 110, gamma: 100, enabled: true }
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
  pendingRevertTweaks: new Set(),
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
    hotkey: "",
    rustSynced: false,
    error: null
  },
  recordingBindId: null,
  openDropdownBindId: null,
  languageDropdownOpen: false,
  fineTuningTab: "vibrance"
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
  binds: { title: "bindsTitle", subtitle: "bindsSubtitle", nav: "bindsTitle", icon: "keyboard" },
  characteristics: { title: "characteristicsTitle", subtitle: "characteristicsSubtitle", nav: "characteristicsTitle", icon: "cpu" },
  backups: { title: "backupsTitle", subtitle: "backupsSubtitle", nav: "backupsTitle", icon: "archive" },
  settings: { title: "settingsTitle", subtitle: "settingsSubtitle", nav: "settingsTitle", icon: "settings" }
};
