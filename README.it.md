<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Un'utilità desktop leggera per la brillantezza dei colori e l'ottimizzazione della latenza su Windows.</p>

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

### Installazione

Scarica l'ultima versione dalla pagina [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Versione Portable (`synchro.exe`)** — Estrai ed esegui `synchro.exe`. Nessuna installazione richiesta.
- **Programma di installazione (`Synchro.Nova_*_x64-setup.exe`)** — Installatore classico con collegamenti e disinstallatore.

---

### FAQ

#### Cos'è Synchro Nova?
Un software leggero per gamer. Rende i colori nei giochi vividi e contrastati, migliora la visibilità degli avversari e riduce l'input lag di Windows. Si attiva e disattiva automaticamente all'avvio e alla chiusura dei giochi.

#### C'è rischio di ban?
No. Non interagisce con la memoria dei giochi, non usa overlay e non inietta DLL. Modifica i colori attraverso le normali WinAPI di Windows (come il pannello NVIDIA/AMD). Sicuro al 100% per VAC, Faceit, Vanguard, EAC e BattlEye.

#### Perché è migliore di VibranceGUI?
Scritto in Rust + Tauri: avvio fulmineo, 0% CPU e circa 4 MB di RAM nella tray. Include profili automatici per gioco e ottimizzazioni di sistema sicure.

#### I tweak possono danneggiare il sistema?
No. Prima di applicare qualunque modifica, il programma crea automaticamente un punto di ripristino di Windows e un backup del registro.

---

### Compilazione da sorgente

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # avvia in modalità di sviluppo
npm run build     # compila l'installer di rilascio
```

---

### Licenza

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
