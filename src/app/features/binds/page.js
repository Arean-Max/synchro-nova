import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { icon } from "../../ui/icons.js";

export function renderBindsPage(appState, viewState, t) {
  const isRu = lang() === "ru";
  const binds = Array.isArray(appState?.keybinds) ? appState.keybinds : [];

  const templateOptions = [
    { value: "template:balanced", label: t("action_template_balanced") },
    { value: "template:vibrant", label: t("action_template_vibrant") },
    { value: "template:soft", label: t("action_template_soft") },
    { value: "template:night", label: t("action_template_night") },
    { value: "template:rust_cold_tactical", label: t("action_template_rust_cold_tactical") || "Template: Rust Cold Tactical" },
    { value: "template:rust_midnight_neon", label: t("action_template_rust_midnight_neon") || "Template: Rust Midnight Neon" }
  ];

  if (appState?.settings?.templateOverrides) {
    for (const name of Object.keys(appState.settings.templateOverrides)) {
      if (!["balanced", "vibrant", "soft", "night"].includes(name.toLowerCase())) {
        templateOptions.push({
          value: `template:${name}`,
          label: `${isRu ? "Шаблон" : "Template"} «${name}»`
        });
      }
    }
  }

  const colorOptions = [
    { value: "toggle_saturation", label: t("action_toggle_saturation") },
    { value: "toggle_black_holo", label: t("action_toggle_black_holo") },
    { value: "reset_color", label: t("action_reset_color") }
  ];

  const systemOptions = [
    { value: "trim_memory", label: t("action_trim_memory") },
    { value: "toggle_window", label: t("action_toggle_window") }
  ];

  const allGroups = [
    { label: t("group_templates"), items: templateOptions },
    { label: t("group_color"), items: colorOptions },
    { label: t("group_system"), items: systemOptions }
  ];

  const findLabel = (val) => {
    for (const g of allGroups) {
      const found = g.items.find((i) => i.value === val);
      if (found) return found.label;
    }
    return val;
  };

  const renderDropdown = (currentValue, bindId) => {
    const isOpen = viewState?.openDropdownBindId === bindId;
    const currentLabel = findLabel(currentValue);

    let menuHtml = "";
    if (isOpen) {
      const groupsHtml = allGroups
        .map((g) => {
          const itemsHtml = g.items
            .map((it) => {
              const isSelected = it.value === currentValue;
              return [
                `<button class="bind-dropdown-item ${isSelected ? "selected" : ""}" type="button" data-action="select-bind-action" data-bind-id="${escapeAttr(bindId)}" data-value="${escapeAttr(it.value)}">`,
                `  <span>${escapeHtml(it.label)}</span>`,
                isSelected ? `  <span class="bind-item-check">${icon("check")}</span>` : "",
                `</button>`
              ].join("");
            })
            .join("");

          return [
            `<div class="bind-dropdown-group">`,
            `  <div class="bind-dropdown-group-title">${escapeHtml(g.label)}</div>`,
            itemsHtml,
            `</div>`
          ].join("");
        })
        .join("");

      menuHtml = `<div class="bind-dropdown-menu">${groupsHtml}</div>`;
    }

    return [
      `<div class="bind-custom-dropdown ${isOpen ? "open" : ""}" data-bind-id="${escapeAttr(bindId)}">`,
      `  <button class="bind-dropdown-trigger" type="button" data-action="toggle-bind-dropdown" data-bind-id="${escapeAttr(bindId)}">`,
      `    <span class="bind-dropdown-label">${escapeHtml(currentLabel)}</span>`,
      `    <span class="bind-dropdown-arrow">${icon("chevronDown")}</span>`,
      `  </button>`,
      menuHtml,
      `</div>`
    ].join("");
  };

  const rows = binds
    .map((bind) => {
      const isRecording = viewState?.recordingBindId === bind.id;
      let badgeContent = "";
      if (isRecording) {
        badgeContent = `<span class="recording-dot"></span><span>${escapeHtml(t("pressAnyKey"))}</span>`;
      } else if (bind.key) {
        badgeContent = `<span class="bind-key-text">${escapeHtml(bind.key)}</span>`;
      } else {
        badgeContent = `<span class="bind-key-placeholder">${escapeHtml(t("clickToRecord"))}</span>`;
      }

      const recClass = isRecording ? " recording" : "";
      const emptyClass = !bind.key && !isRecording ? " empty" : "";

      return [
        `<div class="bind-row ${isRecording ? "recording-row" : ""}" data-bind-id="${escapeAttr(bind.id)}">`,
        `  <div class="bind-key-col">`,
        `    <button class="bind-key-badge${recClass}${emptyClass}" type="button" data-action="record-bind" data-bind-id="${escapeAttr(bind.id)}" title="${escapeAttr(t("clickToRecord"))}">`,
        badgeContent,
        `    </button>`,
        `  </div>`,
        `  <div class="bind-arrow">${icon("arrowRight")}</div>`,
        `  <div class="bind-action-wrap">`,
        renderDropdown(bind.action, bind.id),
        `  </div>`,
        `  <div class="bind-controls">`,
        `    <button class="bind-toggle-btn ${bind.enabled ? "active" : ""}" type="button" data-action="toggle-bind" data-bind-id="${escapeAttr(bind.id)}" role="switch" aria-checked="${bind.enabled ? "true" : "false"}" title="${escapeAttr(t("bindActive"))}">`,
        `      <span class="ios-switch ${bind.enabled ? "active" : ""}"><span class="ios-switch-thumb"></span></span>`,
        `    </button>`,
        `    <button class="bind-delete-btn" type="button" data-action="delete-bind" data-bind-id="${escapeAttr(bind.id)}" title="${escapeAttr(t("deleteBind"))}">`,
        `      ${icon("trash")}`,
        `    </button>`,
        `  </div>`,
        `</div>`
      ].join("");
    })
    .join("");

  if (!binds.length) {
    return [
      '<section class="card binds-card empty-card">',
      '  <div class="binds-empty-state">',
      `    <div class="binds-empty-icon">${icon("keyboard")}</div>`,
      `    <h3 class="binds-empty-title">${escapeHtml(t("noBinds"))}</h3>`,
      `    <p class="binds-empty-desc">${escapeHtml(t("noBindsDesc"))}</p>`,
      `    <button class="btn btn-primary" type="button" data-action="add-bind">${icon("plus")}<span>${escapeHtml(t("createFirstBind"))}</span></button>`,
      '  </div>',
      '</section>'
    ].join("");
  }

  return [
    '<section class="card binds-card">',
    `  <div class="binds-list">${rows}</div>`,
    '  <div class="binds-footer-row">',
    `    <button class="btn btn-primary bind-add-btn" type="button" data-action="add-bind" title="${escapeAttr(t("addBind"))}">`,
    `      ${icon("plus")}`,
    `      <span>${escapeHtml(t("addBind"))}</span>`,
    '    </button>',
    '  </div>',
    '</section>'
  ].join("");
}

export const bindsFeature = {
  render: (viewState, t) => renderBindsPage(appState, viewState, t),
  mount() {},
  unmount() {}
};
