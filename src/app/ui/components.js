import { escapeAttr, escapeHtml } from "../core/html.js";
import { icon } from "./icons.js";

export function button(label, iconName, variant, action) {
  const className = variant === "primary" ? "btn btn-primary" : "btn btn-outline";
  const data = action ? ` data-action="${action}"` : "";
  return `<button class="${className}" type="button"${data}>${iconName ? icon(iconName) : ""}<span>${label}</span></button>`;
}

export function checkbox(label, checked, key) {
  const pressed = checked ? "true" : "false";
  return `<div class="check-row checkbox-control ${checked ? "active" : ""}" data-setting="${escapeAttr(key)}" role="switch" aria-checked="${pressed}" tabindex="0"><span class="checkbox-label">${escapeHtml(label)}</span><div class="check-row-actions"><button class="toggle-help-btn" type="button" data-action="setting-help" data-setting-key="${escapeAttr(key)}" aria-label="${escapeAttr(label)}" title="Info">${icon("helpCircle")}</button><span class="ios-switch ${checked ? "active" : ""}" aria-hidden="true"><span class="ios-switch-thumb"></span></span></div></div>`;
}

export function card(title, body, extra) {
  return `<section class="card ${extra || ""}"><h2>${title}</h2>${body}</section>`;
}
