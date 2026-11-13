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

### Perché l'ho costruito

Ogni volta che lanciavo CS2 o Apex, dovevo aprire il pannello di controllo NVIDIA, alzare Digital Vibrance per individuare i nemici negli angoli bui, quindi riattivarlo manualmente dopo aver chiuso il gioco in modo che il desktop e il browser non mi bruciassero la retina.

Synchro risolve questo problema una volta per tutte:
- **Cambio colore automatico**: Avvia il gioco, i colori si attivano. Alt-Tab o chiudi il gioco, i colori del desktop torneranno immediatamente alla normalità.
- **Pulizia della latenza**: Elimina l'accelerazione del mouse di Windows, ottimizza la risoluzione del timer e riduce il buffering dei pacchetti di rete in modo che l'input sia istantaneo.
- **Zero gonfiore**: Il download inferiore a 5 MB si avvia in pochi millisecondi e rimane silenzioso nel vassoio utilizzando circa 2 MB di RAM.

> [!TIP]
> **Preimpostazioni di vivacità testate per i giochi più popolari:**
> - **CS2 (Dust II, Mirage, Inferno)**: impostato su "150–165%". I modelli dei giocatori smettono di mimetizzarsi nei muri e nelle casse.
> - **Valorant**: circa "135-140%" è sufficiente. Il gioco presenta già i punti salienti dei contorni dei nemici; valori più alti sbiadiscono semplicemente i colori.
> - **Apex Legends e Rust**: "160–170%" fa miracoli per individuare sagome tra i cespugli e a lunghe distanze.
> - **Doppio monitor**: la vividezza si applica solo al monitor principale su cui è in esecuzione il gioco. Il tuo secondo schermo con Discord o Chrome rimane con colori normali.

---

### Come scaricare ed eseguire (Guida per principianti)

Se sei nuovo su GitHub, ecco l'intero processo:

1. Fare clic su [**Versioni**](https://github.com/Arean-Max/synchro-nova/releases) sul lato destro di questa pagina.
2. Scarica **`synchro.exe`**: è la versione portatile. Salvalo ovunque e fai doppio clic per avviarlo, non è richiesto alcun programma di installazione. (Oppure prendi `Synchro.Nova_*_x64-setup.exe` se desideri le scorciatoie del menu Start).
3. **Se Windows mostra un popup SmartScreen blu ("Windows ha protetto il tuo PC")**:
   - Non preoccuparti. Windows lo mostra per qualsiasi programma indipendente gratuito che non acquisti una firma aziendale da $ 500 all'anno da Microsoft.
   - Fai clic su **"Ulteriori informazioni"** ➔ **"Esegui comunque"**.
   - Il progetto è open source al 100% proprio qui. Zero telemetria, zero miner, zero pubblicità.

---

### Avvio rapido: 3 semplici passaggi

1. Avvia "synchro.exe".
2. Nella scheda **"Colore"**, scegli il tuo livello di vivacità (ad esempio `150%`).
3. Seleziona il tuo gioco dall'elenco (o punta al suo file `.exe`).
Riduci a icona nel vassoio e riproduci. Non dovrai mai più regolarlo manualmente.

---

### Domande frequenti

#### Verrò bannato in CS2, Valorant, Rust o Faceit?
**100% No. Il rischio di ban è pari a zero.**  
Synchro Nova **non tocca la RAM del gioco, non inserisce DLL nei processi e non disegna sovrapposizioni**. Regola la gamma esclusivamente tramite le API di visualizzazione standard di Windows, esattamente come trascinando il cursore nel pannello di controllo NVIDIA o AMD. I sistemi anti-cheat (VAC, Faceit, Vanguard, EAC, BattlEye) non lo vedono nemmeno come un'interazione di gioco.

#### Perché utilizzarlo sul pannello di controllo NVIDIA/AMD?
I pannelli di controllo della GPU non possono attivare/disattivare automaticamente le impostazioni per gioco. Se aumenti la vivacità lì, l'intero desktop rimane saturo durante la navigazione o la visione di video. Synchro lo gestisce automaticamente al volo.

#### Le modifiche di Windows sono sicure?
SÌ. Non tocchiamo l'integrità principale del sistema operativo, non interrompiamo Windows Defender e non disabilitiamo il tuo file di paging. Prima di toccare qualsiasi impostazione, Synchro crea un punto di ripristino di Windows e un backup del registro. Se una modifica non ti piace, facendo clic su **"Ripristina modifiche"** si ripristina immediatamente tutto.

---

### Documentazione e approfondimento

Per dettagli tecnici approfonditi, consulta le nostre risorse dedicate:
- 📖 [Documentazione interattiva completa su DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagramma dell'architettura su GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Tweaks completi e riferimento al registro (TWEAKS_REFERENCE.it.md)](docs/TWEAKS_REFERENCE.it.md)

---

### Compila dal codice sorgente (per sviluppatori)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # modalità di sviluppo
npm run build     # rilascio build
```

---

### Collaboratori

- [sotoykor-source](https://github.com/sotoykor-source) — Creatore originale e autore a monte
- [Arean-Max](https://github.com/Arean-Max) — Sviluppo e manutenzione

---

### Licenza

[MIT](LICENSE) — liberi di utilizzare, modificare e condividere.
