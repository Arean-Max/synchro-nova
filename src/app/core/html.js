const ESCAPE_MAP = {
  "&": "&amp;",
  "<": "&lt;",
  ">": "&gt;",
  '"': "&quot;",
  "'": "&#39;"
};

const ESCAPE_REGEX = /[&<>"']/g;

export function escapeHtml(value) {
  return String(value ?? "").replace(ESCAPE_REGEX, char => ESCAPE_MAP[char]);
}

export function escapeAttr(value) {
  return escapeHtml(value);
}

export function safeValue(value, fallback = "-") {
  if (value === undefined || value === null || value === "") return escapeHtml(fallback);
  return escapeHtml(value);
}
