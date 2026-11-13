<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Et lettvektsverktøy for fargemetning på skjerm og optimalisering av forsinkelse i Windows.</p>

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

### Hva gjør Synchro Nova?

Kort sagt: det er et lett, gratis verktøy som øker fargelevende og kontraster i spill, og hjelper fiender å skille seg ut mot mørke omgivelser.

- **Automatiske per-spill-profiler**: Still inn ditt favorittlivsnivå for spillet ditt én gang. Når du starter CS2, Valorant eller Apex, kommer fargene automatisk inn. Når du alt-tabulator eller avslutter, går skjermfargene umiddelbart tilbake til det normale, slik at øynene dine ikke får vondt på skrivebordet eller i nettleseren.
- **Latens og respons**: Inkluderer testede, trygge Windows-responsjusteringer (tidtakeroppløsning, rå museinngang, nettverkspakkehåndtering) for å få inndataene til å føles raske.
- **Zero Resource Drag**: Hele appen er under 5 MB, åpnes på millisekunder og ligger i skuffen din med bare ~2 MB RAM.

---

### Hvordan laste ned og løpe (begynnerveiledning)

Hvis du besøker GitHub for første gang, er dette nøyaktig hva du skal gjøre:

1. Gå over til [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) side (eller klikk "Utgivelser" på høyre sidefelt).
2. Last ned under den siste utgivelsen:
   - **`synchro.exe`** — **bærbar versjon (anbefalt)**. Bare last den ned til hvilken som helst mappe og dobbeltklikk for å kjøre. Ingen oppsett nødvendig.
   - Eller **`Synchro.Nova_*_x64-setup.exe`** — standard Windows-installasjonsprogram hvis du foretrekker snarveier på skrivebordet og startmenyen.
3. **Hvis Windows viser en blå popup ("Windows beskyttet PC-en din" / SmartScreen)**:
   - Dette skjer med alle gratis verktøy med åpen kildekode som ikke kjøper et dyrt bedriftssertifikat på $500/år fra Microsoft.
   - Klikk **"Mer info"**, klikk deretter **"Løp uansett"**.
   - Hele kildekoden er 100 % åpen her på GitHub – ingen adware, ingen telemetri, ingen tull.

---

### Hurtigstart (1-minutters oppsett)

1. Lansering **Synchro Nova**.
2. Gå til **"Farge"** fanen og velg ditt vibrasjonsnivå (f.eks. `150%` eller `200%`).
3. Koble det til spillet ditt (appen oppdager automatisk kjørende spill, eller du kan velge `.exe` manuelt).
4. Det er det! Minimer til brettet og spill.

---

### FAQ

#### Vil jeg bli utestengt i CS2, Valorant, Rust eller Faceit?
**100 % Nei. Banrisiko er null.**  
Synchro Nova **berører ikke spillminnet, injiserer ikke DLL-er i prosesser og tegner ikke spilloverlegg**. Den justerer bare skrivebordsgamma via standard Windows APIer (DWM- og GPU-drivere) – akkurat som å dra skyveknappen Digital Vibrance i NVIDIA- eller AMD-kontrollpanelet. Anti-cheat-motorer (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) flagger ikke fargeendringer på skrivebordet.

#### Hvorfor ikke bare bruke NVIDIA- eller AMD-kontrollpanelet?
GPU-kontrollpaneler bytter ikke automatisk farger når du starter eller lukker et spill. Hvis du øker pulsen der, vil hele Windows-skrivebordet og nettleseren forbli overmettet. Synchro håndterer dette automatisk på flukt: spillet kjører = rike farger; spillet lukket = rene skrivebordsfarger.

#### Vil systemjusteringene ødelegge Windows?
Nei. Alle justeringer bruker standard, godt dokumenterte Windows-parametere. Før du endrer noe, oppretter Synchro Nova et Windows-gjenopprettingspunkt og en sikkerhetskopi av modifiserte registernøkler. Du kan når som helst tilbakestille alt til Windows-standardene ved å klikke **"Tilbakestill endringer"**.

---

### Dokumentasjon og dypdykk

For å holde denne siden ren og lett å lese, finnes omfattende teknisk dokumentasjon på vår dedikerte wiki:
- 📖 [Full interaktiv dokumentasjon på DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Arkitekturdiagram på GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Fullfør Tweaks og registerreferanse (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Bygg fra kilde (for utviklere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Bidragsytere

- [sotoykor-source](https://github.com/sotoykor-source) — Opprinnelig skaper og oppstrømsforfatter
- [Arean-Max](https://github.com/Arean-Max) — Utvikling og vedlikehold

---

### Tillatelse

Distribuert under [MIT License](LICENSE).
