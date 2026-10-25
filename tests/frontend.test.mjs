import test from "node:test";
import assert from "node:assert/strict";

// Global DOM mock for Node environment tests
globalThis.window = globalThis;
const docAttrs = new Map();
globalThis.document = {
  documentElement: {
    lang: "en",
    dir: "ltr",
    style: {
      setProperty: () => {}
    },
    setAttribute: (name, val) => docAttrs.set(name, String(val)),
    getAttribute: (name) => docAttrs.get(name) || null
  },
  querySelectorAll: () => [],
  querySelector: () => null,
  addEventListener: () => {}
};

test("IPC Bridge: returns null when native invoke is missing", async () => {
  delete globalThis.__TAURI__;
  const { invokeCommand } = await import("../src/app/core/bridge.js");
  const res = await invokeCommand("test_command", {});
  assert.equal(res, null);
});

test("IPC Bridge: resolves value from native invoke", async () => {
  globalThis.__TAURI__ = {
    core: {
      invoke: async (cmd, payload) => ({ cmd, payload, ok: true })
    }
  };
  const { invokeCommand } = await import("../src/app/core/bridge.js");
  const res = await invokeCommand("test_echo", { x: 42 });
  assert.deepEqual(res, { cmd: "test_echo", payload: { x: 42 }, ok: true });
});

test("IPC Bridge: throws and propagates error on command failure", async () => {
  globalThis.__TAURI__ = {
    core: {
      invoke: async () => {
        throw new Error("Tauri command rejected");
      }
    }
  };
  const { invokeCommand } = await import("../src/app/core/bridge.js");
  await assert.rejects(
    async () => {
      await invokeCommand("failing_cmd");
    },
    { message: "Tauri command rejected" }
  );
});

test("State: default version is 2.3.4", async () => {
  const { viewState } = await import("../src/app/core/state.js");
  assert.equal(viewState.appVersion, "2.3.4");
});

test("State: pageDefs contains configs and gameColor routes", async () => {
  const { pageDefs } = await import("../src/app/core/state.js");
  assert.ok(pageDefs.configs, "pageDefs.configs must be defined");
  assert.ok(pageDefs.gameColor, "pageDefs.gameColor must be defined");
  assert.equal(pageDefs.configs.icon, "folder");
});

test("State: mergeState preserves and deep-merges templateOverrides", async () => {
  const { mergeState } = await import("../src/app/core/state.js");
  const s0 = mergeState({
    settings: {
      templateOverrides: {
        rust_cold_tactical: {
          name: "Tactical Custom",
          saturation: 120,
          hue: -2,
          contrast: 105,
          gamma: 100
        }
      }
    }
  });

  assert.ok(s0.settings.templateOverrides.rust_cold_tactical);
  assert.equal(s0.settings.templateOverrides.rust_cold_tactical.name, "Tactical Custom");

  // Partial settings update without templateOverrides must preserve existing
  const s1 = mergeState({
    settings: {
      applyInstantly: true,
      accentColor: "#10b981"
    }
  });

  assert.ok(s1.settings.templateOverrides.rust_cold_tactical);
  assert.equal(s1.settings.templateOverrides.rust_cold_tactical.name, "Tactical Custom");

  // Another update adding vibrant must preserve tactical custom
  const s2 = mergeState({
    settings: {
      templateOverrides: {
        vibrant: {
          name: "Super Vibrant",
          saturation: 180,
          hue: 0,
          contrast: 110,
          gamma: 100
        }
      }
    }
  });

  assert.equal(s2.settings.templateOverrides.rust_cold_tactical.name, "Tactical Custom");
  assert.equal(s2.settings.templateOverrides.vibrant.name, "Super Vibrant");
});

test("State: Arabic sets RTL direction on documentElement", async () => {
  const { mergeState } = await import("../src/app/core/state.js");
  mergeState({ settings: { language: "ar" } });
  assert.equal(document.documentElement.dir, "rtl");

  mergeState({ settings: { language: "en" } });
  assert.equal(document.documentElement.dir, "ltr");
});

test("Tweaks Apply Modal: truthful header and badges for errors and empty results", async () => {
  const { renderApplyModal } = await import("../src/app/features/tweaks/applyModal.js");
  const t = (k) => k;

  // Case 1: Empty results
  const emptyHtml = renderApplyModal({ phase: "complete", results: [] }, t);
  assert.match(emptyHtml, /noResultsReceived/);
  assert.match(emptyHtml, /role="dialog"/);
  assert.match(emptyHtml, /aria-modal="true"/);

  // Case 2: All failed
  const failedHtml = renderApplyModal({
    phase: "complete",
    results: [
      { id: "disable-nagle", status: "failed", message: "Access denied" },
      { id: "disable-game-bar", status: "failed", message: "Registry error" }
    ]
  }, t);
  assert.match(failedHtml, /applyFailed/);
  assert.match(failedHtml, /apply-stat-pill failed/);

  // Case 3: Partial success
  const partialHtml = renderApplyModal({
    phase: "complete",
    results: [
      { id: "clean-temp-junk", status: "applied", message: "Cleaned 12 files (3 MB)" },
      { id: "disable-nagle", status: "failed", message: "Access denied" }
    ]
  }, t);
  assert.match(partialHtml, /appliedWithErrors/);
  assert.match(partialHtml, /apply-stat-pill applied/);
  assert.match(partialHtml, /apply-stat-pill failed/);

  // Case 4: Full success
  const successHtml = renderApplyModal({
    phase: "complete",
    results: [
      { id: "clean-temp-junk", status: "applied" }
    ]
  }, t);
  assert.match(successHtml, /Settings Successfully Applied|Настройки успешно применены/);
  assert.match(successHtml, /apply-stat-pill applied/);
});

test("Vendor Logos: sanitizes className against attribute injection", async () => {
  const { getVendorLogo } = await import("../src/app/features/characteristics/vendorLogos.js");
  const safeSvg = getVendorLogo("nvidia", 'gpu-card" onload="alert(1)');
  assert.ok(!safeSvg.includes('onload="alert(1)'));
  assert.ok(safeSvg.includes('vendor-nvidia gpu-card onloadalert1'));
});

test("Characteristics Page: AMD driver version is not corrupted into fake marketing version", async () => {
  const { renderCharacteristicsPage } = await import("../src/app/features/characteristics/page.js");
  const t = (k) => k;

  const html = renderCharacteristicsPage({
    drivers: [
      {
        name: "AMD Radeon RX 7900 XTX",
        vendor: "amd",
        className: "display",
        version: "31.0.24027.1012",
        isOutdated: false
      }
    ]
  }, t);

  assert.ok(html.includes("31.0.24027.1012"), "AMD Driver Store build number should be preserved");
  assert.ok(!html.includes("v24.2.7"), "Must not invent 24.2.7 marketing version");
});

test("Characteristics Page: Outdated driver with unknown update version does not display current version as update", async () => {
  const { renderCharacteristicsPage } = await import("../src/app/features/characteristics/page.js");
  const t = (k) => k;

  const html = renderCharacteristicsPage({
    drivers: [
      {
        name: "Realtek High Definition Audio",
        vendor: "realtek",
        className: "media",
        version: "6.0.9239.1",
        isOutdated: true,
        latestVersion: "" // unknown latest version
      }
    ]
  }, t);

  assert.ok(!html.includes("Available v6.0.9239.1") && !html.includes("Доступна v6.0.9239.1"), "Must not claim current version as update");
});

test("Localization: All 22 supported languages contain critical keys without returning literal key name", async () => {
  const { supportedLanguages, translate } = await import("../src/app/core/i18n.js");
  const criticalKeys = [
    "settings",
    "unknownError",
    "ready",
    "checkingUpdates",
    "latestVersionInstalled",
    "updateError",
    "rollbackFailed",
    "appliedWithErrors",
    "applyFailed",
    "noResultsReceived",
    "searchDrivers"
  ];

  for (const { code } of supportedLanguages) {
    for (const key of criticalKeys) {
      const val = translate(code, key);
      assert.ok(val && val.trim().length > 0, `Language [${code}] missing key [${key}]`);
      assert.notEqual(val, key, `Language [${code}] returned literal key name for [${key}]`);
    }
  }
});

test("HTML Security: settings accent color attribute is escaped", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k;
  const mockState = {
    settings: {
      language: "en",
      accentColor: '#34d399" onfocus="alert(1)'
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(html.includes('&quot; onfocus=&quot;alert(1)'), "Accent color input value must be HTML-attribute-escaped");
  assert.ok(!html.includes('value="#34d399" onfocus="alert(1)"'));
});

test("CSP Compliance: game cards do not contain inline onerror handlers", async () => {
  const { renderColorPage } = await import("../src/app/features/color/page.js");
  const t = (k) => k;
  const mockState = {
    color: { saturation: 100, hue: 0, contrast: 100, gamma: 100, activeFilter: "" },
    settings: { language: "en", accentColor: "#ffffff", templateOverrides: {} }
  };
  const mockView = {
    colorGames: [{ id: "rust", name: "Rust", source: "Steam" }],
    selectedColorGame: "rust"
  };
  const html = renderColorPage(mockState, mockView, t);
  assert.ok(!html.includes("onerror="), "Must not contain inline onerror attributes violating CSP");
});

test("Settings: does not render sotoykor-source original repository button", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k;
  const mockState = {
    settings: {
      language: "ru",
      accentColor: "#ffffff"
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(!html.includes("sotoykor-source"), "Upstream sotoykor-source repository button must be removed");
  assert.ok(!html.includes("upstreamRepoCaps"), "Upstream repository title label must be removed");
  assert.ok(html.includes("Arean-Max/synchro-nova"), "Fork repository link must be present");
  assert.ok(html.includes("t.me/synchronova"), "Telegram community link must be present");
});

test("Settings: does not render manual check for updates button or row", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k;
  const mockState = {
    settings: {
      language: "ru",
      accentColor: "#ffffff"
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(!html.includes("check-updates-manually"), "Manual check for updates button must be removed");
  assert.ok(!html.includes("checkForUpdates"), "Check for updates label must be removed from settings");
});

test("Settings: unified card layout with eacAutoProtect toggle and without deprecated buttons", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k;
  const mockState = {
    settings: {
      language: "en",
      accentColor: "#ffffff",
      eacAutoProtect: true
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(html.includes("settings-unified-card"), "Must contain unified settings card");
  assert.ok(html.includes('data-setting="eacAutoProtect"'), "Must contain eacAutoProtect setting toggle");
  assert.ok(!html.includes("check-updates-manually"), "Manual check for updates button must not be present");
  assert.ok(!html.includes("audit-anticheat"), "Manual EAC audit button must not be present");
  assert.ok(!html.includes("fix-anticheat"), "Manual EAC fix button must not be present");
  assert.ok(!html.includes("sotoykor-source"), "Upstream repository button must not be present");
});

test("Characteristics: driver group sections have anchor IDs and unconstrained search bar is removed", async () => {
  const { renderCharacteristicsPage } = await import("../src/app/features/characteristics/page.js");
  const t = (k) => k;
  const html = renderCharacteristicsPage({
    drivers: [
      { name: "NVIDIA GeForce RTX 4090", vendor: "nvidia", className: "display", version: "551.23", isOutdated: false },
      { name: "Realtek Audio", vendor: "realtek", className: "media", version: "6.0.1", isOutdated: false }
    ]
  }, t);
  assert.ok(html.includes('id="driver-section-display"'), "Display category section anchor must be present");
  assert.ok(html.includes('id="driver-section-media"'), "Media category section anchor must be present");
  assert.ok(!html.includes("driver-search-toolbar"), "Search toolbar with unconstrained SVG icon must be removed");
});

test("Tweaks: group sections have anchor IDs", async () => {
  const { renderTweaksPage } = await import("../src/app/features/tweaks/page.js");
  const t = (k) => k;
  const html = renderTweaksPage({
    installedTweaks: new Set(),
    pendingRevertTweaks: new Set(),
    selectedTweaks: new Set()
  }, t);
  assert.ok(html.includes('id="tweak-section-gaming_latency"'), "Gaming latency anchor must be present");
  assert.ok(html.includes('id="tweak-section-scheduler"'), "Scheduler anchor must be present");
  assert.ok(html.includes('id="tweak-section-input"'), "Input anchor must be present");
  assert.ok(html.includes('id="tweak-section-storage"'), "Storage anchor must be present");
  assert.ok(html.includes('id="tweak-section-network"'), "Network anchor must be present");
  assert.ok(html.includes('id="tweak-section-background"'), "Background anchor must be present");
});

test("Layout Navigation: renderNavList generates smooth-scroll sub-buttons under active page", async () => {
  const { renderNavList } = await import("../src/app/ui/layout.js");
  const { setActivePage } = await import("../src/app/core/state.js");
  const t = (k) => k;

  // Tweaks active
  setActivePage("tweaks");
  const tweaksNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(tweaksNavHtml.includes('class="nav-sub-list"'), "Sub-navigation list must be rendered");
  assert.ok(!tweaksNavHtml.includes('class="nav-chevron"'), "Chevron indicator must be removed");
  assert.ok(tweaksNavHtml.includes('data-action="scroll-to-section"'), "Sub-buttons must have scroll action");
  assert.ok(tweaksNavHtml.includes('data-section="tweak-section-gaming_latency"'), "Must scroll to gaming_latency");
  assert.ok(tweaksNavHtml.includes('data-section="tweak-section-scheduler"'), "Must scroll to scheduler");

  // Tweaks active but toggled closed (accordion behavior)
  const tweaksCollapsedHtml = renderNavList(t, { sidebarCollapsed: false, collapsedSubNav: { tweaks: true } });
  assert.ok(!tweaksCollapsedHtml.includes('class="nav-sub-list"'), "Sub-navigation list must be hidden when toggled closed");

  // Characteristics active
  setActivePage("characteristics");
  const driversNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(driversNavHtml.includes('data-section="driver-section-display"'), "Must scroll to driver-section-display");
  assert.ok(driversNavHtml.includes('data-section="driver-section-media"'), "Must scroll to driver-section-media");

  // Settings active (unified card: no sub-items)
  setActivePage("settings");
  const settingsNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(!settingsNavHtml.includes('class="nav-sub-list"'), "Settings must not render sub-navigation in unified layout");

  // Reset active page to color
  setActivePage("color");
});

test("Backups Page: configs block is removed from backups page", async () => {
  const { renderBackupsPage } = await import("../src/app/features/storage/page.js");
  const t = (k) => k;
  const mockView = {
    backups: [],
    configs: [],
    selectedBackup: null
  };
  const html = renderBackupsPage(mockView, t);
  assert.ok(html.includes("backup-card"), "Backup card must be present");
  assert.ok(!html.includes("configs-card"), "Configs card must be removed from backups page");
  assert.ok(!html.includes("savedConfigs"), "Saved configs title must be removed from backups page");
});

test("Characteristics Page: Outdated driver omits current version row and shows available update", async () => {
  const { renderCharacteristicsPage } = await import("../src/app/features/characteristics/page.js");
  const t = (k) => k;
  const html = renderCharacteristicsPage({
    drivers: [
      {
        name: "Realtek High Definition Audio",
        vendor: "realtek",
        className: "media",
        version: "6.0.9000.1",
        latestVersion: "6.0.9735.1",
        isOutdated: true,
        date: "8/4/2020"
      }
    ]
  }, t);
  assert.ok(!html.includes("v6.0.9000.1"), "Outdated driver must not show current version");
  assert.ok(html.includes("v6.0.9735.1"), "Outdated driver must show available version");
});


test("Themes: definitions contain 4 seasonal themes with valid properties", async () => {
  const { THEMES } = await import("../src/app/ui/themes.js");
  assert.strictEqual(THEMES.length, 4, "Must define 4 seasonal themes");
  const ids = THEMES.map((t) => t.id);
  assert.deepStrictEqual(ids, ["default", "winter", "autumn", "spring"]);
  for (const theme of THEMES) {
    assert.ok(theme.nameRu, `Theme ${theme.id} must have Russian name`);
    assert.ok(theme.nameEn, `Theme ${theme.id} must have English name`);
    assert.ok(theme.badge, `Theme ${theme.id} must have badge`);
    assert.ok(theme.icon, `Theme ${theme.id} must have icon`);
    assert.strictEqual(theme.previewColors.length, 3, `Theme ${theme.id} must have 3 preview colors`);
  }
});

test("Themes: Settings page renders theme picker grid with active card selection", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k;
  const mockState = {
    settings: {
      language: "ru",
      theme: "winter"
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(html.includes("settings-theme-section"), "Must render theme section");
  assert.ok(html.includes("theme-cards-grid"), "Must render theme cards grid");
  assert.ok(html.includes('class="theme-card active"') && html.includes('data-theme="winter"'), "Winter theme must be active");
  assert.ok(html.includes('data-theme="default"') && !html.includes('data-theme="default" class="theme-card active"'), "Default theme must be inactive");
  assert.ok(html.includes('data-theme="autumn"') && !html.includes('data-theme="autumn" class="theme-card active"'), "Autumn theme must be inactive");
  assert.ok(html.includes('data-theme="spring"') && !html.includes('data-theme="spring" class="theme-card active"'), "Spring theme must be inactive");
  assert.ok(!html.includes("theme-card-desc"), "Theme cards must not render description text");
  assert.ok(!html.includes("theme-card-badge"), "Theme cards must not render top-right badges");
});

test("State: mergeState preserves active theme across backend syncs", async () => {
  const { mergeState } = await import("../src/app/core/state.js");
  const stateWithWinter = mergeState({
    settings: {
      theme: "winter"
    }
  });
  assert.strictEqual(stateWithWinter.settings.theme, "winter", "Must store winter theme");

  const syncedState = mergeState({
    settings: {
      accentColor: "#38bdf8"
    }
  });
  assert.strictEqual(syncedState.settings.theme, "winter", "Must preserve winter theme across sync");
});

test("Themes: applyTheme validates and sets data-theme attribute on documentElement", async () => {
  const { applyTheme } = await import("../src/app/ui/themes.js");
  assert.strictEqual(applyTheme("winter"), "winter");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "winter");

  assert.strictEqual(applyTheme("spring"), "spring");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "spring");

  assert.strictEqual(applyTheme("autumn"), "autumn");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "autumn");

  assert.strictEqual(applyTheme("invalid_theme_name"), "default");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "default");
});

test("Icons: seasonal theme icons render valid SVGs", async () => {
  const { icon } = await import("../src/app/ui/icons.js");
  const snowflakeSvg = icon("snowflake");
  const leafSvg = icon("leaf");
  const flowerSvg = icon("flower");
  const sparklesSvg = icon("sparkles");

  assert.ok(snowflakeSvg.includes("<svg") && snowflakeSvg.includes("x1="), "Snowflake icon must be SVG");
  assert.ok(leafSvg.includes("<svg") && leafSvg.includes("<path"), "Leaf icon must be SVG");
  assert.ok(flowerSvg.includes("<svg") && flowerSvg.includes("<circle"), "Flower icon must be SVG");
  assert.ok(sparklesSvg.includes("<svg") && sparklesSvg.includes("<path"), "Sparkles icon must be SVG");
});

test("Themes: getCurrentSeason accurately maps calendar months to seasons", async () => {
  const { getCurrentSeason } = await import("../src/app/ui/themes.js");
  assert.strictEqual(getCurrentSeason(new Date("2026-01-15")), "winter", "January must be winter");
  assert.strictEqual(getCurrentSeason(new Date("2026-02-20")), "winter", "February must be winter");
  assert.strictEqual(getCurrentSeason(new Date("2026-12-25")), "winter", "December must be winter");

  assert.strictEqual(getCurrentSeason(new Date("2026-04-10")), "spring", "April must be spring");
  assert.strictEqual(getCurrentSeason(new Date("2026-05-01")), "spring", "May must be spring");

  assert.strictEqual(getCurrentSeason(new Date("2026-09-15")), "autumn", "September must be autumn");
  assert.strictEqual(getCurrentSeason(new Date("2026-10-25")), "autumn", "October must be autumn");
  assert.strictEqual(getCurrentSeason(new Date("2026-11-10")), "autumn", "November must be autumn");
});

test("Themes: resolveSeasonalTheme preserves user manual theme until season changes", async () => {
  const { resolveSeasonalTheme } = await import("../src/app/ui/themes.js");

  // User in autumn chooses default
  globalThis.localStorage = {
    _data: new Map([
      ["synchro_last_auto_season", "autumn"],
      ["synchro_theme", "default"]
    ]),
    getItem(k) { return this._data.get(k) || null; },
    setItem(k, v) { this._data.set(k, String(v)); }
  };

  const themeSameSeason = resolveSeasonalTheme();
  assert.strictEqual(themeSameSeason, "default", "Manual theme default must be preserved during current season");

  // Season changes from summer/autumn to winter
  globalThis.localStorage.setItem("synchro_last_auto_season", "summer");
  const themeNewSeason = resolveSeasonalTheme();
  // Today is October -> autumn, so it will update to autumn!
  assert.strictEqual(themeNewSeason, "autumn", "New season must automatically apply seasonal theme");
});

test("Sidebar: renders divider and disabled Scripts (Beta) button", async () => {
  const { renderShell } = await import("../src/app/ui/layout.js");
  const t = (k) => k;
  const html = renderShell({ sidebarCollapsed: false }, t);
  assert.ok(html.includes("sidebar-divider"), "Must render sidebar divider");
  assert.ok(html.includes("nav-item-disabled"), "Must render disabled Scripts button");
  assert.ok(html.includes("Scripts"), "Must include Scripts label");
  assert.ok(html.includes("nav-beta-badge") && html.includes("Beta"), "Must include Beta badge");
});

test("Titlebar: macTrafficLights toggle switches window controls to macOS traffic lights", async () => {
  const { renderShell } = await import("../src/app/ui/layout.js");
  const { appState } = await import("../src/app/core/state.js");
  const t = (k) => k;

  // Disabled: standard window buttons
  appState.settings.macTrafficLights = false;
  const defaultTitlebar = renderShell({ sidebarCollapsed: false }, t);
  assert.ok(defaultTitlebar.includes("window-btn"), "Must render standard window buttons");
  assert.ok(!defaultTitlebar.includes("mac-traffic-lights"), "Must not render mac traffic lights when disabled");

  // Enabled: macOS traffic lights
  appState.settings.macTrafficLights = true;
  const macTitlebar = renderShell({ sidebarCollapsed: false }, t);
  assert.ok(macTitlebar.includes("mac-traffic-lights"), "Must render macOS traffic lights when enabled");
  assert.ok(macTitlebar.includes("mac-light minimize"), "Must render minimize dot");
  assert.ok(macTitlebar.includes("mac-light close"), "Must render close dot");
  assert.ok(!macTitlebar.includes("mac-light maximize"), "Maximize dot must be removed");
  assert.ok(
    macTitlebar.indexOf("mac-light minimize") < macTitlebar.indexOf("mac-light close"),
    "Minimize must appear before close"
  );

  // Reset
  appState.settings.macTrafficLights = false;
});
