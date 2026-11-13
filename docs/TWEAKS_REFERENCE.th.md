# Synchro Nova — การปรับเปลี่ยนระบบและการอ้างอิงการปรับแต่ง

เอกสารนี้ให้รายละเอียดที่ครบถ้วนและโปร่งใสของการตั้งค่าระบบ รีจิสตรีคีย์ และคำสั่งที่ดำเนินการโดย Synchro Nova การดำเนินการทั้งหมดเป็นไปตามท้องถิ่นอย่างเคร่งครัด มีการกำหนดไว้ และมีการสำรองข้อมูลก่อนดำเนินการ

---

## รับประกันความปลอดภัยและย้อนกลับ

1. **การสำรองข้อมูลก่อนการดำเนินการอัตโนมัติ**:
ก่อนที่จะแก้ไขคีย์รีจิสทรีใดๆ Synchro Nova จะสร้างจุดคืนค่าระบบและบันทึกสถานะปัจจุบันของคีย์เป้าหมายทั้งหมดลงในไฟล์สำรองข้อมูล `.reg` ใน `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`
2. **1 คลิกย้อนกลับ**:
การคลิก **"Откатить изменения"** (ย้อนกลับ) ในแอปพลิเคชันจะคืนค่าสแนปชอต `.reg` ก่อนหน้าผ่านทาง Windows `reg.exe` และอ่านค่าปัจจุบันอีกครั้ง
3. **การดำเนินการตามคำสั่งที่เข้มงวดยิ่งขึ้น**:
เครื่องมือการดูแลระบบทั้งหมด (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) ได้รับการแก้ไขอย่างเข้มงวดจาก `%SystemRoot%\System32` (ป้องกันการไฮแจ็ก PATH ที่ยกระดับ) และทำงานโดยไม่มีการบล็อกการหมดเวลาของ Watchdog (5 วินาทีสำหรับยูทิลิตี้, 25 วินาทีสำหรับการกู้คืนระบบ)
4. **รายการบล็อกยากและโปรแกรมป้องกันการโกง**:
การดำเนินการที่ไม่ปลอดภัย (การปิดใช้งานความสมบูรณ์ของหน่วยความจำ/HVCI, การแพทช์ตัวจับเวลาเคอร์เนล, การปิดใช้งานไฟล์เพจจิ้ง หรือการใช้การแทนที่ไดรเวอร์ตามอำเภอใจ) จะถูกบล็อกอย่างถาวรในกลไก การฉีดกระบวนการเป็นศูนย์หรือการเชื่อมต่อหน่วยความจำช่วยให้มั่นใจว่าสอดคล้องกับ Easy Anti-Cheat, BattlEye, Vanguard และ VAC 100%

---

## แบ่ง Tweaks ให้สมบูรณ์

### 1. การปรับแต่งระดับผู้ใช้ (ผู้ใช้มาตรฐาน — ไม่ต้องมีผู้ดูแลระบบ)

การปรับแต่งเหล่านี้แก้ไขกลุ่ม `HKEY_CURRENT_USER` เท่านั้น ไม่ต้องการสิทธิ์ระดับสูงและมีผลเฉพาะกับโปรไฟล์ผู้ใช้ที่ใช้งานอยู่เท่านั้น

| Tweak ID | Hive & Path | Value / Command | วัตถุประสงค์ |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | ป้องกันป๊อปอัป Xbox Game Bar เมื่อเปิดเกม |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | ปิดใช้งานการเร่งความเร็วเมาส์ของ Windows (อินพุตเชิงเส้นแบบดิบ) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | ปิดใช้งานตัวระบุโฆษณา Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | ปิดใช้งานเคล็ดลับส่วนบุคคลตามข้อมูลการวินิจฉัย |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | เก็บคลิปบอร์ดอย่างเคร่งครัดใน RAM ในเครื่อง ปิดใช้งานการซิงค์บนคลาวด์ |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | ปิดใช้งานความโปร่งใสของหน้าต่าง DWM เพื่อบันทึกหน่วยความจำ GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | กำหนดค่า Windows Explorer เพื่อประสิทธิภาพการมองเห็นที่ดีที่สุด |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | ลบความล่าช้าในการเริ่มต้นเดสก์ท็อป Windows เทียม |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | ลดความล่าช้าของภาพเคลื่อนไหวเมนู UI เป็น 10ms |

---

### 2. การปรับแต่งระดับระบบ (ต้องการสิทธิ์ของผู้ดูแลระบบ)

การปรับแต่งเหล่านี้กำหนดค่านโยบายทั้งระบบ (`HKEY_LOCAL_MACHINE`) หรือเรียกใช้เครื่องมือการดูแลระบบ Windows มาตรฐาน (`powercfg`, `netsh`, `fsutil`) ซิงโครจะตรวจสอบระดับความสูงก่อนที่จะดำเนินการเหล่านี้

#### ก. การกำหนดเวลาการเล่นเกมและมัลติมีเดีย (MMCSS)
| Tweak ID | Hive & Path | Value / Command | วัตถุประสงค์ |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | ปิดใช้งานการจับภาพวิดีโอ Game DVR ในเบื้องหลังเพื่อใช้ตัวเข้ารหัส GPU ฟรี |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | ปิดใช้งานกลไกการควบคุมปริมาณเครือข่าย Windows ในระหว่างงานที่มีลำดับความสำคัญสูง |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | สำรองเวลา CPU 90% สำหรับเกมเบื้องหน้า (ค่าเริ่มต้นคือ 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | ยกระดับลำดับความสำคัญของงานตัวกำหนดเวลามัลติมีเดียสำหรับเกมที่ใช้งานอยู่ |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | เปิดใช้งานการตั้งเวลา GPU ที่เร่งด้วยฮาร์ดแวร์ (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | ปิดการใช้งาน Multi-Plane Overlays (แก้ไขปัญหาหน้าจอดำ / พูดติดอ่างใน GPU บางตัว) |

#### B. การเพิ่มประสิทธิภาพเครือข่ายสแต็ก (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | วัตถุประสงค์ |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | ล้างแคชตัวแก้ไข DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | คืนค่ามาตราส่วนหน้าต่าง RFC 1323 TCP มาตรฐาน |
| `rss-on` | `netsh interface tcp set global rss=enabled` | เปิดใช้งานการปรับขนาดฝั่งรับเพื่อกระจายการประมวลผลแพ็กเก็ตเครือข่ายผ่านคอร์ CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | ปิดใช้งานการรับเซ็กเมนต์รวมบน NIC เพื่อลดเวลาแฝงในการเล่นเกมแบบเรียลไทม์ |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | ปิดใช้งานการแจ้งเตือนความแออัดอย่างชัดเจนเพื่อป้องกันแพ็กเก็ตที่ตกบนเราเตอร์รุ่นเก่า |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | ล้างตารางโปรโตคอลการแก้ไขที่อยู่ภายในเครื่อง |
| `winsock-reset` | `netsh winsock reset` | คืนค่าแค็ตตาล็อกซ็อกเก็ตเครือข่าย Windows ให้เป็นสถานะสะอาด |

#### C. การจัดการพลังงานและการจัดเก็บ (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | วัตถุประสงค์ |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | เปิดใช้งานแผนพลังงาน Windows High Performance มาตรฐาน |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | ปลดล็อคและเปิดใช้งานแผนการใช้พลังงาน Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | ปิดใช้งานการไฮเบอร์เนตและลบ `hiberfil.sys` เพื่อเพิ่มพื้นที่ว่างในดิสก์ |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | หยุดการอัปเดตการประทับเวลาการเข้าถึงล่าสุดของ NTFS ในทุกไฟล์ที่อ่าน |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | ตรวจสอบให้แน่ใจว่าการแจ้งเตือน TRIM ทำงานเพื่อสุขภาพของ SSD และการปรับระดับการสึกหรอ |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | สร้างจุดคืนค่าระบบ Windows แบบเต็มก่อนการเปลี่ยนแปลง |

#### D. การวัดและส่งข้อมูลทางไกลของระบบและการซิงค์พื้นหลัง
| Tweak ID | Hive & Path | Value | วัตถุประสงค์ |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | ปิดใช้งานการอัพโหลดไทม์ไลน์ของ Windows และประวัติกิจกรรม |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | จำกัด การเพิ่มประสิทธิภาพการจัดส่ง Windows Update ให้กับ LAN ภายในเท่านั้น (ไม่มีการอัพโหลด P2P ไปยังอินเทอร์เน็ต) |

---

## การปรับแต่งที่ไม่ปลอดภัยที่ถูกบล็อก

Synchro จงใจบล็อก "การปรับแต่งเชิงรุก" ทั่วไปหลายอย่างที่พบในสคริปต์ของบุคคลที่สามที่กระทบต่อความสมบูรณ์ของระบบหรือความเข้ากันได้ของการป้องกันการโกง:

- **ปิดการใช้งาน Memory Integrity (HVCI) / Virtualization-Based Security**: ถูกปฏิเสธ
- **ปิดการใช้งาน Windows Defender / Firewall ทั้งหมด**: ถูกปฏิเสธ
- **ปิดการใช้งานไฟล์เพจจิ้ง (`pagefile.sys`)**: ถูกปฏิเสธ (ทำให้เกมล่มเนื่องจากหน่วยความจำพุ่งสูง)
- **ปิดการใช้งาน Dynamic Tick / Invariant TSC tampering**: ถูกปฏิเสธ (สามารถยกเลิกการซิงโครไนซ์นาฬิกาและทริปป้องกันการโกง)
- **การแฮ็กยูทิลิตี้ MSI จำนวนมาก (ข้อความส่งสัญญาณขัดจังหวะ)**: ถูกปฏิเสธ (ความเสี่ยงของ GPU อิฐ / หน้าจอสีน้ำเงิน)
