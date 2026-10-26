import { escapeHtml } from "./html.js";

export function safeRender(renderFn, fallbackTitle = "Component Error") {
  return function safelyRendered(...args) {
    try {
      return renderFn(...args);
    } catch (err) {
      console.error(`[ErrorBoundary] Render failure in ${fallbackTitle}:`, err);
      return `
        <div class="component-error-boundary" role="alert">
          <div class="error-boundary-card">
            <div class="error-boundary-header">
              <span class="error-boundary-badge">PROTECTED</span>
              <h3 class="error-boundary-title">${escapeHtml(fallbackTitle)}</h3>
            </div>
            <p class="error-boundary-message">${escapeHtml(String(err?.message || err))}</p>
            <div class="error-boundary-actions">
              <button class="btn btn-outline error-boundary-retry" type="button" data-action="route-navigate" data-page="color">
                Return to Dashboard
              </button>
            </div>
          </div>
        </div>
      `;
    }
  };
}

export function registerGlobalErrorBoundaries(notifyFn) {
  if (typeof window === "undefined") return;

  window.addEventListener("error", (event) => {
    console.error("[GlobalError]", event.error || event.message);
    if (typeof notifyFn === "function" && event.message) {
      notifyFn({
        type: "error",
        message: `Runtime notice: ${event.message.slice(0, 120)}`
      });
    }
  });

  window.addEventListener("unhandledrejection", (event) => {
    console.error("[UnhandledRejection]", event.reason);
    if (typeof notifyFn === "function" && event.reason) {
      const msg = event.reason?.message || String(event.reason);
      notifyFn({
        type: "error",
        message: `Async task notice: ${msg.slice(0, 120)}`
      });
    }
  });
}
