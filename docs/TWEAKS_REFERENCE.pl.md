# Synchro Nova — informacje o modyfikacjach i ulepszeniach systemu

Ten dokument zawiera pełny, przejrzysty opis wszystkich ustawień systemowych, kluczy rejestru i poleceń wykonywanych przez Synchro Nova. Wszystkie działania mają charakter ściśle lokalny, deterministyczny i są archiwizowane przed wykonaniem.

---

## Gwarancja bezpieczeństwa i wycofania

1. **Automatyczna kopia zapasowa przed wykonaniem**:
Przed modyfikacją dowolnego klucza rejestru Synchro Nova tworzy punkt przywracania systemu i zapisuje bieżący stan wszystkich kluczy docelowych w pliku kopii zapasowej `.reg` w `%APPDATA%\synchro\backups\tweak_safety_backup_<znacznik czasu>.reg`.
2. **1-kliknięciem Przywróć**:
Kliknięcie **"Откатить изменения"** (Przywróć) w aplikacji przywraca poprzednią migawkę `.reg` poprzez Windows `reg.exe` i ponownie odczytuje bieżące wartości.
3. **Wzmocnione wykonywanie poleceń**:
Wszystkie narzędzia administracyjne (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) są ściśle powiązane z `%SystemRoot%\System32` (zapobiegając przejęciu podwyższonej ścieżki PATH) i działają z nieblokującymi limitami czasu watchdoga (5s dla narzędzi, 25s dla przywracania systemu).
4. **Twarda lista blokowania i zabezpieczenie przed oszustwami**:
Niebezpieczne operacje (wyłączanie integralności pamięci/HVCI, poprawianie liczników czasu jądra, wyłączanie pliku stronicowania lub stosowanie dowolnych nadpisań sterowników) są trwale blokowane w silniku. Zerowe wstrzykiwanie procesów lub przechwytywanie pamięci zapewnia 100% zgodność z Easy Anti-Cheat, BattlEye, Vanguard i VAC.

---

## Pełny podział poprawek

### 1. Ulepszenia na poziomie użytkownika (użytkownik standardowy — nie jest wymagany administrator)

Te poprawki modyfikują jedynie gałąź „HKEY_CURRENT_USER”. Nie wymagają podwyższonych uprawnień i wpływają tylko na aktywny profil użytkownika.

| Tweak ID | Hive & Path | Value / Command | Zamiar |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Zapobiega wyświetlaniu wyskakujących okienek Xbox Game Bar podczas uruchamiania gry |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Wyłącza przyspieszanie myszy w systemie Windows (surowe wejście liniowe) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Wyłącza identyfikator reklamowy systemu Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Wyłącza spersonalizowane wskazówki oparte na danych diagnostycznych |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Utrzymuje schowek wyłącznie w lokalnej pamięci RAM, wyłącza synchronizację z chmurą |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Wyłącza przezroczystość okna DWM, aby oszczędzać pamięć GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Konfiguruje Eksploratora Windows w celu uzyskania najlepszej wydajności wizualnej |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Usuwa sztuczne opóźnienie uruchamiania pulpitu Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Zmniejsza opóźnienie animacji menu interfejsu użytkownika do 10 ms |

---

### 2. Ulepszenia na poziomie systemu (wymagane uprawnienia administratora)

Te poprawki konfigurują zasady ogólnosystemowe (`HKEY_LOCAL_MACHINE`) lub wywołują standardowe narzędzia administracyjne systemu Windows (`powercfg`, `netsh`, `fsutil`. Synchro sprawdza wysokość przed ich wykonaniem.

#### A. Planowanie gier i multimediów (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Zamiar |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Wyłącza przechwytywanie wideo DVR z gry w tle w celu zwolnienia kodera GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Wyłącza mechanizm ograniczania przepustowości sieci Windows podczas zadań o wysokim priorytecie |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Rezerwuje 90% czasu procesora dla gier pierwszoplanowych (domyślnie jest to 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Podwyższa priorytet zadania harmonogramu multimediów dla aktywnych gier |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Umożliwia przyspieszane sprzętowo planowanie GPU (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Wyłącza nakładki wielopłaszczyznowe (rozwiązuje problemy z czarnym ekranem/zacinaniem się na niektórych procesorach graficznych) |

#### B. Optymalizacja stosu sieciowego (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Zamiar |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Czyści pamięć podręczną modułu rozpoznawania nazw DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Przywraca standardowe skalowanie okien TCP RFC 1323 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Umożliwia skalowanie po stronie odbioru w celu dystrybucji przetwarzania pakietów sieciowych pomiędzy rdzeniami procesora |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Wyłącza łączenie segmentów odbieranych na kartach sieciowych, aby zmniejszyć opóźnienia w grach w czasie rzeczywistym |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Wyłącza jawne powiadomienie o przeciążeniu, aby zapobiec upuszczaniu pakietów na starszych routerach |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Czyści lokalną tabelę protokołu rozpoznawania adresów |
| `winsock-reset` | `netsh winsock reset` | Przywraca katalog gniazd sieciowych systemu Windows do stanu czystego |

#### C. Zarządzanie energią i pamięcią masową („powercfg”, „fsutil”)
| Tweak ID | Tool / Command | Zamiar |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Aktywuje standardowy schemat zasilania systemu Windows o wysokiej wydajności |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Odblokowuje i aktywuje schemat zasilania Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Wyłącza hibernację i usuwa `hiberfil.sys`, zwalniając miejsce na dysku |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Zatrzymuje aktualizację znaczników czasu ostatniego dostępu NTFS przy każdym odczycie pliku |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Zapewnia, że ​​powiadomienia TRIM są aktywne w celu wyrównywania stanu dysku SSD i jego zużycia |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Tworzy pełny punkt przywracania systemu Windows przed zmianami |

#### D. Telemetria systemowa i synchronizacja w tle
| Tweak ID | Hive & Path | Value | Zamiar |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Wyłącza przesyłanie osi czasu systemu Windows i historii aktywności |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Ogranicza optymalizację dostarczania Windows Update tylko do lokalnej sieci LAN (bez przesyłania P2P do Internetu) |

---

## Lista zablokowanych niebezpiecznych poprawek

Synchro celowo blokuje kilka typowych „agresywnych poprawek” występujących w skryptach innych firm, które zagrażają integralności systemu lub zgodności z zabezpieczeniami:

- **Wyłączanie integralności pamięci (HVCI) / zabezpieczeń opartych na wirtualizacji**: Odrzucone.
- **Całkowite wyłączenie programu Windows Defender/Zapory sieciowej**: Odrzucone.
- **Wyłączenie pliku stronicowania (`pagefile.sys`)**: Odrzucone (powoduje awarię gry w przypadku skoków pamięci).
- **Wyłączenie dynamicznego tika / niezmiennej manipulacji TSC**: Odrzucone (może desynchronizować zegary i wyłączać heurystykę zapobiegającą oszustwom).
- **Masowe hakowanie narzędzia MSI (Message Signaled Interrupts)**: Odrzucone (ryzyko zablokowania procesora graficznego / niebieskich ekranów).
