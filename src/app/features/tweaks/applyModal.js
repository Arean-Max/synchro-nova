import { escapeHtml, escapeAttr } from "../../core/html.js";
import { icon } from "../../ui/icons.js";
import { lang } from "../../core/state.js";
import { allTweaks, tweakTitle } from "./catalog.js";

export function renderApplyModal(modalState, t) {
  if (!modalState) return "";

  const phase = modalState.phase || "loading";
  const progress = Math.min(100, Math.max(0, Math.round(modalState.progress || 0)));
  const isComplete = phase === "complete";
  const isRollingBack = phase === "rollingBack";

  if (!isComplete) {
    const title = isRollingBack ? t("rollingBackTitle") : t("applyingTweaksTitle");
    const sub = isRollingBack ? t("rollingBackSubtitle") : t("applyingTweaksSubtitle");

    return [
      '<div class="apply-modal-backdrop" id="apply-modal-backdrop">',
      '  <div class="apply-modal-loading">',
      '    <div class="apply-spinner-wrapper">',
      '      <div class="apply-spinner-ring"></div>',
      '      <div class="apply-spinner-glow"></div>',
      `      <div class="apply-spinner-icon">${icon("zap")}</div>`,
      '    </div>',
      `    <h3 class="apply-modal-title">${escapeHtml(title)}</h3>`,
      `    <p class="apply-modal-sub">${escapeHtml(sub)}</p>`,
      '    <div class="apply-progress-track">',
      `      <div class="apply-progress-fill" style="width: ${progress}%;"></div>`,
      '    </div>',
      `    <div class="apply-progress-pct">${progress}%</div>`,
      '  </div>',
      '</div>'
    ].join("");
  }

  const isRu = lang() === "ru";
  const results = Array.isArray(modalState.results) ? modalState.results : [];
  const tweaksMap = new Map(allTweaks().map((tw) => [tw.id, tw]));

  const appliedCount = results.filter((r) => r.status === "applied").length;
  const adminCount = results.filter((r) => r.status === "requiresAdmin").length;

  const rows = results.map((item) => {
    const tweak = tweaksMap.get(item.id);
    const title = tweak ? tweakTitle(tweak, t) : (item.id || "-");

    let statusClass = "applied";
    let iconContent = icon("check");
    let statusTag = isRu ? "Применено" : "Applied";
    let desc = item.message || (isRu ? "Оптимизация активна" : "Tweak enabled");
    if (item.id === "clean-temp-junk" && item.message) {
      const match = item.message.match(/Cleaned\s+(\d+)\s+temporary files\s+\(([\d.]+)\s*MB/i);
      if (match && isRu) {
        desc = `Очищено ${match[1]} временных файлов (${match[2]} МБ мусора и кэша шейдеров DirectX)`;
      } else if (!match && !isRu && item.message.includes("Очищено")) {
        const ruMatch = item.message.match(/Очищено\s+(\d+)\s+временных файлов\s+\(([\d.]+)\s*МБ/i);
        if (ruMatch) {
          desc = `Cleaned ${ruMatch[1]} temporary files (${ruMatch[2]} MB junk and DirectX shader cache)`;
        }
      }
    }

    if (item.status === "reverted" || item.status === "restored") {
      statusClass = "reverted";
      iconContent = icon("rotate");
      statusTag = isRu ? "Откачен" : "Restored";
      desc = item.message || (isRu ? "Восстановлены значения Windows по умолчанию" : "Restored to Windows defaults");
    } else if (item.status === "requiresAdmin") {
      statusClass = "requires-admin";
      iconContent = icon("shield");
      statusTag = isRu ? "Нужен админ" : "Requires Admin";
      desc = isRu ? "Требуются права администратора" : "Requires administrator privileges";
    } else if (item.status === "skipped") {
      statusClass = "skipped";
      iconContent = icon("minus");
      statusTag = isRu ? "Уже активен" : "Already applied";
      desc = item.message || (isRu ? "Твик уже настроен в системе" : "Tweak is already active in system");
    } else if (item.status === "failed") {
      statusClass = "failed";
      iconContent = icon("x");
      statusTag = isRu ? "Ошибка" : "Failed";
      desc = item.message || (isRu ? "Не удалось применить твик" : "Failed to apply tweak");
    }

    const isFailed = item.status === "failed";
    const helpTooltip = isFailed
      ? (isRu ? "Почему этот твик не поставился?" : "Why did this tweak fail?")
      : (t("whatBreaks") || (isRu ? "Что меняет этот твик?" : "What does this tweak affect?"));
