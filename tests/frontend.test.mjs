import test from "node:test";
import assert from "node:assert/strict";

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

test("State: default version is 2.5.0-beta", async () => {
  const { viewState } = await import("../src/app/core/state.js");
  assert.equal(viewState.appVersion, "2.5.0-beta");
});

test("State: pageDefs strictly matches active pageOrder", async () => {
  const { pageDefs, pageOrder } = await import("../src/app/core/state.js");
  for (const page of pageOrder) {
    assert.ok(pageDefs[page], `pageDefs.${page} must be defined`);
    assert.ok(pageDefs[page].icon, `pageDefs.${page} must have an icon`);
  }
  assert.equal(Object.keys(pageDefs).length, pageOrder.length);
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

  const s1 = mergeState({
    settings: {
      applyInstantly: true,
      accentColor: "#10b981"
    }
  });

  assert.ok(s1.settings.templateOverrides.rust_cold_tactical);
  assert.equal(s1.settings.templateOverrides.rust_cold_tactical.name, "Tactical Custom");

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

  const emptyHtml = renderApplyModal({ phase: "complete", results: [] }, t);
  assert.match(emptyHtml, /noResultsReceived/);
  assert.match(emptyHtml, /role="dialog"/);
  assert.match(emptyHtml, /aria-modal="true"/);

  const failedHtml = renderApplyModal({
    phase: "complete",
    results: [
      { id: "disable-nagle", status: "failed", message: "Access denied" },
      { id: "disable-game-bar", status: "failed", message: "Registry error" }
    ]
  }, t);
  assert.match(failedHtml, /applyFailed/);
  assert.match(failedHtml, /apply-stat-pill failed/);

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
        latestVersion: ""
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

test("HTML Security: settings attributes and content are safely escaped", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const t = (k) => k === "madeWithLove" ? 'Made with love" onfocus="alert(1)' : k;
  const mockState = {
    settings: {
      language: "en"
    }
  };
  const html = renderSettingsPage(mockState, {}, t);
  assert.ok(html.includes('&quot; onfocus=&quot;alert(1)'), "Localized content must be HTML-escaped");
  assert.ok(!html.includes('onfocus="alert(1)"'), "Raw unescaped quotes must not appear");
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

  setActivePage("tweaks");
  const tweaksNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(tweaksNavHtml.includes('class="nav-sub-list"'), "Sub-navigation list must be rendered");
  assert.ok(!tweaksNavHtml.includes('class="nav-chevron"'), "Chevron indicator must be removed");
  assert.ok(tweaksNavHtml.includes('data-action="scroll-to-section"'), "Sub-buttons must have scroll action");
  assert.ok(tweaksNavHtml.includes('data-section="tweak-section-gaming_latency"'), "Must scroll to gaming_latency");
  assert.ok(tweaksNavHtml.includes('data-section="tweak-section-scheduler"'), "Must scroll to scheduler");

  const tweaksCollapsedHtml = renderNavList(t, { sidebarCollapsed: false, collapsedSubNav: { tweaks: true } });
  assert.ok(!tweaksCollapsedHtml.includes('class="nav-sub-list"'), "Sub-navigation list must be hidden when toggled closed");

  setActivePage("characteristics");
  const driversNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(driversNavHtml.includes('data-section="driver-section-display"'), "Must scroll to driver-section-display");
  assert.ok(driversNavHtml.includes('data-section="driver-section-media"'), "Must scroll to driver-section-media");

  setActivePage("settings");
  const settingsNavHtml = renderNavList(t, { sidebarCollapsed: false });
  assert.ok(!settingsNavHtml.includes('class="nav-sub-list"'), "Settings must not render sub-navigation in unified layout");

  setActivePage("color");
});

test("Scroll Spy: resolveActiveSubSection accurately resolves boundaries including terminal section", async () => {
  const { resolveActiveSubSection } = await import("../src/app/ui/layout.js");

  const mockSections = [
    { id: "sec-1", getBoundingClientRect: () => ({ top: 0 }) },
    { id: "sec-2", getBoundingClientRect: () => ({ top: 400 }) },
    { id: "sec-3", getBoundingClientRect: () => ({ top: 800 }) },
    { id: "sec-4", getBoundingClientRect: () => ({ top: 1200 }) }
  ];

  const topPanel = {
    scrollHeight: 2000,
    clientHeight: 600,
    scrollTop: 0,
    getBoundingClientRect: () => ({ top: 0 })
  };
  assert.strictEqual(resolveActiveSubSection(topPanel, mockSections), "sec-1");

  const midPanel = {
    scrollHeight: 2000,
    clientHeight: 600,
    scrollTop: 750,
    getBoundingClientRect: () => ({ top: 0 })
  };
  const midSections = [
    { id: "sec-1", getBoundingClientRect: () => ({ top: -750 }) },
    { id: "sec-2", getBoundingClientRect: () => ({ top: -350 }) },
    { id: "sec-3", getBoundingClientRect: () => ({ top: 50 }) },
    { id: "sec-4", getBoundingClientRect: () => ({ top: 450 }) }
  ];
  assert.strictEqual(resolveActiveSubSection(midPanel, midSections), "sec-3");

  const bottomPanel = {
    scrollHeight: 2000,
    clientHeight: 600,
    scrollTop: 1400,
    getBoundingClientRect: () => ({ top: 0 })
  };
  const bottomSections = [
    { id: "sec-1", getBoundingClientRect: () => ({ top: -1400 }) },
    { id: "sec-2", getBoundingClientRect: () => ({ top: -1000 }) },
    { id: "sec-3", getBoundingClientRect: () => ({ top: -600 }) },
    { id: "sec-4", getBoundingClientRect: () => ({ top: 250 }) }
  ];
  assert.strictEqual(resolveActiveSubSection(bottomPanel, bottomSections), "sec-4");
});

test("Layout Navigation: terminal sub-item maintains active state", async () => {
  const { renderNavList } = await import("../src/app/ui/layout.js");
  const { setActivePage } = await import("../src/app/core/state.js");
  const t = (k) => k;

  setActivePage("tweaks");
  const html = renderNavList(t, {
    sidebarCollapsed: false,
    activeSubSection: "tweak-section-background"
  });

  assert.ok(html.includes('class="nav-sub-item active" type="button" data-action="scroll-to-section" data-section="tweak-section-background"'));
  assert.ok(html.includes('class="nav-sub-item " type="button" data-action="scroll-to-section" data-section="tweak-section-network"'));

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
  assert.ok(!html.includes("restore-backup"), "Restore button must not be present when no backup is selected");
  const selectedHtml = renderBackupsPage({ backups: [{ id: 'b1', name: 'Test' }], selectedBackup: 'b1' }, t);
  assert.ok(selectedHtml.includes("restore-backup"), "Restore button must be present when backup is selected");
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
  assert.strictEqual(THEMES.length, 5, "Must define 5 themes");
  const ids = THEMES.map((t) => t.id);
  assert.deepStrictEqual(ids, ["default", "winter", "autumn", "spring", "summer"]);
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
  assert.strictEqual(applyTheme("summer"), "summer");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "summer");
  assert.strictEqual(applyTheme("summer"), "summer");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "summer");

  assert.strictEqual(applyTheme("invalid_theme_name"), "default");
  assert.strictEqual(document.documentElement.getAttribute("data-theme"), "default");
});

test("Icons: seasonal theme icons render valid SVGs", async () => {
  const { icon } = await import("../src/app/ui/icons.js");
  const snowflakeSvg = icon("snowflake");
  const leafSvg = icon("leaf");
  const flowerSvg = icon("flower");
  const sunSvg = icon("sun");
  const dropletSvg = icon("droplet");
  const sparklesSvg = icon("sparkles");

  assert.ok(snowflakeSvg.includes("<svg") && snowflakeSvg.includes("x1="), "Snowflake icon must be SVG");
  assert.ok(leafSvg.includes("<svg") && leafSvg.includes("<path"), "Leaf icon must be SVG");
  assert.ok(flowerSvg.includes("<svg") && flowerSvg.includes("<circle"), "Flower icon must be SVG");
  assert.ok(sparklesSvg.includes("<svg") && sparklesSvg.includes("<path"), "Sparkles icon must be SVG");
  assert.ok(sunSvg.includes("<svg") && sunSvg.includes("<circle"), "Sun icon must be SVG");
  assert.ok(dropletSvg.includes("<svg") && dropletSvg.includes("<path"), "Droplet icon must be SVG");
});

test("Themes: getCurrentSeason accurately maps calendar months to seasons", async () => {
  const { getCurrentSeason } = await import("../src/app/ui/themes.js");
  assert.strictEqual(getCurrentSeason(new Date("2026-01-15")), "winter", "January must be winter");
  assert.strictEqual(getCurrentSeason(new Date("2026-02-20")), "winter", "February must be winter");
  assert.strictEqual(getCurrentSeason(new Date("2026-12-25")), "winter", "December must be winter");

  assert.strictEqual(getCurrentSeason(new Date("2026-04-10")), "spring", "April must be spring");
  assert.strictEqual(getCurrentSeason(new Date("2026-05-01")), "spring", "May must be spring");
  assert.strictEqual(getCurrentSeason(new Date("2026-07-15")), "summer", "July must be summer");

  assert.strictEqual(getCurrentSeason(new Date("2026-09-15")), "autumn", "September must be autumn");
  assert.strictEqual(getCurrentSeason(new Date("2026-10-25")), "autumn", "October must be autumn");
  assert.strictEqual(getCurrentSeason(new Date("2026-11-10")), "autumn", "November must be autumn");
});

test("Themes: resolveSeasonalTheme preserves user manual theme until season changes", async () => {
  const { resolveSeasonalTheme } = await import("../src/app/ui/themes.js");

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

  globalThis.localStorage.setItem("synchro_last_auto_season", "summer");
  const themeNewSeason = resolveSeasonalTheme();
  assert.strictEqual(themeNewSeason, "autumn", "New season must automatically apply seasonal theme");
});

test("Sidebar: renders divider and disabled Rust (Beta) button", async () => {
  const { renderShell } = await import("../src/app/ui/layout.js");
  const t = (k) => k;
  const html = renderShell({ sidebarCollapsed: false }, t);
  assert.ok(html.includes("sidebar-divider"), "Must render sidebar divider");
  assert.ok(html.includes("nav-item-disabled"), "Must render disabled Rust button");
  assert.ok(html.includes("Rust"), "Must include Rust label");
  assert.ok(html.includes("nav-beta-badge") && html.includes("Beta"), "Must include Beta badge");
});

test("Titlebar: macTrafficLights toggle switches window controls to macOS traffic lights", async () => {
  const { renderShell } = await import("../src/app/ui/layout.js");
  const { appState } = await import("../src/app/core/state.js");
  const t = (k) => k;

  appState.settings.macTrafficLights = false;
  const defaultTitlebar = renderShell({ sidebarCollapsed: false }, t);
  assert.ok(defaultTitlebar.includes("window-btn"), "Must render standard window buttons");
  assert.ok(!defaultTitlebar.includes("mac-traffic-lights"), "Must not render mac traffic lights when disabled");

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

  appState.settings.macTrafficLights = false;
});

test("Language List & Ordering: strictly contains the 22 user-specified languages in exact order", async () => {
  const { supportedLanguages } = await import("../src/app/core/i18n.js");
  const expectedCodes = [
    "en", "zh", "zht", "ko", "de", "es", "fr", "it", "da", "ja", "pl", "ru",
    "bs", "ar", "no", "br", "th", "tr", "uk", "bn", "gr", "vi"
  ];
  assert.strictEqual(supportedLanguages.length, 22, "Must contain exactly 22 supported languages");
  const actualCodes = supportedLanguages.map((l) => l.code);
  assert.deepStrictEqual(actualCodes, expectedCodes, "Languages must strictly match user-specified sequence");
});

test("Universal Translation Coverage: all 22 languages translate full UI, tweaks, and themes", async () => {
  const { supportedLanguages, translate } = await import("../src/app/core/i18n.js");
  const testKeys = [
    "themeTitle", "theme_default", "theme_winter", "theme_autumn", "theme_spring",
    "exit", "scriptsNav", "scriptsBeta", "scriptsBetaTitle", "skipSplash",
    "template", "backupPrefix", "totalDrivers", "scanDrivers", "safeTweak",
    "notification_success", "notification_warning", "notification_error",
    "tweak_win32_priority_rust_title", "tweak_clean_temp_junk_title",
    "tweak_gpu_adaptive_power_title", "tweak_network_udp_buffers_title"
  ];

  for (const { code } of supportedLanguages) {
    for (const key of testKeys) {
      const val = translate(code, key);
      assert.ok(val && val.trim().length > 0, `Language [${code}] returned empty string for [${key}]`);
      assert.notStrictEqual(val, key, `Language [${code}] returned literal key name for [${key}]`);
    }
  }
});

test("Notification Coverage: all 22 languages translate all notification and toast keys, tweak impact banners, and incident popups", async () => {
  const { supportedLanguages, translate } = await import("../src/app/core/i18n.js");
  const { getTweakImpactDetails } = await import("../src/app/features/tweaks/page.js");
  const { allTweaks } = await import("../src/app/features/tweaks/catalog.js");

  const notificationKeys = [
    "notification_error", "notification_info", "notification_success", "notification_warning",
    "unknownError", "systemError", "executionError",
    "securityShieldTitle", "securityShieldBlocked",
    "eacFixedTitle", "eacFixedMsg", "eacShieldTitle",
    "rollbackSuccess", "rollbackFailed", "noSelection",
    "blackHolo", "blackHoloEnabled", "blackHoloDisabled",
    "screenshot", "screenshotCopiedToast", "screenshotCopied",
    "action_trim_memory", "ramPurgedToast",
    "colorTitle", "templateAppliedToast", "colorsUpdatedToast",
    "tweakErrorTitle", "tweakErrorMsg", "statusFailed",
    "warning", "safetyBackup", "riskWarningMsg",
    "safetySnapshot", "backupNotificationTitle", "backupNotificationMsg",
    "adminRequired", "adminNotificationTitle", "adminNotificationMsg",
    "safeTweak", "safeTweakDesc", "riskTweak", "riskTweakDesc",
    "experimentalTweak", "experimentalTweakDesc", "systemTweak", "systemTweakDesc",
    "noKnownConflict", "whatBreaks", "whyTweakFailed", "appliedTweaks", "close",
    "cleanedTempFilesMsg", "copiedToClipboard",
    "statusSuccess", "statusSkipped", "statusReverted"
  ];

  for (const { code } of supportedLanguages) {
    for (const key of notificationKeys) {
      const val = translate(code, key);
      assert.ok(val && val.trim().length > 0, `Language [${code}] returned empty string for notification key [${key}]`);
      assert.notStrictEqual(val, key, `Language [${code}] returned literal key name for notification key [${key}]`);
    }
  }

  const tweaks = allTweaks();
  for (const { code } of supportedLanguages) {
    const t = (k) => translate(code, k);
    for (const tweak of tweaks.slice(0, 5)) {
      const details = getTweakImpactDetails(tweak, t, code === "ru");
      assert.ok(details.appName && details.appName.length > 0, `Empty tweak impact appName for [${code}] ${tweak.id}`);
      assert.ok(details.impactText && details.impactText.length > 0, `Empty tweak impactText for [${code}] ${tweak.id}`);
      if (code === "ru") {
        assert.ok(!details.impactText.startsWith("Windows prioritizes"), `Russian impactText should not be English for ${tweak.id}`);
      }
      if (code === "zh") {
        assert.ok(!details.impactText.startsWith("Windows prioritizes"), `Chinese impactText should not be English for ${tweak.id}`);
      }
    }
  }
});


test("Settings and Help Banners: all 22 languages translate macTrafficLights, eacAutoProtect, and theme card names", async () => {
  const { supportedLanguages, translate } = await import("../src/app/core/i18n.js");
  const { THEMES } = await import("../src/app/ui/themes.js");

  for (const { code } of supportedLanguages) {
    const t = (k) => translate(code, k);
    assert.ok(t("macTrafficLights") && t("macTrafficLights") !== "macTrafficLights", `[${code}] missing macTrafficLights`);
    assert.ok(t("help_macTrafficLights") && t("help_macTrafficLights") !== "help_macTrafficLights", `[${code}] missing help_macTrafficLights`);
    assert.ok(t("eacAutoProtect") && t("eacAutoProtect") !== "eacAutoProtect", `[${code}] missing eacAutoProtect`);
    assert.ok(t("help_eacAutoProtect") && t("help_eacAutoProtect") !== "help_eacAutoProtect", `[${code}] missing help_eacAutoProtect`);

    for (const theme of THEMES) {
      const themeName = t("theme_" + theme.id);
      assert.ok(themeName && themeName !== "theme_" + theme.id, `[${code}] missing theme_${theme.id}`);
    }
  }
});


test("Backup Modal: renders full structured dialog card with header, input label, and action buttons", async () => {
  const { renderBackupNameModal } = await import("../src/app/features/storage/page.js");
  const { translate } = await import("../src/app/core/i18n.js");

  const t = (k) => translate("ru", k);
  const html = renderBackupNameModal({ backupName: "TestBackup" }, t);

  assert.ok(html.includes("template-config-dialog"), "Must contain template-config-dialog card class");
  assert.ok(html.includes("backup-name-dialog"), "Must contain backup-name-dialog class");
  assert.ok(html.includes("template-config-header"), "Must render dialog header");
  assert.ok(html.includes("dialog-close-btn"), "Must render close (x) button");
  assert.ok(html.includes("template-name-input"), "Must render styled name input");
  assert.ok(html.includes('data-action="dismiss-backup-modal"'), "Must have cancel / dismiss action");
  assert.ok(html.includes('data-action="confirm-create-backup"'), "Must have confirm-create-backup action");
});


test("Color Template Modal: omits gray subtext and layout eliminates empty black bands", async () => {
  const { renderTemplateConfigModal } = await import("../src/app/features/color/page.js");
  const { translate } = await import("../src/app/core/i18n.js");
  const fsMod = await import("node:fs");

  const t = (k) => translate("ru", k);
  const appState = {
    settings: { templateOverrides: {} },
    color: { saturation: 100, hue: 0, contrast: 100, gamma: 100 }
  };
  const viewState = {
    editingTemplate: "balanced",
    editingTemplateName: "Сбалансированный",
    editingTemplateColor: { saturation: 160, hue: -5, contrast: 97, gamma: 118 }
  };

  const html = renderTemplateConfigModal(appState, viewState, t);

  assert.ok(html.includes("template-config-dialog"), "Must render template-config-dialog");
  assert.ok(html.includes('id="template-config-title"'), "Must render title");
  assert.ok(!html.includes("configureTemplateDesc"), "Must NOT contain configureTemplateDesc");
  assert.ok(!html.includes("<p>"), "Must NOT contain gray subtext paragraph in modal header");

  const css = fsMod.readFileSync("src/styles.css", "utf8");
  assert.ok(css.includes(".template-config-title p") && css.includes("display: none"), "CSS must hide template-config-title p");
  assert.ok(css.includes(".global-color-grid") && css.includes("flex: 1 1 auto;"), "CSS must allow global-color-grid to stretch");
  assert.ok(css.includes(".smart-tips-backdrop") && css.includes("backdrop-filter: blur(12px);"), "Backdrop must provide smooth frosted blur");
});
test("Module Integration: all feature modules export valid handlers without missing dependencies", async () => {
  const { router } = await import("../src/app/core/router.js");
  const { backupsFeature } = await import("../src/app/features/storage/page.js");
  const { colorFeature } = await import("../src/app/features/color/page.js");
  const { tweaksFeature } = await import("../src/app/features/tweaks/page.js");
  const { bindsFeature } = await import("../src/app/features/binds/page.js");
  const { characteristicsFeature } = await import("../src/app/features/characteristics/page.js");
  const { settingsFeature } = await import("../src/app/features/settings/page.js");

  assert.equal(typeof backupsFeature.render, "function");
  assert.equal(typeof colorFeature.render, "function");
  assert.equal(typeof tweaksFeature.render, "function");
  assert.equal(typeof bindsFeature.render, "function");
  assert.equal(typeof characteristicsFeature.render, "function");
  assert.equal(typeof settingsFeature.render, "function");
  assert.equal(typeof router.activePageName, "function");
  assert.equal(typeof router.getCurrentPage, "function");
});

test("Language Picker Flags: settings dropdown renders authentic iOS emoji flag images for all 22 supported languages", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const { supportedLanguages } = await import("../src/app/core/i18n.js");
  const fs = await import("node:fs");
  const path = await import("node:path");

  const mockState = {
    settings: {
      language: "ru",
      accentColor: "#ffffff",
      templateOverrides: {}
    }
  };
  const t = (key) => key;

  const htmlClosed = renderSettingsPage(mockState, { languageDropdownOpen: false }, t);
  assert.ok(htmlClosed.includes('class="language-dropdown-flag" src="./assets/flags/ru.png"'), "Trigger must render current language flag");

  const htmlOpen = renderSettingsPage(mockState, { languageDropdownOpen: true }, t);
  for (const { code } of supportedLanguages) {
    const flagPath = path.resolve("src", "assets", "flags", `${code}.png`);
    assert.ok(fs.existsSync(flagPath), `Flag asset file must exist on disk for [${code}]`);
    const stats = fs.statSync(flagPath);
    assert.ok(stats.size > 1000, "Flag asset must be non-empty image");

    assert.ok(htmlOpen.includes(`class="language-item-flag" src="./assets/flags/${code}.png"`), `Dropdown menu must render flag image for [${code}]`);
  }
});

test("Settings: renders Made with love badge with iOS heart emoji across all 22 languages", async () => {
  const { renderSettingsPage } = await import("../src/app/features/settings/page.js");
  const { supportedLanguages, translate } = await import("../src/app/core/i18n.js");
  const fs = await import("node:fs");
  const path = await import("node:path");

  const heartPath = path.resolve("src/assets/heart.png");
  assert.ok(fs.existsSync(heartPath), "Apple iOS heart.png asset must exist");
  assert.ok(fs.statSync(heartPath).size > 1000, "heart.png must be a valid image");

  for (const { code } of supportedLanguages) {
    const localized = translate(code, "madeWithLove");
    assert.ok(localized && localized.trim().length > 0, `Language [${code}] missing madeWithLove translation`);
    assert.notStrictEqual(localized, "madeWithLove", `Language [${code}] returned literal key name`);

    const html = renderSettingsPage({ settings: { language: code } }, {}, (k) => translate(code, k));
    assert.ok(html.includes("settings-made-with-love"), "Must render settings-made-with-love container");
    assert.ok(html.includes("./assets/heart.png"), "Must render heart.png emoji image");
    const { escapeHtml } = await import("../src/app/core/html.js");
    assert.ok(html.includes(escapeHtml(localized)), `Settings HTML must include localized string for [${code}]`);
    assert.ok(!html.includes("accent-picker-wrapper"), "Must not render removed RGB accent picker");
  }
});

test("Keybinds: Create First Bind button successfully creates a bind and updates UI from empty state", async () => {
  const { renderBindsPage } = await import("../src/app/features/binds/page.js");
  const t = (k) => k;

  const emptyState = { keybinds: [] };
  const emptyHtml = renderBindsPage(emptyState, {}, t);
  assert.ok(emptyHtml.includes('data-action="add-bind"'), "Must render add-bind button");
  assert.ok(emptyHtml.includes("binds-empty-state"), "Empty state must be rendered when binds are empty");

  const populatedState = {
    keybinds: [
      { id: "bind_test_1", key: "F1", modifiers: 0, vk: 112, action: "template:vibrant", enabled: true }
    ]
  };
  const populatedHtml = renderBindsPage(populatedState, {}, t);
  assert.ok(!populatedHtml.includes("binds-empty-state"), "Must not show empty state when bind exists");
  assert.ok(populatedHtml.includes("bind-row"), "Must render bind rows");
  assert.ok(populatedHtml.includes("bind_test_1"), "Must render the created bind");
});

test("Characteristics: renders scanning card when loading and structured categories when loaded", async () => {
  const { renderCharacteristicsPage } = await import("../src/app/features/characteristics/page.js");
  const t = (k) => k;

  const scanningHtml = renderCharacteristicsPage({ drivers: [], driversLoading: true }, t);
  assert.ok(scanningHtml.includes("driver-scanning-card"));
  assert.ok(scanningHtml.includes("scan-drivers"));

  const populatedHtml = renderCharacteristicsPage({
    drivers: [
      { name: "NVIDIA GeForce RTX 3060", vendor: "nvidia", className: "Display", version: "32.0.16.2101", isOutdated: false },
      { name: "Realtek High Definition Audio", vendor: "realtek", className: "Media", version: "6.0.9000.1", isOutdated: false }
    ],
    driversLoading: false
  }, t);

  assert.ok(!populatedHtml.includes("driver-empty-card"));
  assert.ok(populatedHtml.includes("driver-section-display"));
  assert.ok(populatedHtml.includes("driver-section-media"));
  assert.ok(populatedHtml.includes("NVIDIA GeForce RTX 3060"));
  assert.ok(populatedHtml.includes("Realtek High Definition Audio"));
});
