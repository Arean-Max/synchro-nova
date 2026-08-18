import { safeValue, escapeAttr, escapeHtml } from "../../core/html.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

function rawValue(value, fallback = "-") {
  if (value === undefined || value === null || value === "") return fallback;
  return String(value);
}

function numberValue(value) {
  const number = Number.parseFloat(String(value || "").replace(",", "."));
  return Number.isFinite(number) ? number : 0;
}

function clamp(value, min = 0, max = 100) {
  return Math.min(max, Math.max(min, value));
}

function gb(value) {
  const number = numberValue(value);
  return number ? `${number.toFixed(1)} GB` : "-";
}

function liveAttr(key) {
  return key ? ` data-live="${escapeAttr(key)}"` : "";
}

function fillAttr(key) {
  return key ? ` data-fill="${escapeAttr(key)}"` : "";
}

function compactName(value, fallback = "-") {
  const name = rawValue(value, fallback).replace(/\s+/g, " ").trim();
  return name.length > 46 ? `${name.slice(0, 43)}...` : name;
}

function percent(info) {
  return clamp(numberValue(info.ramUsedPercent));
}

function smallMeter(label, value, width, key = "") {
  return `<div class="mini-meter"><span>${escapeHtml(label)}</span><strong${liveAttr(key)}>${safeValue(value)}</strong><i><b${fillAttr(key)} style="width:${clamp(width)}%"></b></i></div>`;
}

function topMetric(type, body) {
  return `<section class="character-metric metric-${escapeAttr(type)}">${body}</section>`;
}

function cpuGraphPoints(history) {
  const values = [...(Array.isArray(history) ? history : [])];
  while (values.length < 28) values.unshift(values[0] || 0);
  const data = values.slice(-36);
  return data.map((value, index) => {
    const x = data.length <= 1 ? 0 : (index / (data.length - 1)) * 132;
    const y = 64 - (clamp(value) / 100) * 56 - 4;
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  }).join(" ");
}

function cpuGraph(history, current) {
  const values = [...(Array.isArray(history) ? history : [])];
  if (!values.length) values.push(current);
  return `<svg class="cpu-graph" viewBox="0 0 132 64" preserveAspectRatio="none" aria-hidden="true"><path d="M0 16H132M0 32H132M0 48H132M33 0V64M66 0V64M99 0V64"></path><polyline points="${cpuGraphPoints(values)}"></polyline></svg>`;
}

function cpuMetric(info, history) {
  const cpuUsage = clamp(numberValue(info.cpuUsagePercent));
  return topMetric(
    "cpu",
    `<div class="metric-head">${icon("cpu")}<span>CPU</span></div><div class="metric-main"><strong>${safeValue(info.cpuCores || "--")}</strong><em data-live="cpu-usage">${cpuUsage}% usage</em></div>${cpuGraph(history, cpuUsage)}<h3>${safeValue(compactName(info.cpu))}</h3><p>${safeValue(info.cpuCores || "--")} Logical Cores</p>`
  );
}

function vramTotalLabel(info) {
  const total = numberValue(info.vramTotalGb);
  return total ? `${total.toFixed(1)} GB` : "Not reported";
}

function gpuMetric(info) {
  const vramPercent = clamp(numberValue(info.vramUsedPercent));
  const gpuUsage = clamp(numberValue(info.gpuUsagePercent));
  return topMetric(
    "gpu",
    `<div class="metric-head">${icon("monitor")}<span>GPU</span></div><div class="metric-main"><strong>${safeValue(info.gpuCount || "--")}</strong><em>Devices</em></div><div class="metric-stack">${smallMeter("VRAM", vramTotalLabel(info), vramPercent, "vram-total")}${smallMeter("Usage", `${gpuUsage}%`, gpuUsage, "gpu-usage")}</div><h3>${safeValue(compactName(info.gpu))}</h3>`
  );
}

function ramMetric(info) {
  const ramPercent = percent(info);
  const total = gb(info.ramTotalGb || info.ram);
  return topMetric(
    "ram",
    `<div class="metric-head">${icon("archive")}<span>RAM</span></div><div class="metric-main"><strong>${safeValue(total)}</strong></div><div class="ram-ring" data-ring="ram" style="--ram:${ramPercent}%"><strong data-live="ram-percent">${ramPercent}%</strong></div>`
  );
}

function hzMetric(info) {
  return topMetric(
    "hz",
    `<div class="metric-head">${icon("video")}<span>Hz</span></div><div class="metric-main"><strong>${safeValue(info.refreshRateHz || "--")}</strong><em>${safeValue(info.colorDepth || "-")}</em></div><div class="screen-glyph"><span>Hz</span></div><h3>Adaptive Sync indicator</h3><p>Status unavailable</p>`
  );
}

function detailRow(label, value, key = "") {
  return `<div class="spec-row"><span>${escapeHtml(label)}</span><strong${liveAttr(key)}>${safeValue(value)}</strong></div>`;
}

function progressRow(label, value, width, key = "") {
  return `<div class="spec-progress"><div><span>${escapeHtml(label)}</span><strong${liveAttr(key)}>${safeValue(value)}</strong></div><i><b${fillAttr(key)} style="width:${clamp(width)}%"></b></i></div>`;
}

function panel(title, body, extra = "") {
  return `<section class="character-panel ${extra}"><h2>${escapeHtml(title)}</h2>${body}</section>`;
}

function systemPanel(info, t) {
  return panel(
    t("system"),
    [
      detailRow("CPU", compactName(info.cpu, "-")),
      detailRow("OS", compactName(info.os || "Windows", "-")),
      detailRow(t("architecture"), info.architecture)
    ].join("")
  );
}

function memoryPanel(info, t) {
  const ramPercent = percent(info);
  const available = numberValue(info.ramAvailableGb || info.ramAvailable);
  const total = numberValue(info.ramTotalGb || info.ram);
  const availablePercent = total ? (available / total) * 100 : 0;
  return panel(
    t("memory"),
    [
      progressRow(t("memory"), gb(info.ramTotalGb || info.ram), 100),
      progressRow(t("available"), gb(info.ramAvailableGb || info.ramAvailable), availablePercent, "ram-available"),
      progressRow(t("used"), `${ramPercent}%`, ramPercent, "ram-used")
    ].join("")
  );
}

function displayPanel(info, t) {
  return panel(
    t("display"),
    `${detailRow("Resolution", info.display)}${detailRow("Refresh rate", info.refreshRate)}<div class="resolution-tile"><span>Resolution</span><strong>${safeValue(info.display)}</strong></div>`
  );
}

function graphicsPanel(info, t) {
  const gpuList = Array.isArray(info.gpus) && info.gpus.length ? info.gpus : [info.gpu].filter(Boolean);
  return panel(
    t("graphics"),
    [
      detailRow(t("devices"), info.gpuCount),
      detailRow(t("primaryDevice"), compactName(gpuList[0], "-"))
    ].join("")
  );
}

function driverKey(driver) {
  return `${driver.className || ""}|${driver.name || ""}|${driver.version || ""}`.toLowerCase();
}

function normalizeDrivers(drivers) {
  const seen = new Set();
  return (Array.isArray(drivers) ? drivers : []).filter((driver) => {
    const name = String(driver?.name || "").trim();
    if (!name || name === "Unknown driver") return false;
    const key = driverKey(driver);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function driverStatus(driver) {
  const status = String(driver.status || "").toLowerCase();
  if (status.includes("disabled") || status.includes("problem") || status.includes("unknown")) return ["problem", "Problem"];
  if (status.includes("demand")) return ["warn", "Manual"];
  return ["ok", "Detected"];
}

function driverKind(driver) {
  const text = `${driver.name || ""} ${driver.provider || ""} ${driver.className || ""} ${driver.path || ""}`.toLowerCase();
  if (text.includes("nvidia")) return { id: "nvidia" };
  if (text.includes("amd") || text.includes("radeon")) return { id: "amd" };
  if (text.includes("intel")) return { id: "intel" };
  if (text.includes("realtek")) return { id: "realtek", icon: "volume" };
  if (text.includes("bluetooth") || text.includes("bth")) return { id: "bluetooth", icon: "bluetooth" };
  if (text.includes("audio") || text.includes("sound") || text.includes("hdaud")) return { id: "audio", icon: "volume" };
  if (text.includes("usb") || text.includes("hid")) return { id: "usb", icon: "usb" };
  if (text.includes("wifi") || text.includes("wi-fi") || text.includes("network") || text.includes("ndis") || text.includes("ethernet")) return { id: "network", icon: "network" };
  if (text.includes("nvme") || text.includes("storage") || text.includes("disk") || text.includes("stor")) return { id: "storage", icon: "hardDrive" };
  if (text.includes("display") || text.includes("graphics") || text.includes("directx") || text.includes("dxg")) return { id: "display", icon: "monitor" };
  if (text.includes("keyboard") || text.includes("mouse") || text.includes("input")) return { id: "input", icon: "mouse" };
  if (text.includes("microsoft") || text.includes("windows")) return { id: "windows", icon: "grid" };
  return { id: "unknown", label: "?" };
}

function brandBadge(id) {
  const badges = {
    nvidia: '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3.5 12c3.8-4.8 10.4-5.4 15.7-.9-4.1-1.4-7.6-.7-10.1 2.1 1.6-1 3.8-1.3 5.8-.3-1.4 2.2-4.4 3.3-7.2 1.8 2.8 3.5 7.8 3 12.8-2.7-5.1 8-13.8 7.1-17 0z"></path></svg>',
    amd: '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M5 5h14v14h-5v-8H5z"></path><path d="M9 15h4v4H5v-8h4z"></path></svg>',
    intel: '<svg viewBox="0 0 24 24" aria-hidden="true"><rect x="5" y="5" width="14" height="14" rx="3"></rect><path d="M9 9h6M9 12h6M9 15h4"></path></svg>'
  };
  return badges[id] || "";
}

function driverIcon(driver) {
  const kind = driverKind(driver);
  const body = brandBadge(kind.id) || (kind.icon ? icon(kind.icon) : escapeHtml(kind.label || "?"));
  return `<span class="driver-logo driver-logo-${escapeAttr(kind.id)}">${body}</span>`;
}

function driverCategory(driver) {
  const text = `${driver.name || ""} ${driver.provider || ""} ${driver.className || ""} ${driver.path || ""}`.toLowerCase();
  if (text.includes("nvidia") || text.includes("radeon") || text.includes("display") || text.includes("graphics") || text.includes("directx") || text.includes("dxg") || text.includes("lddm")) return "graphics";
  if (text.includes("audio") || text.includes("sound") || text.includes("hdaud") || text.includes("realtek")) return "audio";
  if (text.includes("processor") || text.includes("cpu") || text.includes("intelppm") || text.includes("amdppm")) return "processor";
  if (text.includes("chipset") || text.includes("pci") || text.includes("smbus") || text.includes("gpio") || text.includes("serial io") || text.includes("management engine") || text.includes("acpi")) return "chipset";
  if (text.includes("network") || text.includes("ethernet") || text.includes("wi-fi") || text.includes("wifi") || text.includes("ndis")) return "network";
  if (text.includes("nvme") || text.includes("storage") || text.includes("disk") || text.includes("stor")) return "storage";
  return "other";
}

function driverRow(driver, t) {
  const query = driver.searchQuery || `${driver.name || ""} ${driver.version || ""} driver latest version`;
  const [state, label] = driverStatus(driver);
  const details = [
    detailRow(t("provider"), driver.provider),
    detailRow(t("version"), driver.version),
    detailRow(t("date"), driver.date),
    detailRow(t("className"), driver.className),
    detailRow(t("status"), driver.status),
    detailRow(t("path"), driver.path)
  ].join("");
  return `<details class="driver-row driver-${escapeAttr(state)}"><summary>${driverIcon(driver)}<strong>${safeValue(driver.name)}</strong><em>${safeValue(driver.version)}</em><b>${escapeHtml(label)}</b></summary><div class="driver-details">${details}<button class="driver-search" type="button" data-driver-search="${escapeAttr(query)}">${t("driverLatestQuestion")}</button></div></details>`;
}

function driverGroup(title, drivers, t) {
  const rows = drivers.length
    ? drivers.map((driver) => driverRow(driver, t)).join("")
    : `<div class="driver-empty">${t("waiting")}</div>`;
  return `<div class="driver-group"><h3>${escapeHtml(title)}</h3>${rows}</div>`;
}

function driverPanel(info, t, showAll) {
  const drivers = normalizeDrivers(info.drivers);
  const by = (categories) => drivers.filter((driver) => categories.includes(driverCategory(driver)));
  const groups = showAll
    ? [
        ["Graphics Drivers", by(["graphics"])],
        ["Chipset & Processor", by(["chipset", "processor"])],
        ["Audio Drivers", by(["audio"])],
        ["Network & Storage", by(["network", "storage"])],
        ["Other Drivers", by(["other"])]
      ]
    : [
        ["Graphics Drivers", by(["graphics"])],
        ["Chipset & Processor", by(["chipset", "processor"])],
        ["Audio Drivers", by(["audio"])]
      ];
  const body = groups.map(([title, items]) => driverGroup(title, items, t)).join("");
  const label = showAll ? t("showKeyDrivers") : `${t("viewAllDrivers")} ${drivers.length || 0} ${t("drivers")}`;
  return `<section class="character-panel drivers-card"><h2>${t("drivers")}</h2><div class="drivers-body">${body}</div><button class="driver-view-all" type="button" data-action="toggle-all-drivers">${escapeHtml(label)}</button></section>`;
}

export function renderCharacteristicsPage(viewState, t) {
  const info = viewState.system || {};
  const metrics = [cpuMetric(info, viewState.cpuHistory), gpuMetric(info), ramMetric(info), hzMetric(info)].join("");
  const panels = [systemPanel(info, t), memoryPanel(info, t), displayPanel(info, t), graphicsPanel(info, t)].join("");
  return `<div class="characteristics-page"><div class="characteristics-actions">${button(t("refresh"), "rotate", "outline", "refresh-characteristics")}</div><div class="character-summary">${metrics}</div><div class="characteristics-dashboard"><div class="characteristics-grid">${panels}</div>${driverPanel(info, t, viewState.showAllDrivers)}</div></div>`;
}

export function updateCharacteristicsLiveDom(viewState) {
  const info = viewState.system || {};
  const cpuUsage = clamp(numberValue(info.cpuUsagePercent));
  const gpuUsage = clamp(numberValue(info.gpuUsagePercent));
  const ramPercent = percent(info);
  const available = numberValue(info.ramAvailableGb || info.ramAvailable);
  const total = numberValue(info.ramTotalGb || info.ram);
  const availablePercent = total ? (available / total) * 100 : 0;
  const availableLabel = gb(info.ramAvailableGb || info.ramAvailable);
  const vramPercent = clamp(numberValue(info.vramUsedPercent));
  const setText = (key, value) => {
    document.querySelectorAll(`[data-live="${key}"]`).forEach((element) => {
      element.textContent = value;
    });
  };
  const setFill = (key, value) => {
    document.querySelectorAll(`[data-fill="${key}"]`).forEach((element) => {
      element.style.width = `${clamp(value)}%`;
    });
  };
  setText("cpu-usage", `${cpuUsage}% usage`);
  const cpuLine = document.querySelector(".cpu-graph polyline");
  if (cpuLine) cpuLine.setAttribute("points", cpuGraphPoints(viewState.cpuHistory));
  setText("gpu-usage", `${gpuUsage}%`);
  setFill("gpu-usage", gpuUsage);
  setText("vram-total", vramTotalLabel(info));
  setFill("vram-total", vramPercent);
  setText("ram-percent", `${ramPercent}%`);
  setText("ram-available", availableLabel);
  setFill("ram-available", availablePercent);
  setText("ram-used", `${ramPercent}%`);
  setFill("ram-used", ramPercent);
  const ring = document.querySelector('[data-ring="ram"]');
  if (ring) ring.style.setProperty("--ram", `${ramPercent}%`);
}
