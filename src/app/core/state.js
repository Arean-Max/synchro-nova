export const defaultState = {
  color: { saturation: 100, hue: 0, contrast: 100, gamma: 100, enabled: true },
  settings: {
    applyInstantly: true,
    saveColorCorrection: true,
    autostartWindows: false,
    closeToTray: true,
    startMinimized: false,
    autoBackupOnStart: true,
    acceptedAgreement: true,
    language: "en"
  },
  isAdmin: false
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
  selectedTweaks: new Set(),
  installedTweaks: new Set(),
  tweakResults: [],
  applyingTweaks: false,
  sidebarCollapsed: false,
  detectedApps: [],
  smartTipsOpen: false,
  showAdminPrompt: false,
  dismissedAdminPrompt: false,
  tweakFilter: "all"
};


export const sliderDefs = {
  saturation: { label: "Saturation", min: 0, max: 200, step: 1, suffix: "%" },
  hue: { label: "Hue", min: -180, max: 180, step: 1, suffix: " deg", zero: true },
  contrast: { label: "Contrast", min: 50, max: 150, step: 1, suffix: "%" },
  gamma: { label: "Gamma", min: 50, max: 150, step: 1, suffix: "%" }
};

export const pageDefs = {
  color: { title: "colorTitle", subtitle: "colorSubtitle", nav: "colorTitle", icon: "layers" },
  gameColor: { title: "gameProfilesTitle", subtitle: "gameProfilesSubtitle", nav: "gameProfilesTitle", icon: "gamepad" },
  tweaks: { title: "tweaksTitle", subtitle: "tweaksSubtitle", nav: "tweaksTitle", icon: "fileText" },
  characteristics: { title: "characteristicsTitle", subtitle: "characteristicsSubtitle", nav: "characteristicsTitle", icon: "monitor" },
  backups: { title: "backupsTitle", subtitle: "backupsSubtitle", nav: "backupsTitle", icon: "archive" },
  configs: { title: "configsTitle", subtitle: "configsSubtitle", nav: "configsTitle", icon: "grid" },
  settings: { title: "settingsTitle", subtitle: "settingsSubtitle", nav: "settingsTitle", icon: "settings" }
};

export const pageOrder = ["color", "gameColor", "tweaks", "characteristics", "backups", "configs", "settings"];

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

export function mergeState(state) {
  appState = {
    color: { ...defaultState.color, ...(state?.color || {}) },
    settings: { ...defaultState.settings, ...(state?.settings || {}) },
    isAdmin: Boolean(state?.isAdmin ?? state?.is_admin ?? defaultState.isAdmin)
  };
  document.documentElement.lang = lang();
  return appState;
}
