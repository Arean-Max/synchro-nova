/*
   ,--,                                            
,---.'|                                            
|   | :                                            
:   : |                        ,--,                
|   ' :                      ,--.'|                
;   ; '                 .---.|  |,      .--.--.    
'   | |__   ,---.     /.  ./|`--'_     /  /    '   
|   | :.'| /       .-' . ' |,' ,'|   |  :  /`./   
'   :    ;/    /  |/___/ \: |'  | |   |  :  ;_     
|   |  ./.    ' / |.   \  ' .|  | :    \  \    `.  
;   : ;  '   ;   /| \   \   ''  : |__   `----.   \ 
|   ,/   '   |  / |  \   \   |  | '.'| /  /`--'  / 
'---'    |   :    |   \   \ |;  :    ;'--'.     /  
          \   \  /     '---" |  ,   /   `--'---'   
           `----'             ---`-'               
                                                   

author - vkmef.t,me
*/
import { activePage, setActivePage, pageDefs } from "./state.js";

class Router {
  constructor() {
    this.routes = new Map();
    this.pageCache = new Map();
    this.currentMounted = null;
    this.currentRouteName = activePage;
  }

  cachePageHtml(name, html) {
    this.pageCache.set(name, html);
  }

  getCachedPageHtml(name) {
    return this.pageCache.get(name);
  }

  clearPageCache(name) {
    if (name) {
      this.pageCache.delete(name);
    } else {
      this.pageCache.clear();
    }
  }

  clearCache() {
    this.pageCache.clear();
  }

  register(name, handler) {
    this.routes.set(name, handler);
  }

  getCurrentPage() {
    return activePage;
  }

  activePageName() {
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
    if (!pageDefs[pageName]) {
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
    } else {
      const container = document.querySelector(`.page-${pageName}`) || document.querySelector(".page-body");
      this.mount(pageName, container, context);
    }
  }
}

export const router = new Router();
