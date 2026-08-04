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
