# Synchro Nova — Referenca za modifikacije i podešavanja sistema

Ovaj dokument pruža potpunu, transparentnu analizu svake sistemske postavke, ključa registratora i komande koje izvršava Synchro Nova. Sve akcije su striktno lokalne, determinističke i sigurnosno kopirane prije izvršenja.

---

## Sigurnost i garancija povrata

1. **Automatsko sigurnosno kopiranje prije izvršenja**:
Prije izmjene bilo kojeg ključa registratora, Synchro Nova kreira tačku vraćanja sistema i sprema trenutno stanje svih ciljnih ključeva u `.reg` sigurnosnu kopiju u `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-Klik na Vraćanje**:
Klikom na **"Otkatitʹ izmene"** (Rollback) u aplikaciji vraća se prethodni `.reg` snimak preko Windows `reg.exe` i ponovo čita trenutne vrednosti.
3. **Učvršćeno izvršavanje komandi**:
Svi administrativni alati (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) su striktno riješeni od `%SystemRoot%\System32` (sprečavajući povišenu otmicu PATH-a) i pokreću se sa 2 vremenskih ograničenja 5 za sisteme koji ne blokiraju vrijeme vratiti).
4. **Hard Blocklist & Anti-Cheat Safe**:
Nesigurne operacije (onemogućavanje integriteta memorije/HVCI, zakrpe tajmera kernela, onemogućavanje datoteke stranične memorije ili primjena proizvoljnih zamjena upravljačkih programa) su trajno blokirane u mašini. Nulto ubrizgavanje procesa ili spajanje memorije osigurava 100% usklađenost sa Easy Anti-Cheat, BattlEye, Vanguard i VAC.

---

## Potpuna razgradnja podešavanja

### 1. Podešavanja na korisničkom nivou (standardni korisnik — nije potreban administrator)

Ova podešavanja samo modificiraju `HKEY_CURRENT_USER` košnicu. Ne zahtijevaju povišene privilegije i utječu samo na aktivni korisnički profil.

| Tweak ID | Hive & Path | Value / Command | Svrha |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Sprečava iskačući prozor Xbox Game Bara pri pokretanju igre |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Onemogućuje Windows ubrzanje miša (neobrađeni linearni unos) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Onemogućuje Windows identifikator oglašavanja |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Onemogućuje personalizirane savjete zasnovane na dijagnostičkim podacima |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Čuva međuspremnik strogo u lokalnoj RAM memoriji, onemogućuje sinhronizaciju u oblaku |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Onemogućuje transparentnost DWM prozora radi uštede GPU memorije |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Konfiguriše Windows Explorer za najbolje vizuelne performanse |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Uklanja vještačko kašnjenje pokretanja Windows desktopa |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Smanjuje kašnjenje animacije menija korisničkog interfejsa na 10 ms |

---

### 2. Podešavanja na nivou sistema (zahtevaju administratorske privilegije)

Ova podešavanja konfigurišu politike širom sistema (`HKEY_LOCAL_MACHINE`) ili pozivaju standardne Windows administrativne alate (`powercfg`, `netsh`, `fsutil`). Sinhro provjerava elevaciju prije nego što ih izvrši.

#### A. Zakazivanje igara i multimedije (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Svrha |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Onemogućuje pozadinsko snimanje video zapisa Game DVR-a na besplatni GPU koder |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Onemogućuje Windows mehanizam prigušivanja mreže tokom zadataka visokog prioriteta |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Rezervira 90% CPU vremena za igre u prvom planu (podrazumevano je 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Podiže prioritet zadatka multimedijalnog planera za aktivne igre |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Omogućuje hardverski ubrzano GPU zakazivanje (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Onemogućuje preklapanja u više ravni (rješava probleme sa crnim ekranom/mucanjem na nekim GPU-ovima) |

#### B. Optimizacija mrežnog steka (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Svrha |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Briše keš DNS rezolvera |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Vraća standardno RFC 1323 TCP skaliranje prozora |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Omogućava skaliranje na strani prijema za distribuciju obrade mrežnih paketa kroz CPU jezgre |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Onemogućuje spajanje segmenata prijema na NIC-ovima kako bi se smanjilo kašnjenje u igrama u realnom vremenu |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Onemogućuje eksplicitno obavještenje o zagušenju kako bi spriječio ispuštene pakete na starijim ruterima |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Briše tablicu lokalnog protokola za razrješenje adresa |
| `winsock-reset` | `netsh winsock reset` | Vraća katalog Windows mrežnih utičnica u čisto stanje |

#### C. Upravljanje napajanjem i pohranom (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Svrha |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Aktivira standardnu ​​Windows shemu napajanja visokih performansi |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Otključava i aktivira Windows Ultimate Performance shemu napajanja |
| `hibernate-off` | `powercfg /hibernate off` | Onemogućuje hibernaciju i briše `hiberfil.sys`, oslobađajući prostor na disku |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Zaustavlja ažuriranje vremenskih oznaka NTFS posljednjeg pristupa pri svakom čitanju datoteke |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Osigurava da su TRIM obavještenja aktivna za nivelaciju zdravlja i trošenja SSD-a |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Kreira punu tačku za vraćanje sistema Windows prije promjena |

#### D. Sistemska telemetrija i sinhronizacija u pozadini
| Tweak ID | Hive & Path | Value | Svrha |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Onemogućuje Windows vremensku liniju i otpremanje historije aktivnosti |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Ograničava optimizaciju isporuke Windows Update samo na lokalni LAN (nema P2P otpremanja na internet) |

---

## Nesigurna podešavanja na blokiranoj listi

Synchro namjerno blokira nekoliko uobičajenih "agresivnih podešavanja" pronađenih u skriptama trećih strana koje ugrožavaju integritet sistema ili kompatibilnost protiv varanja:

- **Onemogućavanje integriteta memorije (HVCI) / Sigurnost zasnovana na virtuelizaciji**: Odbijeno.
- **Potpuno onemogućavanje Windows zaštitnika/zaštitnog zida**: Odbijeno.
- **Onemogućavanje datoteke stranične memorije (`pagefile.sys`)**: Odbijeno (uzrokuje rušenje igre na skokovima memorije).
- **Onemogućavanje Dynamic Tick / Invarijantnog TSC manipulisanja**: Odbijeno (može desinhronizovati satove i aktivirati heuristiku protiv varanja).
- **Maksimalno hakovanje uslužnog programa MSI (Message Signaled Interrupts)**: Odbijeno (rizik od GPU-a / plavi ekrani).
