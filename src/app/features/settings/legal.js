import { escapeHtml } from "../../core/html.js";

const documents = {
  en: {
    title: "Synchro Proprietary Subscription License and User Agreement",
    effective: "Effective date: July 9, 2026",
    owner: "Owner and licensor: Synchro Foundation",
    sections: [
      ["License grant", "Synchro Foundation grants the user a limited, revocable, non-exclusive, non-transferable monthly subscription license to install and use one authorized copy of Synchro for personal system tuning and diagnostics during the paid subscription period."],
      ["Restrictions", "The user must not publish, resell, rent, sublicense, redistribute, leak, mirror, share access to, decompile, disassemble, reverse engineer, bypass protection, tamper with integrity checks, sabotage internal operation, or assist third parties in doing so. Any attempt to bypass licensing, protection, payment, or security boundaries terminates the license immediately."],
      ["Termination and funds", "If the user violates this agreement, Synchro Foundation may revoke access to the application. In that case Synchro Foundation is not obligated to return subscription fees or other funds, except where mandatory consumer law requires otherwise."],
      ["Data protection", "Synchro is designed as a no-logs application. Synchro Foundation will not sell or disclose user data. The only allowed aggregate disclosure is the number of users who have enabled the optional daily ping feature. Sensitive data must be protected in confidence and encrypted where technically applicable."],
      ["Telemetry", "Daily ping and technical crash information are optional beta features. They remain disabled unless the user enables them. Local application operation must not depend on these beta features."],
      ["Security", "The user agrees not to attack, abuse, bypass, or degrade protection, anti-tamper, module integrity, backup, licensing, or safety mechanisms. Security research requires prior written permission from Synchro Foundation."],
      ["Updates", "Synchro Foundation may update this agreement for future versions or subscription periods. Continued use after an update means acceptance of the updated terms."],
      ["Disclaimer", "System tuning can affect Windows behavior. Synchro Foundation provides backups and safe guards, but the user remains responsible for reviewing changes and keeping independent recovery options."]
    ]
  },
  ru: {
    title: "Проприетарная подписочная лицензия и пользовательское соглашение Synchro",
    effective: "Дата вступления в силу: 9 июля 2026 года",
    owner: "Правообладатель и лицензиар: Synchro Foundation",
    sections: [
      ["Предоставление лицензии", "Synchro Foundation предоставляет пользователю ограниченную, отзывную, неисключительную, непередаваемую месячную подписочную лицензию на установку и использование одного авторизованного экземпляра Synchro для персональной настройки и диагностики системы в течение оплаченного периода подписки."],
      ["Ограничения", "Пользователь обязуется не публиковать, не продавать, не сдавать в аренду, не сублицензировать, не распространять, не сливать, не зеркалировать, не передавать доступ, не декомпилировать, не дизассемблировать, не выполнять обратную разработку, не обходить защиту, не вмешиваться в проверки целостности, не саботировать внутреннюю работу приложения и не помогать третьим лицам делать это. Любая попытка обхода лицензирования, защиты, оплаты или границ безопасности немедленно прекращает действие лицензии."],
      ["Прекращение доступа и средства", "При нарушении соглашения Synchro Foundation вправе лишить пользователя возможности использовать приложение. В таком случае Synchro Foundation не обязуется возвращать оплату подписки или иные средства, кроме случаев, прямо предусмотренных обязательным законодательством о защите потребителей."],
      ["Защита данных", "Synchro проектируется как no-logs приложение. Synchro Foundation не продает и не раскрывает пользовательские данные. Единственное допустимое агрегированное раскрытие — количество пользователей, у которых включена опциональная функция ежедневного пинга. Чувствительные данные должны храниться конфиденциально и шифроваться там, где это технически применимо."],
      ["Телеметрия", "Ежедневный пинг и отправка технической информации при сбоях являются опциональными beta-функциями. Они остаются выключенными, пока пользователь не включит их самостоятельно. Локальная работа приложения не должна зависеть от этих beta-функций."],
      ["Безопасность", "Пользователь обязуется не атаковать, не злоупотреблять, не обходить и не ухудшать работу механизмов защиты, анти-тампера, целостности модулей, бэкапов, лицензирования и безопасности. Исследование безопасности допускается только с предварительного письменного разрешения Synchro Foundation."],
      ["Обновления", "Synchro Foundation может обновлять настоящее соглашение для будущих версий или периодов подписки. Продолжение использования после обновления означает принятие обновленных условий."],
      ["Отказ от гарантий", "Системные твики могут влиять на поведение Windows. Synchro Foundation предоставляет бэкапы и защитные механизмы, но пользователь отвечает за оценку изменений и наличие независимых способов восстановления."]
    ]
  }
};

export function agreementHtml(language) {
  const document = documents[language === "ru" ? "ru" : "en"];
  const sections = document.sections
    .map(([title, body]) => `<h3>${escapeHtml(title)}</h3><p>${escapeHtml(body)}</p>`)
    .join("");
  return `<div class="agreement-document"><strong>${escapeHtml(document.title)}</strong><em>${escapeHtml(document.effective)}</em><em>${escapeHtml(document.owner)}</em>${sections}</div>`;
}
