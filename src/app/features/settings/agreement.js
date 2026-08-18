import { appState } from "../../core/state.js";
import { agreementHtml } from "./legal.js";

export function renderAgreementGate(t) {
  const language = appState.settings.language;
  return [
    '<div class="agreement-overlay" role="dialog" aria-modal="true">',
    '<section class="agreement-modal">',
    '<div class="agreement-modal-head">',
    '<div>',
    `<h2>${t("agreement")}</h2>`,
    `<p>${t("agreementIntro")}</p>`,
    "</div>",
    '<div class="agreement-language">',
    `<button class="${language === "en" ? "active" : ""}" type="button" data-language="en">EN</button>`,
    `<button class="${language === "ru" ? "active" : ""}" type="button" data-language="ru">RU</button>`,
    "</div>",
    "</div>",
    agreementHtml(language),
    '<div class="agreement-modal-actions">',
    `<button class="btn btn-outline" type="button" data-action="decline-agreement"><span>${t("declineAndExit")}</span></button>`,
    `<button class="btn btn-primary" type="button" data-action="accept-agreement"><span>${t("acceptAndContinue")}</span></button>`,
    "</div>",
    "</section>",
    "</div>"
  ].join("");
}
