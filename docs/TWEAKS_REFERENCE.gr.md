# Synchro Nova — Αναφορά Τροποποιήσεων & Τροποποιήσεων Συστήματος

Αυτό το έγγραφο παρέχει μια πλήρη, διαφανή ανάλυση κάθε ρύθμισης συστήματος, κλειδιού μητρώου και εντολής που εκτελούνται από το Synchro Nova. Όλες οι ενέργειες είναι αυστηρά τοπικές, ντετερμινιστικές και υποστηρίζονται πριν από την εκτέλεση.

---

## Ασφάλεια & Εγγύηση επαναφοράς

1. **Αυτοματοποιημένη δημιουργία αντιγράφων ασφαλείας πριν από την εκτέλεση**:
Πριν τροποποιήσετε οποιοδήποτε κλειδί μητρώου, το Synchro Nova δημιουργεί ένα σημείο επαναφοράς συστήματος και αποθηκεύει την τρέχουσα κατάσταση όλων των κλειδιών προορισμού σε ένα αρχείο αντιγράφου ασφαλείας `.reg` στο `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-Κάντε κλικ στο Rollback**:
Κάνοντας κλικ στο **"Откатить изменения"** (Επαναφορά) στην εφαρμογή επαναφέρει το προηγούμενο στιγμιότυπο `.reg` μέσω του Windows `reg.exe` και διαβάζει ξανά τις τρέχουσες τιμές.
3. **Hardened Command Execution**:
Όλα τα εργαλεία διαχείρισης («netsh», «powercfg», «sc», «bcdedit», «reg», «powershell») επιλύονται αυστηρά από το «%SystemRoot%\System32» (αποτροπή αυξημένης πειρατείας PATH) και εκτελούνται με μη αποκλειστικά χρονικά όρια παρακολούθησης, 25 δευτερόλεπτα για το σύστημα ανάπαυσης.
4. **Hard Blocklist & Anti-Cheat Safe**:
Μη ασφαλείς λειτουργίες (απενεργοποίηση ακεραιότητας μνήμης/HVCI, επιδιόρθωση χρονοδιακόπτων πυρήνα, απενεργοποίηση του αρχείου σελιδοποίησης ή εφαρμογή αυθαίρετων παρακάμψεων προγραμμάτων οδήγησης) αποκλείονται μόνιμα στον κινητήρα. Η μηδενική έγχυση διαδικασίας ή η σύνδεση μνήμης διασφαλίζει 100% συμμόρφωση με τα Easy Anti-Cheat, BattlEye, Vanguard και VAC.

---

## Πλήρης ανάλυση Tweaks

### 1. Προσαρμογές σε επίπεδο χρήστη (Τυπικός χρήστης — Δεν απαιτείται διαχειριστής)

Αυτές οι τροποποιήσεις τροποποιούν μόνο την ομάδα "HKEY_CURRENT_USER". Δεν απαιτούν αυξημένα προνόμια και επηρεάζουν μόνο το ενεργό προφίλ χρήστη.

| Tweak ID | Hive & Path | Value / Command | Σκοπός |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Αποτρέπει το αναδυόμενο παράθυρο του Xbox Game Bar κατά την εκκίνηση του παιχνιδιού |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Απενεργοποιεί την επιτάχυνση του ποντικιού των Windows (ακατέργαστη γραμμική είσοδος) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Απενεργοποιεί το αναγνωριστικό διαφήμισης των Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Απενεργοποιεί εξατομικευμένες συμβουλές που βασίζονται σε διαγνωστικά δεδομένα |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Διατηρεί το πρόχειρο αυστηρά στην τοπική μνήμη RAM, απενεργοποιεί τον συγχρονισμό στο cloud |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Απενεργοποιεί τη διαφάνεια του παραθύρου DWM για εξοικονόμηση μνήμης GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Διαμορφώνει την Εξερεύνηση των Windows για βέλτιστη οπτική απόδοση |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Καταργεί την τεχνητή καθυστέρηση εκκίνησης της επιφάνειας εργασίας των Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Μειώνει την καθυστέρηση της κίνησης του μενού διεπαφής χρήστη στα 10 ms |

---

### 2. Τροποποιήσεις σε επίπεδο συστήματος (Απαιτούνται δικαιώματα διαχειριστή)

Αυτές οι τροποποιήσεις διαμορφώνουν πολιτικές σε όλο το σύστημα (`HKEY_LOCAL_MACHINE`) ή καλούν τυπικά εργαλεία διαχείρισης των Windows (`powercfg`, `netsh`, `fsutil`). Το Synchro ελέγχει την ανύψωση πριν από την εκτέλεση αυτών.

#### A. Προγραμματισμός παιχνιδιών και πολυμέσων (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Σκοπός |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Απενεργοποιεί τη λήψη βίντεο στο παρασκήνιο Game DVR σε δωρεάν κωδικοποιητή GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Απενεργοποιεί τον μηχανισμό στραγγαλισμού δικτύου των Windows κατά τη διάρκεια εργασιών υψηλής προτεραιότητας |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Διατηρεί το 90% του χρόνου της CPU για παιχνίδια στο προσκήνιο (η προεπιλογή είναι 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Αυξάνει την προτεραιότητα εργασιών προγραμματιστή πολυμέσων για ενεργά παιχνίδια |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Ενεργοποιεί τον προγραμματισμό επιτάχυνσης GPU με υλικό (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Απενεργοποιεί τις επικαλύψεις πολλαπλών επιπέδων (επιλύει προβλήματα μαύρης οθόνης / τραυλισμού σε ορισμένες μονάδες GPU) |

#### Β. Βελτιστοποίηση στοίβας δικτύου («netsh», «ipconfig»)
| Tweak ID | Tool / Command | Σκοπός |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Διαγράφει την προσωρινή μνήμη του προγράμματος επίλυσης DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Επαναφέρει την τυπική κλιμάκωση παραθύρου RFC 1323 TCP |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Ενεργοποιεί την κλίμακα λήψης για τη διανομή της επεξεργασίας πακέτων δικτύου στους πυρήνες της CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Απενεργοποιεί τη συνένωση τμημάτων λήψης σε NIC για μείωση του λανθάνοντος χρόνου στα παιχνίδια σε πραγματικό χρόνο |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Απενεργοποιεί τη ρητή ειδοποίηση συμφόρησης για να αποτρέψει την πτώση πακέτων σε παλαιότερους δρομολογητές |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Διαγράφει τον τοπικό πίνακα πρωτοκόλλου ανάλυσης διευθύνσεων |
| `winsock-reset` | `netsh winsock reset` | Επαναφέρει τον κατάλογο υποδοχών δικτύου των Windows σε καθαρή κατάσταση |

#### Γ. Διαχείριση ενέργειας και αποθήκευσης («powercfg», «fsutil»)
| Tweak ID | Tool / Command | Σκοπός |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Ενεργοποιεί τον τυπικό συνδυασμό τροφοδοσίας υψηλής απόδοσης των Windows |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Ξεκλειδώνει και ενεργοποιεί τον συνδυασμό ισχύος Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Απενεργοποιεί την αδρανοποίηση και διαγράφει το "hiberfil.sys", ελευθερώνοντας χώρο στο δίσκο |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Σταματά την ενημέρωση των χρονικών σφραγίδων τελευταίας πρόσβασης NTFS σε κάθε ανάγνωση του αρχείου |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Διασφαλίζει ότι οι ειδοποιήσεις TRIM είναι ενεργές για την υγεία και τη φθορά του SSD |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Δημιουργεί ένα πλήρες σημείο επαναφοράς συστήματος των Windows πριν από τις αλλαγές |

#### Δ. Τηλεμετρία Συστήματος & Συγχρονισμός Παρασκηνίου
| Tweak ID | Hive & Path | Value | Σκοπός |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Απενεργοποιεί τις μεταφορτώσεις του χρονολογίου και του ιστορικού δραστηριότητας των Windows |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Περιορίζει τη βελτιστοποίηση παράδοσης του Windows Update μόνο σε τοπικό LAN (χωρίς μεταφορτώσεις P2P στο διαδίκτυο) |

---

## Μη ασφαλείς τροποποιήσεις στη λίστα αποκλεισμού

Το Synchro αποκλείει σκόπιμα πολλές κοινές "επιθετικές τροποποιήσεις" που βρίσκονται σε σενάρια τρίτων που διακυβεύουν την ακεραιότητα του συστήματος ή τη συμβατότητα κατά της εξαπάτησης:

- **Απενεργοποίηση ακεραιότητας μνήμης (HVCI) / Ασφάλεια βασισμένη σε εικονικοποίηση**: Απορρίφθηκε.
- **Πλήρης απενεργοποίηση του Windows Defender/Firewall**: Απορρίφθηκε.
- **Απενεργοποίηση αρχείου σελιδοποίησης (`pagefile.sys`)**: Απορρίφθηκε (προκαλεί σφάλματα του παιχνιδιού στις αιχμές της μνήμης).
- **Απενεργοποίηση Dynamic Tick / Invariant TSC tampering**: Απορρίφθηκε (μπορεί να αποσυγχρονίσει τα ρολόγια και να ενεργοποιήσει τα ευρετικά anti-cheat).
- **Μαζική παραβίαση βοηθητικού προγράμματος MSI (Message Signaled Interrupts)**: Απορρίφθηκε (κίνδυνος μπλοκαρίσματος GPU / μπλε οθόνες).
