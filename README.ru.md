<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Легкая настольная утилита для оптимизации монитора и задержки Windows.</p>
<p align="center">📖 <b><a href="https://deepwiki.com/Arean-Max/synchro-nova">Полная интерактивная документация на DeepWiki</a></b></p>

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

### Зачем я это сделал

Каждый раз, когда я запускал CS2 или Apex, приходилось открывать панель управления NVIDIA, выкручивать Digital Vibrance на максимум, чтобы видеть врагов в темных углах, а после катки вручную снижать обратно, чтобы рабочий стол и браузер не выжигали глаза.

Synchro решает эту проблему раз и навсегда:
- **Автоматическое переключение цвета**: запускаете игру — цвета становятся сочными. Свернули игру по Alt-Tab или вышли — цвета рабочего стола мгновенно возвращаются в дефолт.
- **Устранение задержки**: убирает встроенную акселерацию мыши Windows, оптимизирует таймер прерываний и разгружает сетевой стек.
- **Ноль мусора**: вес меньше 5 МБ, запускается за миллисекунды и спокойно сидит в трее, потребляя всего ~2 МБ памяти.

> [!TIP]
> **Проверенные пресеты сочности для популярных игр:**
> - **CS2 (Dust II, Mirage, Inferno)**: ставьте `150–165%`. Модельки игроков перестают сливаться со стенами и ящиками.
> - **Valorant**: около `135–140%` более чем достаточно. В игре уже есть контурная подсветка врагов; большие значения сделают картинку кислотной.
> - **Apex Legends & Rust**: `160–170%` творит чудеса при обнаружении силуэтов в кустах и на дальних дистанциях.
> - **Два монитора**: сочность меняется только на основном мониторе с игрой. Второй экран с Discord или браузером сохраняет обычные цвета.

---

### Как скачать и запустить (для тех, кто первый раз на GitHub)

Если вы раньше не качали программы с GitHub, всё очень просто:

1. Нажмите [**Releases (Релизы)**](https://github.com/Arean-Max/synchro-nova/releases) в правой колонке этой страницы.
2. Скачайте **`synchro.exe`** — это портативная версия. Закиньте в любую папку и запустите двойным кликом, установка не нужна. (Или скачайте инсталлятор `Synchro.Nova_*_x64-setup.exe`, если хотите ярлыки в меню «Пуск»).
3. **Если Windows покажет синее окно SmartScreen («Система Windows защитила ваш компьютер»)**:
   - Не переживайте. Windows показывает это окно для любых новых бесплатных инди-программ без купленного за $500/год сертификата Microsoft.
   - Нажмите **"Подробнее"** (More info) ➔ **"Выполнить в любом случае"** (Run anyway).
   - Проект полностью открытый — весь исходный код перед вами. Ноль телеметрии, ноль рекламы, ноль скрытых процессов.

---

### Быстрый старт: 3 простых шага

1. Запустите `synchro.exe`.
2. На вкладке **Цвет** выберите желаемую сочность (например, `150%`).
3. Выберите игру из списка (или укажите её `.exe` вручную).
Сверните программу в трей и играйте. Настраивать что-то заново больше никогда не придется.

---

### Частые вопросы (FAQ)

#### Меня точно не забанят в CS2, Valorant, Rust или на Faceit?
**На 100% нет. Риск бана равен нулю.**  
Synchro Nova **не трогает оперативную память игры, не внедряет DLL в процессы и не рисует оверлеи**. Гамма регулируется исключительно через официальный Windows API — точно так же, как если бы вы двигали ползунок в панели NVIDIA или AMD. Античит-системы (VAC, Faceit, Vanguard, EAC, BattlEye) не считают это вмешательством в игру.

#### Зачем это нужно, если есть панель NVIDIA / AMD?
Панели видеокарт не умеют автоматически переключать профили при запуске и закрытии игр. Если выкрутить сочность там, весь рабочий стол останется ядовито-ярким во время серфинга в браузере или просмотра фильмов. Synchro переключает всё сама на лету.

#### Безопасны ли твики Windows?
Да. Мы не трогаем системные файлы, не ломаем Защитник Windows и не отключаем файл подкачки. Перед любым изменением Synchro создает системную точку восстановления и бэкап реестра. Если что-то не понравится, нажмите **«Откатить изменения»**, чтобы мгновенно вернуть всё в исходное состояние.

---

### Документация и подробное описание

Подробные технические подробности можно найти на наших специализированных ресурсах:
- 📖 [Полная интерактивная документация на DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Диаграмма архитектуры на GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Полная информация по настройкам и реестру (TWEAKS_REFERENCE.ru.md)](docs/TWEAKS_REFERENCE.ru.md)

---

### Сборка из исходного кода (для разработчиков)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # режим разработки
npm run build     # сборка релизного exe
```

---

### Авторы

- [sotoykor-source](https://github.com/sotoykor-source) — Автор оригинального upstream-проекта
- [Arean-Max](https://github.com/Arean-Max) — Разработка и поддержка

---

### Лицензия

[MIT](LICENSE) — свободно используйте, модифицируйте и делитесь.
