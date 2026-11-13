# Synchro Nova — Systemmodifikasjoner og tweaksreferanse

Dette dokumentet gir en fullstendig, gjennomsiktig oversikt over hver systeminnstilling, registernøkkel og kommando utført av Synchro Nova. Alle handlinger er strengt lokale, deterministiske og sikkerhetskopiert før utførelse.

---

## Sikkerhet og tilbakeføringsgaranti

1. **Automatisert sikkerhetskopiering før utførelse**:
Før du endrer en registernøkkel, oppretter Synchro Nova et systemgjenopprettingspunkt og lagrer gjeldende tilstand for alle målnøkler til en `.reg` sikkerhetskopifil i `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-klikk Tilbakestill**:
Ved å klikke på **"Откатить изменения"** (Tilbakeulling) i programmet gjenoppretter det forrige `.reg` øyeblikksbildet via Windows `reg.exe` og leser gjeldende verdier på nytt.
3. **Herdet kommandoutførelse**:
Alle administrative verktøy (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) er strengt løst fra `%SystemRoot%\System32` (forhindrer forhøyet PATH-kapring) og kjøres med ikke-blokkerende watchdog-tidsavbrudd (5s for verktøy, 25s for systemgjenoppretting).
4. **Hard blokkeringsliste og anti-juks safe**:
Usikre operasjoner (deaktivering av minneintegritet/HVCI, patching av kjernetidtakere, deaktivering av personsøkingsfilen eller bruk av vilkårlige driveroverstyringer) er permanent blokkert i motoren. Null prosessinjeksjon eller minnekroking sikrer 100 % samsvar med Easy Anti-Cheat, BattlEye, Vanguard og VAC.

---

## Fullfør justeringer

### 1. Tweaks på brukernivå (standardbruker – ingen administrator kreves)

Disse justeringene endrer bare `HKEY_CURRENT_USER`-strukturen. De krever ikke forhøyede rettigheter og påvirker kun den aktive brukerprofilen.

| Tweak ID | Hive & Path | Value / Command | Hensikt |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Forhindrer Xbox Game Bar-popup ved spillstart |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Deaktiverer Windows-museakselerasjon (rå lineær inngang) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Deaktiverer Windows-annonseringsidentifikator |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Deaktiverer diagnostiske databaserte personlig tilpassede tips |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Holder utklippstavlen strengt i lokal RAM, deaktiverer skysynkronisering |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Deaktiverer DWM-vindusgjennomsiktighet for å spare GPU-minne |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Konfigurerer Windows Utforsker for best visuell ytelse |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Fjerner kunstig oppstartsforsinkelse for Windows-skrivebord |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reduserer UI-menyens animasjonsforsinkelse til 10 ms |

---

### 2. Tweaks på systemnivå (krever administratorrettigheter)

Disse justeringene konfigurerer systemomfattende retningslinjer (`HKEY_LOCAL_MACHINE`) eller kaller standard Windows-administrasjonsverktøy (`powercfg`, `netsh`, `fsutil`). Synchro sjekker høyden før disse utføres.

#### A. Spill og multimedieplanlegging (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Hensikt |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Deaktiverer bakgrunnsspill DVR-videoopptak til gratis GPU-koder |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Deaktiverer Windows-nettverksreguleringsmekanismen under høyprioriterte oppgaver |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserverer 90 % av CPU-tiden for forgrunnsspill (standard er 80 %) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Øker multimedieplanleggeroppgaveprioriteten for aktive spill |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Aktiverer maskinvareakselerert GPU-planlegging (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Deaktiverer flerplansoverlegg (løser problemer med svart skjerm / hakking på enkelte GPUer) |

#### B. Network Stack Optimization (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Hensikt |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Tømmer DNS-løserbufferen |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Gjenoppretter standard RFC 1323 TCP-vinduskalering |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Aktiverer mottakssideskalering for å distribuere nettverkspakkebehandling på tvers av CPU-kjerner |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Deaktiverer Coalescing av mottakssegmenter på NIC-er for å redusere ventetiden i sanntidsspilling |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Deaktiverer varsling om eksplisitt overbelastning for å forhindre at pakker mistes på eldre rutere |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Sletter lokal adresseoppløsningsprotokolltabell |
| `winsock-reset` | `netsh winsock reset` | Gjenoppretter Windows-nettverkskontaktkatalogen til ren tilstand |

#### C. Strøm- og lagringsadministrasjon (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Hensikt |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Aktiverer standard Windows High Performance-strømoppsett |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Låser opp og aktiverer Windows Ultimate Performance strømskjema |
| `hibernate-off` | `powercfg /hibernate off` | Deaktiverer dvalemodus og sletter `hiberfil.sys`, og frigjør diskplass |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Slutter å oppdatere NTFS siste-tilgang-tidsstempler på hver fil som leses |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Sikrer at TRIM-varslinger er aktive for SSD-helse og utjevning av slitasje |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Oppretter et fullstendig Windows-systemgjenopprettingspunkt før endringer |

#### D. Systemtelemetri og bakgrunnssynkronisering
| Tweak ID | Hive & Path | Value | Hensikt |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Deaktiverer Windows-tidslinje og opplasting av aktivitetshistorikk |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Begrenser Windows Update-leveringsoptimalisering til kun lokalt LAN (ingen P2P-opplastinger til internett) |

---

## Blokkert usikre tilpasninger

Synchro blokkerer med vilje flere vanlige "aggressive justeringer" som finnes i tredjepartsskript som kompromitterer systemintegritet eller anti-juksekompatibilitet:

- **Deaktivering av minneintegritet (HVCI) / virtualiseringsbasert sikkerhet**: Avvist.
- **Deaktivering av Windows Defender/brannmur helt**: Avvist.
- **Deaktivering av personsøkerfil (`pagefile.sys`)**: Avvist (forårsaker spillkrasj på minnetopper).
- **Deaktivere Dynamic Tick / Invariant TSC-tukling**: Avvist (kan desynkronisere klokker og utløse anti-jukseheuristikk).
- **Mass MSI (Message Signaled Interrupts)-verktøyhacking**: Avvist (risiko for GPU-muring / blå skjermer).
