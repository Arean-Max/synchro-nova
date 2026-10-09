<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Et letvægtsværktøj til skærmmætning og optimering af Windows-forsinkelse.</p>

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

### Installation

Hent den nyeste udgave fra [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Portabel version (`Synchro-Nova-*-Portable.zip`)** — Udpak og kør `synchro.exe`. Kræver ingen installation.
- **Installationsprogram (`Synchro.Nova_*_x64-setup.exe`)** — Standard installationsprogram til Windows.

---

### Ofte stillede spørgsmål (FAQ)

#### Hvad gør dette værktøj?
Et hurtigt værktøj til gamere. Det gør spillets farver klare og mættede, forbedrer synligheden af fjender og fjerner unødvendig forsinkelse i Windows. Skifter automatisk profil, når du åbner et spil.

#### Bliver jeg bannet for at bruge det?
Nej. Synchro Nova rører ikke spillets hukommelse, bruger ingen overlays og injecter ikke DLL-filer. Det styrer farver via standard Windows API, præcis som NVIDIA/AMD-kontrolpanelerne. 100% sikkert med VAC, Faceit, Vanguard, EAC og BattlEye.

#### Hvorfor er det bedre end VibranceGUI?
Udviklet i Rust + Tauri: starter øjeblikkeligt, bruger 0% CPU og blot ~4 MB RAM i proceslinjen. Understøtter automatiske spilprofiler samt sikre optimeringer.

#### Kan ændringerne ødelægge Windows?
Nej. Før ændringer udføres, opretter programmet automatisk et Windows-gendannelsespunkt og en sikkerhedskopi af registreringsdatabasen.

---

### Byg fra kildekode

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # kør i udviklingstilstand
npm run build     # byg installationspakke
```

---

### Licens

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
