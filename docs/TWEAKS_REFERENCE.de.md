# Synchro Nova – Referenz zu Systemmodifikationen und -optimierungen

Dieses Dokument bietet eine vollständige, transparente Aufschlüsselung aller Systemeinstellungen, Registrierungsschlüssel und Befehle, die von Synchro Nova ausgeführt werden. Alle Aktionen sind streng lokal, deterministisch und werden vor der Ausführung gesichert.

---

## Sicherheit und Rollback-Garantie

1. **Automatisierte Sicherung vor der Ausführung**:
Bevor ein Registrierungsschlüssel geändert wird, erstellt Synchro Nova einen Systemwiederherstellungspunkt und speichert den aktuellen Status aller Zielschlüssel in einer „.reg“-Sicherungsdatei unter „%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg“.
2. **1-Klick-Rollback**:
Wenn Sie in der Anwendung auf „Rollback“ (Rollback) klicken, wird der vorherige „.reg“-Snapshot über Windows „reg.exe“ wiederhergestellt und die aktuellen Werte werden erneut gelesen.
3. **Gehärtete Befehlsausführung**:
Alle Verwaltungstools („netsh“, „powercfg“, „sc“, „bcdedit“, „reg“, „powershell“) werden strikt von „%SystemRoot%\System32“ aufgelöst (um erhöhtes PATH-Hijacking zu verhindern) und mit nicht blockierenden Watchdog-Timeouts ausgeführt (5 Sekunden für Dienstprogramme, 25 Sekunden für die Systemwiederherstellung).
4. **Hard-Blocklist und Anti-Cheat-sicher**:
Unsichere Vorgänge (Deaktivieren der Speicherintegrität/HVCI, Patchen von Kernel-Timern, Deaktivieren der Auslagerungsdatei oder Anwenden willkürlicher Treiberüberschreibungen) werden in der Engine dauerhaft blockiert. Keine Prozessinjektion oder Speicher-Hooking gewährleistet 100 % Konformität mit Easy Anti-Cheat, BattlEye, Vanguard und VAC.

---

## Vollständige Aufschlüsselung der Optimierungen

### 1. Optimierungen auf Benutzerebene (Standardbenutzer – kein Administrator erforderlich)

Diese Optimierungen ändern nur die Struktur „HKEY_CURRENT_USER“. Sie erfordern keine erhöhten Berechtigungen und wirken sich nur auf das aktive Benutzerprofil aus.

| Tweak ID | Hive & Path | Value / Command | Zweck |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Verhindert das Popup der Xbox Game Bar beim Spielstart |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Deaktiviert die Windows-Mausbeschleunigung (rohe lineare Eingabe) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Deaktiviert die Windows-Werbekennung |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Deaktiviert diagnosedatenbasierte personalisierte Tipps |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Behält die Zwischenablage ausschließlich im lokalen RAM und deaktiviert die Cloud-Synchronisierung |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Deaktiviert die DWM-Fenstertransparenz, um GPU-Speicher zu sparen |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Konfiguriert Windows Explorer für beste visuelle Leistung |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Entfernt die künstliche Startverzögerung des Windows-Desktops |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reduziert die Verzögerung der UI-Menüanimation auf 10 ms |

---

### 2. Optimierungen auf Systemebene (Administratorrechte erforderlich)

Diese Optimierungen konfigurieren systemweite Richtlinien (`HKEY_LOCAL_MACHINE`) oder rufen Standard-Windows-Verwaltungstools auf (`powercfg`, `netsh`, `fsutil`). Synchro prüft die Höhe, bevor es diese ausführt.

#### A. Gaming- und Multimedia-Planung (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Zweck |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Deaktiviert die Hintergrundaufnahme von Spiel-DVR-Videos, um den GPU-Encoder freizugeben |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Deaktiviert den Windows-Netzwerkdrosselungsmechanismus bei Aufgaben mit hoher Priorität |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserviert 90 % der CPU-Zeit für Vordergrundspiele (Standard ist 80 %). |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Erhöht die Aufgabenpriorität des Multimedia-Planers für aktive Spiele |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Ermöglicht hardwarebeschleunigte GPU-Planung (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Deaktiviert Multi-Plane-Overlays (behebt Probleme mit schwarzem Bildschirm/Ruckeln bei einigen GPUs) |

#### B. Netzwerk-Stack-Optimierung („netsh“, „ipconfig“)
| Tweak ID | Tool / Command | Zweck |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Löscht den DNS-Resolver-Cache |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Stellt die standardmäßige RFC 1323 TCP-Fensterskalierung wieder her |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Ermöglicht die empfangsseitige Skalierung, um die Netzwerkpaketverarbeitung auf CPU-Kerne zu verteilen |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Deaktiviert die Zusammenführung von Empfangssegmenten auf Netzwerkkarten, um die Latenz bei Echtzeitspielen zu reduzieren |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Deaktiviert die explizite Überlastungsbenachrichtigung, um verlorene Pakete auf älteren Routern zu verhindern |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Löscht die lokale Address Resolution Protocol-Tabelle |
| `winsock-reset` | `netsh winsock reset` | Stellt den Windows-Netzwerk-Socket-Katalog in den sauberen Zustand zurück |

#### C. Energie- und Speicherverwaltung („powercfg“, „fsutil“)
| Tweak ID | Tool / Command | Zweck |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Aktiviert das Standard-Windows-Hochleistungs-Energieschema |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Entsperrt und aktiviert das Windows Ultimate Performance-Energieschema |
| `hibernate-off` | `powercfg /hibernate off` | Deaktiviert den Ruhezustand und löscht „hiberfil.sys“, wodurch Speicherplatz frei wird |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Stoppt die Aktualisierung der NTFS-Zeitstempel für den letzten Zugriff bei jedem Dateilesen |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Stellt sicher, dass TRIM-Benachrichtigungen für SSD-Zustand und Verschleißausgleich aktiv sind |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Erstellt vor Änderungen einen vollständigen Windows-Systemwiederherstellungspunkt |

#### D. Systemtelemetrie und Hintergrundsynchronisierung
| Tweak ID | Hive & Path | Value | Zweck |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Deaktiviert das Hochladen der Windows-Zeitleiste und des Aktivitätsverlaufs |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Beschränkt die Optimierung der Windows Update-Bereitstellung nur auf das lokale LAN (keine P2P-Uploads ins Internet) |

---

## Unsichere Tweaks auf der Blockliste

Synchro blockiert absichtlich mehrere häufige „aggressive Optimierungen“, die in Skripten von Drittanbietern zu finden sind und die Systemintegrität oder Anti-Cheat-Kompatibilität gefährden:

- **Deaktivieren der Speicherintegrität (HVCI)/virtualisierungsbasierte Sicherheit**: Abgelehnt.
- **Windows Defender/Firewall vollständig deaktivieren**: Abgelehnt.
- **Deaktivieren der Auslagerungsdatei („pagefile.sys“)**: Abgelehnt (verursacht Spielabstürze bei Speicherspitzen).
- **Dynamische Tick-/invariante TSC-Manipulation deaktivieren**: Abgelehnt (kann Uhren desynchronisieren und Anti-Cheat-Heuristiken auslösen).
- **Bulk MSI (Message Signaled Interrupts) Utility-Hacking**: Abgelehnt (Risiko von GPU-Bricking/Bluescreens).
