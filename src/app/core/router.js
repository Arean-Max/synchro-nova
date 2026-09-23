import { activePage, setActivePage, pageDefs } from "./state.js";

/**
 * Route registry and lifecycle controller.
 * Enforces clean mount/unmount semantics to prevent memory leaks,
 * detached DOM references, and lingering timers/event listeners.
 */
class Router {
  constructor() {
    this.routes = new Map();
    this.currentMounted = null;
    this.currentRouteName = activePage;
  }

  /**
   * Register a route handler with optional mount and unmount hooks.
   * @param {string} name Route name matching pageDefs key
   * @param {{ render?: Function, mount?: Function, unmount?: Function }} handler
   */
  register(name, handler) {
    this.routes.set(name, handler);
  }

  /**
   * Get the currently active page name.
   */
  getCurrentPage() {
    return activePage;
  }

  /**
   * Unmount the currently active route, releasing listeners and DOM references.
   */
  unmountCurrent() {
    if (this.currentMounted && typeof this.currentMounted.unmount === "function") {
      try {
        this.currentMounted.unmount();
      } catch (err) {
        console.error(`Router: error unmounting route ${this.currentRouteName}:`, err);
      }
      this.currentMounted = null;
    }
  }

  /**
   * Mount the target route into a DOM container.
   * @param {string} pageName Target page
   * @param {HTMLElement} container DOM element containing the route view
   * @param {object} context Global context / dispatch helpers
   */
  mount(pageName, container, context = {}) {
    const handler = this.routes.get(pageName);
    if (!handler) return;

    this.currentMounted = handler;
    this.currentRouteName = pageName;

    if (typeof handler.mount === "function" && container) {
      try {
        handler.mount(container, context);
      } catch (err) {
        console.error(`Router: error mounting route ${pageName}:`, err);
      }
    }
  }

  /**
   * Navigate to a new route:
   * 1. Unmount existing route.
   * 2. Update activePage state.
   * 3. Invoke optional onNavigate callback.
   * 4. Mount target route on newly rendered DOM.
   */
  navigate(pageName, context = {}) {
    if (!pageDefs[pageName] && pageName !== "gameColor") {
      pageName = "color";
    }

    if (this.currentRouteName === pageName && this.currentMounted) {
      return;
    }

    this.unmountCurrent();
    setActivePage(pageName);
    this.currentRouteName = pageName;

    if (typeof context.onNavigate === "function") {
      context.onNavigate(pageName);
    }

    const container = document.querySelector(`.page-${pageName}`) || document.querySelector(".page-body");
    this.mount(pageName, container, context);
  }
}

export const router = new Router();
