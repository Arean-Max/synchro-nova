import { sliderDefs } from "../../core/state.js";
import { escapeAttr, escapeHtml, safeValue } from "../../core/html.js";
import { button, card, checkbox } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

const demoColorGames = [
  { id: "demo-cs2", name: "Counter-Strike 2", source: "Demo", launchable: false },
  { id: "demo-valorant", name: "Valorant", source: "Demo", launchable: false },
  { id: "demo-dota2", name: "Dota 2", source: "Demo", launchable: false },
  { id: "demo-cyberpunk", name: "Cyberpunk 2077", source: "Demo", launchable: false },
  { id: "demo-apex", name: "Apex Legends", source: "Demo", launchable: false }
];

export function fallbackColorGames() {
  return demoColorGames.map((game) => ({ ...game }));
}

export function colorGames(viewState) {
  return Array.isArray(viewState.colorGames) ? viewState.colorGames : [];
}

export function selectedColorGameIndex(viewState) {
  const games = colorGames(viewState);
  if (!games.length) return -1;
  const index = games.findIndex((game) => game.id === viewState.selectedColorGame);
  return index >= 0 ? index : 0;
}

export function selectedColorGame(viewState) {
  const games = colorGames(viewState);
  const index = selectedColorGameIndex(viewState);
  return index >= 0 ? games[index] : null;
}

export function ensureColorGameSelection(viewState) {
  const games = colorGames(viewState);
  if (!games.length) {
    viewState.selectedColorGame = "";
    viewState.colorGamePanelOpen = false;
    return;
  }
  if (!games.some((game) => game.id === viewState.selectedColorGame)) {
    viewState.selectedColorGame = games[0].id;
  }
}

export function selectColorGame(viewState, gameId, openPanel = true) {
  const games = colorGames(viewState);
  const game = games.find((item) => item.id === gameId);
  if (!game) return false;
  viewState.selectedColorGame = game.id;
  if (openPanel) viewState.colorGamePanelOpen = true;
  return true;
}

export function stepColorGame(viewState, step) {
  const games = colorGames(viewState);
  if (!games.length) return false;
  const index = selectedColorGameIndex(viewState);
  const nextIndex = (index + step + games.length) % games.length;
  viewState.selectedColorGame = games[nextIndex].id;
  return true;
}

export function sliderPercent(field, value) {
  const def = sliderDefs[field];
  return ((value - def.min) / (def.max - def.min)) * 100;
}

export function formatSliderValue(field, value) {
  const def = sliderDefs[field];
  return `${Math.round(value)}${def.suffix}`;
}

function slider(field, appState, t) {
  const def = sliderDefs[field];
  const label = t ? t(field) : def.label;
  const value = Number(appState.color[field]);
  const zero = def.zero ? '<span class="zero-mark"></span>' : "";
  return `<div class="slider-row" data-slider-row="${field}"><div class="slider-meta"><span>${escapeHtml(label)}</span><span data-slider-value="${field}">${formatSliderValue(field, value)}</span></div><div class="slider-track" style="--slider-percent:${sliderPercent(field, value)}%"><span class="slider-fill"></span>${zero}<span class="slider-thumb"></span><input class="range-input" type="range" min="${def.min}" max="${def.max}" step="${def.step}" value="${escapeAttr(value)}" data-color-field="${field}" aria-label="${escapeAttr(label)}"></div></div>`;
}

function hashString(value) {
  let hash = 0;
  for (let index = 0; index < value.length; index += 1) {
    hash = (hash * 31 + value.charCodeAt(index)) >>> 0;
  }
  return hash;
}

function gameAccent(game) {
  const palette = ["#3b82f6", "#14b8a6", "#f97316", "#ef4444", "#eab308", "#22c55e", "#8b5cf6", "#06b6d4"];
  return palette[hashString(game.id || game.name || "") % palette.length];
}

function gameLogoText(name) {
  const words = String(name || "")
    .replace(/[^\p{L}\p{N}\s:-]/gu, " ")
    .split(/\s+/)
    .filter(Boolean);
  if (!words.length) return "GAME";
  if (words.length === 1) return escapeHtml(words[0].slice(0, 12));
  return words
    .slice(0, 2)
    .map((word) => escapeHtml(word.slice(0, 10)))
    .join("<br>");
}

function gameImageSrc(game) {
  const imagePath = game?.imagePath;
  if (!imagePath) return "";
  if (imagePath.startsWith("data:") || imagePath.startsWith("http:") || imagePath.startsWith("https:")) {
    return imagePath;
  }
  const convertFileSrc = window.__TAURI__?.core?.convertFileSrc || window.__TAURI__?.tauri?.convertFileSrc;
  if (!convertFileSrc) return "";
  try {
    return convertFileSrc(imagePath);
  } catch {
    return "";
  }
}

function gameLogoSrc(game) {
  const logoPath = game?.logoPath;
  if (!logoPath) return "";
  if (logoPath.startsWith("data:") || logoPath.startsWith("http:") || logoPath.startsWith("https:")) {
    return logoPath;
  }
  const convertFileSrc = window.__TAURI__?.core?.convertFileSrc || window.__TAURI__?.tauri?.convertFileSrc;
  if (!logoPath || !convertFileSrc) return "";
  try {
    return convertFileSrc(logoPath);
  } catch {
    return "";
  }
}

function relativeGameOffset(index, selectedIndex, total) {
  let offset = index - selectedIndex;
  if (offset > total / 2) offset -= total;
  if (offset < -total / 2) offset += total;
  return offset;
}

function renderGameCard(game, offset, active) {
  const imageSrc = gameImageSrc(game);
  const logoSrc = gameLogoSrc(game);
  const image = imageSrc ? `<img class="game-card-image" src="${escapeAttr(imageSrc)}" alt="" onerror="this.style.display='none'">` : "";
  const source = game.source ? `<span class="game-card-source">${escapeHtml(game.source)}</span>` : "";
  const logo = gameLogoText(game.name);
  const logoImage = logoSrc ? `<img class="game-card-logo-image" src="${escapeAttr(logoSrc)}" alt="" onerror="this.style.display='none'">` : "";
  const watermark = logoSrc
    ? `<img class="game-card-watermark image" src="${escapeAttr(logoSrc)}" alt="" aria-hidden="true" onerror="this.style.display='none'">`
    : `<span class="game-card-watermark" aria-hidden="true">${logo}</span>`;
  return [
    `<button class="game-card" type="button" data-game-id="${escapeAttr(game.id)}" data-offset="${offset}" data-logo-loaded="${logoSrc ? "true" : "false"}" aria-current="${active ? "true" : "false"}" style="--game-accent:${gameAccent(game)}">`,
    image,
    '<span class="game-card-glow" aria-hidden="true"></span>',
    watermark,
    logoImage,
    `<span class="game-card-logo">${logo}</span>`,
    source,
    "</button>"
  ].join("");
}

function renderGameCarousel(viewState, t) {
  const games = colorGames(viewState);
  if (!viewState.colorGamesLoaded) {
    return `<section class="game-carousel game-carousel-empty"><div class="game-empty-state">${t("loading")}</div></section>`;
  }
  if (!games.length) {
    return `<section class="game-carousel game-carousel-empty"><div class="game-empty-state">${t("noGamesFound")}</div></section>`;
  }

  ensureColorGameSelection(viewState);
  const selectedIndex = selectedColorGameIndex(viewState);
  const selected = games[selectedIndex];
  const cards = games
    .map((game, index) => {
      const offset = relativeGameOffset(index, selectedIndex, games.length);
      if (Math.abs(offset) > 2) return "";
      return renderGameCard(game, offset, offset === 0);
    })
    .join("");
  const status = selected.launchable ? t("readyToLaunch") : t("launchUnavailable");
  const source = selected.source ? `${escapeHtml(selected.source)} - ` : "";
  const selectedLogo = gameLogoText(selected.name);
  const selectedLogoSrc = gameLogoSrc(selected);
  const titleLogo = selectedLogoSrc
    ? `<img class="game-title-watermark image" src="${escapeAttr(selectedLogoSrc)}" alt="" aria-hidden="true">`
    : `<span class="game-title-watermark" aria-hidden="true">${selectedLogo}</span>`;

  return [
    `<section class="game-carousel" tabindex="0" aria-label="${escapeAttr(t("gamesLibrary"))}">`,
    `<div class="game-carousel-stage">${cards}</div>`,
    `<div class="game-carousel-title" style="--game-accent:${gameAccent(selected)}">`,
    titleLogo,
    `<strong>${escapeHtml(selected.name)}</strong>`,
    `<span>${source}${escapeHtml(status)}</span>`,
    "</div>",
    "</section>"
  ].join("");
}

function colorSummary(color) {
  if (!color) return "";
  return [
    `${Math.round(Number(color.saturation ?? 100))}%`,
    `${Math.round(Number(color.contrast ?? 100))}%`,
    `${Math.round(Number(color.gamma ?? 100))}%`
  ].join(" / ");
}

function templateButton(id, label, meta, attrName, iconName = "chevronRight") {
  const metaHtml = meta ? `<small>${escapeHtml(meta)}</small>` : "";
  return `<button type="button" class="color-template-btn" ${attrName}="${escapeAttr(id)}">${icon(iconName)}<span>${escapeHtml(label)}</span>${metaHtml}</button>`;
}

function renderTemplates(viewState, t) {
  const presetButtons = [
    ["balanced", t("balanced")],
    ["vibrant", t("vibrant")],
    ["soft", t("soft")],
    ["night", t("night")]
  ]
    .map(([id, label]) => templateButton(id, label, t("builtInTemplate"), "data-preset"))
    .join("");

  const saved = (Array.isArray(viewState.configs) ? viewState.configs : [])
    .slice(0, 6)
    .map((config) => templateButton(config.id, config.name, colorSummary(config.color), "data-color-template-id", "save"))
    .join("");

  const savedTemplates = saved || `<div class="color-template-empty">${t("noTemplates")}</div>`;
  const saveName = escapeAttr(viewState.colorTemplateName || "");
  const saveTemplate = [
    '<div class="color-template-save">',
    `<input class="text-input wide" type="text" value="${saveName}" placeholder="${escapeAttr(t("templateName"))}" data-field="colorTemplateName">`,
    `<button class="btn btn-outline" type="button" data-action="save-color-template">${icon("plus")}<span>${t("saveTemplate")}</span></button>`,
    "</div>"
  ].join("");

  return [
    '<div class="color-template-section">',
    `<h3>${t("quickTemplates")}</h3>`,
    `<div class="color-template-grid">${presetButtons}</div>`,
    `<h3>${t("myTemplates")}</h3>`,
    `<div class="color-template-grid saved">${savedTemplates}</div>`,
    saveTemplate,
    "</div>"
  ].join("");
}

function renderColorSettingsPanel(appState, viewState, t) {
  const controls = ["saturation", "hue", "contrast", "gamma"].map((field) => slider(field, appState, t)).join("");
  const options =
    checkbox(t("applyInstantly"), appState.settings.applyInstantly, "applyInstantly") +
    checkbox(t("saveColorCorrection"), appState.settings.saveColorCorrection, "saveColorCorrection");
  const selected = selectedColorGame(viewState);
  const launch = selected?.launchable
    ? button(t("launchGame"), "play", "primary", "launch-color-game")
    : `<button class="btn btn-outline" type="button" disabled>${icon("play")}<span>${t("launchUnavailable")}</span></button>`;
  const headStyle = selected ? ` style="--game-accent:${gameAccent(selected)}"` : "";
  const headLogoSrc = gameLogoSrc(selected);
  const headMark = selected
    ? headLogoSrc
      ? `<img class="color-game-head-mark image" src="${escapeAttr(headLogoSrc)}" alt="" aria-hidden="true">`
      : `<span class="color-game-head-mark" aria-hidden="true">${gameLogoText(selected.name)}</span>`
    : "";
  const gameHeader = [
    `<div class="color-game-head"${headStyle}>`,
    headMark,
    '<div class="color-game-head-info">',
    `<span class="color-game-head-label">${t("selectedGame")}</span>`,
    `<strong class="color-game-head-title">${safeValue(selected?.name)}</strong>`,
    "</div>",
    `<div class="color-game-head-actions">${launch}</div>`,
    "</div>"
  ].join("");

  return [
    '<div class="color-game-panel">',
    '<div class="stack">',
    gameHeader,
    card(t("controls"), controls),
    card(t("statusOptions"), options),
    `<div class="actions">${button(t("apply"), "check", "primary", "apply-color")}${button(t("reset"), "rotate", "outline", "reset-color")}</div>`,
    "</div>",
    card(t("templates"), renderTemplates(viewState, t), "color-templates-card"),
    "</div>"
  ].join("");
}

export function renderColorPage(appState, viewState, t) {
  const controls = ["saturation", "hue", "contrast", "gamma"].map((field) => slider(field, appState, t)).join("");
  const options =
    checkbox(t("applyInstantly"), appState.settings.applyInstantly, "applyInstantly") +
    checkbox(t("saveColorCorrection"), appState.settings.saveColorCorrection, "saveColorCorrection");
  return [
    '<div class="global-color-page">',
    '<div class="global-color-grid">',
    '<div class="stack">',
    card(t("controls"), controls),
    card(t("statusOptions"), options),
    `<div class="actions">${button(t("apply"), "check", "primary", "apply-color")}${button(t("reset"), "rotate", "outline", "reset-color")}</div>`,
    "</div>",
    card(t("templates"), renderTemplates(viewState, t), "color-templates-card"),
    "</div>",
    "</div>"
  ].join("");
}

export function renderGameColorPage(appState, viewState, t) {
  const panelOpen = Boolean(viewState.colorGamePanelOpen && selectedColorGame(viewState));
  const panel = panelOpen ? `<aside class="color-game-drawer">${renderColorSettingsPanel(appState, viewState, t)}</aside>` : "";
  return `<div class="color-page ${panelOpen ? "panel-open" : "selection-only"}">${renderGameCarousel(viewState, t)}${panel}</div>`;
}

export function updateSliderDom(field, appState) {
  const row = document.querySelector(`[data-slider-row="${field}"]`);
  if (!row) return;
  const input = row.querySelector("[data-color-field]");
  const valueLabel = row.querySelector("[data-slider-value]");
  const track = row.querySelector(".slider-track");
  const value = Number(appState.color[field]);
  if (input) input.value = String(value);
  if (valueLabel) valueLabel.textContent = formatSliderValue(field, value);
  if (track) track.style.setProperty("--slider-percent", `${sliderPercent(field, value)}%`);
}

export function syncAllSliders(appState) {
  Object.keys(sliderDefs).forEach((field) => updateSliderDom(field, appState));
}
