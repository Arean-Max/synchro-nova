import { escapeAttr, escapeHtml } from "../../core/html.js";
import { lang } from "../../core/state.js";
import { button } from "../../ui/components.js";
import { icon } from "../../ui/icons.js";
import {
  allTweaks,
  tweakAppImpacts,
  tweakCategory,
  tweakCatalog,
  tweakDescription,
  tweakGroupTitle,
  tweakNote,
  tweakTitle
} from "./catalog.js";

function tweakTile(tweak, viewState, t) {
  const isRu = lang() === "ru";
  const installed = viewState.installedTweaks?.has(tweak.id);
  const category = tweakCategory(tweak);
  const title = tweakTitle(tweak, t);
  const desc = tweakDescription(tweak, t);
  const helpTitle = t("whatBreaks") || (isRu ? "Что меняет этот твик?" : "What does this tweak affect?");

  return `<div class="tweak-tile ${installed ? "installed" : "not-installed"}" role="button" tabindex="0" data-tweak-id="${escapeAttr(tweak.id)}" data-category="${category}"><div class="tweak-head"><span class="tweak-title">${escapeHtml(title)}</span><button class="tweak-help-btn" type="button" data-action="show-tweak-impact" data-tweak-id="${escapeAttr(tweak.id)}" title="${escapeAttr(helpTitle)}" aria-label="${escapeAttr(helpTitle)}">?</button></div><p>${escapeHtml(desc)}</p></div>`;
}

function tweakGroup(group, viewState, t) {
  const tweaks = group.tweaks;
  if (!tweaks.length) return "";
  const tiles = tweaks.map((tweak) => tweakTile(tweak, viewState, t)).join("");
  const title = tweakGroupTitle(group, t);
  return `<section class="card tweak-group"><h2 class="group-title">${icon(group.icon)}<span>${escapeHtml(title)}</span></h2><div class="tweak-grid">${tiles}</div></section>`;
}

function tweakResults(viewState, t) {
  const isRu = lang() === "ru";
  const results = Array.isArray(viewState.tweakResults) ? viewState.tweakResults : [];
  if (!results.length) return "";
  const rows = results
    .slice(0, 8)
    .map((item) => {
      const statusKey = `status${String(item.status || "").replace(/^./, (ch) => ch.toUpperCase())}`;
      let msg = item.message || "";
      if (item.id === "clean-temp-junk" && msg) {
        const match = msg.match(/Cleaned\s+(\d+)\s+temporary files\s+\(([\d.]+)\s*MB/i);
        if (match && isRu) {
          msg = `Очищено ${match[1]} временных файлов (${match[2]} МБ мусора и кэша шейдеров DirectX)`;
        } else if (!match && !isRu && msg.includes("Очищено")) {
          const ruMatch = msg.match(/Очищено\s+(\d+)\s+временных файлов\s+\(([\d.]+)\s*МБ/i);
          if (ruMatch) {
            msg = `Cleaned ${ruMatch[1]} temporary files (${ruMatch[2]} MB junk and DirectX shader cache)`;
          }
        }
      }
      return `<div class="tweak-result-row ${escapeAttr(item.status || "skipped")}"><span>${escapeHtml(item.id || "-")}</span><strong>${escapeHtml(t(statusKey))}</strong><em>${escapeHtml(msg)}</em></div>`;
    })
    .join("");
  return [
    '<section class="card tweak-results">',
    '  <div class="tweak-results-head">',
    `    <h2>${escapeHtml(t("appliedTweaks"))}</h2>`,
    `    <button class="dialog-close-btn" type="button" data-action="dismiss-tweak-results" title="${escapeAttr(t("close"))}">${icon("x")}</button>`,
    '  </div>',
    rows,
    '</section>'
  ].join("");
}

export function getTweakImpactDetails(tweak, t, isRu) {
  const category = tweakCategory(tweak);
  const impactInfo = tweakAppImpacts[tweak.id];
  if (impactInfo) {
    const appName = isRu
      ? (impactInfo.appNameRu || impactInfo.appName)
      : (impactInfo.appNameEn || impactInfo.appName);
    return {
      appName,
      impactText: isRu ? impactInfo.impactRu : impactInfo.impactEn
    };
  }

  const note = tweakNote(tweak, t);
  if (note && note !== t("noKnownConflict")) {
    let catLabel = isRu ? "Безопасный твик" : "Safe Tweak";
    if (category === "risk") catLabel = isRu ? "Рискованный твик" : "Risk Tweak";
    else if (category === "experimental") catLabel = isRu ? "Экспериментальный твик" : "Experimental Tweak";
    else if (category === "admin") catLabel = isRu ? "Системный твик" : "System Tweak";

    return {
      appName: catLabel,
      impactText: note
    };
  }

  if (category === "risk") {
    return {
      appName: isRu ? "Рискованный твик" : "Risk Tweak",
      impactText: isRu
        ? "Агрессивный твик: может приводить к повышенному нагреву, сбоям или нестабильности. Перед применением создайте бэкап."
        : "Aggressive tweak: may cause excessive heat, crashes, or instability. Creating a backup is recommended."
    };
  }

  if (category === "experimental") {
    return {
      appName: isRu ? "Экспериментальный твик" : "Experimental Tweak",
      impactText: isRu
        ? "Экспериментальная настройка: находится в стадии тестирования, её поведение зависит от конфигурации системы."
        : "Experimental setting: currently in testing; behavior depends on your system hardware configuration."
    };
  }

  if (category === "admin") {
    return {
      appName: isRu ? "Системный твик (Admin)" : "System Tweak (Admin)",
      impactText: isRu
        ? "Требует прав администратора и изменяет системные параметры Windows."
        : "Requires administrator privileges and modifies system-level Windows parameters."
    };
  }

  return {
    appName: isRu ? "Безопасный твик" : "Safe Tweak",
    impactText: isRu
      ? "У этого твика нет подтверждённых конфликтов с играми и приложениями."
      : "This tweak has no confirmed conflicts with games or apps."
  };
}

export function renderIosNotification(viewState, t) {
  const banner = viewState.activeImpactBanner;
  if (!banner) return "";

  const category = banner.category || "safe";
  const iconName = banner.isAdminPrompt ? "shield" : (category === "risk" || category === "experimental" ? "alert" : "info");
  const bannerAction = banner.isAdminPrompt
    ? "restart-as-admin"
    : (banner.isBackupPrompt ? "open-backup-name-modal" : "dismiss-impact-banner");

  return [
    '<div class="ios-banner-container">',
    `  <div class="ios-banner ${escapeAttr(category)}" data-action="${bannerAction}">`,
    `    <div class="ios-banner-icon ${escapeAttr(category)}">`,
    `      ${icon(iconName)}`,
    '    </div>',
    '    <div class="ios-banner-content">',
    '      <div class="ios-banner-header">',
    `        <span class="ios-banner-app">${escapeHtml(banner.appName)}</span>`,
    '      </div>',
    `      <div class="ios-banner-title">${escapeHtml(banner.title)}</div>`,
    `      <div class="ios-banner-message">${escapeHtml(banner.impactText)}</div>`,
    '    </div>',
    `    <button class="ios-banner-close" type="button" data-action="dismiss-impact-banner" title="${escapeAttr(t("close"))}">${icon("x")}</button>`,
    '  </div>',
    '</div>'
  ].join("");
}

export function renderTweaksPage(viewState, t) {
  const groups = tweakCatalog.map((group) => tweakGroup(group, viewState, t)).join("");
  const applyLabel = viewState.applyingTweaks ? t("loading") : t("applySelected");

  return [
    '<div class="tweaks-page">',
    `<div class="scroll-panel">${tweakResults(viewState, t)}${groups}</div>`,
    `<div class="actions tweaks-actions">${button(applyLabel, "check", "primary", "apply-tweaks")}${button(t("rollbackTweaks"), "rotate", "outline", "rollback-tweaks")}</div>`,
    '</div>'
  ].join("");
}

export const tweaksFeature = {
  render: renderTweaksPage,
  mount(container, context) {
    if (context?.loadTweakStatuses) {
      context.loadTweakStatuses();
    }
  },
  unmount() {
    // Clean up references
  }
};

