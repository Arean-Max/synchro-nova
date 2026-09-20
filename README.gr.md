<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Ένα ελαφρύ εργαλείο επιφάνειας εργασίας για τη ζωντάνια των χρωμάτων της οθόνης και τη μείωση καθυστέρησης στα Windows.</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
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

### Εγκατάσταση

Κατεβάστε την πιο πρόσφατη έκδοση από τις [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Φορητή έκδοση (`Synchro-Nova-*-Portable.zip`)** — Αποσυμπιέστε και εκτελέστε το `synchro.exe`. Δεν απαιτείται εγκατάσταση.
- **Πρόγραμμα εγκατάστασης (`Synchro.Nova_*_x64-setup.exe`)** — Κλασικό πρόγραμμα εγκατάστασης Windows.

---

### Συχνές Ερωτήσεις (FAQ)

#### Τι είναι αυτό το εργαλείο;
Ένα γρήγορο εργαλείο για gamers. Κάνει τα χρώματα στα παιχνίδια πιο έντονα, βελτιώνει τον εντοπισμό αντιπάλων και μειώνει την περιττή καθυστέρηση των Windows. Εναλλάσσει προφίλ αυτόματα με την εκκίνηση του παιχνιδιού.

#### Υπάρχει κίνδυνος ban από anti-cheat;
Όχι, 100% ασφαλές. Το Synchro Nova δεν αγγίζει τη μνήμη των παιχνιδιών, δεν εισάγει DLL και δεν χρησιμοποιεί επικαλύψεις (overlays). Προσαρμόζει τα χρώματα μέσω των επίσημων WinAPI των Windows, ακριβώς όπως τα πάνελ της NVIDIA και AMD. Ασφαλές για VAC, Faceit, Vanguard, EAC και BattlEye.

#### Γιατί είναι καλύτερο από το VibranceGUI;
Φτιαγμένο με Rust + Tauri: ξεκινά άμεσα, καταναλώνει 0% CPU και μόνο ~4 MB RAM. Προσφέρει αυτόματα προφίλ ανά παιχνίδι και ασφαλείς βελτιστοποιήσεις απόκρισης συστήματος.

#### Μπορούν οι ρυθμίσεις να προκαλέσουν πρόβλημα στα Windows;
Όχι. Πριν από κάθε αλλαγή, δημιουργείται αυτόματα ένα σημείο επαναφοράς των Windows (Restore Point) και αντίγραφο ασφαλείας του μητρώου (registry).

---

### Κατασκευή από τον πηγαίο κώδικα

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # εκτέλεση σε κατάσταση ανάπτυξης
npm run build     # δημιουργία προγράμματος εγκατάστασης
```

---

### Άδεια χρήσης

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

