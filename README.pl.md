<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Lekkie narzędzie do regulacji nasycenia kolorów monitora i optymalizacji opóźnień w systemie Windows.</p>

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

### Instalacja

Pobierz najnowszą wersję ze strony [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Wersja przenośna (`synchro.exe`)** — Rozpakuj i uruchom `synchro.exe`. Bez konieczności instalacji.
- **Instalator (`Synchro.Nova_*_x64-setup.exe`)** — Standardowy instalator Windows ze skrótami.

---

### FAQ

#### Czym jest to narzędzie?
Niezwykle lekki program dla graczy. Zwiększa nasycenie kolorów w grach, ułatwia zauważenie przeciwników i eliminuje zbędne opóźnienia Windows. Automatycznie przełącza profile przy starcie gry.

#### Czy dostanę za to bana?
Nie. Program nie ingeruje w pamięć gier, nie używa nakładek ani nie wstrzykuje bibliotek DLL. Zmienia barwy przez oficjalny WinAPI, dokładnie jak panele NVIDIA czy AMD. W 100% bezpieczny dla VAC, Faceit, Vanguard, EAC i BattlEye.

#### Dlaczego warto wybrać Synchro Nova zamiast paneli GPU?
Napisany w Rust + Tauri: błyskawiczny start, 0% CPU i zaledwie ~4 MB RAM w zasobniku systemowym. Oferuje automatyczne profile dla gier i bezpieczne optymalizacje czasu reakcji.

#### Czy optymalizacje nie uszkodzą Windowsa?
Nie. Przed wprowadzeniem zmian aplikacja automatycznie tworzy punkt przywracania systemu Windows oraz kopię zapasową rejestru.

---

### Kompilacja ze źródeł

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # uruchom w trybie developerskim
npm run build     # zbuduj instalator
```

---

### Licencja

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
