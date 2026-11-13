# 싱크로 노바 — 시스템 수정 및 조정 참조

이 문서는 모든 시스템 설정, 레지스트리 키, Synro Nova에서 실행되는 명령에 대한 완전하고 투명한 분석을 제공합니다. 모든 작업은 엄격하게 로컬이고 결정적이며 실행 전에 백업됩니다.

---

## 안전 및 롤백 보장

1. **자동화된 사전 실행 백업**:
레지스트리 키를 수정하기 전에 Syncro Nova는 시스템 복원 지점을 생성하고 모든 대상 키의 현재 상태를 `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`의 `.reg` 백업 파일에 저장합니다.
2. **1-클릭 롤백**:
응용 프로그램에서 **"Откатить изменения"**(롤백)를 클릭하면 Windows `reg.exe`를 통해 이전 `.reg` 스냅샷을 복원하고 현재 값을 다시 읽습니다.
3. **강화된 명령 실행**:
모든 관리 도구(`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`)는 `%SystemRoot%\System32`(상승된 PATH 하이재킹 방지)에서 엄격하게 확인되며 비차단 감시 시간 제한(유틸리티의 경우 5초, 시스템 복원의 경우 25초)으로 실행됩니다.
4. **하드 차단 목록 및 치트 방지 안전 장치**:
안전하지 않은 작업(메모리 무결성/HVCI 비활성화, 커널 타이머 패치, 페이징 파일 비활성화 또는 임의 드라이버 재정의 적용)은 엔진에서 영구적으로 차단됩니다. 제로 프로세스 주입 또는 메모리 후킹은 Easy Anti-Cheat, BattlEye, Vanguard 및 VAC를 100% 준수하도록 보장합니다.

---

## 완전한 조정 분석

### 1. 사용자 수준 조정(표준 사용자 - 관리자 필요 없음)

이러한 조정은 `HKEY_CURRENT_USER` 하이브만 수정합니다. 높은 권한이 필요하지 않으며 활성 사용자 프로필에만 영향을 미칩니다.

| Tweak ID | Hive & Path | Value / Command | 목적 |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | 게임 시작 시 Xbox Game Bar 팝업을 방지합니다. |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Windows 마우스 가속을 비활성화합니다(원시 선형 입력). |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Windows 광고 식별자를 비활성화합니다. |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | 진단 데이터 기반 맞춤형 팁을 비활성화합니다. |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | 클립보드를 로컬 RAM에만 유지하고 클라우드 동기화를 비활성화합니다. |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | GPU 메모리를 절약하기 위해 DWM 창 투명도를 비활성화합니다. |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | 최고의 시각적 성능을 위해 Windows 탐색기를 구성합니다. |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | 인위적인 Windows 데스크탑 시작 지연을 제거합니다. |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | UI 메뉴 애니메이션 지연을 10ms로 줄입니다. |

---

### 2. 시스템 수준 조정(관리자 권한 필요)

이러한 조정은 시스템 전체 정책(`HKEY_LOCAL_MACHINE`)을 구성하거나 표준 Windows 관리 도구(`powercfg`, `netsh`, `fsutil`)를 호출합니다. 싱크로는 이를 실행하기 전에 고도를 확인합니다.

#### A. 게임 및 멀티미디어 예약(MMCSS)
| Tweak ID | Hive & Path | Value / Command | 목적 |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | GPU 인코더를 확보하기 위해 백그라운드 게임 DVR 비디오 캡처를 비활성화합니다. |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | 우선 순위가 높은 작업 중에 Windows 네트워크 조절 메커니즘을 비활성화합니다. |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | 전경 게임을 위해 CPU 시간의 90%를 예약합니다(기본값은 80%). |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | 활성 게임에 대한 멀티미디어 스케줄러 작업 우선순위를 높입니다. |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | 하드웨어 가속 GPU 스케줄링(WDDM 2.7+) 활성화 |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | 다중 평면 오버레이 비활성화(일부 GPU의 검은색 화면/끊김 문제 해결) |

#### B. 네트워크 스택 최적화(`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | 목적 |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | DNS 확인자 캐시를 지웁니다. |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | 표준 RFC 1323 TCP 창 크기 조정을 복원합니다. |
| `rss-on` | `netsh interface tcp set global rss=enabled` | 수신측 확장을 활성화하여 CPU 코어 전체에 네트워크 패킷 처리를 분산시킵니다. |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | 실시간 게임에서 대기 시간을 줄이기 위해 NIC에서 수신 세그먼트 병합을 비활성화합니다. |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | 이전 라우터에서 패킷 손실을 방지하기 위해 명시적 혼잡 알림을 비활성화합니다. |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | 로컬 주소 확인 프로토콜 테이블을 지웁니다. |
| `winsock-reset` | `netsh winsock reset` | Windows 네트워크 소켓 카탈로그를 깨끗한 상태로 복원합니다. |

#### C. 전원 및 스토리지 관리(`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | 목적 |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | 표준 Windows 고성능 전원 구성표를 활성화합니다. |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Windows Ultimate Performance 전원 구성표 잠금 해제 및 활성화 |
| `hibernate-off` | `powercfg /hibernate off` | 최대 절전 모드를 비활성화하고 'hiberfil.sys'를 삭제하여 디스크 공간을 확보합니다. |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | 모든 파일 읽기에서 NTFS 마지막 액세스 타임스탬프 업데이트를 중지합니다. |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | SSD 상태 및 마모 레벨링에 대한 TRIM 알림이 활성화되어 있는지 확인합니다. |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | 변경 전에 전체 Windows 시스템 복원 지점을 생성합니다. |

#### D. 시스템 원격 측정 및 배경 동기화
| Tweak ID | Hive & Path | Value | 목적 |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Windows 타임라인 및 활동 기록 업로드를 비활성화합니다. |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Windows 업데이트 전달 최적화를 로컬 LAN으로만 제한합니다(인터넷에 P2P 업로드 없음). |

---

## 차단된 안전하지 않은 조정

Syncro는 시스템 무결성이나 치트 방지 호환성을 손상시키는 타사 스크립트에서 발견되는 몇 가지 일반적인 "공격적인 조정"을 의도적으로 차단합니다.

- **메모리 무결성(HVCI) 비활성화 / 가상화 기반 보안**: 거부되었습니다.
- **Windows Defender/방화벽을 완전히 비활성화**: 거부되었습니다.
- **페이징 파일(`pagefile.sys`) 비활성화**: 거부됨(메모리 급증으로 게임 충돌 발생)
- **동적 틱/불변 TSC 변조 비활성화**: 거부됨(시계를 비동기화하고 치트 방지 휴리스틱을 실행할 수 있음)
- **대량 MSI(Message Signaled Interrupts) 유틸리티 해킹**: 거부됨(GPU 브리킹/블루 스크린 위험)
