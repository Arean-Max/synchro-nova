# Synchro Nova — システムの変更と微調整のリファレンス

このドキュメントでは、すべてのシステム設定、レジストリ キー、および Synchro Nova によって実行されるコマンドの詳細をわかりやすく説明します。すべてのアクションは厳密にローカルで決定的であり、実行前にバックアップされます。

---

## 安全性とロールバック保証

1. **自動実行前バックアップ**:
レジストリ キーを変更する前に、Synchro Nova はシステム復元ポイントを作成し、すべてのターゲット キーの現在の状態を `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg` の `.reg` バックアップ ファイルに保存します。
2. **1-クリックロールバック**:
アプリケーションで **「Откатить изменения」** (ロールバック) をクリックすると、Windows の `reg.exe` を介して以前の `.reg` スナップショットが復元され、現在の値が再読み取りされます。
3. **強化されたコマンドの実行**:
すべての管理ツール (`netsh`、`powercfg`、`sc`、`bcdedit`、`reg`、`powershell`) は `%SystemRoot%\System32` から厳密に解決され (昇格された PATH ハイジャックを防止)、ノンブロッキング ウォッチドッグ タイムアウト (ユーティリティの場合は 5 秒、システムの復元の場合は 25 秒) で実行されます。
4. **ハードブロックリストとアンチチートセーフ**:
安全でない操作 (メモリ整合性/HVCI の無効化、カーネル タイマーのパッチ適用、ページング ファイルの無効化、または任意のドライバー オーバーライドの適用) は、エンジン内で永続的にブロックされます。プロセス インジェクションやメモリ フックがゼロなので、Easy Anti-Cheat、BattlEye、Vanguard、VAC への 100% の準拠が保証されます。

---

## 完全な調整の内訳

### 1. ユーザーレベルの調整 (標準ユーザー - 管理者は不要)

これらの微調整は、`HKEY_CURRENT_USER` ハイブのみを変更します。これらは昇格された特権を必要とせず、アクティブなユーザー プロファイルにのみ影響します。

| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | ゲーム起動時の Xbox ゲーム バーのポップアップを防止します |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Windows マウス アクセラレーションを無効にします (生のリニア入力) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Windows 広告識別子を無効にします |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | 診断データに基づいたパーソナライズされたヒントを無効にします |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | クリップボードをローカル RAM に厳密に保持し、クラウド同期を無効にします |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | DWM ウィンドウの透明度を無効にして GPU メモリを節約します |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | 最高の視覚パフォーマンスを実現するために Windows エクスプローラーを構成します |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Windows デスクトップの起動の人工的な遅延を解消します。 |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | UI メニューのアニメーション遅延を 10 ミリ秒に短縮します。 |

---

### 2. システムレベルの調整 (管理者権限が必要)

これらの調整により、システム全体のポリシー (`HKEY_LOCAL_MACHINE`) が設定されるか、標準の Windows 管理ツール (`powercfg`、`netsh`、`fsutil`) が呼び出されます。 Synchro はこれらを実行する前に標高をチェックします。

#### A. ゲームとマルチメディアのスケジューリング (MMCSS)
| Tweak ID | Hive & Path | Value / Command | 目的 |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | バックグラウンドのゲーム DVR ビデオ キャプチャを無効にして、GPU エンコーダを解放します |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | 優先度の高いタスク中に Windows ネットワーク スロットル メカニズムを無効にします |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | CPU 時間の 90% をフォアグラウンド ゲーム用に予約します (デフォルトは 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | アクティブなゲームのマルチメディア スケジューラ タスクの優先順位を引き上げます |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | ハードウェア アクセラレーションによる GPU スケジューリング (WDDM 2.7+) を有効にします。 |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | マルチプレーン オーバーレイを無効にします (一部の GPU での黒い画面/途切れの問題を解決します) |

#### B. ネットワーク スタックの最適化 (`netsh`、`ipconfig`)
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | DNSリゾルバーキャッシュをクリアします |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | 標準の RFC 1323 TCP ウィンドウ スケーリングを復元します |
| `rss-on` | `netsh interface tcp set global rss=enabled` | 受信側スケーリングを有効にしてネットワーク パケット処理を CPU コア全体に分散します |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | NIC での受信セグメント結合を無効にして、リアルタイム ゲームの遅延を削減します。 |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | 明示的な輻輳通知を無効にして、古いルータでのパケットのドロップを防止します |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | ローカルアドレス解決プロトコルテーブルをクリアします |
| `winsock-reset` | `netsh winsock reset` | Windows ネットワーク ソケット カタログをクリーンな状態に復元します |

#### C. 電源とストレージの管理 (`powercfg`、`fsutil`)
| Tweak ID | Tool / Command | 目的 |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | 標準の Windows ハイパフォーマンス電源設定をアクティブ化します。 |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Windows Ultimate Performance 電源スキームのロックを解除してアクティブ化します |
| `hibernate-off` | `powercfg /hibernate off` | ハイバネーションを無効にして「hiberfil.sys」を削除し、ディスク領域を解放します。 |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | ファイルが読み取られるたびに NTFS 最終アクセス タイムスタンプの更新を停止します |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | SSD の健全性とウェアレベリングのために TRIM 通知がアクティブであることを確認します |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | 変更前に完全な Windows システム復元ポイントを作成します |

#### D. システム テレメトリとバックグラウンド同期
| Tweak ID | Hive & Path | Value | 目的 |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Windows タイムラインとアクティビティ履歴のアップロードを無効にします |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Windows Update 配信の最適化をローカル LAN のみに制限します (インターネットへの P2P アップロードは禁止) |

---

## ブロックリストに登録された安全でない調整

Synchro は、システムの整合性やチート対策の互換性を損なう、サードパーティのスクリプトに見られるいくつかの一般的な「積極的な調整」を意図的にブロックします。

- **メモリ整合性 (HVCI) / 仮想化ベースのセキュリティの無効化**: 拒否されました。
- **Windows Defender / ファイアウォールを完全に無効にする**: 拒否されました。
- **ページング ファイル (`pagefile.sys`) の無効化**: 拒否されました (メモリの急増によりゲームがクラッシュします)。
- **Dynamic Tick / Invariant TSC 改ざんの無効化**: 拒否されました (クロックが非同期になり、アンチチート ヒューリスティックがトリップする可能性があります)。
- **一括 MSI (メッセージ シグナル割り込み) ユーティリティ ハッキング**: 拒否されました (GPU ブリック/ブルー スクリーンのリスク)。
