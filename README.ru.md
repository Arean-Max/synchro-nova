<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Быстрая и легковесная утилита для сочности монитора и оптимизации задержки Windows.</p>
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

### Что умеет Synchro Nova?

Если коротко: это удобная бесплатная программа, которая делает картинку в играх сочной и контрастной, чтобы противники не сливались с окружением и темнотой.

- **Автоматическое включение**: вы один раз настраиваете нужную сочность для игры. Запустили CS2, Valorant или Apex — сочность включилась. Свернули игру или вышли — цвета на мониторе сразу возвращаются в обычные стандартные, чтобы не резать глаза в браузере или на рабочем столе.
- **Оптимизация задержки (Latency)**: проверенные настройки отклика Windows (таймер прерываний, отключение сглаживания мыши, сетевой стек), чтобы управление ощущалось резким и плавным.
- **Не жрёт ресурсы**: весит меньше 5 МБ, запускается за доли секунды и в фоне сидит в трее, потребляя всего ~2 МБ оперативки.

---

### Как скачать и запустить (для тех, кто первый раз на GitHub)

Если вы никогда раньше не скачивали программы с GitHub — не пугайтесь, всё просто:

1. Перейдите на страницу [**Releases (Релизы)**](https://github.com/Arean-Max/synchro-nova/releases) (кнопка справа на этой странице или по ссылке).
2. В самом низу последнего релиза найдите блок с файлами:
   - **`synchro.exe`** — **портативная версия (рекомендуется)**. Скачали в любую папку и запустили. Никакой установки не нужно.
   - **`Synchro.Nova_*_x64-setup.exe`** — обычный установщик, если вам нужны ярлыки на рабочем столе и в меню «Пуск».
3. **Если при запуске Windows ругается («Система Windows защитила ваш компьютер» / SmartScreen)**:
   - Это стандартное окно Windows для всех новых бесплатных программ без платного сертификата от Microsoft.
   - Нажмите **«Подробнее»** (More info), а затем появившуюся кнопку **«Выполнить в любом случае»** (Run anyway).
   - Весь код проекта на 100% открыт прямо здесь — в нём нет скрытых майнеров, стиллеров или телеметрии.

---

### Быстрый старт: как настроить за 1 минуту

1. Запустите **Synchro Nova**.
2. Во вкладке **«Цвет»** настройте желаемый уровень сочности (например, `150%` или `200%`).
3. Выберите игру из списка запущенных (или добавьте её `.exe` вручную).
4. Всё готово. Программу можно свернуть в трей и спокойно играть.

---

### Частые вопросы (FAQ)

#### За это точно не забанят в CS2, Valorant, Rust или на Faceit?
**На 100% нет, бан исключён.**  
Synchro Nova **не трогает файлы игр, не внедряет DLL в процессы, не читает оперативную память и не рисует оверлеи**. Она меняет гамму на уровне рабочего стола Windows через стандартный WinAPI — ровно так же, как это делают официальные драйверы NVIDIA Control Panel или AMD Software. Античиты (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) не считают это вмешательством в игру.

#### Зачем это нужно, если у меня есть панель NVIDIA / AMD?
В стандартных драйверах видеокарты нет удобного автоматического переключения. Если выкрутить сочность там, то при выходе из игры цвета на всем мониторе останутся едкими. Либо придётся каждый раз вручную двигать ползунки. Synchro делает это на автомате: зашли в игру — сочно, вышли — стандартно.

#### Системные твики не сломают мне Windows?
Нет. В программе используются только проверенные и безопасные параметры отклика. Перед применением любых твиков Synchro Nova автоматически создает точку восстановления Windows и резервную копию реестра. В любой момент одна кнопка **«Откатить изменения»** вернёт всё в исходное состояние.

---

### Документация и детальная информация

Мы не перегружаем эту страницу тоннами технического текста, потому что для этого есть отдельная вики:
- 📖 [Полная интерактивная документация на DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Схема архитектуры на GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Справочник реестровых твиков (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Сборка из исходников (для разработчиков)

Если хотите собрать бинарник самостоятельно:

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # запуск в режиме разработки
npm run build     # сборка релизного exe
```

---

### Авторы

- [sotoykor-source](https://github.com/sotoykor-source) — Автор оригинального upstream-проекта
- [Arean-Max](https://github.com/Arean-Max) — Разработка, доработка и мейнтейнинг

---

### Лицензия

Проект распространяется под открытой лицензией [MIT](LICENSE).
