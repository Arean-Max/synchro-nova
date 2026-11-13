# Synchro Nova — Systemændringer & Tweaks-reference

Dette dokument giver en fuldstændig, gennemsigtig oversigt over alle systemindstillinger, registreringsnøgler og kommandoer, der udføres af Synchro Nova. Alle handlinger er strengt lokale, deterministiske og sikkerhedskopieres før udførelse.

---

## Sikkerhed og tilbagerulningsgaranti

1. **Automatiseret Pre-Execution Backup**:
Før du ændrer en registreringsnøgle, opretter Synchro Nova et systemgendannelsespunkt og gemmer den aktuelle tilstand for alle målnøgler i en `.reg` backup-fil i `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-klik på Rollback**:
Ved at klikke på **"Откатить изменения"** (tilbageføring) i applikationen gendanner det forrige `.reg`-øjebliksbillede via Windows `reg.exe` og genlæser aktuelle værdier.
3. **Hærdet kommandoudførelse**:
Alle administrative værktøjer (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) er strengt løst fra `%SystemRoot%\System32` (forhindrer forhøjet PATH-kapring) og kører med ikke-blokerende watchdog-timeouts (5s for hjælpeprogrammer, 25s for systemgendannelse).
4. **Hard Blocklist & Anti-cheat Safe**:
Usikre operationer (deaktivering af hukommelsesintegritet/HVCI, patching af kernetimere, deaktivering af personsøgningsfilen eller anvendelse af vilkårlige drivertilsidesættelser) er permanent blokeret i motoren. Nul procesindsprøjtning eller memory hooking sikrer 100 % overensstemmelse med Easy Anti-Cheat, BattlEye, Vanguard og VAC.

---

## Fuldfør opdeling af Tweaks

### 1. Tweaks på brugerniveau (standardbruger – ingen administrator påkrævet)

Disse justeringer ændrer kun `HKEY_CURRENT_USER`-hivet. De kræver ikke forhøjede rettigheder og påvirker kun den aktive brugerprofil.

| Tweak ID | Hive & Path | Value / Command | Formål |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Forhindrer Xbox Game Bar-popup ved spilstart |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Deaktiverer Windows-musacceleration (rå lineær input) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Deaktiverer Windows-annonce-id |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Deaktiverer diagnostiske databaserede personlige tips |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Holder udklipsholder strengt i lokal RAM, deaktiverer skysynkronisering |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Deaktiverer DWM-vinduesgennemsigtighed for at spare GPU-hukommelse |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Konfigurerer Windows Stifinder for den bedste visuelle ydeevne |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Fjerner kunstig Windows-skrivebords startforsinkelse |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reducerer UI-menuanimationsforsinkelse til 10ms |

---

### 2. Tweaks på systemniveau (kræver administratorrettigheder)

Disse tweaks konfigurerer systemdækkende politikker (`HKEY_LOCAL_MACHINE`) eller kalder standard Windows-administrationsværktøjer (`powercfg`, `netsh`, `fsutil`). Synchro kontrollerer højden før de udføres.

#### A. Spil- og multimedieplanlægning (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Formål |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Deaktiverer baggrundsspil DVR-videooptagelse til gratis GPU-koder |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Deaktiverer Windows-netværksreguleringsmekanismen under opgaver med høj prioritet |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserverer 90 % af CPU-tiden til forgrundsspil (standard er 80 %) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Hæver multimedieplanlægningsopgaveprioriteten for aktive spil |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Aktiverer hardware-accelereret GPU-planlægning (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Deaktiverer Multi-Plane Overlays (løser problemer med sort skærm / hakken på nogle GPU'er) |

#### B. Netværksstabeloptimering ('netsh', 'ipconfig')
| Tweak ID | Tool / Command | Formål |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Rydder DNS-resolver-cachen |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Gendanner standard RFC 1323 TCP vinduesskalering |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Aktiverer Receive-Side Scaling til at distribuere netværkspakkebehandling på tværs af CPU-kerner |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Deaktiverer modtagelsessegmentkoalescering på NIC'er for at reducere latens i realtidsspil |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Deaktiverer Explicit Congestion Notification for at forhindre tabte pakker på ældre routere |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Rydder den lokale adresseopløsningsprotokoltabel |
| `winsock-reset` | `netsh winsock reset` | Gendanner Windows netværkssockets katalog til ren tilstand |

#### C. Strøm- og lagerstyring ("powercfg", "fsutil")
| Tweak ID | Tool / Command | Formål |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Aktiverer standard Windows High Performance strømstyringsskema |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Låser op og aktiverer Windows Ultimate Performance-strømskema |
| `hibernate-off` | `powercfg /hibernate off` | Deaktiverer dvaletilstand og sletter `hiberfil.sys`, hvilket frigør diskplads |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Stopper opdatering af NTFS-tidsstempler for sidste adgang på hver læst fil |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Sikrer, at TRIM-meddelelser er aktive for SSD-sundhed og slidudjævning |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Opretter et komplet Windows-systemgendannelsespunkt før ændringer |

#### D. Systemtelemetri og baggrundssynkronisering
| Tweak ID | Hive & Path | Value | Formål |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Deaktiverer Windows tidslinje og upload af aktivitetshistorik |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Begrænser Windows Update-leveringsoptimering til kun lokalt LAN (ingen P2P-uploads til internettet) |

---

## Blokeret usikre tweaks

Synchro blokerer med vilje adskillige almindelige "aggressive tweaks" fundet i tredjepartsscripts, der kompromitterer systemintegritet eller anti-cheat-kompatibilitet:

- **Deaktivering af hukommelsesintegritet (HVCI)/virtualiseringsbaseret sikkerhed**: Afvist.
- **Deaktivering af Windows Defender/Firewall helt**: Afvist.
- **Deaktivering af personsøgningsfil (`pagefile.sys`)**: Afvist (forårsager spilnedbrud på hukommelsesspidser).
- **Deaktivering af Dynamic Tick / Invariant TSC-manipulation**: Afvist (kan desynkronisere ure og udløse anti-cheat-heuristik).
- **Bulk MSI (Message Signaled Interrupts) utility hacking**: Afvist (risiko for GPU mursten / blå skærme).
