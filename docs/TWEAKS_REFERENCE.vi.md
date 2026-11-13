# Synchro Nova — Tham khảo sửa đổi và tinh chỉnh hệ thống

Tài liệu này cung cấp bản phân tích đầy đủ, minh bạch về mọi cài đặt hệ thống, khóa đăng ký và lệnh được thực thi bởi Synchro Nova. Tất cả các hành động đều mang tính cục bộ, xác định và được sao lưu trước khi thực hiện.

---

## Đảm bảo an toàn & hoàn vốn

1. **Sao lưu tự động trước khi thực hiện**:
Trước khi sửa đổi bất kỳ khóa đăng ký nào, Synchro Nova tạo một điểm khôi phục hệ thống và lưu trạng thái hiện tại của tất cả các khóa mục tiêu vào tệp sao lưu `.reg` trong `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Quay lại bằng 1 cú nhấp chuột**:
Nhấp vào **"Откатить изменения"** (Rollback) trong ứng dụng sẽ khôi phục ảnh chụp nhanh `.reg` trước đó thông qua Windows `reg.exe` và đọc lại các giá trị hiện tại.
3. **Thực thi lệnh cứng**:
Tất cả các công cụ quản trị (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) đều được giải quyết nghiêm ngặt từ `%SystemRoot%\System32` (ngăn chặn việc chiếm quyền điều khiển PATH nâng cao) và chạy với thời gian chờ của cơ quan giám sát không chặn (5 giây đối với tiện ích, 25 giây để khôi phục hệ thống).
4. **Danh sách chặn cứng & An toàn chống gian lận**:
Các hoạt động không an toàn (vô hiệu hóa tính toàn vẹn của bộ nhớ/HVCI, vá bộ đếm thời gian kernel, vô hiệu hóa tệp hoán trang hoặc áp dụng ghi đè trình điều khiển tùy ý) sẽ bị chặn vĩnh viễn trong công cụ. Không cần xử lý quy trình hoặc nối bộ nhớ đảm bảo tuân thủ 100% với Easy Anti-Cheat, BattlEye, Vanguard và VAC.

---

## Hoàn thành phân tích tinh chỉnh

### 1. Tinh chỉnh cấp độ người dùng (Người dùng chuẩn - Không cần quản trị viên)

Những chỉnh sửa này chỉ sửa đổi tổ ong `HKEY_CURRENT_USER`. Chúng không yêu cầu đặc quyền nâng cao và chỉ ảnh hưởng đến hồ sơ người dùng đang hoạt động.

| Tweak ID | Hive & Path | Value / Command | Mục đích |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Ngăn cửa sổ bật lên Xbox Game Bar khi khởi chạy trò chơi |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Vô hiệu hóa khả năng tăng tốc chuột của Windows (đầu vào tuyến tính thô) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Vô hiệu hóa mã nhận dạng quảng cáo Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Tắt các mẹo được cá nhân hóa dựa trên dữ liệu chẩn đoán |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Giữ clipboard nghiêm ngặt trong RAM cục bộ, tắt đồng bộ hóa đám mây |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Tắt tính minh bạch của cửa sổ DWM để tiết kiệm bộ nhớ GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Định cấu hình Windows Explorer để có hiệu suất hình ảnh tốt nhất |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Loại bỏ độ trễ khởi động máy tính để bàn Windows nhân tạo |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Giảm độ trễ hoạt ảnh của menu UI xuống 10 mili giây |

---

### 2. Tinh chỉnh cấp hệ thống (Yêu cầu đặc quyền của quản trị viên)

Những chỉnh sửa này định cấu hình các chính sách trên toàn hệ thống (`HKEY_LOCAL_MACHINE`) hoặc gọi các công cụ quản trị Windows tiêu chuẩn (`powercfg`, `Netsh`, `fsutil`). Synchro kiểm tra độ cao trước khi thực hiện những điều này.

#### A. Lập kế hoạch chơi game & đa phương tiện (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Mục đích |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Tắt tính năng quay video Game DVR nền để giải phóng bộ mã hóa GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Vô hiệu hóa cơ chế điều tiết mạng Windows trong các tác vụ có mức độ ưu tiên cao |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Dành 90% thời gian CPU cho các trò chơi nền trước (mặc định là 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Nâng cao mức độ ưu tiên tác vụ của bộ lập lịch đa phương tiện cho các trò chơi đang hoạt động |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Cho phép lập lịch GPU tăng tốc phần cứng (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Vô hiệu hóa Lớp phủ nhiều mặt phẳng (giải quyết các vấn đề về màn hình đen/giật hình trên một số GPU) |

#### B. Tối ưu hóa ngăn xếp mạng (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Mục đích |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Xóa bộ đệm của trình phân giải DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Khôi phục tỷ lệ cửa sổ TCP RFC 1323 tiêu chuẩn |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Cho phép chia tỷ lệ bên nhận để phân phối xử lý gói mạng trên các lõi CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Tắt tính năng liên kết phân đoạn nhận trên NIC để giảm độ trễ khi chơi trò chơi thời gian thực |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Tắt Thông báo tắc nghẽn rõ ràng để ngăn chặn các gói bị rơi trên các bộ định tuyến cũ |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Xóa bảng Giao thức phân giải địa chỉ cục bộ |
| `winsock-reset` | `netsh winsock reset` | Khôi phục danh mục ổ cắm mạng Windows về trạng thái sạch |

#### C. Quản lý nguồn và lưu trữ (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Mục đích |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Kích hoạt sơ đồ nguồn Windows High Performance tiêu chuẩn |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Mở khóa và kích hoạt sơ đồ sức mạnh Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Tắt chế độ ngủ đông và xóa `hiberfil.sys`, giải phóng dung lượng ổ đĩa |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Dừng cập nhật dấu thời gian truy cập lần cuối NTFS trên mỗi tệp được đọc |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Đảm bảo thông báo TRIM hoạt động để kiểm tra tình trạng và mức độ hao mòn của SSD |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Tạo điểm Khôi phục Hệ thống Windows đầy đủ trước khi thay đổi |

#### D. Đồng bộ hóa từ xa và nền hệ thống
| Tweak ID | Hive & Path | Value | Mục đích |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Vô hiệu hóa tải lên lịch sử hoạt động và dòng thời gian của Windows |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Hạn chế Tối ưu hóa phân phối Windows Update chỉ trong mạng LAN cục bộ (không tải P2P lên internet) |

---

## Tinh chỉnh không an toàn được liệt kê trong danh sách chặn

Synchro cố tình chặn một số "tinh chỉnh mạnh mẽ" phổ biến được tìm thấy trong các tập lệnh của bên thứ ba làm tổn hại đến tính toàn vẹn của hệ thống hoặc khả năng tương thích chống gian lận:

- **Vô hiệu hóa tính toàn vẹn bộ nhớ (HVCI) / Bảo mật dựa trên ảo hóa**: Bị từ chối.
- **Vô hiệu hóa hoàn toàn Windows Defender/Tường lửa**: Bị từ chối.
- **Vô hiệu hóa tệp hoán trang (`pagefile.sys`)**: Bị từ chối (khiến trò chơi bị treo do tăng đột biến bộ nhớ).
- **Vô hiệu hóa Tick động / giả mạo TSC bất biến**: Bị từ chối (có thể giải đồng bộ hóa đồng hồ và các phương pháp chẩn đoán chống gian lận chuyến đi).
- ** Hack tiện ích MSI (Ngắt tín hiệu tin nhắn) hàng loạt**: Bị từ chối (nguy cơ bị đóng cục GPU/màn hình xanh).
