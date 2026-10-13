<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Lagana alatka za pojačavanje boja monitora i optimizaciju odziva u sistemu Windows.</p>

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

### Instalacija

Preuzmite najnoviju verziju sa stranice [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Portabilna verzija (`synchro.exe`)** — Raspakujte i pokrenite `synchro.exe`. Instalacija nije potrebna.
- **Instaler (`Synchro.Nova_*_x64-setup.exe`)** — Standardni instalacioni paket za Windows.

---

### Često postavljana pitanja (FAQ)

#### Šta je Synchro Nova?
Kompaktna aplikacija za gejmere. Čini boje u igrama živopisnim, olakšava uočavanje neprijatelja i smanjuje kašnjenje unosa (input lag). Automatski aktivira profile pri pokretanju igara.

#### Mogu li dobiti ban?
Ne. Synchro Nova ne dira memoriju igara, ne ubacuje DLL-ove i ne koristi preklapanja (overlays). Boje podešava kroz zvanični Windows WinAPI, identično kao NVIDIA ili AMD paneli. Potpuno sigurno za VAC, Faceit, Vanguard, EAC i BattlEye.

#### Zašto odabrati Synchro Nova umjesto standardnih GPU panela?
Napisana u Rust + Tauri: pokreće se u sekundi, troši 0% procesora i samo oko 4 MB RAM-a. Posjeduje automatske profile za igre i bezbjedne optimizacije sistema.

#### Mogu li podešavanja oštetiti Windows?
Ne. Prije svake izmjene program automatski kreira tačku vraćanja sistema (Restore Point) i rezervnu kopiju registra.

---

### Kompajliranje iz koda

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # pokretanje u dev modu
npm run build     # izrada instalera
```

---

### Licenca

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
