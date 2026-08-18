export function escapeHtml(value) {
  return String(value ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

export function escapeAttr(value) {
  return escapeHtml(value).replace(/"/g, "&quot;");
}

export function safeValue(value, fallback = "-") {
  if (value === undefined || value === null || value === "") return escapeHtml(fallback);
  return escapeHtml(value);
}
