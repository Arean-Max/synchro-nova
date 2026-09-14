<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">A lightweight desktop utility for monitor vibrance and Windows latency optimization.</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
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

### Installation

Grab the latest build from [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Portable (`Synchro-Nova-*-Portable.zip`)** — Extract and run `synchro.exe`. No install required.
- **Installer (`Synchro.Nova_*_x64-setup.exe`)** — Standard Windows setup with shortcuts and uninstaller.

---

### FAQ

#### What is this tool?
A lightweight gamer utility. It makes colors in competitive games pop, improves target visibility, and trims unnecessary Windows latency. When you launch a game it switches profiles automatically, and resets back to normal when you exit.

#### Will I get banned?
No. Synchro Nova does not touch game memory, inject DLLs, or draw overlays. It adjusts color output through standard Windows APIs (DWM & gamma ramp), exactly like the NVIDIA or AMD control panels. It is completely safe with VAC, Faceit, Vanguard, EAC, and BattlEye.

#### How is it better than VibranceGUI or GPU control panels?
Built with Rust + Tauri: ultra-lightweight, starts instantly, uses 0% CPU and ~4 MB RAM in the tray. Includes automatic per-game profiles and safe Windows timer / latency tweaks.

#### Will the Windows tweaks break my system?
No. They only apply tested responsiveness settings (timer resolution, MMCSS, network stack). Synchro Nova automatically creates a Windows Restore Point and a registry backup before any changes, so you can revert anytime with one click.

---

### Documentation & Architecture


### Build from source

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build release installer
```

---

### License

[AGPL-3.0](LICENSE)
