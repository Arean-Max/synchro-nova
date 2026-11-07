import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import { getVendorLogo } from "./vendorLogos.js";

export const DRIVER_CATEGORIES = [
  {
    id: "display",
    titleEn: "Graphics & Display",
    titleRu: "Видеокарта и дисплей",
    icon: "video",
    match: (cls) => cls.includes("display") || cls.includes("video") || cls.includes("graphics")
  },
  {
    id: "media",
    titleEn: "Audio & Sound",
    titleRu: "Звуковые устройства",
    icon: "volume",
    match: (cls) => cls.includes("media") || cls.includes("audio") || cls.includes("sound")
  },
  {
    id: "net",
    titleEn: "Network & Connectivity",
    titleRu: "Сетевые адаптеры",
    icon: "network",
    match: (cls) => cls.includes("net") || cls.includes("wifi") || cls.includes("lan")
  },
  {
    id: "storage",
    titleEn: "Storage & Drives",
    titleRu: "Накопители и диски",
    icon: "hardDrive",
    match: (cls) => cls.includes("storage") || cls.includes("scsi") || cls.includes("disk")
  },
  {
    id: "system",
    titleEn: "Chipset & Processor",
    titleRu: "Чипсет и процессор",
    icon: "cpu",
    match: (cls) => cls.includes("system") || cls.includes("processor") || cls.includes("chip")
  },
  {
    id: "peripherals",
    titleEn: "Peripherals & Input",
    titleRu: "Периферия и устройства",
    icon: "gamepad",
    match: (cls) =>
      cls.includes("peripheral") ||
      cls.includes("mouse") ||
      cls.includes("keyboard") ||
      cls.includes("hid") ||
      cls.includes("bluetooth")
  }
];

export function renderCharacteristicsPage(viewState, t) {
  const isRu = lang() === "ru";
  const rawDrivers = Array.isArray(viewState.drivers) && viewState.drivers.length
    ? viewState.drivers
    : (Array.isArray(viewState.system?.drivers) ? viewState.system.drivers : []);

  const isScanning = Boolean(viewState.driversLoading);
  const scanLabel = isScanning
    ? (t("scanningDrivers") || (isRu ? "Сканирование..." : "Scanning system..."))
    : (t("scanDrivers") || (isRu ? "Сканировать ПК" : "Scan PC for Drivers"));

  const drivers = rawDrivers.filter((d) => d && d.name && d.name !== "Unknown");

  if (isScanning && !drivers.length) {
    return `
      <div class="drivers-page">
        <div class="scroll-panel">
          <div class="empty-state"><div class="empty-state-spinner">${icon("rotate", "spin")}</div><span>${escapeHtml(t("scanningDrivers") || (isRu ? "Сканирование ПК..." : "Scanning system..."))}</span></div>
        </div>
      </div>
    `;
  }

  const groupCards = DRIVER_CATEGORIES.map((cat) => {
    const items = drivers.filter((d) => {
      const cls = (d.className || d.class_name || "").toLowerCase();
      return cat.match(cls);
    });

    if (!items.length) return "";

    const title = t("driverGroup_" + cat.id) || (isRu ? cat.titleRu : cat.titleEn);
    const tiles = items.map((driver) => renderDriverTile(driver, t, isRu)).join("");

    return `
      <section class="card driver-group" id="driver-section-${escapeAttr(cat.id)}">
        <h2 class="group-title">${icon(cat.icon)}<span>${escapeHtml(title)}</span></h2>
        <div class="driver-grid">${tiles}</div>
      </section>
    `;
  }).filter(Boolean);

  const categorized = new Set();
  DRIVER_CATEGORIES.forEach((cat) => {
    drivers.forEach((d) => {
      const cls = (d.className || d.class_name || "").toLowerCase();
      if (cat.match(cls)) categorized.add(d);
    });
  });
  const remaining = drivers.filter((d) => !categorized.has(d));
  if (remaining.length) {
    const title = t("driverGroup_other") || (isRu ? "Прочие устройства" : "Other Hardware");
    const tiles = remaining.map((driver) => renderDriverTile(driver, t, isRu)).join("");
    groupCards.push(`
      <section class="card driver-group" id="driver-section-other">
        <h2 class="group-title">${icon("layers")}<span>${escapeHtml(title)}</span></h2>
        <div class="driver-grid">${tiles}</div>
      </section>
    `);
  }

  const scrollContent = groupCards.length
    ? groupCards.join("")
    : `<div class="empty-state">${escapeHtml(t("noDriversFound") || (isRu ? "Устройства не обнаружены" : "No hardware drivers found"))}</div>`;

  return `
    <div class="drivers-page">
      <div class="scroll-panel">${scrollContent}</div>
      <div class="actions tweaks-actions drivers-actions">
        ${button(scanLabel, "rotate", "primary", "scan-drivers")}
        ${button(
          t("driverUpdateViaWindows") || t("updateViaWindows") || (isRu ? "Обновить через Windows" : "Windows Update"),
          "settings",
          "outline",
          "open-windows-update-drivers"
        )}
      </div>
    </div>
  `;
}

function renderDriverTile(driver, t, isRu) {
  const isOutdated = Boolean(driver.isOutdated ?? driver.is_outdated);
  const latestVer = (driver.latestVersion || driver.latest_version || "").trim();
  const vendor = (driver.vendor || "generic").toLowerCase();
  const className = driver.className || driver.class_name || "System";
  const logo = getVendorLogo(vendor, className);
  const officialUrl = driver.officialUrl || driver.official_url || "https://www.catalog.update.microsoft.com";

  let rawVer = driver.version ? driver.version.replace(/^v/, "").trim() : "";
  let versionText = rawVer ? `v${rawVer}` : "-";
  if (vendor === "nvidia" && rawVer) {
    const parts = rawVer.split(".");
    if (parts.length === 4 && parts[3].length >= 4) {
      const p2 = parts[2];
      const p3 = parts[3];
      const lastDigit = p2.slice(-1);
      const geforce = `${lastDigit}${p3.slice(0, 2)}.${p3.slice(2)}`;
      versionText = `v${geforce}`;
    }
  }

  let cleanLatest = latestVer ? latestVer.replace(/^v/, "").trim() : "";
  if (vendor === "nvidia" && cleanLatest && cleanLatest.includes(".")) {
    const parts = cleanLatest.split(".");
    if (parts.length === 4 && parts[3].length >= 4) {
      const p2 = parts[2];
      const p3 = parts[3];
      const lastDigit = p2.slice(-1);
      cleanLatest = `${lastDigit}${p3.slice(0, 2)}.${p3.slice(2)}`;
    }
  }

  const btnLabel = isOutdated
    ? (t("driverDownloadUpdate") || (isRu ? "Скачать обновление" : "Download Update"))
    : (t("driverOfficialSite") || (isRu ? "Официальный сайт" : "Official Site"));

  const dateText = driver.date || "-";
  const providerText = driver.provider || "-";
  const newVerSpec = isOutdated && cleanLatest && cleanLatest !== "-"
    ? `<div class="driver-spec-item update-hint"><span class="spec-label">${escapeHtml(t("driverAvailableVersion") || (isRu ? "Доступна:" : "Available:"))}</span> <span class="spec-val">${escapeHtml(`v${cleanLatest}`)}</span></div>`
    : "";

  const currentVerSpec = !isOutdated
    ? `<div class="driver-spec-item" title="${escapeAttr(driver.version || "")}"><span class="spec-label">${escapeHtml(t("version"))}:</span> <span class="spec-val version">${escapeHtml(versionText)}</span></div>`
    : "";

  return `
    <div class="driver-tile ${isOutdated ? "has-update" : "uptodate"}">
      <div class="driver-tile-top">
        <div class="driver-vendor-box">
          ${logo}
        </div>
        <div class="driver-tile-info">
          <span class="driver-tile-name" title="${escapeAttr(driver.name)}">${escapeHtml(driver.name)}</span>
          <span class="driver-tile-provider">${escapeHtml(providerText)}</span>
        </div>
      </div>
      <div class="driver-tile-specs">
        ${currentVerSpec}
        ${newVerSpec}
        <div class="driver-spec-item"><span class="spec-label">${escapeHtml(t("date"))}:</span> <span class="spec-val">${escapeHtml(dateText)}</span></div>
      </div>
      <div class="driver-tile-footer">
        <button class="btn ${isOutdated ? "btn-primary" : "btn-outline"} driver-tile-btn ${isOutdated ? "" : "is-uptodate"}" type="button" data-action="open-driver-url" data-url="${escapeAttr(officialUrl)}" title="${escapeAttr(driver.name)}">
          ${isOutdated ? icon("zap") : icon("rotate")}
          <span>${escapeHtml(btnLabel)}</span>
        </button>
      </div>
    </div>
  `;
}

export const characteristicsFeature = {
  render: renderCharacteristicsPage,
  mount() {},
  unmount() {}
};

