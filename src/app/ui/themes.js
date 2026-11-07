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
    previewColors: ["#141414", "#1f1f1f", "#333333"]
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
    previewColors: ["#0b0f15", "#131923", "#38bdf8"]
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
    previewColors: ["#0e0d0c", "#1c1815", "#f59e0b"]
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
    previewColors: ["#0d0b10", "#1a1620", "#f472b6"]
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
    previewColors: ["#080e12", "#121c23", "#06b6d4"]
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
    this.lastFrameTime = 0;
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
    const rawW = Math.max(window.innerWidth || 0, (typeof document !== "undefined" ? document.documentElement?.clientWidth : 0) || 0, 980);
    const rawH = Math.max(window.innerHeight || 0, (typeof document !== "undefined" ? document.documentElement?.clientHeight : 0) || 0, 620);
    const dpr = Math.min((typeof window !== "undefined" && window.devicePixelRatio) || 1, 2);
    this.width = rawW;
    this.height = rawH;
    this.canvas.width = Math.floor(rawW * dpr);
    this.canvas.height = Math.floor(rawH * dpr);
    this.canvas.style.width = `${rawW}px`;
    this.canvas.style.height = `${rawH}px`;
    if (this.ctx) {
      this.ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }
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
    const count = this.currentTheme === "winter" ? 28 : (this.currentTheme === "autumn" ? 24 : 22);

    for (let i = 0; i < count; i++) {
      const p = {
        x: Math.random() * this.width,
        y: Math.random() * this.height,
        phase: Math.random() * Math.PI * 2,
        rot: Math.random() * Math.PI * 2,
        vRot: (Math.random() - 0.5) * 0.03,
        flipPhase: Math.random() * Math.PI * 2,
        flipSpeed: 0.015 + Math.random() * 0.02
      };

      if (this.currentTheme === "winter") {
        p.kind = i % 3;
        if (p.kind === 2) {
          p.size = 1.2 + Math.random() * 1.5;
          p.speedY = 0.2 + Math.random() * 0.35;
          p.speedX = 0.15;
          p.alpha = 0.4 + Math.random() * 0.5;
        } else if (p.kind === 1) {
          p.size = 3.6 + Math.random() * 2.4;
          p.speedY = 0.35 + Math.random() * 0.6;
          p.speedX = 0.25;
          p.alpha = 0.3 + Math.random() * 0.45;
        } else {
          p.size = 2.0 + Math.random() * 3.0;
          p.speedY = 0.3 + Math.random() * 0.55;
          p.speedX = 0.2;
          p.alpha = 0.2 + Math.random() * 0.4;
        }
      } else if (this.currentTheme === "autumn") {
        p.isEmber = i % 3 === 0;
        if (p.isEmber) {
          p.size = 1.5 + Math.random() * 1.8;
          p.speedY = -(0.3 + Math.random() * 0.5);
          p.speedX = (Math.random() - 0.5) * 0.5;
          p.alpha = 0.45 + Math.random() * 0.5;
          p.color = Math.random() > 0.5 ? "#fbbf24" : "#f59e0b";
        } else {
          p.size = 5.0 + Math.random() * 4.0;
          p.speedY = 0.5 + Math.random() * 0.8;
          p.speedX = 0.4 + Math.random() * 0.6;
          p.alpha = 0.35 + Math.random() * 0.45;
          const colors = ["#f59e0b", "#d97706", "#ea580c", "#b45309", "#dc2626"];
          p.color = colors[Math.floor(Math.random() * colors.length)];
        }
      } else if (this.currentTheme === "spring") {
        p.isDust = i % 4 === 0;
        if (p.isDust) {
          p.size = 1.2 + Math.random() * 1.6;
          p.speedX = 0.5 + Math.random() * 0.8;
          p.speedY = 0.2 + Math.random() * 0.4;
          p.alpha = 0.3 + Math.random() * 0.5;
          p.color = "#fbcfe8";
        } else {
          p.size = 4.5 + Math.random() * 3.5;
          p.speedX = 0.7 + Math.random() * 1.0;
          p.speedY = 0.3 + Math.random() * 0.55;
          p.alpha = 0.35 + Math.random() * 0.45;
          const colors = ["#f472b6", "#fbcfe8", "#f9a8d4", "#fdf2f8"];
          p.color = colors[Math.floor(Math.random() * colors.length)];
        }
      } else if (this.currentTheme === "summer") {
        p.isMote = i % 3 === 0;
        if (p.isMote) {
          p.size = 1.2 + Math.random() * 1.6;
          p.speedX = (Math.random() - 0.5) * 0.3;
          p.speedY = -(0.2 + Math.random() * 0.4);
          p.alpha = 0.35 + Math.random() * 0.45;
          p.color = "#fde047";
        } else {
          p.size = 2.4 + Math.random() * 2.2;
          p.speedX = (Math.random() - 0.5) * 0.4;
          p.speedY = -(0.25 + Math.random() * 0.45);
          p.alpha = 0.4 + Math.random() * 0.5;
          p.color = Math.random() > 0.5 ? "#06b6d4" : "#22d3ee";
        }
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

  loop(timestamp = 0) {
    if (!this.running || this.paused) return;
    if (typeof document !== "undefined" && document.hidden) {
      this.paused = true;
      return;
    }

    if (timestamp - this.lastFrameTime < 14) {
      this.animId = requestAnimationFrame((ts) => this.loop(ts));
      return;
    }
    this.lastFrameTime = timestamp;

    this.ctx.clearRect(0, 0, this.width, this.height);
    const theme = this.currentTheme;

    for (let i = 0; i < this.particles.length; i++) {
      const p = this.particles[i];

      if (theme === "winter") {
        p.y += p.speedY;
        p.x += Math.sin(p.phase) * 0.6;
        p.phase += 0.02;
        p.rot += p.vRot;

        if (p.y > this.height + 10) {
          p.y = -10;
          p.x = Math.random() * this.width;
        }
        if (p.x > this.width + 10) p.x = -10;
        if (p.x < -10) p.x = this.width + 10;

        if (p.kind === 2) {
          const pulse = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 2.5));
          this.ctx.save();
          this.ctx.translate(p.x, p.y);
          this.ctx.strokeStyle = `rgba(186, 230, 253, ${pulse})`;
          this.ctx.lineWidth = 0.9;
          this.ctx.beginPath();
          const len = p.size * 2.4;
          this.ctx.moveTo(-len, 0);
          this.ctx.lineTo(len, 0);
          this.ctx.moveTo(0, -len);
          this.ctx.lineTo(0, len);
          this.ctx.stroke();
          this.ctx.fillStyle = `rgba(255, 255, 255, ${pulse})`;
          this.ctx.beginPath();
          this.ctx.arc(0, 0, p.size * 0.5, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.restore();
        } else if (p.kind === 1) {
          this.ctx.save();
          this.ctx.translate(p.x, p.y);
          this.ctx.rotate(p.rot);
          this.ctx.strokeStyle = `rgba(224, 242, 254, ${p.alpha})`;
          this.ctx.lineWidth = 0.75;
          this.ctx.beginPath();
          for (let a = 0; a < 6; a++) {
            const ang = (a * Math.PI) / 3;
            const cos = Math.cos(ang);
            const sin = Math.sin(ang);
            this.ctx.moveTo(0, 0);
            this.ctx.lineTo(cos * p.size, sin * p.size);
            const spurDist = p.size * 0.6;
            const spurLen = p.size * 0.35;
            this.ctx.moveTo(cos * spurDist, sin * spurDist);
            this.ctx.lineTo(cos * spurDist - sin * spurLen, sin * spurDist + cos * spurLen);
            this.ctx.moveTo(cos * spurDist, sin * spurDist);
            this.ctx.lineTo(cos * spurDist + sin * spurLen, sin * spurDist - cos * spurLen);
          }
          this.ctx.stroke();
          this.ctx.restore();
        } else {
          const grad = this.ctx.createRadialGradient(p.x, p.y, 0, p.x, p.y, p.size);
          grad.addColorStop(0, `rgba(240, 249, 255, ${p.alpha})`);
          grad.addColorStop(1, `rgba(186, 230, 253, 0)`);
          this.ctx.fillStyle = grad;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
        }
      } else if (theme === "autumn") {
        p.phase += 0.025;
        p.flipPhase += p.flipSpeed;

        if (p.isEmber) {
          p.y += p.speedY;
          p.x += Math.sin(p.phase) * 0.4 + p.speedX;
          if (p.y < -10) {
            p.y = this.height + 10;
            p.x = Math.random() * this.width;
          }
          const emberAlpha = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 3));
          const glow = p.size * 2.2;
          const grad = this.ctx.createRadialGradient(p.x, p.y, 0, p.x, p.y, glow);
          grad.addColorStop(0, `rgba(254, 240, 138, ${emberAlpha})`);
          grad.addColorStop(0.4, p.color);
          grad.addColorStop(1, "rgba(245, 158, 11, 0)");
          this.ctx.fillStyle = grad;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, glow, 0, Math.PI * 2);
          this.ctx.fill();
        } else {
          p.y += p.speedY;
          p.x += Math.sin(p.phase) * (p.speedX * 1.8);
          p.rot += p.vRot;
          if (p.y > this.height + 14) {
            p.y = -14;
            p.x = Math.random() * this.width;
          }
          if (p.x > this.width + 14) p.x = -14;
          if (p.x < -14) p.x = this.width + 14;

          this.ctx.save();
          this.ctx.translate(p.x, p.y);
          this.ctx.rotate(p.rot);
          this.ctx.scale(Math.cos(p.flipPhase), 1);
          this.ctx.beginPath();
          this.ctx.moveTo(0, -p.size);
          this.ctx.bezierCurveTo(p.size * 0.65, -p.size * 0.4, p.size * 0.75, p.size * 0.3, 0, p.size);
          this.ctx.bezierCurveTo(-p.size * 0.75, p.size * 0.3, -p.size * 0.65, -p.size * 0.4, 0, -p.size);
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = p.alpha;
          this.ctx.fill();
          this.ctx.strokeStyle = "rgba(255, 255, 255, 0.25)";
          this.ctx.lineWidth = 0.6;
          this.ctx.beginPath();
          this.ctx.moveTo(0, -p.size * 0.75);
          this.ctx.lineTo(0, p.size * 0.85);
          this.ctx.stroke();
          this.ctx.restore();
        }
      } else if (theme === "spring") {
        p.x += p.speedX;
        p.y += Math.sin(p.phase) * 0.6 + p.speedY;
        p.phase += 0.03;
        p.rot += p.vRot;
        p.flipPhase += p.flipSpeed;

        if (p.x > this.width + 12 || p.y > this.height + 12) {
          p.x = Math.random() > 0.5 ? -12 : Math.random() * this.width;
          p.y = p.x === -12 ? Math.random() * this.height : -12;
        }

        if (p.isDust) {
          const pulse = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 2));
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = pulse;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.globalAlpha = 1.0;
        } else {
          this.ctx.save();
          this.ctx.translate(p.x, p.y);
          this.ctx.rotate(p.rot);
          this.ctx.scale(Math.cos(p.flipPhase), 1);
          this.ctx.beginPath();
          this.ctx.moveTo(0, p.size);
          this.ctx.bezierCurveTo(-p.size * 0.8, p.size * 0.4, -p.size * 0.7, -p.size * 0.6, -p.size * 0.35, -p.size);
          this.ctx.lineTo(0, -p.size * 0.65);
          this.ctx.lineTo(p.size * 0.35, -p.size);
          this.ctx.bezierCurveTo(p.size * 0.7, -p.size * 0.6, p.size * 0.8, p.size * 0.4, 0, p.size);
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = p.alpha;
          this.ctx.fill();
          this.ctx.restore();
        }
      } else if (theme === "summer") {
        p.y += p.speedY;
        p.x += Math.sin(p.phase) * 0.6 + p.speedX;
        p.phase += 0.025;

        if (p.y < -12) {
          p.y = this.height + 12;
          p.x = Math.random() * this.width;
        }
        if (p.x > this.width + 12) p.x = -12;
        if (p.x < -12) p.x = this.width + 12;

        if (p.isMote) {
          const pulse = p.alpha * (0.6 + 0.4 * Math.sin(p.phase * 3));
          this.ctx.fillStyle = p.color;
          this.ctx.globalAlpha = pulse;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.globalAlpha = 1.0;
        } else {
          const glow = p.size * (2.2 + 0.8 * Math.sin(p.phase * 2));
          const grad = this.ctx.createRadialGradient(p.x, p.y, 0, p.x, p.y, glow);
          grad.addColorStop(0, "rgba(254, 240, 138, 0.95)");
          grad.addColorStop(0.35, p.color);
          grad.addColorStop(1, "rgba(6, 182, 212, 0)");
          this.ctx.fillStyle = grad;
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, glow, 0, Math.PI * 2);
          this.ctx.fill();
          this.ctx.fillStyle = "#ffffff";
          this.ctx.beginPath();
          this.ctx.arc(p.x, p.y, p.size * 0.45, 0, Math.PI * 2);
          this.ctx.fill();
        }
      }
    }

    this.animId = requestAnimationFrame((ts) => this.loop(ts));
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
  if (themeDef.defaultAccent) {
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

  if (!lastAutoSeason && savedTheme) {
    try {
      if (typeof localStorage !== "undefined") {
        localStorage.setItem("synchro_last_auto_season", currentSeason);
      }
    } catch (_) {}
    return savedTheme;
  }

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
