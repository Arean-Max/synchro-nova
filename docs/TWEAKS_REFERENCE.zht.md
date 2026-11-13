# Synchro Nova — 系統修改與調整參考

本文檔提供了 Synchro Nova 執行的每個系統設定、登錄項目和命令的完整、透明的詳細資訊。所有操作都是嚴格本地的、確定性的，並在執行前進行備份。

---

## 安全及回滾保證

1. **自動執行前備份**:
在修改任何登錄項目之前，Synchro Nova 會建立一個系統還原點，並將所有目標項目的目前狀態儲存到「%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg」中的「.reg」備份檔案中。
2. **一鍵回滾**:
點擊應用程式中的 **“Откатить изменения”**（回滾）可透過 Windows `reg.exe` 恢復先前的 `.reg` 快照並重新讀取目前值。
3. **強化命令執行**:
所有管理工具（“netsh”、“powercfg”、“sc”、“bcdedit”、“reg”、“powershell”）均嚴格從“%SystemRoot%\System32”解析（防止提升的路徑劫持），並以非阻塞看門狗逾時運行（實用程式為 5 秒，系統恢復為 25 秒）。
4. **硬封鎖清單和防作弊保險箱**:
不安全操作（停用記憶體完整性/HVCI、修補核心計時器、停用分頁檔案或應用任意驅動程式覆蓋）將在引擎中永久阻止。零進程注入或記憶體掛鉤可確保 100% 符合 Easy Anti-Cheat、BattlEye、Vanguard 和 VAC。

---

## 完整的調整細目

### 1. 使用者等級調整（標準使用者 - 無需管理員）

這些調整僅修改“HKEY_CURRENT_USER”配置單元。它們不需要提升權限，並且僅影響活動使用者設定檔。

| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | 防止遊戲啟動時彈出 Xbox 遊戲欄 |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | 停用 Windows 滑鼠加速（原始線性輸入） |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | 停用 Windows 廣告標識符 |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | 停用基於診斷數據的個人化提示 |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | 將剪貼簿嚴格保留在本地 RAM 中，停用雲端同步 |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | 停用 DWM 視窗透明度以節省 GPU 內存 |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | 配置 Windows 資源管理器以獲得最佳視覺效能 |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | 消除人為的 Windows 桌面啟動延遲 |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | 將 UI 選單動畫延遲減少至 10 毫秒 |

---

### 2. 系統級調整（需管理員權限）

這些調整配置系統範圍的策略（「HKEY_LOCAL_MACHINE」）或呼叫標準 Windows 管理工具（「powercfg」、「netsh」、「fsutil」）。 Synchro 在執行這些操作之前會檢查海拔高度。

#### A. 遊戲與多媒體調度 (MMCSS)
| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | 停用後台遊戲 DVR 視訊擷取以釋放 GPU 編碼器 |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | 在高優先權任務期間停用 Windows 網路限制機制 |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | 為前台遊戲保留 90% 的 CPU 時間（預設為 80%） |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | 提高活動遊戲的多媒體調度程序任務優先級 |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | 啟用硬體加速 GPU 調度 (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | 停用多平面疊加（解決某些 GPU 上的黑屏/卡頓問題） |

#### B. 網路堆疊最佳化（`netsh`、`ipconfig`）
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | 清除 DNS 解析器快取 |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | 恢復標準 RFC 1323 TCP 視窗縮放 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | 啟用接收端擴充以跨 CPU 核心分配網路封包處理 |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | 停用 NIC 上的接收段合併以減少即時遊戲中的延遲 |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | 停用明確擁塞通知以防止舊路由器上丟棄資料包 |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | 清除本機位址解析協定表 |
| `winsock-reset` | `netsh winsock reset` | 將 Windows 網路套接字目錄還原到乾淨狀態 |

#### C. 電源供應器與儲存管理（`powercfg`、`fsutil`）
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | 啟動標準 Windows 高效能電源使用方案 |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | 解鎖並啟動 Windows Ultimate Performance 電源方案 |
| `hibernate-off` | `powercfg /hibernate off` | 停用休眠並刪除“hiberfil.sys”，釋放磁碟空間 |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | 停止在讀取每個檔案時更新 NTFS 上次存取時間戳 |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | 確保 TRIM 通知針對 SSD 運作狀況和磨損均衡處於活動狀態 |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | 在變更之前建立完整的 Windows 系統還原點 |

#### D. 系統遙測與背景同步
| Tweak ID | Hive & Path | Value | 目的 |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | 停用 Windows 時間軸和活動歷史記錄上傳 |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | 將 Windows 更新交付最佳化僅限於本機 LAN（無 P2P 上傳至網際網路） |

---

## 列入黑名單的不安全調整

Synchro 有意阻止第三方腳本中發現的幾種常見的“激進調整”，這些調整會損害系統完整性或反作弊相容性：

- **停用記憶體完整性 (HVCI)/基於虛擬化的安全性**：已拒絕。
- **完全停用 Windows Defender / 防火牆**：已拒絕。
- **停用分頁檔案（`pagefile.sys`）**：被拒絕（導致遊戲因記憶體峰值而崩潰）。
- **停用動態刻度/不變 TSC 竄改**：拒絕（可以使時鐘不同步並觸發反作弊啟發法）。
- **批次 MSI（訊息訊號中斷）實用程式駭客攻擊**：拒絕（GPU 變磚/藍螢幕的風險）。
