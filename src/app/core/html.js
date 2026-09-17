export function escapeHtml(value) {
  return String(value ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

export function escapeAttr(value) {
  return escapeHtml(value);
}

export function safeValue(value, fallback = "-") {
  if (value === undefined || value === null || value === "") return escapeHtml(fallback);
  return escapeHtml(value);
}
