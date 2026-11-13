# Synchro Nova: riferimento alle modifiche e ai miglioramenti del sistema

Questo documento fornisce un'analisi completa e trasparente di ogni impostazione di sistema, chiave di registro e comando eseguito da Synchro Nova. Tutte le azioni sono strettamente locali, deterministiche e supportate prima dell'esecuzione.

---

## Garanzia di sicurezza e rollback

1. **Backup pre-esecuzione automatizzato**:
Prima di modificare qualsiasi chiave di registro, Synchro Nova crea un punto di ripristino del sistema e salva lo stato corrente di tutte le chiavi di destinazione in un file di backup `.reg` in `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Ripristino con 1 clic**:
Facendo clic su **"Откатить изменения"** (Rollback) nell'applicazione si ripristina l'istantanea `.reg` precedente tramite Windows `reg.exe` e si rileggono i valori correnti.
3. **Esecuzione dei comandi rafforzata**:
Tutti gli strumenti di amministrazione (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) sono rigorosamente risolti da `%SystemRoot%\System32` (impedendo il dirottamento di PATH elevati) e vengono eseguiti con timeout del watchdog non bloccanti (5 secondi per le utilità, 25 secondi per il ripristino del sistema).
4. **Blocklist rigida e sicurezza anti-cheat**:
Le operazioni non sicure (disabilitazione dell'integrità della memoria/HVCI, patch dei timer del kernel, disabilitazione del file di paging o applicazione di override arbitrarie del driver) vengono bloccate in modo permanente nel motore. L'assenza di iniezione di processi o hook di memoria garantisce la conformità al 100% con Easy Anti-Cheat, BattlEye, Vanguard e VAC.

---

## Analisi completa delle modifiche

### 1. Modifiche a livello di utente (utente standard: non è richiesto alcun amministratore)

Queste modifiche modificano solo l'hive "HKEY_CURRENT_USER". Non richiedono privilegi elevati e influiscono solo sul profilo utente attivo.

| Tweak ID | Hive & Path | Value / Command | Scopo |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Impedisce il popup della Xbox Game Bar all'avvio del gioco |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Disabilita l'accelerazione del mouse di Windows (input lineare non elaborato) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Disabilita l'identificatore pubblicitario di Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Disabilita i suggerimenti personalizzati basati sui dati diagnostici |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Mantiene gli appunti rigorosamente nella RAM locale, disabilita la sincronizzazione cloud |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Disabilita la trasparenza della finestra DWM per risparmiare memoria della GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Configura Esplora risorse per le migliori prestazioni visive |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Rimuove il ritardo artificiale nell'avvio del desktop Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Riduce il ritardo dell'animazione del menu dell'interfaccia utente a 10 ms |

---

### 2. Modifiche a livello di sistema (richiede privilegi di amministratore)

Queste modifiche configurano policy a livello di sistema (`HKEY_LOCAL_MACHINE`) o richiamano strumenti amministrativi standard di Windows (`powercfg`, `netsh`, `fsutil`). Synchro controlla l'elevazione prima di eseguirli.

#### A. Pianificazione di giochi e contenuti multimediali (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Scopo |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Disabilita l'acquisizione video DVR del gioco in background per liberare il codificatore GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Disabilita il meccanismo di limitazione della rete Windows durante le attività ad alta priorità |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Riserva il 90% del tempo della CPU per i giochi in primo piano (l'impostazione predefinita è 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Aumenta la priorità delle attività dello scheduler multimediale per i giochi attivi |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Abilita la pianificazione GPU con accelerazione hardware (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Disabilita gli overlay multipiano (risolve i problemi di schermo nero/stuttering su alcune GPU) |

#### B. Ottimizzazione dello stack di rete (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Scopo |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Cancella la cache del risolutore DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Ripristina il ridimensionamento della finestra TCP standard RFC 1323 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Abilita il Receive-Side Scaling per distribuire l'elaborazione dei pacchetti di rete tra i core della CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Disabilita la coalescenza dei segmenti di ricezione sulle schede NIC per ridurre la latenza nei giochi in tempo reale |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Disabilita la notifica esplicita di congestione per impedire la perdita di pacchetti sui router più vecchi |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Cancella la tabella del protocollo di risoluzione degli indirizzi locale |
| `winsock-reset` | `netsh winsock reset` | Ripristina il catalogo dei socket di rete Windows allo stato pulito |

#### C. Gestione dell'alimentazione e dello spazio di archiviazione (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Scopo |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Attiva la combinazione di risparmio energia standard di Windows High Performance |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Sblocca e attiva la combinazione di risparmio energia di Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Disabilita l'ibernazione ed elimina `hiberfil.sys`, liberando spazio su disco |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Interrompe l'aggiornamento dei timestamp dell'ultimo accesso NTFS a ogni lettura di file |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Garantisce che le notifiche TRIM siano attive per l'integrità dell'SSD e il livellamento dell'usura |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Crea un punto completo di ripristino del sistema Windows prima delle modifiche |

#### D. Telemetria del sistema e sincronizzazione in background
| Tweak ID | Hive & Path | Value | Scopo |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Disabilita la sequenza temporale di Windows e i caricamenti della cronologia delle attività |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Limita l'ottimizzazione del recapito di Windows Update solo alla LAN locale (nessun caricamento P2P su Internet) |

---

## Modifiche non sicure inserite nella lista bloccata

Synchro blocca intenzionalmente diverse "modifiche aggressive" comuni trovate negli script di terze parti che compromettono l'integrità del sistema o la compatibilità anti-cheat:

- **Disabilitazione dell'integrità della memoria (HVCI)/sicurezza basata sulla virtualizzazione**: rifiutata.
- **Disabilitare completamente Windows Defender/Firewall**: rifiutata.
- **Disabilitare il file di paging (`pagefile.sys`)**: rifiutato (causa crash del gioco in caso di picchi di memoria).
- **Disabilitazione del segno di spunta dinamico/manomissione TSC invariante**: rifiutata (può desincronizzare gli orologi e far scattare l'euristica anti-cheat).
- **Hacking in blocco dell'utility MSI (Message Signaled Interrupts)**: rifiutato (rischio di bricking della GPU/schermate blu).
