const SPECIAL_KEY_MAP = {
  Space: "Space",
  Enter: "Enter",
  Tab: "Tab",
  Backspace: "Backspace",
  Delete: "Delete",
  Insert: "Insert",
  Home: "Home",
  End: "End",
  PageUp: "PageUp",
  PageDown: "PageDown",
  ArrowUp: "Up",
  ArrowDown: "Down",
  ArrowLeft: "Left",
  ArrowRight: "Right",
  Minus: "-",
  Equal: "=",
  BracketLeft: "[",
  BracketRight: "]",
  Backslash: "\\",
  Semicolon: ";",
  Quote: "'",
  Backquote: "`",
  Comma: ",",
  Period: ".",
  Slash: "/"
};

const VK_CODE_MAP = {
  Space: 32,
  Enter: 13,
  NumpadEnter: 13,
  Tab: 9,
  Escape: 27,
  Backspace: 8,
  Insert: 45,
  Delete: 46,
  Home: 36,
  End: 35,
  PageUp: 33,
  PageDown: 34,
  ArrowUp: 38,
  ArrowDown: 40,
  ArrowLeft: 37,
  ArrowRight: 39,
  NumpadMultiply: 106,
  NumpadAdd: 107,
  NumpadSubtract: 109,
  NumpadDecimal: 110,
  NumpadDivide: 111,
  Minus: 189,
  Equal: 187,
  BracketLeft: 219,
  BracketRight: 221,
  Backslash: 220,
  Semicolon: 186,
  Quote: 222,
  Backquote: 192,
  Comma: 188,
  Period: 190,
  Slash: 191
};

export function getWin32Vk(event) {
  if (typeof event.keyCode === "number" && event.keyCode > 0 && event.keyCode !== 229) {
    return event.keyCode;
  }
  const code = event.code || "";
  if (/^Key[A-Z]$/.test(code)) return code.charCodeAt(3);
  if (/^Digit[0-9]$/.test(code)) return code.charCodeAt(5);
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(code)) return 111 + parseInt(code.slice(1), 10);
  if (/^Numpad[0-9]$/.test(code)) return 96 + parseInt(code.slice(6), 10);
  return VK_CODE_MAP[code] || 0;
}

export function formatKeyLabel(event) {
  const parts = [];
  if (event.ctrlKey) parts.push("Ctrl");
  if (event.altKey) parts.push("Alt");
  if (event.shiftKey) parts.push("Shift");
  if (event.metaKey) parts.push("Win");

  let base = "";
  const code = event.code || "";
  if (code.startsWith("Key")) {
    base = code.slice(3).toUpperCase();
  } else if (code.startsWith("Digit")) {
    base = code.slice(5);
  } else if (code.startsWith("Numpad") && /^Numpad\d$/.test(code)) {
    base = `Num ${code.slice(6)}`;
  } else if (code === "NumpadEnter") {
    base = "Num Enter";
  } else if (code === "NumpadAdd") {
    base = "Num +";
  } else if (code === "NumpadSubtract") {
    base = "Num -";
  } else if (code === "NumpadMultiply") {
    base = "Num *";
  } else if (code === "NumpadDivide") {
    base = "Num /";
  } else if (code === "NumpadDecimal") {
    base = "Num .";
  } else if (/^F\d+$/.test(code)) {
    base = code;
  } else {
    base = SPECIAL_KEY_MAP[code] || event.key?.toUpperCase() || code;
  }

  parts.push(base);
  return parts.join(" + ");
}
