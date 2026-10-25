import test from "node:test";
import assert from "node:assert/strict";

// Global DOM mock for Node environment tests
globalThis.window = globalThis;
globalThis.document = {
  documentElement: {
    lang: "en",
    dir: "ltr",
    style: {
      setProperty: () => {}
    }
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
