import { escapeAttr } from "../core/html.js";
import { icon } from "./icons.js";

export function button(label, iconName, variant, action) {
  const className = variant === "primary" ? "btn btn-primary" : "btn btn-outline";
  const data = action ? ` data-action="${action}"` : "";
  return `<button class="${className}" type="button"${data}>${iconName ? icon(iconName) : ""}<span>${label}</span></button>`;
}

export function checkbox(label, checked, key) {
  const pressed = checked ? "true" : "false";
  const check = checked ? icon("check", "box-check") : "";
  return `<button class="check-row checkbox-control" type="button" data-setting="${escapeAttr(key)}" aria-pressed="${pressed}"><span class="box ${checked ? "checked" : ""}">${check}</span><span class="checkbox-label">${label}</span></button>`;
}

export function card(title, body, extra) {
  return `<section class="card ${extra || ""}"><h2>${title}</h2>${body}</section>`;
}
