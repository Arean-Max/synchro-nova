import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import { getVendorLogo } from "./vendorLogos.js";

const DRIVER_CATEGORIES = [
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
    return [
      '<div class="drivers-page">',
      '  <div class="scroll-panel">',
      `    <div class="empty-state"><div class="empty-state-spinner">${icon("rotate", "spin")}</div><span>${escapeHtml(t("scanningDrivers") || (isRu ? "Сканирование ПК..." : "Scanning system..."))}</span></div>`,
      '  </div>',
      '</div>'
    ].join("");
  }

  const groupCards = DRIVER_CATEGORIES.map((cat) => {
    const items = drivers.filter((d) => {
      const cls = (d.className || d.class_name || "").toLowerCase();
      return cat.match(cls);
    });

    if (!items.length) return "";

    const title = isRu ? cat.titleRu : cat.titleEn;
    const tiles = items.map((driver) => renderDriverTile(driver, t, isRu)).join("");

    return [
      '<section class="card driver-group">',
      `  <h2 class="group-title">${icon(cat.icon)}<span>${escapeHtml(title)}</span></h2>`,
      `  <div class="driver-grid">${tiles}</div>`,
      '</section>'
    ].join("");
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
    const title = isRu ? "Прочие устройства" : "Other Hardware";
    const tiles = remaining.map((driver) => renderDriverTile(driver, t, isRu)).join("");
    groupCards.push([
      '<section class="card driver-group">',
      `  <h2 class="group-title">${icon("layers")}<span>${escapeHtml(title)}</span></h2>`,
      `  <div class="driver-grid">${tiles}</div>`,
      '</section>'
    ].join(""));
  }

  const scrollContent = groupCards.length
    ? groupCards.join("")
    : `<div class="empty-state">${escapeHtml(t("noDriversFound") || (isRu ? "Устройства не обнаружены" : "No hardware drivers found"))}</div>`;

  return [
    '<div class="drivers-page">',
    `  <div class="scroll-panel">${scrollContent}</div>`,
    '  <div class="actions tweaks-actions drivers-actions">',
    button(scanLabel, "rotate", "primary", "scan-drivers"),
    button(
      t("updateViaWindows") || (isRu ? "Обновить через Windows" : "Windows Update"),
      "settings",
      "outline",
      "open-windows-update-drivers"
    ),
    '  </div>',
    '</div>'
  ].join("");
}
