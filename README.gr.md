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

### Γιατί το έχτισα αυτό

Κάθε φορά που έβγαλα το CS2 ή το Apex, έπρεπε να ανοίξω τον Πίνακα Ελέγχου της NVIDIA, να ενεργοποιήσω το Digital Vibrance για να εντοπίσω εχθρούς σε σκοτεινές γωνίες και μετά να το καλέσω χειροκίνητα μετά το κλείσιμο του παιχνιδιού, ώστε η επιφάνεια εργασίας και το πρόγραμμα περιήγησής μου να μην κάψουν τον αμφιβληστροειδή μου.

Το Synchro το λύνει μια για πάντα:
- **Αυτόματη εναλλαγή χρωμάτων**: Ξεκινήστε το παιχνίδι σας, τα χρώματα ξεκινούν. Alt-Tab ή κλείστε το παιχνίδι, τα χρώματα της επιφάνειας εργασίας σας επανέρχονται αμέσως στο κανονικό.
- **Εκκαθάριση λανθάνοντος χρόνου**: Εξουδετερώνει την επιτάχυνση του ποντικιού των Windows, βελτιστοποιεί την ανάλυση του χρονοδιακόπτη και περιορίζει την προσωρινή μνήμη πακέτων δικτύου, ώστε η εισαγωγή να είναι άμεση.
- **Μηδενικό φούσκωμα**: Κάτω από 5 MB λήψη, ξεκινά σε χιλιοστά του δευτερολέπτου και τοποθετείται αθόρυβα στο δίσκο χρησιμοποιώντας ~2 MB μνήμης RAM.

> [!TIP]
> **Δοκιμασμένες προεπιλογές Vibrance για δημοφιλή παιχνίδια:**
> - **CS2 (Dust II, Mirage, Inferno)**: Ορίστηκε σε `150–165%`. Τα μοντέλα παικτών σταματούν να αναμειγνύονται σε τοίχους και κιβώτια.
> - **Valorant**: Περίπου το `135–140%` είναι αρκετό. Το παιχνίδι έχει ήδη τα κυριότερα σημεία του εχθρού. υψηλότερες τιμές απλώς ξεπλένουν τα χρώματα.
> - **Apex Legends & Rust**: Το «160–170%» κάνει θαύματα για να εντοπίσει σιλουέτες σε θάμνους και σε μεγάλες αποστάσεις.
> - **Διπλές οθόνες**: Το Vibrance ισχύει μόνο για την κύρια οθόνη που εκτελεί το παιχνίδι σας. Η δεύτερη οθόνη σας με Discord ή Chrome παραμένει σε κανονικά χρώματα.

---

### Πώς να κάνετε λήψη και εκτέλεση (Οδηγός για αρχάριους)

Εάν είστε νέοι στο GitHub, εδώ είναι ολόκληρη η διαδικασία:

1. Κάντε κλικ στο [**Κυκλοφορίες**](https://github.com/Arean-Max/synchro-nova/releases) στη δεξιά πλευρά αυτής της σελίδας.
2. Λήψη **`synchro.exe`** — είναι η φορητή έκδοση. Αποθηκεύστε το οπουδήποτε και κάντε διπλό κλικ για εκκίνηση, δεν απαιτείται πρόγραμμα εγκατάστασης. (Εναλλακτικά, πάρτε το `Synchro.Nova_*_x64-setup.exe` εάν θέλετε τις συντομεύσεις του μενού Έναρξη).
3. **Εάν τα Windows εμφανίζουν ένα μπλε αναδυόμενο παράθυρο SmartScreen ("Τα Windows προστατεύουν τον υπολογιστή σας")**:
   - Μην ανησυχείς. Τα Windows το εμφανίζουν για οποιοδήποτε δωρεάν πρόγραμμα indie που δεν αγοράζει εταιρική υπογραφή 500 $/έτος από τη Microsoft.
   - Κάντε κλικ στο **"Περισσότερες πληροφορίες"** ➔ **"Εκτέλεση ούτως ή άλλως"**.
   - Το έργο είναι 100% ανοιχτού κώδικα εδώ. Μηδενική τηλεμετρία, μηδέν εξορύκτες, μηδέν διαφημίσεις.

---

### Γρήγορη εκκίνηση: 3 απλά βήματα

1. Εκκινήστε το `synchro.exe`.
2. Κάτω από την καρτέλα **"Χρώμα"**, επιλέξτε το επίπεδο δόνησης (π.χ. `150%`).
3. Επιλέξτε το παιχνίδι σας από τη λίστα (ή τοποθετήστε το δείκτη του ποντικιού στο αρχείο `.exe`).
Ελαχιστοποιήστε στο δίσκο και παίξτε. Δεν χρειάζεται ποτέ να το ρυθμίσετε ξανά χειροκίνητα.

---

### FAQ

#### Θα μου αποκλειστούν σε CS2, Valorant, Rust ή Faceit;
**100% Όχι. Ο κίνδυνος απαγόρευσης είναι μηδενικός.**  
Synchro Nova **δεν αγγίζει τη μνήμη RAM του παιχνιδιού, δεν εισάγει αρχεία DLL σε διεργασίες και δεν δημιουργεί επικαλύψεις**. Προσαρμόζει το γάμμα αποκλειστικά μέσω τυπικών API οθόνης των Windows — ακριβώς όπως σύρετε το ρυθμιστικό στον πίνακα ελέγχου NVIDIA ή AMD. Τα συστήματα κατά της εξαπάτησης (VAC, Faceit, Vanguard, EAC, BattlEye) δεν το βλέπουν καν ως αλληλεπίδραση παιχνιδιού.

#### Γιατί να το χρησιμοποιήσετε μέσω του Πίνακα Ελέγχου NVIDIA / AMD;
Οι πίνακες ελέγχου GPU δεν μπορούν να αλλάξουν αυτόματα τις ρυθμίσεις ανά παιχνίδι. Εάν ενεργοποιήσετε τη δόνηση εκεί, ολόκληρη η επιφάνεια εργασίας σας παραμένει υπερκορεσμένη κατά την περιήγηση ή την παρακολούθηση βίντεο. Το Synchro το χειρίζεται αυτόματα αμέσως.

#### Είναι ασφαλή τα τσιμπήματα των Windows;
Ναί. Δεν αγγίζουμε την ακεραιότητα του βασικού λειτουργικού συστήματος, δεν παραβιάζουμε το Windows Defender και δεν απενεργοποιούμε το αρχείο σελίδας σας. Πριν αγγίξετε οποιαδήποτε ρύθμιση, το Synchro δημιουργεί ένα σημείο επαναφοράς των Windows και ένα αντίγραφο ασφαλείας μητρώου. Εάν δεν σας αρέσει ένα tweak, κάνοντας κλικ στο **"Αλλαγή επαναφοράς"** επαναφέρει τα πάντα πίσω αμέσως.

---

### Τεκμηρίωση & Βαθιά Κατάδυση

Για λεπτομερείς τεχνικές λεπτομέρειες, ελέγξτε τους ειδικούς πόρους μας:
- 📖 [Πλήρης διαδραστική τεκμηρίωση στο DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Διάγραμμα αρχιτεκτονικής στο GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Πλήρης Αναφορά Tweaks & Registry (TWEAKS_REFERENCE.gr.md)](docs/TWEAKS_REFERENCE.gr.md)

---

### Δημιουργία από την πηγή (για προγραμματιστές)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # λειτουργία ανάπτυξης
npm run build     # έκδοση έκδοσης
```

---

### Συνεισφέροντες

- [sotoykor-source](https://github.com/sotoykor-source) — Πρωτότυπος δημιουργός και συγγραφέας upstream
- [Arean-Max](https://github.com/Arean-Max) — Ανάπτυξη & συντήρηση

---

### Αδεια

[MIT](LICENSE) — δωρεάν για χρήση, τροποποίηση και κοινή χρήση.
