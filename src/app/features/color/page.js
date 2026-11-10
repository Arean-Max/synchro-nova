/*
   ,--,                                            
,---.'|                                            
|   | :                                            
:   : |                        ,--,                
|   ' :                      ,--.'|                
;   ; '                 .---.|  |,      .--.--.    
'   | |__   ,---.     /.  ./|`--'_     /  /    '   
|   | :.'| /       .-' . ' |,' ,'|   |  :  /`./   
'   :    ;/    /  |/___/ \: |'  | |   |  :  ;_     
|   |  ./.    ' / |.   \  ' .|  | :    \  \    `.  
;   : ;  '   ;   /| \   \   ''  : |__   `----.   \ 
|   ,/   '   |  / |  \   \   |  | '.'| /  /`--'  / 
'---'    |   :    |   \   \ |;  :    ;'--'.     /  
          \   \  /     '---" |  ,   /   `--'---'   
           `----'             ---`-'               
                                                   

author - vkmef.t,me
*/
import { defaultPresets, filterPresets, lang, sliderDefs } from "../../core/state.js";
import { escapeAttr, escapeHtml, safeValue } from "../../core/html.js";
import { button, card } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";

export const standardTemplateNames = new Set([
  "Balanced", "Сбалансированный",
  "Vibrant", "Насыщенный", "Saturation", "Насыщенность",
  "Soft", "Мягкий",
  "Night", "Ночной",
  "Rust: Cold Tactical", "Rust: Midnight Neon", "Clear Sight"
]);

export function sliderPercent(field, value) {
  const def = sliderDefs[field];
  return ((value - def.min) / (def.max - def.min)) * 100;
}

export function sliderFillStyle(field, value) {
  const def = sliderDefs[field];
  const pct = Math.max(0, Math.min(100, sliderPercent(field, value)));
  if (def?.fromZero) {
    return `left:0%;width:${pct}%;`;
  }
  const centerPct = 50;
  if (pct >= centerPct) {
    return `left:${centerPct}%;width:${pct - centerPct}%;`;
  }
  return `left:${pct}%;width:${centerPct - pct}%;`;
}

export function formatSliderValue(field, value) {
  const def = sliderDefs[field];
  return `${Math.round(value)}${def.suffix}`;
}

function slider(field, appState, t) {
  const def = sliderDefs[field];
  const label = t ? t(field) : def.label;
  const value = Number(appState.color[field] ?? (def?.fromZero ? 0 : 100));
  const pct = Math.max(0, Math.min(100, sliderPercent(field, value)));
  const fillStyle = sliderFillStyle(field, value);
  const zeroMark = def?.zero ? '<span class="zero-mark"></span>' : '';
  const editTitle = (t && t("clickToEdit")) || (lang() === "ru" ? "Нажмите для ручного ввода" : "Click to edit value");
  return `<div class="slider-row" data-slider-row="${field}"><div class="slider-meta"><span>${escapeHtml(label)}</span><span class="slider-value-badge" data-slider-value="${field}" data-field="${field}" tabindex="0" role="button" title="${escapeAttr(editTitle)}">${formatSliderValue(field, value)}</span></div><div class="slider-track" style="--slider-percent:${pct}%">${zeroMark}<span class="slider-fill" style="${fillStyle}"></span><span class="slider-thumb" style="left:${pct}%"></span><input class="range-input" type="range" min="${def.min}" max="${def.max}" step="${def.step}" value="${escapeAttr(value)}" data-color-field="${field}" aria-label="${escapeAttr(label)}"></div></div>`;
}

export function isTemplateActive(id, appState) {
  const overrides = appState?.settings?.templateOverrides?.[id];
  const target = overrides || defaultPresets[id];
  if (!target || !appState?.color || appState.color.activeFilter) return false;
  return (
    Math.round(Number(appState.color.saturation ?? 100)) === Math.round(Number(target.saturation)) &&
    Math.round(Number(appState.color.hue ?? 0)) === Math.round(Number(target.hue)) &&
    Math.round(Number(appState.color.contrast ?? 100)) === Math.round(Number(target.contrast)) &&
    Math.round(Number(appState.color.gamma ?? 100)) === Math.round(Number(target.gamma))
  );
}

export function updateTemplateActiveDom(appState) {
  document.querySelectorAll(".color-template-item[data-preset-id]").forEach((el) => {
    const id = el.getAttribute("data-preset-id");
    const active = isTemplateActive(id, appState);
    el.classList.toggle("active", active);
  });
}

function templateItem(id, label, t, active = false) {
  const isRu = lang() === "ru";
  const gearTitle = t("configureTemplate") || (isRu ? "Настроить шаблон" : "Template Settings");
  return [
    `<div class="color-template-item ${active ? "active" : ""}" data-preset-id="${escapeAttr(id)}">`,
    `  <button type="button" class="color-template-btn" data-preset="${escapeAttr(id)}">`,
    '    <span class="color-template-indicator" aria-hidden="true"></span>',
    `    <span class="color-template-name">${escapeHtml(label)}</span>`,
    '  </button>',
    `  <button type="button" class="template-gear-btn" data-action="configure-template" data-preset="${escapeAttr(id)}" title="${escapeAttr(gearTitle)}" aria-label="${escapeAttr(gearTitle)}">`,
    `    ${icon("settings")}`,
    '  </button>',
    '</div>'
  ].join("");
}

export function renderGpuVendorIcon(vendor) {
  const v = (vendor || "unknown").toLowerCase();
  if (v === "nvidia") {
    return '<span class="holo-gpu-icon nvidia" title="NVIDIA GeForce"><svg width="24" height="19" viewBox="0 0 24 24" fill="currentColor"><path d="M8.948 8.798v-1.43a6.7 6.7 0 0 1 .424-.018c3.922-.124 6.493 3.374 6.493 3.374s-2.774 3.851-5.75 3.851c-.398 0-.787-.062-1.158-.185v-4.346c1.528.185 1.837.857 2.747 2.385l2.04-1.714s-1.492-1.952-4-1.952a6.016 6.016 0 0 0-.796.035m0-4.735v2.138l.424-.027c5.45-.185 9.01 4.47 9.01 4.47s-4.08 4.964-8.33 4.964c-.37 0-.733-.035-1.095-.097v1.325c.3.035.61.062.91.062 3.957 0 6.82-2.023 9.593-4.408.459.371 2.34 1.263 2.73 1.652-2.633 2.208-8.772 3.984-12.253 3.984-.335 0-.653-.018-.971-.053v1.864H24V4.063zm0 10.326v1.131c-3.657-.654-4.673-4.46-4.673-4.46s1.758-1.944 4.673-2.262v1.237H8.94c-1.528-.186-2.73 1.245-2.73 1.245s.68 2.412 2.739 3.11M2.456 10.9s2.164-3.197 6.5-3.533V6.201C4.153 6.59 0 10.653 0 10.653s2.35 6.802 8.948 7.42v-1.237c-4.84-.6-6.492-5.936-6.492-5.936z"/></svg></span>';
  }
  if (v === "amd") {
    return '<span class="holo-gpu-icon amd" title="AMD Radeon"><svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor"><path d="M22.002 2l-7.002 7.002h4.596l-7.596 7.596v-4.596l-7.002 7.002h17.004v-17.004zm-19.998 0v16.002l5.002-5.002v-6.004h6.004l5.002-4.996h-16.008z"/></svg></span>';
  }
  if (v === "intel") {
    return '<span class="holo-gpu-icon intel" title="Intel Graphics"><svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="9" fill="none" stroke="currentColor" stroke-width="2"/><path d="M9 8h2v8H9zm4 3h2v5h-2zm0-3h2v2h-2z"/></svg></span>';
  }
  return '<span class="holo-gpu-icon generic" title="Hardware GPU"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="6" width="18" height="12" rx="2"></rect><path d="M7 14h.01"></path><path d="M11 14h6"></path></svg></span>';
}

function renderTemplates(appState, viewState, t) {
  const defaultLabels = {
    balanced: t("balanced"),
    vibrant: t("vibrant"),
    soft: t("soft"),
    night: t("night")
  };
  const presetButtons = ["balanced", "vibrant", "soft", "night"]
    .map((id) => {
      let customName = appState?.settings?.templateOverrides?.[id]?.name;
      if (customName && standardTemplateNames.has(customName)) {
        customName = undefined;
      }
      const label = customName || defaultLabels[id] || id;
      const active = isTemplateActive(id, appState);
      return templateItem(id, label, t, active);
    })
    .join("");

  const isHoloActive = Number(appState?.color?.blackHolo || 0) > 0;
  const holoStatus = viewState?.blackHoloStatus || {};
  const vendor = (holoStatus.gpuVendor || "unknown").toLowerCase();

  return [
    '<div class="color-template-section">',
    `  <h3>${t("quickTemplates")}</h3>`,
    `  <div class="color-template-grid">${presetButtons}</div>`,
    '</div>',
    '<div class="black-holo-section">',
    `  <button type="button" class="check-row checkbox-control black-holo-check-row ${isHoloActive ? "active" : ""}" data-action="toggle-black-holo" role="switch" aria-checked="${isHoloActive}" aria-pressed="${isHoloActive}" title="${escapeAttr(t("blackHolo"))}">`,
    '    <div class="black-holo-left">',
    '      <svg class="black-holo-target-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><circle cx="12" cy="12" r="9"></circle><circle cx="12" cy="12" r="3"></circle><line x1="12" y1="1" x2="12" y2="5"></line><line x1="12" y1="19" x2="12" y2="23"></line><line x1="1" y1="12" x2="5" y2="12"></line><line x1="19" y1="12" x2="23" y2="12"></line></svg>',
    `      <strong>${escapeHtml(t("blackHolo"))}</strong>`,
    '    </div>',
    '    <div class="black-holo-right">',
    `      <span class="holo-gpu-icon-slot">${renderGpuVendorIcon(vendor)}</span>`,
    `      <span class="ios-switch ${isHoloActive ? "active" : ""}" aria-hidden="true">`,
    '        <span class="ios-switch-thumb"></span>',
    '      </span>',
    '    </div>',
    '  </button>',
    '</div>'
  ].join("");
}

function templateSlider(field, templateColor, t) {
  const def = sliderDefs[field];
  const label = t ? t(field) : def.label;
  const value = Number(templateColor?.[field] ?? 100);
  const pct = Math.max(0, Math.min(100, sliderPercent(field, value)));
  const fillStyle = sliderFillStyle(field, value);
  const editTitle = (t && t("clickToEdit")) || (lang() === "ru" ? "Нажмите для ручного ввода" : "Click to edit value");
  return `<div class="slider-row" data-template-slider-row="${field}"><div class="slider-meta"><span>${escapeHtml(label)}</span><span class="slider-value-badge" data-template-slider-value="${field}" tabindex="0" role="button" title="${escapeAttr(editTitle)}">${formatSliderValue(field, value)}</span></div><div class="slider-track" style="--slider-percent:${pct}%"><span class="zero-mark"></span><span class="slider-fill" style="${fillStyle}"></span><span class="slider-thumb" style="left:${pct}%"></span><input class="range-input" type="range" min="${def.min}" max="${def.max}" step="${def.step}" value="${escapeAttr(value)}" data-template-color-field="${field}" aria-label="${escapeAttr(label)}"></div></div>`;
}

export function updateTemplateSliderDom(field, values) {
  const row = document.querySelector(`[data-template-slider-row="${field}"]`);
  if (!row) return;
  const input = row.querySelector("[data-template-color-field]");
  const valueLabel = row.querySelector("[data-template-slider-value]");
  const track = row.querySelector(".slider-track");
  const fill = row.querySelector(".slider-fill");
  const thumb = row.querySelector(".slider-thumb");
  const value = Number(values?.[field] ?? 100);
  const pct = Math.max(0, Math.min(100, sliderPercent(field, value)));
  if (input) input.value = String(value);
  if (valueLabel) valueLabel.textContent = formatSliderValue(field, value);
  if (track) track.style.setProperty("--slider-percent", `${pct}%`);
  if (thumb) thumb.style.left = `${pct}%`;
  if (fill) fill.style.cssText = sliderFillStyle(field, value);
}

export function syncAllTemplateSliders(values) {
  if (!values) return;
  Object.keys(sliderDefs).forEach((field) => updateTemplateSliderDom(field, values));
}

export function renderTemplateConfigModal(appState, viewState, t) {
  const presetKey = viewState.editingTemplate;
  if (!presetKey) return "";

  const presetLabels = {
    balanced: t("balanced"),
    vibrant: t("vibrant"),
    soft: t("soft"),
    night: t("night"),
    rust_cold_tactical: t("rustColdTactical"),
    rust_midnight_neon: t("rustMidnightNeon"),
    clear_sight: t("clearSight")
  };
  const defaultName = presetLabels[presetKey] || presetKey;
  let currentName = viewState.editingTemplateName !== undefined && viewState.editingTemplateName !== ""
    ? viewState.editingTemplateName
    : (appState.settings?.templateOverrides?.[presetKey]?.name || defaultName);
  if (currentName && standardTemplateNames.has(currentName)) {
    currentName = defaultName;
  }
  const templateColor = viewState.editingTemplateColor || { saturation: 100, hue: 0, contrast: 100, gamma: 100 };
  const controls = ["saturation", "hue", "contrast", "gamma"].map((field) => templateSlider(field, templateColor, t)).join("");

  return `
    <div class="smart-tips-backdrop">
      <div class="backdrop-dismiss-area" data-action="close-template-config"></div>
      <div class="template-config-dialog" role="dialog" aria-modal="true" aria-labelledby="template-config-title">
        <div class="template-config-header">
          <div class="template-config-title">
            <h3 id="template-config-title">${escapeHtml(t("configureTemplate"))}</h3>
          </div>
          <button class="dialog-close-btn" type="button" data-action="close-template-config" title="${escapeAttr(t("close"))}">${icon("x")}</button>
        </div>
        <div class="template-config-body">
          <div class="template-name-group">
            <label class="template-name-label" for="template-name-input">${escapeHtml(t("templateName"))}</label>
            <input id="template-name-input" class="template-name-input" type="text" data-field="editingTemplateName" value="${escapeAttr(currentName)}" placeholder="${escapeAttr(defaultName)}" maxlength="28" spellcheck="false" autocomplete="off">
          </div>
          <div class="stack template-sliders-stack">${controls}</div>
        </div>
        <div class="template-config-footer">
          <button class="btn btn-outline template-reset-btn" type="button" data-action="reset-template-preset" data-preset="${escapeAttr(presetKey)}"><span>${escapeHtml(t("resetToDefault"))}</span></button>
          <button class="btn btn-primary template-done-btn" type="button" data-action="close-template-config"><span>${escapeHtml(t("done"))}</span></button>
        </div>
      </div>
    </div>
  `;
}

export function getPreviewFilterStyle(color) {
  let sat = Math.max(0, (color?.saturation ?? 100) / 100);
  let hue = color?.hue ?? 0;
  let con = Math.max(0, (color?.contrast ?? 100) / 100);
  let gam = Math.max(0, (color?.gamma ?? 100) / 100);

  if (color?.activeFilter === "rust_cold_tactical") {
    sat *= 0.78;
    con *= 1.16;
    gam *= 1.05;
    return `filter: saturate(${sat}) hue-rotate(${hue}deg) contrast(${con}) brightness(${gam}) sepia(0.06);`;
  }
  if (color?.activeFilter === "rust_midnight_neon") {
    sat *= 1.40;
    con *= 1.22;
    return `filter: saturate(${sat}) hue-rotate(${hue + 12}deg) contrast(${con}) brightness(${gam});`;
  }
  if (color?.activeFilter === "clear_sight") {
    sat *= 1.25;
    con *= 1.10;
    return `filter: saturate(${sat}) hue-rotate(${hue}deg) contrast(${con}) brightness(${gam});`;
  }

  return `filter: saturate(${sat}) hue-rotate(${hue}deg) contrast(${con}) brightness(${gam});`;
}

export function renderColorPreview(appState, viewState, t) {
  const mode = viewState?.colorPreviewMode || "day";
  let imgSrc = "./assets/preview.webp";
  if (mode === "night") imgSrc = "./assets/night.webp";
  else if (mode === "holo") imgSrc = "./assets/blackholo.webp";
  const filterStyle = getPreviewFilterStyle(appState.color);

  return [
    '<div class="color-preview-viewport">',
    `  <img id="color-preview-image" class="color-preview-img" src="${imgSrc}" alt="Preview" loading="lazy" decoding="async" style="${filterStyle}">`,
    '  <div class="color-preview-overlay">',
    '    <div class="color-preview-switch">',
    `      <button class="preview-mode-btn ${mode === "day" ? "active" : ""}" type="button" data-action="set-color-preview-mode" data-mode="day" title="${escapeAttr(t("previewDay"))}">`,
    '        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>',
    `        <span>${escapeHtml(t("day"))}</span>`,
    '      </button>',
    `      <button class="preview-mode-btn ${mode === "night" ? "active" : ""}" type="button" data-action="set-color-preview-mode" data-mode="night" title="${escapeAttr(t("previewNight"))}">`,
    '        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>',
    `        <span>${escapeHtml(t("night"))}</span>`,
    '      </button>',
    `      <button class="preview-mode-btn ${mode === "holo" ? "active" : ""}" type="button" data-action="set-color-preview-mode" data-mode="holo" title="${escapeAttr(t("previewHolo"))}">`,
    '        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="9"></circle><circle cx="12" cy="12" r="3"></circle><line x1="12" y1="1" x2="12" y2="5"></line><line x1="12" y1="19" x2="12" y2="23"></line><line x1="1" y1="12" x2="5" y2="12"></line><line x1="19" y1="12" x2="23" y2="12"></line></svg>',
    `        <span>${escapeHtml(t("holo"))}</span>`,
    '      </button>',
    '    </div>',
    '  </div>',
    '</div>'
  ].join("");
}

export function updateColorPreviewDom(color) {
  const img = document.getElementById("color-preview-image");
  if (!img) return;
  img.style.cssText = getPreviewFilterStyle(color);
}

export function renderFiltersSection(appState, t) {
  const activeFilter = appState.color?.activeFilter || "";
  const filters = [
    {
      id: "rust_cold_tactical",
      name: t("rustColdTactical"),
      icon: "crosshair",
      desc: t("rustColdTacticalDesc")
    },
    {
      id: "rust_midnight_neon",
      name: t("rustMidnightNeon"),
      icon: "moon",
      desc: t("rustMidnightNeonDesc")
    },
    {
      id: "clear_sight",
      name: t("clearSight"),
      icon: "eye",
      desc: t("clearSightDesc")
    }
  ];

  const tiles = filters.map((f) => {
    const isActive = activeFilter === f.id;
    return [
      `<div class="color-filter-tile ${isActive ? "active" : ""}" data-filter-id="${escapeAttr(f.id)}" data-action="toggle-color-filter" role="button" tabindex="0">`,
      '  <div class="filter-tile-left">',
      '    <div class="filter-tile-icon-box">',
      `      ${icon(f.icon)}`,
      '    </div>',
      `    <span class="filter-tile-title">${escapeHtml(f.name)}</span>`,
      '  </div>',
      '  <div class="filter-tile-right">',
      `    <button type="button" class="template-gear-btn filter-gear-btn" data-action="configure-filter" data-filter="${escapeAttr(f.id)}" title="${escapeAttr(t("settings"))}" aria-label="${escapeAttr(t("settings"))}">`,
      `      ${icon("settings")}`,
      '    </button>',
      `    <span class="filter-toggle-switch ios-switch ${isActive ? "active" : ""}" aria-hidden="true">`,
      '      <span class="ios-switch-thumb"></span>',
      '    </span>',
      '  </div>',
      '</div>'
    ].join("");
  }).join("");

  return `<div class="fine-tuning-filters-wrapper"><div class="fine-tuning-filters-grid">${tiles}</div></div>`;
}

export function renderColorPage(appState, viewState, t) {
  const isRu = lang() === "ru";
  const preview = renderColorPreview(appState, viewState, t);
  const controls = ["saturation", "hue", "contrast", "gamma"].map((field) => slider(field, appState, t)).join("");

  const currentTab = viewState?.fineTuningTab || "vibrance";
  const activeFilter = appState.color?.activeFilter || "";
  const filterDot = (activeFilter && currentTab !== "filters")
    ? '<span class="tab-indicator-dot" aria-hidden="true"></span>'
    : '';

  const switchHtml = [
    '<div class="color-preview-switch fine-tuning-switch">',
    `  <button class="fine-tuning-btn ${currentTab === "vibrance" ? "active" : ""}" type="button" data-action="set-fine-tuning-tab" data-tab="vibrance">`,
    '    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="4" y1="21" x2="4" y2="14"></line><line x1="4" y1="10" x2="4" y2="3"></line><line x1="12" y1="21" x2="12" y2="12"></line><line x1="12" y1="8" x2="12" y2="3"></line><line x1="20" y1="21" x2="20" y2="16"></line><line x1="20" y1="12" x2="20" y2="3"></line><line x1="1" y1="14" x2="7" y2="14"></line><line x1="9" y1="8" x2="15" y2="8"></line><line x1="17" y1="16" x2="23" y2="16"></line></svg>',
    `    <span>${escapeHtml(t("vibrance"))}</span>`,
    '  </button>',
    `  <button class="fine-tuning-btn ${currentTab === "filters" ? "active" : ""}" type="button" data-action="set-fine-tuning-tab" data-tab="filters">`,
    '    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polygon points="12 2 2 7 12 12 22 7 12 2"></polygon><polyline points="2 17 12 22 22 17"></polyline><polyline points="2 12 12 17 22 12"></polyline></svg>',
    `    <span>${escapeHtml(t("filters"))}</span>`,
    `    ${filterDot}`,
    '  </button>',
    '</div>'
  ].join("");

  const fineTuningTitle = [
    '<div class="card-head-with-switch">',
    `  <div class="card-title-with-icon">${icon("tune")}<span>${escapeHtml(t("fineTuning") || (isRu ? "Тонкая Настройка" : "Fine Tuning"))}</span></div>`,
    `  ${switchHtml}`,
    '</div>'
  ].join("");

  const templatesTitle = `<div class="card-title-with-icon">${icon("layers")}<span>${escapeHtml(t("templates") || (isRu ? "Шаблоны" : "Templates"))}</span></div>`;

  let hintHtml = "";
  if (currentTab === "vibrance" && activeFilter) {
    const filterNameMap = {
      rust_cold_tactical: t("rustColdTactical"),
      rust_midnight_neon: t("rustMidnightNeon"),
      clear_sight: t("clearSight")
    };
    const activeLabel = filterNameMap[activeFilter] || activeFilter;
    hintHtml = `
      <div class="filter-active-hint">
        <span class="filter-active-hint-dot"></span>
        <span class="filter-active-hint-text">${escapeHtml(t("activeFilter"))}: <strong>${escapeHtml(activeLabel)}</strong></span>
        <button class="filter-hint-disable-btn" type="button" data-action="disable-color-filter" title="${escapeAttr(t("clearFilter"))}" aria-label="${escapeAttr(t("clearFilter"))}">${icon("x")}</button>
      </div>
    `;
  }

  const fineTuningBody = currentTab === "vibrance"
    ? `<div class="fine-tuning-panel fine-tuning-vibrance">${controls}${hintHtml}</div>`
    : `<div class="fine-tuning-panel fine-tuning-filters">${renderFiltersSection(appState, t)}</div>`;

  return `
    <div class="global-color-page">
      ${preview}
      <div class="global-color-grid">
        ${card(fineTuningTitle, fineTuningBody, "color-controls-card")}
        ${card(templatesTitle, renderTemplates(appState, viewState, t), "color-templates-card")}
      </div>
      ${viewState.editingTemplate ? renderTemplateConfigModal(appState, viewState, t) : ""}
    </div>
  `;
}

export function updateSliderDom(field, appState) {
  const row = document.querySelector(`[data-slider-row="${field}"]`);
  if (!row) return;
  const input = row.querySelector("[data-color-field]");
  const valueLabel = row.querySelector("[data-slider-value]");
  const track = row.querySelector(".slider-track");
  const fill = row.querySelector(".slider-fill");
  const thumb = row.querySelector(".slider-thumb");
  const value = Number(appState.color[field]);
  const pct = Math.max(0, Math.min(100, sliderPercent(field, value)));
  if (input) input.value = String(value);
  if (valueLabel && !valueLabel.querySelector("input")) {
    valueLabel.textContent = formatSliderValue(field, value);
  }
  if (track) track.style.setProperty("--slider-percent", `${pct}%`);
  if (thumb) thumb.style.left = `${pct}%`;
  if (fill) fill.style.cssText = sliderFillStyle(field, value);
  updateColorPreviewDom(appState.color);
  updateTemplateActiveDom(appState);
}

export function updateBlackHoloDom(active, holoStatus) {
  const row = document.querySelector(".black-holo-check-row");
  if (!row) return;
  const isAct = Boolean(active);
  row.classList.toggle("active", isAct);
  row.setAttribute("aria-checked", String(isAct));
  row.setAttribute("aria-pressed", String(isAct));
  const sw = row.querySelector(".ios-switch");
  if (sw) {
    sw.classList.toggle("active", isAct);
  }
  if (holoStatus?.gpuVendor) {
    const iconSlot = row.querySelector(".holo-gpu-icon-slot");
    if (iconSlot) {
      iconSlot.innerHTML = renderGpuVendorIcon(holoStatus.gpuVendor);
    }
  }
}

export function syncAllSliders(appState, viewState) {
  Object.keys(sliderDefs).forEach((field) => updateSliderDom(field, appState));
  updateBlackHoloDom(Number(appState?.color?.blackHolo || 0) > 0, viewState?.blackHoloStatus);
  updateColorPreviewDom(appState?.color);
  updateTemplateActiveDom(appState);
}

export const colorFeature = {
  render: renderColorPage,
  mount(container, context) {
    if (context?.appState) {
      syncAllSliders(context.appState, context.viewState);
    }
  },
  unmount() {
    const img = document.getElementById("color-preview-image");
    if (img) {
      img.src = "";
    }
  }
};
