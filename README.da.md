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

### Hvad laver Synchro Nova?

Kort sagt: det er et let, gratis værktøj, der booster farves liv og kontrast i spil, og hjælper fjender med at skille sig ud i mørke omgivelser.

- **Automatiske Per-Game Profiler**: Indstil dit yndlingsniveau for dit spil én gang. Når du starter CS2, Valorant eller Apex, starter farverne automatisk. Når du alt-tab eller afslutter, vender dine skærmfarver straks tilbage til normale, så dine øjne ikke gør ondt på skrivebordet eller i din browser.
- **Latens og reaktionsevne**: Inkluderer testede, sikre Windows-responstweaks (timeropløsning, rå museindgang, netværkspakkehåndtering) for at få input til at føles hurtig.
- **Nul ressourcetræk**: Hele appen er under 5 MB, åbner på millisekunder og sidder i din bakke med kun ~2 MB RAM.

---

### Sådan downloader og kører du (begyndervejledning)

Hvis du besøger GitHub for første gang, er her præcis, hvad du skal gøre:

1. Gå over til [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) side (eller klik på "Udgivelser" i højre sidebjælke).
2. Under den seneste udgivelse skal du downloade:
   - **`synchro.exe`** — **bærbar version (anbefales)**. Bare download det til en hvilken som helst mappe og dobbeltklik for at køre. Ingen opsætning nødvendig.
   - Eller **`Synchro.Nova_*_x64-setup.exe`** — standard Windows-installationsprogram, hvis du foretrækker skrivebords- og startmenugenveje.
3. **Hvis Windows viser en blå popup ("Windows beskyttede din pc" / SmartScreen)**:
   - Dette sker med alle gratis open source-værktøjer, der ikke køber et dyrt virksomhedscertifikat på $500/år fra Microsoft.
   - Klik **"Mere info"**, klik derefter **"Løb alligevel"**.
   - Hele kildekoden er 100 % åben lige her på GitHub - ingen adware, ingen telemetri, ingen nonsens.

---

### Hurtig start (1 minuts opsætning)

1. Lancering **Synchro Nova**.
2. Gå til **"Farve"** fanen og vælg dit vibreringsniveau (f.eks. `150%` eller `200%`).
3. Link det til dit spil (appen registrerer automatisk kørende spil, eller du kan vælge `.exe` manuelt).
4. Det er det! Minimer til bakken og leg.

---

### FAQ

#### Bliver jeg banned i CS2, Valorant, Rust eller Faceit?
**100 % Nej. Banrisiko er nul.**  
Synchro Nova **rører ikke spilhukommelsen, injicerer ikke DLL'er i processer og tegner ikke spiloverlejringer**. Den justerer kun desktop-gamma via standard Windows API'er (DWM- og GPU-drivere) - præcis som at trække skyderen Digital Vibrance i NVIDIA- eller AMD-kontrolpanelet. Anti-cheat-motorer (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) markerer ikke skrivebordsfarveændringer.

#### Hvorfor ikke bare bruge NVIDIA- eller AMD-kontrolpanelet?
GPU-kontrolpaneler skifter ikke automatisk farver, når du starter eller lukker et spil. Hvis du skruer op for liv der, vil hele dit Windows-skrivebord og din browser forblive overmættet. Synchro håndterer dette automatisk i farten: spil kører = rige farver; spil lukket = rene skrivebordsfarver.

#### Vil systemjusteringerne ødelægge mit Windows?
Nej. Alle tweaks bruger standard, veldokumenterede Windows-parametre. Før du ændrer noget, opretter Synchro Nova et Windows-gendannelsespunkt og en sikkerhedskopi af ændrede registreringsnøgler. Du kan til enhver tid vende alt tilbage til Windows-standarder ved at klikke **"Tilbageføringsændringer"**.

---

### Dokumentation & Deep Dive

For at holde denne side ren og let at læse, findes omfattende teknisk dokumentation på vores dedikerede wiki:
- 📖 [Fuld interaktiv dokumentation på DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Arkitekturdiagram på GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Komplet Tweaks & Registry Reference (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Byg fra kilde (for udviklere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Bidragydere

- [sotoykor-source](https://github.com/sotoykor-source) — Original skaber og upstream forfatter
- [Arean-Max](https://github.com/Arean-Max) — Udvikling & vedligeholdelse

---

### Licens

Fordelt under [MIT License](LICENSE).
