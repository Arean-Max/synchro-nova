import { activePage, setActivePage, pageDefs } from "./state.js";

class Router {
  constructor() {
    this.routes = new Map();
    this.currentMounted = null;
    this.currentRouteName = activePage;
  }

  register(name, handler) {
    this.routes.set(name, handler);
  }

  getCurrentPage() {
    return activePage;
  }

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
