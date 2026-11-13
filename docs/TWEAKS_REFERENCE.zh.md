# Synchro Nova — 系统修改和调整参考

本文档提供了 Synchro Nova 执行的每个系统设置、注册表项和命令的完整、透明的详细信息。所有操作都是严格本地的、确定性的，并在执行前进行备份。

---

## 安全及回滚保证

1. **自动执行前备份**:
在修改任何注册表项之前，Synchro Nova 会创建一个系统还原点，并将所有目标项的当前状态保存到“%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg”中的“.reg”备份文件中。
2. **一键回滚**:
单击应用程序中的 **“Откатить изменения”**（回滚）可通过 Windows `reg.exe` 恢复之前的 `.reg` 快照并重新读取当前值。
3. **强化命令执行**:
所有管理工具（“netsh”、“powercfg”、“sc”、“bcdedit”、“reg”、“powershell”）均严格从“%SystemRoot%\System32”解析（防止提升的路径劫持），并以非阻塞看门狗超时运行（实用程序为 5 秒，系统恢复为 25 秒）。
4. **硬阻止列表和防作弊保险箱**:
Unsafe operations (disabling memory integrity/HVCI, patching kernel timers, disabling the paging file, or applying arbitrary driver overrides) are permanently blocked in the engine.零进程注入或内存挂钩可确保 100% 符合 Easy Anti-Cheat、BattlEye、Vanguard 和 VAC。

---

## 完整的调整细目

### 1. 用户级别调整（标准用户 - 无需管理员）

这些调整仅修改“HKEY_CURRENT_USER”配置单元。它们不需要提升权限，并且仅影响活动用户配置文件。

| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | 防止游戏启动时弹出 Xbox 游戏栏 |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | 禁用 Windows 鼠标加速（原始线性输入） |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | 禁用 Windows 广告标识符 |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | 禁用基于诊断数据的个性化提示 |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | 将剪贴板严格保留在本地 RAM 中，禁用云同步 |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | 禁用 DWM 窗口透明度以节省 GPU 内存 |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | 配置 Windows 资源管理器以获得最佳视觉性能 |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | 消除人为的 Windows 桌面启动延迟 |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | 将 UI 菜单动画延迟减少至 10 毫秒 |

---

### 2. 系统级调整（需要管理员权限）

这些调整配置系统范围的策略（“HKEY_LOCAL_MACHINE”）或调用标准 Windows 管理工具（“powercfg”、“netsh”、“fsutil”）。 Synchro 在执行这些操作之前会检查海拔高度。

#### A. 游戏和多媒体调度 (MMCSS)
| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | 禁用后台游戏 DVR 视频捕获以释放 GPU 编码器 |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | 在高优先级任务期间禁用 Windows 网络限制机制 |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | 为前台游戏保留 90% 的 CPU 时间（默认为 80%） |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | 提高活动游戏的多媒体调度程序任务优先级 |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | 启用硬件加速 GPU 调度 (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | 禁用多平面叠加（解决某些 GPU 上的黑屏/卡顿问题） |

#### B. 网络堆栈优化（`netsh`、`ipconfig`）
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | 清除 DNS 解析器缓存 |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | 恢复标准 RFC 1323 TCP 窗口缩放 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | 启用接收端扩展以跨 CPU 内核分配网络数据包处理 |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | 禁用 NIC 上的接收段合并以减少实时游戏中的延迟 |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | 禁用显式拥塞通知以防止旧路由器上丢弃数据包 |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | 清除本地地址解析协议表 |
| `winsock-reset` | `netsh winsock reset` | 将 Windows 网络套接字目录恢复到干净状态 |

#### C. 电源和存储管理（`powercfg`、`fsutil`）
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | 激活标准 Windows 高性能电源使用方案 |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | 解锁并激活 Windows Ultimate Performance 电源方案 |
| `hibernate-off` | `powercfg /hibernate off` | 禁用休眠并删除“hiberfil.sys”，释放磁盘空间 |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | 停止在读取每个文件时更新 NTFS 上次访问时间戳 |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | 确保 TRIM 通知针对 SSD 运行状况和磨损均衡处于活动状态 |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | 在更改之前创建完整的 Windows 系统还原点 |

#### D. 系统遥测和后台同步
| Tweak ID | Hive & Path | Value | 目的 |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | 禁用 Windows 时间轴和活动历史记录上传 |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | 将 Windows 更新交付优化仅限于本地 LAN（无 P2P 上传到互联网） |

---

## 列入黑名单的不安全调整

Synchro 有意阻止第三方脚本中发现的几种常见的“激进调整”，这些调整会损害系统完整性或反作弊兼容性：

- **禁用内存完整性 (HVCI)/基于虚拟化的安全性**：已拒绝。
- **完全禁用 Windows Defender / 防火墙**：已拒绝。
- **禁用分页文件（`pagefile.sys`）**：被拒绝（导致游戏因内存峰值而崩溃）。
- **禁用动态刻度/不变 TSC 篡改**：拒绝（可以使时钟不同步并触发反作弊启发法）。
- **批量 MSI（消息信号中断）实用程序黑客攻击**：拒绝（GPU 变砖/蓝屏的风险）。
