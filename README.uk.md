<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Швидка та легка утиліта для насиченості кольорів монітора та оптимізації затримок Windows.</p>
<p align="center">📖 <b><a href="https://deepwiki.com/Arean-Max/synchro-nova">Повна інтерактивна документація на DeepWiki</a></b></p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
  <a href="https://deepwiki.com/Arean-Max/synchro-nova"><img alt="Docs" src="https://img.shields.io/badge/Docs-DeepWiki-blueviolet?style=flat-square" /></a>
  <a href="https://gitdiagram.com/arean-max/synchro-nova"><img alt="Architecture" src="https://img.shields.io/badge/Diagram-GitDiagram-orange?style=flat-square" /></a>
  <a href="https://github.com/Arean-Max/synchro-nova/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" /></a>
  <a href="https://microsoft.com/windows"><img alt="Platform" src="https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square" /></a>
  <a href="https://github.com/sotoykor-source/synchro-nova"><img alt="Fork of" src="https://img.shields.io/badge/Fork-sotoykor--source%2Fsynchro--nova-blue?style=flat-square&logo=github" /></a>
</p>

<p align="center">
  <a href="README.md">English</a> |
  <a href="README.zh.md">简体中文</a> |
  <a href="README.zht.md">繁體中文</a> |
  <a href="README.ko.md">한국어</a> |
  <a href="README.de.md">Deutsch</a> |
  <a href="README.es.md">Español</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.it.md">Italiano</a> |
  <a href="README.da.md">Dansk</a> |
  <a href="README.ja.md">日本語</a> |
  <a href="README.pl.md">Polski</a> |
  <a href="README.ru.md">Русский</a> |
  <a href="README.bs.md">Bosanski</a> |
  <a href="README.ar.md">العربية</a> |
  <a href="README.no.md">Norsk</a> |
  <a href="README.br.md">Português (Brasil)</a> |
  <a href="README.th.md">ไทย</a> |
  <a href="README.tr.md">Türkçe</a> |
  <a href="README.uk.md">Українська</a> |
  <a href="README.bn.md">বাংলা</a> |
  <a href="README.gr.md">Ελληνικά</a> |
  <a href="README.vi.md">Tiếng Việt</a>
</p>

[![Synchro Nova Preview](docs/screenshots/preview.png)](https://github.com/Arean-Max/synchro-nova/releases)

---

### Що вміє Synchro Nova?

Коротко: це зручна безкоштовна програма для геймерів, яка робить картинку в іграх насиченою та контрастною, щоб ворогів було чітко видно в темних місцях.

- **Автоматичне перемикання**: ви один раз налаштовуєте насиченість під гру. Запустили CS2, Valorant або Apex — насиченість увімкнулася. Згорнули гру або вийшли — кольори монітора відразу повертаються до стандартних.
- **Оптимізація відгуку (Latency)**: перевірені безпечні налаштування Windows (таймер переривань, вимкнення прискорення миші, мережевий стек) для максимальної плавності керування.
- **Не навантажує ПК**: важить менше 5 МБ, стартує за долі секунди та споживає всього ~2 МБ оперативної пам'яті в треї.

---

### Як завантажити та запустити (інструкція для новачків)

Якщо ви вперше на GitHub:

1. Перейдіть до розділу [**Releases (Релізи)**](https://github.com/Arean-Max/synchro-nova/releases) (кнопка праворуч на цій сторінці або за посиланням).
2. Завантажте файл:
   - **`synchro.exe`** — **портативна версія (рекомендовано)**. Завантажте в будь-яку папку та запустіть подвійним кліком. Встановлення не потрібне.
   - **`Synchro.Nova_*_x64-setup.exe`** — звичайний інсталятор, якщо потрібні ярлики на робочому столі та в меню «Пуск».
3. **Якщо Windows показує синє вікно («SmartScreen захистив ваш комп'ютер»)**:
   - Це стандартне сповіщення для безкоштовних програм з відкритим кодом без платного корпоративного сертифіката.
   - Натисніть **«Докладніше»** (More info), а потім кнопку **«Виконати в будь-якому разі»** (Run anyway).
   - Весь вихідний код відкритий прямо тут — жодної прихованої реклами, майнерів чи телеметрії.

---

### Швидкий старт за 1 хвилину

1. Запустіть **Synchro Nova**.
2. У вкладці **«Колір»** виберіть потрібну насиченість (наприклад, `150%` або `200%`).
3. Виберіть гру зі списку запущених (або вкажіть її `.exe` вручну).
4. Готово! Згорніть програму в трей і грайте.

---

### Часті питання (FAQ)

#### Чи є ризик отримати бан у CS2, Valorant, Rust або на Faceit?
**На 100% ні, бан виключено.**  
Synchro Nova **не втручається у файли ігор, не інжектує DLL, не читає пам'ять процесів і не малює оверлеї**. Зміна кольорів відбувається виключно через офіційний Windows API (DWM та драйвери відеокарти) — так само, як це роблять панелі NVIDIA Control Panel або AMD Software. Античити (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) не сприймають це як втручання в гру.

#### Навіщо ця програма, якщо є панель NVIDIA / AMD?
У панелях відеокарт немає зручного автоперемикання: якщо викрутити насиченість там, то після виходу з гри весь робочий стіл та браузер залишаться неприємно яскравими. Synchro перемикає кольори автоматично: у грі — соковито, на робочому столі — стандартно.

#### Чи безпечні твіки для Windows?
Так. Використовуються лише перевірені налаштування. Перед застосуванням будь-яких змін Synchro автоматично створює точку відновлення Windows і бекап реєстру. Будь-коли можна натиснути одну кнопку **«Відкотити зміни»** та повернути все назад.

---

### Документація та технічні деталі

Щоб не перевантажувати головну сторінку, всі технічні подробиці та архітектура винесені до окремої вікі:
- 📖 [Повна інтерактивна документація на DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Схема архітектури на GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Довідник твіків реєстру (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Збірка з вихідного коду (для розробників)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # запуск у режимі розробки
npm run build     # збірка релізного exe
```

---

### Автори

- [sotoykor-source](https://github.com/sotoykor-source) — Автор оригінального upstream-проекту
- [Arean-Max](https://github.com/Arean-Max) — Розробка та підтримка

---

### Ліцензія

Проект поширюється за ліцензією [MIT](LICENSE).
