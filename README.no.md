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

### Hvorfor jeg bygget dette

Hver gang jeg lanserte CS2 eller Apex, måtte jeg åpne NVIDIA-kontrollpanelet, skru opp Digital Vibrance for å oppdage fiender i mørke hjørner, og deretter manuelt slå det ned igjen etter å ha lukket spillet, slik at skrivebordet og nettleseren ikke brenner netthinnene mine.

Synchro løser dette en gang for alle:
- **Automatisk fargebytte**: Start spillet ditt, fargene starter. Alt-Tab eller lukk spillet, skrivebordsfargene går umiddelbart tilbake til det normale.
- **Latensopprydding**: Dreper Windows-museakselerasjon, optimerer timeroppløsning og trimmer nettverkspakkebuffring slik at input føles øyeblikkelig.
- **Null oppblåsthet**: Nedlasting under 5 MB, starter på millisekunder og ligger stille i skuffen med ~2 MB RAM.

> [!TIP]
> **Testet Vibrance-forhåndsinnstillinger for populære spill:**
> - **CS2 (Dust II, Mirage, Inferno)**: Sett til `150–165 %`. Spillermodeller slutter å blande seg inn i vegger og kasser.
> - **Valorant**: Rundt `135–140 %` er nok. Spillet har allerede fiendtlige høydepunkter; høyere verdier vasker bare ut farger.
> - **Apex Legends & Rust**: `160–170%` gjør underverker for å se silhuetter i busker og på lange avstander.
> - **Doble skjermer**: Vibrance gjelder bare for den primære skjermen som kjører spillet ditt. Den andre skjermen din med Discord eller Chrome beholder normale farger.

---

### Hvordan laste ned og løpe (begynnerveiledning)

Hvis du er ny på GitHub, her er hele prosessen:

1. Klikk på [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) på høyre side av denne siden.
2. Last ned **`synchro.exe`** — det er den bærbare versjonen. Lagre den hvor som helst og dobbeltklikk for å starte, ingen installasjon kreves. (Eller ta tak i `Synchro.Nova_*_x64-setup.exe` hvis du vil ha snarveier på Start-menyen).
3. **Hvis Windows viser en blå SmartScreen popup ("Windows beskyttet din PC")**:
   - Ikke bekymre deg. Windows viser dette for alle gratis indie-program som ikke kjøper en bedriftssignatur på $500/år fra Microsoft.
   - Klikk på **"Mer info"** ➔ **"Kjør likevel"**.
   - Prosjektet er 100 % åpen kildekode her. Null telemetri, null gruvearbeidere, null annonser.

---

### Rask start: 3 enkle trinn

1. Start `synchro.exe`.
2. Under **"Farge"**-fanen velger du ditt vibrasjonsnivå (f.eks. «150 %)».
3. Velg spillet ditt fra listen (eller pek på .exe-filen).
Minimer til brett og lek. Du trenger aldri å justere den manuelt igjen.

---

### FAQ

#### Vil jeg bli utestengt i CS2, Valorant, Rust eller Faceit?
**100 % Nei. Banrisiko er null.**  
Synchro Nova **berører ikke spill-RAM, injiserer ikke DLL-er i prosesser og tegner ikke overlegg**. Den justerer gamma rent gjennom standard Windows-skjerm-APIer - akkurat som å dra glidebryteren i NVIDIA- eller AMD-kontrollpanelet. Anti-juksesystemer (VAC, Faceit, Vanguard, EAC, BattlEye) ser det ikke engang som en spillinteraksjon.

#### Hvorfor bruke dette over NVIDIA / AMD kontrollpanel?
GPU-kontrollpaneler kan ikke automatisk bytte innstillinger per spill. Hvis du gir liv der, forblir hele skrivebordet ditt overmettet mens du surfer eller ser på videoer. Synchro håndterer dette automatisk i farten.

#### Er Windows-justeringene trygge?
Ja. Vi berører ikke kjerne OS-integritet, vi bryter ikke Windows Defender, og vi deaktiverer ikke sidefilen din. Før du berører noen innstilling, oppretter Synchro et Windows-gjenopprettingspunkt og en registersikkerhetskopi. Hvis du ikke liker en tweak, kan du tilbakestille alt umiddelbart ved å klikke på **"Rullback Changes"**.

---

### Dokumentasjon og dypdykk

For dyptgående tekniske detaljer, sjekk våre dedikerte ressurser:
- 📖 [Full interaktiv dokumentasjon på DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Arkitekturdiagram på GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Fullfør Tweaks og registerreferanse (TWEAKS_REFERENCE.no.md)](docs/TWEAKS_REFERENCE.no.md)

---

### Bygg fra kilde (for utviklere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # utviklingsmodus
npm run build     # utgivelsesbygg
```

---

### Bidragsytere

- [sotoykor-source](https://github.com/sotoykor-source) — Opprinnelig skaper og oppstrømsforfatter
- [Arean-Max](https://github.com/Arean-Max) — Utvikling og vedlikehold

---

### Tillatelse

[MIT](LICENSE) — gratis å bruke, endre og dele.
