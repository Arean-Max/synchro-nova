<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Легка утиліта для робочого столу для оптимізації яскравості монітора та затримки Windows.</p>
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

### Чому я побудував це

Кожного разу, коли я запускав CS2 або Apex, мені доводилося відкривати панель керування NVIDIA, запускати Digital Vibrance, щоб виявляти ворогів у темних кутах, а потім вручну вимикати його після закриття гри, щоб мій робочий стіл і браузер не спалили мою сітківку ока.

Synchro вирішує це раз і назавжди:
- **Автоматичне перемикання кольорів**: Запустіть гру, кольори починають працювати. Alt-Tab або закрийте гру, кольори вашого робочого столу негайно повертаються до нормальних.
- **Очищення затримки**: Вимикає прискорення миші Windows, оптимізує роздільну здатність таймера та обрізає буферизацію мережевих пакетів, щоб введення було миттєвим.
- **Нульове здуття**: Завантажується менше 5 МБ, починається за мілісекунди та тихо лежить у треї, використовуючи ~2 МБ оперативної пам’яті.

> [!TIP]
> **Перевірені пресети Vibrance для популярних ігор:**
> - **CS2 (Dust II, Mirage, Inferno)**: установіть значення «150–165%». Моделі гравців перестають змішуватися зі стінами та ящиками.
> - **Valorant**: Близько 135–140% – достатньо. У грі вже є підсвічування контуру ворога; вищі значення просто змивають кольори.
> - **Apex Legends & Rust**: `160–170%` творить чудеса для виявлення силуетів у кущах і на великій відстані.
> - **Дві монітори**: Vibrance застосовується лише до основного монітора, на якому запущена ваша гра. Ваш другий екран із Discord або Chrome залишатиметься зі звичайними кольорами.

---

### Як завантажити та запустити (посібник для початківців)

Якщо ви новачок у GitHub, ось весь процес:

1. Натисніть [**Релізи**](https://github.com/Arean-Max/synchro-nova/releases) праворуч на цій сторінці.
2. Завантажте **`synchro.exe`** — це портативна версія. Збережіть його будь-де та двічі клацніть, щоб запустити, інсталятор не потрібен. (Або візьміть `Synchro.Nova_*_x64-setup.exe`, якщо вам потрібні ярлики меню «Пуск»).
3. **Якщо Windows відображає синє спливаюче вікно SmartScreen («Windows захистила ваш ПК»)**:
   - не хвилюйся Windows показує це для будь-якої безкоштовної інді-програми, яка не купує корпоративний підпис від Microsoft за 500 доларів США на рік.
   - Натисніть **"Додаткова інформація"** ➔ **"Усе одно запустити"**.
   - Проект є 100% відкритим кодом прямо тут. Нуль телеметрії, нуль майнерів, нуль реклами.

---

### Швидкий старт: 3 прості кроки

1. Запустіть `synchro.exe`.
2. На вкладці **"Колір"** виберіть рівень яскравості (наприклад, `150%`).
3. Виберіть свою гру зі списку (або наведіть курсор на її файл `.exe`).
Згорнути в трей і грати. Вам більше ніколи не доведеться регулювати його вручну.

---

### FAQ

#### Чи отримаю мене бан у CS2, Valorant, Rust або Faceit?
**100% Ні. Ризик заборони дорівнює нулю.**  
Synchro Nova **не торкається ігрової оперативної пам'яті, не впроваджує DLL-файли в процеси та не малює оверлеїв**. Він налаштовує гамму виключно за допомогою стандартних API відображення Windows — так само, як перетягування повзунка на панелі керування NVIDIA або AMD. Античіт-системи (VAC, Faceit, Vanguard, EAC, BattlEye) навіть не розглядають це як ігрову взаємодію.

#### Навіщо використовувати це через панель керування NVIDIA / AMD?
Панелі керування GPU не можуть автоматично перемикати налаштування для кожної гри. Якщо ви включите там яскравість, весь ваш робочий стіл залишиться перенасиченим під час перегляду веб-сторінок або перегляду відео. Synchro виконує це автоматично на льоту.

#### Чи безпечні налаштування Windows?
так Ми не зачіпаємо цілісність ядра ОС, не порушуємо Windows Defender і не вимикаємо ваш файл сторінки. Перш ніж торкнутися будь-якого налаштування, Synchro створює точку відновлення Windows і резервну копію реєстру. Якщо вам не подобається налаштування, натискання **"Відкотити зміни"** негайно повертає все назад.

---

### Документація та глибоке занурення

Щоб отримати докладні технічні відомості, перегляньте наші спеціальні ресурси:
- 📖 [Повна інтерактивна документація на DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Діаграма архітектури на GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Повні налаштування та довідник реєстру (TWEAKS_REFERENCE.uk.md)](docs/TWEAKS_REFERENCE.uk.md)

---

### Збірка з джерела (для розробників)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # режим розробки
npm run build     # випуск збірки
```

---

### Дописувачі

- [sotoykor-source](https://github.com/sotoykor-source) — Оригінальний творець і попередній автор
- [Arean-Max](https://github.com/Arean-Max) — Розробка та обслуговування

---

### Ліцензія

[MIT](LICENSE) — вільно використовувати, змінювати та ділитися.
