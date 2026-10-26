import { applyInterfaceAccent } from "../core/state.js";

export const THEMES = [
  {
    id: "default",
    nameRu: "Дефолт",
    nameEn: "Default",
    descRu: "Строгий минималистичный тёмный стиль",
    descEn: "Clean minimalist monochrome dark style",
    badge: "STANDARD",
    icon: "layers",
    defaultAccent: "#ffffff",
    previewColors: ["#141414", "#1f1f1f", "#ffffff"]
  },
  {
    id: "winter",
    nameRu: "Новогодняя",
    nameEn: "New Year",
    descRu: "Морозная зимняя ночь и падающий снег",
    descEn: "Frosty winter midnight with drifting snow",
    badge: "FESTIVE",
    icon: "snowflake",
    defaultAccent: "#38bdf8",
    previewColors: ["#0c131f", "#131d2e", "#38bdf8"]
  },
  {
    id: "autumn",
    nameRu: "Осенняя",
    nameEn: "Autumn",
    descRu: "Тёплый янтарь и золотой листопад",
    descEn: "Warm amber glow with golden falling leaves",
    badge: "SEASONAL",
    icon: "leaf",
    defaultAccent: "#f59e0b",
    previewColors: ["#150f0c", "#201612", "#f59e0b"]
  },
  {
    id: "spring",
    nameRu: "Весенняя",
    nameEn: "Spring",
    descRu: "Цветущая сакура и нежный весенний бриз",
    descEn: "Blooming sakura with drifting petals",
    badge: "BLOOM",
    icon: "flower",
    defaultAccent: "#f472b6",
    previewColors: ["#130f18", "#1d1524", "#f472b6"]
  },
  {
    id: "summer",
    nameRu: "Летняя",
    nameEn: "Summer",
    descRu: "Солнечный лазурный бриз и теплые светлячки",
    descEn: "Solar azure breeze with glowing fireflies",
    badge: "SOLAR",
    icon: "sun",
    defaultAccent: "#06b6d4",
    previewColors: ["#08171e", "#0e232e", "#06b6d4"]
  }
];

class ParticleSystem {
  constructor() {
    this.canvas = null;
    this.ctx = null;
    this.animId = null;
    this.currentTheme = "default";
    this.particles = [];
    this.width = 0;
    this.height = 0;
    this.running = false;
    this.paused = false;
  }

  init(container = (typeof document !== "undefined" ? document.body : null)) {
    if (typeof document === "undefined" || typeof document.createElement !== "function" || !container) return;
    if (this.canvas) return;

    this.canvas = document.createElement("canvas");
    this.canvas.id = "theme-fx-canvas";
    this.canvas.className = "theme-fx-canvas";
    this.canvas.setAttribute("aria-hidden", "true");
    container.appendChild(this.canvas);

    this.ctx = this.canvas.getContext("2d", { alpha: true });
    this.resize();

    window.addEventListener("resize", () => this.resize(), { passive: true });
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) {
        this.pause();
      } else {
        this.resume();
      }
    });
  }

  resize() {
    if (!this.canvas) return;
    this.width = Math.max(window.innerWidth || 0, (typeof document !== "undefined" ? document.documentElement?.clientWidth : 0) || 0, 980);
    this.height = Math.max(window.innerHeight || 0, (typeof document !== "undefined" ? document.documentElement?.clientHeight : 0) || 0, 620);
    this.canvas.width = this.width;
    this.canvas.height = this.height;
  }

  setTheme(themeId, disableAnimations = false) {
    this.currentTheme = themeId;
    if (!this.canvas) this.init();

    if (themeId === "default" || disableAnimations) {
      this.stop();
      return;
    }

    this.stop();
    this.spawnParticles();
    this.start();
  }

  spawnParticles() {
    this.particles = [];
    const count = this.currentTheme === "winter" ? 32 : (this.currentTheme === "autumn" ? 26 : 24);

    for (let i = 0; i < count; i++) {
      const p = {
        x: Math.random() * this.width,
        y: Math.random() * this.height,
        phase: Math.random() * Math.PI * 2,
        rot: Math.random() * Math.PI * 2,
        vRot: (Math.random() - 0.5) * 0.04
      };

      if (this.currentTheme === "winter") {
        p.isSparkle = i < 8;
        p.size = p.isSparkle ? 1.0 + Math.random() * 1.2 : 1.2 + Math.random() * 1.8;
        p.speedY = p.isSparkle ? 0.2 + Math.random() * 0.3 : 0.4 + Math.random() * 0.8;
        p.speedX = 0.2;
        p.alpha = 0.25 + Math.random() * 0.55;
      } else if (this.currentTheme === "autumn") {
        p.isEmber = i < 6;
        if (p.isEmber) {
          p.size = 1.4 + Math.random() * 1.6;
          p.speedY = -(0.3 + Math.random() * 0.5);
          p.speedX = (Math.random() - 0.5) * 0.6;
          p.alpha = 0.4 + Math.random() * 0.5;
          p.color = Math.random() > 0.5 ? "#fbbf24" : "#f59e0b";
        } else {
          p.size = 3.5 + Math.random() * 3.5;
          p.speedY = 0.5 + Math.random() * 0.9;
          p.speedX = 0.4 + Math.random() * 0.6;
          p.alpha = 0.3 + Math.random() * 0.5;
          p.color = Math.random() > 0.4 ? "#f59e0b" : (Math.random() > 0.5 ? "#ea580c" : "#b45309");
        }
      } else if (this.currentTheme === "spring") {
        p.size = 3.2 + Math.random() * 3.2;
        p.speedX = 0.7 + Math.random() * 1.1;
        p.speedY = 0.3 + Math.random() * 0.6;
        p.alpha = 0.35 + Math.random() * 0.5;
        p.color = Math.random() > 0.3 ? "#f472b6" : (Math.random() > 0.5 ? "#fbcfe8" : "#f9a8d4");
      } else if (this.currentTheme === "summer") {
        p.size = 1.6 + Math.random() * 2.2;
        p.speedX = (Math.random() - 0.5) * 0.4;
        p.speedY = -(0.3 + Math.random() * 0.5);
        p.alpha = 0.35 + Math.random() * 0.55;
        p.color = Math.random() > 0.4 ? "#06b6d4" : (Math.random() > 0.5 ? "#38bdf8" : "#fde047");
      }

      this.particles.push(p);
    }
  }

  start() {
    if (this.running || !this.ctx) return;
    this.running = true;
    this.paused = false;
    this.loop();
  }

  stop() {
    this.running = false;
    if (this.animId) {
      cancelAnimationFrame(this.animId);
      this.animId = null;
    }
    this.particles = [];
    if (this.ctx && this.canvas) {
      this.ctx.clearRect(0, 0, this.width || this.canvas.width || 2000, this.height || this.canvas.height || 2000);
    }
  }

  pause() {
    this.paused = true;
  }

  resume() {
    if (this.running && this.paused) {
      this.paused = false;
      this.loop();
    }
  }

  loop() {
    if (!this.running || this.paused) return;

    this.ctx.clearRect(0, 0, this.width, this.height);
    const theme = this.currentTheme;

    for (let i = 0; i < this.particles.length; i++) {
      const p = this.particles[i];

      if (theme === "winter") {
        p.y += p.speedY;
        p.x += Math.sin(p.phase) * 0.5;
        p.phase += 0.02;

        if (p.y > this.height + 4) {
          p.y = -4;
          p.x = Math.random() * this.width;
        }
        if (p.x > this.width + 4) p.x = -4;
        if (p.x < -4) p.x = this.width + 4;

        if (p.isSparkle) {
          const sparkleAlpha = p.alpha * (0.5 + 0.5 * Math.sin(p.phase * 2));
          this.ctx.fillStyle = `rgba(186, 230, 253, ${sparkleAlpha})`;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
        } else {
          this.ctx.fillStyle = `rgba(224, 242, 254, ${p.alpha})`;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
        }
      } else if (theme === "autumn") {
        p.y += p.speedY;
        p.x += Math.sin(p.phase) * (p.isEmber ? 0.3 : p.speedX);
        p.phase += 0.025;

        if (p.isEmber) {
          if (p.y < -10) {
            p.y = this.height + 10;
            p.x = Math.random() * this.width;
          }
          const emberAlpha = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 3));
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = emberAlpha;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.globalAlpha = 1.0;
        } else {
          p.rot += p.vRot;
          if (p.y > this.height + 10) {
            p.y = -10;
            p.x = Math.random() * this.width;
          }
          if (p.x > this.width + 10) p.x = -10;
          if (p.x < -10) p.x = this.width + 10;

          this.ctx.save();
          this.ctx.translate(p.x, p.y);
          this.ctx.rotate(p.rot);
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = p.alpha;
          this.ctx.beginPath();
          this.ctx.ellipse(0, 0, p.size, p.size * 0.55, 0, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.restore();
        }
      } else if (theme === "spring") {
        p.x += p.speedX;
        p.y += Math.sin(p.phase) * 0.5 + p.speedY;
        p.phase += 0.03;
        p.rot += p.vRot;

        if (p.x > this.width + 10 || p.y > this.height + 10) {
          p.x = Math.random() > 0.5 ? -10 : Math.random() * this.width;
          p.y = p.x === -10 ? Math.random() * this.height : -10;
        }

        this.ctx.save();
        this.ctx.translate(p.x, p.y);
        this.ctx.rotate(p.rot);
        this.ctx.fillStyle = p.color;
        this.ctx.globalAlpha = p.alpha;
        this.ctx.beginPath();
        this.ctx.ellipse(0, 0, p.size, p.size * 0.6, Math.PI / 4, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.restore();
      } else if (theme === "summer") {
        p.y += p.speedY;
        p.x += Math.sin(p.phase) * 0.5 + p.speedX;
        p.phase += 0.025;

        if (p.y < -10) {
          p.y = this.height + 10;
          p.x = Math.random() * this.width;
        }
        if (p.x > this.width + 10) p.x = -10;
        if (p.x < -10) p.x = this.width + 10;

        const pulseAlpha = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 2));
        this.ctx.fillStyle = p.color;
        this.ctx.globalAlpha = pulseAlpha;
        this.ctx.beginPath();
        this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.globalAlpha = 1.0;
      }
    }

    this.animId = requestAnimationFrame(() => this.loop());
  }
}

export const themeFx = new ParticleSystem();

export function applyTheme(themeId, options = {}) {
  const validThemes = ["default", "winter", "autumn", "spring", "summer"];
  const selectedTheme = validThemes.includes(themeId) ? themeId : "default";

  try {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("synchro_theme", selectedTheme);
    }
  } catch (_) {}

  if (typeof document !== "undefined" && document.documentElement) {
    document.documentElement.setAttribute("data-theme", selectedTheme);
  }

  const themeDef = THEMES.find((t) => t.id === selectedTheme) || THEMES[0];
  if (options.updateAccent && themeDef.defaultAccent) {
    applyInterfaceAccent(themeDef.defaultAccent);
  }

  themeFx.setTheme(selectedTheme, options.disableAnimations);
  return selectedTheme;
}

export function getCurrentSeason(date = new Date()) {
  const month = date.getMonth();
  if (month === 11 || month === 0 || month === 1) return "winter";
  if (month >= 5 && month <= 7) return "summer";
  if (month >= 8 && month <= 10) return "autumn";
  return "spring";
}

export function resolveSeasonalTheme() {
  const currentSeason = getCurrentSeason();
  let lastAutoSeason = null;
  let savedTheme = null;
  try {
    if (typeof localStorage !== "undefined") {
      lastAutoSeason = localStorage.getItem("synchro_last_auto_season");
      savedTheme = localStorage.getItem("synchro_theme");
    }
  } catch (_) {}

  if (!lastAutoSeason || lastAutoSeason !== currentSeason) {
    try {
      if (typeof localStorage !== "undefined") {
        localStorage.setItem("synchro_last_auto_season", currentSeason);
        localStorage.setItem("synchro_theme", currentSeason);
      }
    } catch (_) {}
    return currentSeason;
  }

  return savedTheme || currentSeason;
}
