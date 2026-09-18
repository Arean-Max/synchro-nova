# Synchro Nova — System Modifications & Tweaks Reference

This document provides a full, transparent breakdown of every system setting, registry key, and command executed by Synchro Nova. All actions are strictly local, deterministic, and backed up before execution.

---

## Safety & Rollback Guarantee

1. **Automated Pre-Execution Backup**:
   Before modifying any registry key, Synchro Nova dumps the current state of all target keys to a `.reg` file in `%APPDATA%\app.synchro.performance\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-Click Rollback**:
   Clicking **"Откатить изменения"** (Rollback) in the application restores the previous `.reg` snapshot via Windows `reg import` and re-reads current values.
3. **Audit Trail**:
   All operations are appended to `%APPDATA%\app.synchro.performance\tweaks_audit.log` with timestamps and result codes.
4. **Hard Blocklist**:
   Unsafe operations (disabling memory integrity/HVCI, patching kernel timers, disabling the paging file, or applying arbitrary driver overrides) are permanently blocked in the engine.

---

## Complete Tweaks Breakdown

### 1. User-Level Tweaks (Standard User — No Admin Required)

These tweaks only modify the `HKEY_CURRENT_USER` hive. They do not require elevated privileges and affect only the active user profile.

| Tweak ID | Hive & Path | Value / Command | Purpose |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Prevents Xbox Game Bar popup on game launch |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Disables Windows mouse acceleration (raw linear input) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Disables Windows advertising identifier |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Disables diagnostic data-based personalized tips |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Keeps clipboard strictly in local RAM, disables cloud sync |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Disables DWM window transparency to save GPU memory |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Configures Windows Explorer for best visual performance |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Removes artificial Windows desktop startup delay |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reduces UI menu animation delay to 10ms |

---

### 2. System-Level Tweaks (Requires Administrator Privileges)

These tweaks configure system-wide policies (`HKEY_LOCAL_MACHINE`) or call standard Windows administrative tools (`powercfg`, `netsh`, `fsutil`). Synchro checks elevation before executing these.

#### A. Gaming & Multimedia Scheduling (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Purpose |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Disables background Game DVR video capture to free GPU encoder |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Disables Windows network throttling mechanism during high-priority tasks |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserves 90% of CPU time for foreground games (default is 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Elevates multimedia scheduler task priority for active games |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Enables Hardware-Accelerated GPU Scheduling (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Disables Multi-Plane Overlays (resolves black screen / stutter issues on some GPUs) |

#### B. Network Stack Optimization (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Purpose |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Clears DNS resolver cache |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Restores standard RFC 1323 TCP window scaling |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Enables Receive-Side Scaling to distribute network packet processing across CPU cores |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Disables Receive Segment Coalescing on NICs to reduce latency in real-time gaming |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Disables Explicit Congestion Notification to prevent dropped packets on older routers |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Clears local Address Resolution Protocol table |
| `winsock-reset` | `netsh winsock reset` | Restores Windows network sockets catalog to clean state |

#### C. Power & Storage Management (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Purpose |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Activates standard Windows High Performance power scheme |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Unlocks and activates Windows Ultimate Performance power scheme |
| `hibernate-off` | `powercfg /hibernate off` | Disables hibernation and deletes `hiberfil.sys`, freeing disk space |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Stops updating NTFS last-access timestamps on every file read |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Ensures TRIM notifications are active for SSD health and wear leveling |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Creates a full Windows System Restore point before changes |

#### D. System Telemetry & Background Sync
| Tweak ID | Hive & Path | Value | Purpose |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Disables Windows Timeline and activity history uploads |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Restricts Windows Update Delivery Optimization to local LAN only (no P2P uploads to internet) |

---

## Blocklisted Unsafe Tweaks

Synchro intentionally blocks several common "aggressive tweaks" found in third-party scripts that compromise system integrity or anti-cheat compatibility:

- **Disabling Memory Integrity (HVCI) / Virtualization-Based Security**: Rejected.
- **Disabling Windows Defender / Firewall entirely**: Rejected.
- **Disabling Paging File (`pagefile.sys`)**: Rejected (causes game crashes on memory spikes).
- **Disabling Dynamic Tick / Invariant TSC tampering**: Rejected (can desynchronize clocks and trip anti-cheat heuristics).
- **Bulk MSI (Message Signaled Interrupts) utility hacking**: Rejected (risk of GPU bricking / blue screens).
