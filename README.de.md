<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Ein schlankes Windows-Tool für Monitor-Farbsättigung und Latenz-Optimierung.</p>

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

### Was macht Synchro Nova?

Kurz gesagt: Es handelt sich um ein leichtes, kostenloses Dienstprogramm, das die Farbbrillanz und den Kontrast in Spielen steigert und Gegnern hilft, sich von dunklen Umgebungen abzuheben.

- **Automatische Profile pro Spiel**: Stellen Sie einmalig Ihre bevorzugte Vibrationsstufe für Ihr Spiel ein. Wenn Sie CS2, Valorant oder Apex starten, werden die Farben automatisch aktiviert. Wenn Sie die Alt-Tab-Taste drücken oder beenden, kehren Ihre Anzeigefarben sofort zur Normalität zurück, sodass Ihre Augen auf dem Desktop oder in Ihrem Browser nicht schmerzen.
- **Latenz und Reaktionsfähigkeit**: Enthält getestete, sichere Windows-Reaktionsoptimierungen (Timer-Auflösung, rohe Mauseingabe, Netzwerkpaketverarbeitung), damit sich die Eingabe schnell anfühlt.
- **Kein Ressourcenwiderstand**: Die gesamte App ist weniger als 5 MB groß, lässt sich in Millisekunden öffnen und befindet sich in Ihrer Taskleiste, wobei sie nur etwa 2 MB RAM benötigt.

---

### Herunterladen und Ausführen (Einsteigerhandbuch)

Wenn Sie GitHub zum ersten Mal besuchen, ist Folgendes genau zu tun:

1. Gehen Sie rüber zum [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) Seite (oder klicken Sie in der rechten Seitenleiste auf „Releases“).
2. Laden Sie unter der neuesten Version Folgendes herunter:
   - **`synchro.exe`** — **portable Version (empfohlen)**. Laden Sie es einfach in einen beliebigen Ordner herunter und doppelklicken Sie, um es auszuführen. Keine Einrichtung erforderlich.
   - Oder **`Synchro.Nova_*_x64-setup.exe`** — Standard-Windows-Installationsprogramm, wenn Sie Desktop- und Startmenü-Verknüpfungen bevorzugen.
3. **Wenn Windows ein blaues Popup anzeigt („Windows hat Ihren PC geschützt“ / SmartScreen)**:
   - Dies geschieht bei allen kostenlosen Open-Source-Tools, für die kein teures Unternehmenszertifikat für 500 US-Dollar pro Jahr von Microsoft erworben werden muss.
   - Klicken **"Weitere Informationen"**, dann klicken **"Lauf trotzdem"**.
   - Der gesamte Quellcode ist hier auf GitHub zu 100 % offen – keine Adware, keine Telemetrie, kein Unsinn.

---

### Schnellstart (1-minütige Einrichtung)

1. Start **Synchro Nova**.
2. Gehe zum **"Farbe"** Tab und wählen Sie Ihre Vibrationsstufe (z. B. `150%` oder `200%`).
3. Verknüpfen Sie es mit Ihrem Spiel (die App erkennt laufende Spiele automatisch, oder Sie können das auswählen `.exe` manuell).
4. Das ist es! Auf das Tablett legen und spielen.

---

### FAQ

#### Werde ich in CS2, Valorant, Rust oder Faceit gesperrt?
**100 % Nein. Das Ban-Risiko ist Null.**  
Synchro Nova **Berührt den Spielspeicher nicht, fügt keine DLLs in Prozesse ein und zeichnet keine Spiel-Overlays**. Es passt das Desktop-Gamma nur über Standard-Windows-APIs (DWM- und GPU-Treiber) an – genau wie das Ziehen des Digital Vibrance-Schiebereglers in der NVIDIA- oder AMD-Systemsteuerung. Anti-Cheat-Engines (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) kennzeichnen keine Desktop-Farbänderungen.

#### Warum nicht einfach die NVIDIA- oder AMD-Systemsteuerung verwenden?
GPU-Kontrollfelder schalten die Farben nicht automatisch um, wenn Sie ein Spiel starten oder schließen. Wenn Sie dort die Dynamik erhöhen, bleibt Ihr gesamter Windows-Desktop und Browser überlastet. Synchro erledigt dies automatisch im laufenden Betrieb: Spiel läuft = satte Farben; Spiel geschlossen = saubere Desktop-Farben.

#### Werden die Systemoptimierungen mein Windows durcheinander bringen?
Nein. Alle Optimierungen verwenden standardmäßige, gut dokumentierte Windows-Parameter. Bevor etwas geändert wird, erstellt Synchro Nova einen Windows-Wiederherstellungspunkt und eine Sicherung der geänderten Registrierungsschlüssel. Sie können jederzeit alles auf die Windows-Standardeinstellungen zurücksetzen, indem Sie auf klicken **"Rollback-Änderungen"**.

---

### Dokumentation & Deep Dive

Um diese Seite übersichtlich und leicht lesbar zu halten, finden Sie in unserem speziellen Wiki eine umfangreiche technische Dokumentation:
- 📖 [Vollständige interaktive Dokumentation auf DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Architekturdiagramm auf GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Vollständige Referenz zu Optimierungen und Registrierung (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Aus dem Quellcode erstellen (für Entwickler)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Mitwirkende

- [sotoykor-source](https://github.com/sotoykor-source) — Ursprünglicher Ersteller und Upstream-Autor
- [Arean-Max](https://github.com/Arean-Max) — Entwicklung & Wartung

---

### Lizenz

Verteilt unter der [MIT License](LICENSE).
