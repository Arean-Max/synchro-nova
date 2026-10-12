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

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
  <a href="https://deepwiki.com/Arean-Max/synchro-nova"><img alt="Docs" src="https://img.shields.io/badge/Docs-DeepWiki-blueviolet?style=flat-square" /></a>
  <a href="https://gitdiagram.com/arean-max/synchro-nova"><img alt="Architecture" src="https://img.shields.io/badge/Diagram-GitDiagram-orange?style=flat-square" /></a>
  <a href="https://github.com/Arean-Max/synchro-nova/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-AGPL--3.0-blue?style=flat-square" /></a>
  <a href="https://microsoft.com/windows"><img alt="Platform" src="https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square" /></a>
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

### Установка

Скачайте свежую сборку на странице [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Портативная версия (`synchro.exe`)** — Автономный файл. Запуск без установки.
- **Инсталлятор (`Synchro.Nova_*_x64-setup.exe`)** — Классический установщик для Windows с ярлыками.

---

### FAQ

#### Что это такое?
Легкая утилита для соревновательных игр. Делает цвета ярче и контрастнее, улучшает видимость врагов и снижает задержки в Windows. Работает на автомате: запустили игру — профиль включился, вышли — цвета сбросились в стандартные.

#### За это банят?
Нет. Программа не лезет в память игр, не использует оверлеи и не внедряет библиотеки. Цветокоррекция работает через стандартный WinAPI, ровно как в панелях NVIDIA или AMD. Это на 100% безопасно для VAC, Faceit, Vanguard, EAC и BattlEye.

#### Чем лучше VibranceGUI или панелей видеокарт?
Написана на Rust + Tauri: запускается моментально, ест 0% процессора и ~4 МБ памяти в трее. Умеет автоматически переключать профили под игры и содержит проверенные твики отклика системы.

#### Твики не сломают Windows?
Нет. Перед каждым изменением программа сама делает точку восстановления Windows и бэкап реестра. В любой момент всё можно вернуть обратно в один клик.

---

### Документация и Архитектура

- [Документация на DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Диаграмма архитектуры на GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)

### Сборка из исходников

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # запуск в режиме разработки
npm run build     # сборка релизного инсталлятора
```

---

### Лицензия

[AGPL-3.0](LICENSE)
