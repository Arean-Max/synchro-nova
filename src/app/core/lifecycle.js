import { invokeCommand } from "./bridge.js";

let trimTimer = null;

export function scheduleTrimMemory(delayMs = 200) {
  if (typeof window === "undefined") return;
  window.clearTimeout(trimTimer);
  trimTimer = window.setTimeout(() => {
    invokeCommand("trim_memory").catch(() => {});
  }, delayMs);
}

export function registerLifecycleHooks() {
  if (typeof window === "undefined") return;

  // Do not perform aggressive working set trimming on window blur,
  // as it causes page-fault stutter in active foreground games.
}

export function withErrorBoundary(fn, contextName = "Action") {
  return async function (...args) {
    try {
      return await fn(...args);
    } catch (err) {
      const errorMsg = err?.message || String(err);
      console.error(`[Synchro:${contextName}] Caught handled error:`, errorMsg);
      if (typeof window !== "undefined" && typeof window.__synchroShowErrorToast === "function") {
        window.__synchroShowErrorToast(contextName, errorMsg);
      }
    }
  };
}
