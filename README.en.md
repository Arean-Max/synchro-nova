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
<p align="center">📖 <b><a href="https://deepwiki.com/Arean-Max/synchro-nova">Full Interactive Documentation on DeepWiki</a></b></p>

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

### What does Synchro Nova do?

In short: it's a lightweight, free utility that boosts color vibrancy and contrast in games, helping enemies stand out against dark environments.

- **Automatic Per-Game Profiles**: Set your favorite vibrance level for your game once. When you launch CS2, Valorant, or Apex, the colors kick in automatically. When you alt-tab or quit, your display colors immediately return to normal so your eyes don't hurt on the desktop or in your browser.
- **Latency & Responsiveness**: Includes tested, safe Windows responsiveness tweaks (timer resolution, raw mouse input, network packet handling) to make input feel snappy.
- **Zero Resource Drag**: The entire app is under 5 MB, opens in milliseconds, and sits in your tray using just ~2 MB of RAM.

---

### How to Download & Run (Beginner's Guide)

If you're visiting GitHub for the first time, here is exactly what to do:

1. Head over to the [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) page (or click "Releases" on the right sidebar).
2. Under the latest release, download:
   - **`synchro.exe`** — **portable version (recommended)**. Just download it to any folder and double-click to run. No setup needed.
   - Or **`Synchro.Nova_*_x64-setup.exe`** — standard Windows installer if you prefer desktop and start menu shortcuts.
3. **If Windows shows a blue popup ("Windows protected your PC" / SmartScreen)**:
   - This happens with all free open-source tools that don't buy an expensive $500/year enterprise certificate from Microsoft.
   - Click **"More info"**, then click **"Run anyway"**.
   - The entire source code is 100% open right here on GitHub — no adware, no telemetry, no nonsense.

---

### Quick Start (1-Minute Setup)

1. Launch **Synchro Nova**.
2. Go to the **"Color"** tab and pick your vibrance level (e.g. `150%` or `200%`).
3. Link it to your game (the app auto-detects running games, or you can pick the `.exe` manually).
4. That's it! Minimize to the tray and play.

---

### FAQ

#### Will I get banned in CS2, Valorant, Rust, or Faceit?
**100% No. Ban risk is zero.**  
Synchro Nova **does not touch game memory, does not inject DLLs into processes, and does not draw game overlays**. It only adjusts desktop gamma via standard Windows APIs (DWM and GPU drivers) — exactly like dragging the Digital Vibrance slider in the NVIDIA or AMD control panel. Anti-cheat engines (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) do not flag desktop color changes.

#### Why not just use the NVIDIA or AMD Control Panel?
GPU control panels don't automatically toggle colors when you launch or close a game. If you crank up vibrance there, your entire Windows desktop and browser will stay oversaturated. Synchro handles this automatically on the fly: game running = rich colors; game closed = clean desktop colors.

#### Will the system tweaks mess up my Windows?
No. All tweaks use standard, well-documented Windows parameters. Before modifying anything, Synchro Nova creates a Windows Restore Point and a backup of modified registry keys. You can revert everything back to Windows defaults anytime by clicking **"Rollback Changes"**.

---

### Documentation & Deep Dive

To keep this page clean and easy to read, extensive technical documentation lives on our dedicated wiki:
- 📖 [Full Interactive Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Complete Tweaks & Registry Reference (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Build from source (for developers)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Contributors

- [sotoykor-source](https://github.com/sotoykor-source) — Original creator & upstream author
- [Arean-Max](https://github.com/Arean-Max) — Development & maintenance

---

### License

Distributed under the [MIT License](LICENSE).
