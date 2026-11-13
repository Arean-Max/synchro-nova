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

### Cosa fa Synchro Nova?

In breve: è un'utilità leggera e gratuita che aumenta la vivacità dei colori e il contrasto nei giochi, aiutando i nemici a risaltare negli ambienti bui.

- **Profili automatici per gioco**: Imposta una volta il livello di vivacità preferito per il tuo gioco. Quando avvii CS2, Valorant o Apex, i colori si attivano automaticamente. Quando premi alt-tab o esci, i colori del display tornano immediatamente alla normalità in modo che gli occhi non ti facciano male sul desktop o nel browser.
- **Latenza e reattività**: Include modifiche testate e sicure alla reattività di Windows (risoluzione del timer, input non elaborato del mouse, gestione dei pacchetti di rete) per rendere l'input scattante.
- **Zero trascinamento delle risorse**: L'intera app occupa meno di 5 MB, si apre in millisecondi e si trova nel vassoio utilizzando solo circa 2 MB di RAM.

---

### Come scaricare ed eseguire (Guida per principianti)

Se stai visitando GitHub per la prima volta, ecco esattamente cosa fare:

1. Dirigiti al [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) (o fare clic su "Versioni" nella barra laterale destra).
2. Nell'ultima versione, scaricare:
   - **`synchro.exe`** — **versione portatile (consigliata)**. Basta scaricarlo in qualsiasi cartella e fare doppio clic per eseguirlo. Nessuna configurazione necessaria.
   - O **`Synchro.Nova_*_x64-setup.exe`** — programma di installazione standard di Windows se preferisci i collegamenti sul desktop e sul menu Start.
3. **Se Windows mostra un popup blu ("Windows protegge il tuo PC"/SmartScreen)**:
   - Ciò accade con tutti gli strumenti open source gratuiti che non acquistano un costoso certificato aziendale da $ 500 all'anno da Microsoft.
   - Clic **"Maggiori informazioni"**, quindi fare clic **"Corri comunque"**.
   - L'intero codice sorgente è aperto al 100% proprio qui su GitHub: nessun adware, nessuna telemetria, nessuna sciocchezza.

---

### Avvio rapido (configurazione in 1 minuto)

1. Lancio **Synchro Nova**.
2. Vai a **"Colore"** scheda e scegli il livello di vivacità (ad es. `150%` O `200%`).
3. Collegalo al tuo gioco (l'app rileva automaticamente i giochi in esecuzione oppure puoi scegliere il file `.exe` manualmente).
4. Questo è tutto! Riduci a icona nel vassoio e gioca.

---

### Domande frequenti

#### Verrò bannato in CS2, Valorant, Rust o Faceit?
**100% No. Il rischio di ban è pari a zero.**  
Synchro Nova **non tocca la memoria del gioco, non inserisce DLL nei processi e non disegna sovrapposizioni di gioco**. Regola la gamma del desktop solo tramite API Windows standard (driver DWM e GPU), esattamente come trascinando il cursore Digital Vibrance nel pannello di controllo NVIDIA o AMD. I motori anti-cheat (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) non segnalano le modifiche ai colori del desktop.

#### Perché non utilizzare semplicemente il pannello di controllo NVIDIA o AMD?
I pannelli di controllo della GPU non cambiano automaticamente i colori quando avvii o chiudi un gioco. Se aumenti la vivacità lì, l'intero desktop e browser di Windows rimarranno saturati. Synchro lo gestisce automaticamente al volo: gioco in esecuzione = colori ricchi; gioco chiuso = colori del desktop puliti.

#### Le modifiche al sistema rovineranno il mio Windows?
No. Tutte le modifiche utilizzano parametri Windows standard e ben documentati. Prima di modificare qualsiasi cosa, Synchro Nova crea un punto di ripristino di Windows e un backup delle chiavi di registro modificate. Puoi ripristinare tutto alle impostazioni predefinite di Windows in qualsiasi momento facendo clic **"Modifiche di ripristino"**.

---

### Documentazione e approfondimento

Per mantenere questa pagina pulita e facile da leggere, un'ampia documentazione tecnica si trova nella nostra wiki dedicata:
- 📖 [Documentazione interattiva completa su DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagramma dell'architettura su GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Tweaks completi e riferimento al registro (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Compila dal codice sorgente (per sviluppatori)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Collaboratori

- [sotoykor-source](https://github.com/sotoykor-source) — Creatore originale e autore a monte
- [Arean-Max](https://github.com/Arean-Max) — Sviluppo e manutenzione

---

### Licenza

Distribuito sotto il [MIT License](LICENSE).
