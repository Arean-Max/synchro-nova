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
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" /></a>
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

### Hvorfor jeg byggede dette

Hver gang jeg lancerede CS2 eller Apex, var jeg nødt til at åbne NVIDIA Kontrolpanel, skrue Digital Vibrance op for at få øje på fjender i mørke hjørner og derefter manuelt ringe ned igen efter at have lukket spillet, så mit skrivebord og min browser ikke brændte mine nethinder.

Synchro løser dette én gang for alle:
- **Automatisk farveskift**: Start dit spil, farverne starter. Alt-Tab eller luk spillet, dine skrivebordsfarver vender straks tilbage til det normale.
- **Latency oprydning**: Dræber Windows-musacceleration, optimerer timeropløsning og trimmer netværkspakkebuffer, så input føles øjeblikkeligt.
- **Nul oppustethed**: Under 5 MB download, starter på millisekunder og sidder stille i bakken ved hjælp af ~2 MB RAM.

> [!TIP]
> **Testede Vibrance Presets for populære spil:**
> - **CS2 (Dust II, Mirage, Inferno)**: Indstil til `150–165 %`. Spillermodeller holder op med at blande sig ind i vægge og kasser.
> - **Valorant**: Omkring `135-140%` er rigeligt. Spillet har allerede fjendens højdepunkter; højere værdier udvasker bare farver.
> - **Apex Legends & Rust**: `160–170%` gør underværker til at se silhuetter i buske og på lange afstande.
> - **Dobbelte skærme**: Vibrance gælder kun for den primære skærm, der kører dit spil. Din anden skærm med Discord eller Chrome forbliver i normale farver.

---

### Sådan downloader og kører du (begyndervejledning)

Hvis du er ny til GitHub, her er hele processen:

1. Klik på [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) til højre på denne side.
2. Download **`synchro.exe`** — det er den bærbare version. Gem det hvor som helst, og dobbeltklik for at starte, ingen installation påkrævet. (Eller tag 'Synchro.Nova_*_x64-setup.exe', hvis du vil have genveje til Start-menuen).
3. **Hvis Windows viser en blå SmartScreen popup ("Windows beskyttede din pc")**:
   - Bare rolig. Windows viser dette for ethvert gratis indie-program, der ikke køber en virksomhedssignatur på $500/år fra Microsoft.
   - Klik på **"Mere info"** ➔ **"Kør alligevel"**.
   - Projektet er 100% open source lige her. Nul telemetri, nul minearbejdere, nul annoncer.

---

### Hurtig start: 3 enkle trin

1. Start `synchro.exe`.
2. Under fanen **"Farve"** skal du vælge dit livniveau (f.eks. "150 %)".
3. Vælg dit spil fra listen (eller peg på dets `.exe`-fil).
Minimer til bakke og leg. Du behøver aldrig at justere det manuelt igen.

---

### FAQ

#### Bliver jeg banned i CS2, Valorant, Rust eller Faceit?
**100 % Nej. Banrisiko er nul.**  
Synchro Nova **rører ikke spil-RAM, injicerer ikke DLL'er i processer og tegner ikke overlejringer**. Den justerer gamma udelukkende gennem standard Windows-skærm-API'er - præcis som at trække skyderen i NVIDIA- eller AMD-kontrolpanelet. Anti-cheat-systemer (VAC, Faceit, Vanguard, EAC, BattlEye) ser det ikke engang som en spilinteraktion.

#### Hvorfor bruge dette over NVIDIA / AMD Kontrolpanel?
GPU-kontrolpaneler kan ikke automatisk skifte indstillinger pr. spil. Hvis du skruer op for liv der, forbliver hele dit skrivebord overmættet, mens du gennemser eller ser videoer. Synchro håndterer dette automatisk i farten.

#### Er Windows tweaks sikre?
Ja. Vi berører ikke kerne OS-integritet, vi bryder ikke Windows Defender, og vi deaktiverer ikke din sidefil. Inden du trykker på en indstilling, opretter Synchro et Windows-gendannelsespunkt og en sikkerhedskopi af registreringsdatabasen. Hvis du ikke kan lide en tweak, vil et klik på **"Rollback Changes"** vende alt tilbage med det samme.

---

### Dokumentation & Deep Dive

For dybdegående tekniske detaljer, tjek vores dedikerede ressourcer:
- 📖 [Fuld interaktiv dokumentation på DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Arkitekturdiagram på GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Komplet Tweaks & Registry Reference (TWEAKS_REFERENCE.da.md)](docs/TWEAKS_REFERENCE.da.md)

---

### Byg fra kilde (for udviklere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # udviklingstilstand
npm run build     # release build
```

---

### Bidragydere

- [sotoykor-source](https://github.com/sotoykor-source) — Original skaber og upstream forfatter
- [Arean-Max](https://github.com/Arean-Max) — Udvikling & vedligeholdelse

---

### Licens

[MIT](LICENSE) — gratis at bruge, ændre og dele.
