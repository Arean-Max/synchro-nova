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

### Why I built this

Every time I launched CS2 or Apex, I had to open NVIDIA Control Panel, crank Digital Vibrance up to spot enemies in dark corners, and then manually dial it back down after closing the game so my desktop and browser wouldn't burn my retinas.

Synchro solves this once and for all:
- **Automatic color switching**: Launch your game, colors kick in. Alt-Tab or close the game, your desktop colors immediately snap back to normal.
- **Latency cleanup**: Kills Windows mouse acceleration, optimizes timer resolution, and trims network packet buffering so input feels instant.
- **Zero bloat**: Under 5 MB download, starts in milliseconds, and sits quietly in the tray using ~2 MB of RAM.

> [!TIP]
> **Tested Vibrance Presets for Popular Games:**
> - **CS2 (Dust II, Mirage, Inferno)**: Set to `150–165%`. Player models stop blending into walls and crates.
> - **Valorant**: Around `135–140%` is plenty. The game already has enemy outline highlights; higher values just wash out colors.
> - **Apex Legends & Rust**: `160–170%` works wonders for spotting silhouettes in bushes and at long distances.
> - **Dual Monitors**: Vibrance only applies to the primary monitor running your game. Your second screen with Discord or Chrome stays at normal colors.

---

### How to Download & Run (Beginner's Guide)

If you're new to GitHub, here is the whole process:

1. Click on [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) on the right side of this page.
2. Download **`synchro.exe`** — it's the portable version. Save it anywhere and double-click to launch, no installer required. (Or grab `Synchro.Nova_*_x64-setup.exe` if you want Start menu shortcuts).
3. **If Windows shows a blue SmartScreen popup ("Windows protected your PC")**:
   - Don't worry. Windows shows this for any free indie program that doesn't buy a $500/year corporate signature from Microsoft.
   - Click **"More info"** ➔ **"Run anyway"**.
   - The project is 100% open source right here. Zero telemetry, zero miners, zero ads.

---

### Quick Start: 3 Simple Steps

1. Launch `synchro.exe`.
2. Under the **"Color"** tab, pick your vibrance level (e.g. `150%`).
3. Select your game from the list (or point to its `.exe` file).
Minimize to tray and play. You never have to adjust it manually again.

---

### FAQ

#### Will I get banned in CS2, Valorant, Rust, or Faceit?
**100% No. Ban risk is zero.**  
Synchro Nova **does not touch game RAM, does not inject DLLs into processes, and does not draw overlays**. It adjusts gamma purely through standard Windows display APIs — exactly like dragging the slider in the NVIDIA or AMD control panel. Anti-cheat systems (VAC, Faceit, Vanguard, EAC, BattlEye) don't even see it as a game interaction.

#### Why use this over the NVIDIA / AMD Control Panel?
GPU control panels cannot automatically toggle settings per game. If you crank vibrance there, your entire desktop stays oversaturated while browsing or watching videos. Synchro handles this automatically on the fly.

#### Are the Windows tweaks safe?
Yes. We don't touch core OS integrity, we don't break Windows Defender, and we don't disable your pagefile. Before touching any setting, Synchro creates a Windows Restore Point and a registry backup. If you don't like a tweak, clicking **"Rollback Changes"** reverts everything back immediately.

---

### Documentation & Deep Dive

For in-depth technical details, check our dedicated resources:
- 📖 [Full Interactive Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Complete Tweaks & Registry Reference (TWEAKS_REFERENCE.en.md)](docs/TWEAKS_REFERENCE.en.md)

---

### Build from source (for developers)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # development mode
npm run build     # release build
```

---

### Contributors

- [sotoykor-source](https://github.com/sotoykor-source) — Original creator & upstream author
- [Arean-Max](https://github.com/Arean-Max) — Development & maintenance

---

### License

[MIT](LICENSE) — free to use, modify, and share.
