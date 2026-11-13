# Synchro Nova - مرجع تعديلات النظام والقرص

يوفر هذا المستند تحليلاً كاملاً وشفافًا لكل إعدادات النظام ومفتاح التسجيل والأمر الذي يتم تنفيذه بواسطة Synchro Nova. جميع الإجراءات محلية تمامًا وحتمية ويتم دعمها قبل التنفيذ.

---

## ضمان السلامة والتراجع

1. **النسخ الاحتياطي الآلي قبل التنفيذ**:
قبل تعديل أي مفتاح تسجيل، يقوم Synchro Nova بإنشاء نقطة استعادة للنظام وحفظ الحالة الحالية لجميع المفاتيح المستهدفة في ملف النسخ الاحتياطي `.reg` في `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-انقر فوق التراجع**:
يؤدي النقر فوق **"إلغاء الاستدعاء"** (التراجع) في التطبيق إلى استعادة لقطة `.reg` السابقة عبر Windows `reg.exe` وإعادة قراءة القيم الحالية.
3. **تنفيذ الأوامر المتشددة**:
يتم حل جميع الأدوات الإدارية (`netsh`، و`powercfg`، و`sc`، و`bcdedit`، و`reg`، و`powershell`) بشكل صارم من `%SystemRoot%\System32` (لمنع اختطاف PATH المرتفع) وتشغيلها مع مهلات مراقبة غير محظورة (5 ثوانٍ للأدوات المساعدة، و25 ثانية لاستعادة النظام).
4. **قائمة الحظر الصارمة وآمنة لمكافحة الغش**:
يتم حظر العمليات غير الآمنة (تعطيل تكامل الذاكرة/HVCI، أو تصحيح مؤقتات kernel، أو تعطيل ملف ترحيل الصفحات، أو تطبيق تجاوزات عشوائية لبرنامج التشغيل) بشكل دائم في المحرك. يضمن عدم حقن العملية أو ربط الذاكرة التوافق بنسبة 100% مع Easy Anti-Cheat وBattlEye وVanguard وVAC.

---

## انهيار القرص الكامل

### 1. تعديلات على مستوى المستخدم (المستخدم القياسي - لا يلزم وجود مسؤول)

تعمل هذه التعديلات على تعديل خلية `HKEY_CURRENT_USER` فقط. وهي لا تتطلب امتيازات عالية وتؤثر فقط على ملف تعريف المستخدم النشط.

| Tweak ID | Hive & Path | Value / Command | غاية |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | يمنع ظهور نافذة Xbox Game Bar المنبثقة عند تشغيل اللعبة |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | تعطيل تسريع ماوس Windows (الإدخال الخطي الخام) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | تعطيل معرف إعلانات Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | تعطيل النصائح الشخصية المستندة إلى البيانات التشخيصية |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | يحافظ على الحافظة بشكل صارم في ذاكرة الوصول العشوائي المحلية، ويعطل المزامنة السحابية |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | تعطيل شفافية نافذة DWM لحفظ ذاكرة GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | يقوم بتكوين Windows Explorer للحصول على أفضل أداء مرئي |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | يزيل التأخير الاصطناعي في بدء تشغيل سطح مكتب Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | يقلل تأخير الرسوم المتحركة لقائمة واجهة المستخدم إلى 10 مللي ثانية |

---

### 2. القرص على مستوى النظام (يتطلب امتيازات المسؤول)

تعمل هذه التعديلات على تكوين السياسات على مستوى النظام (`HKEY_LOCAL_MACHINE`) أو استدعاء أدوات Windows الإدارية القياسية (`powercfg`، `netsh`، `fsutil`). يتحقق التزامن من الارتفاع قبل تنفيذه.

#### أ. جدولة الألعاب والوسائط المتعددة (MMCSS)
| Tweak ID | Hive & Path | Value / Command | غاية |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | تعطيل التقاط فيديو DVR في الخلفية لتحرير برنامج تشفير GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | تعطيل آلية تقييد شبكة Windows أثناء المهام ذات الأولوية العالية |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | حجز 90% من وقت وحدة المعالجة المركزية للألعاب الأمامية (الافتراضي هو 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | يرفع أولوية مهمة جدولة الوسائط المتعددة للألعاب النشطة |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | تمكين جدولة GPU المعجلة للأجهزة (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | تعطيل التراكبات متعددة المستويات (يحل مشكلات الشاشة السوداء/التلعثم في بعض وحدات معالجة الرسومات) |

#### ب. تحسين مكدس الشبكة (`netsh`، `ipconfig`)
| Tweak ID | Tool / Command | غاية |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | يمسح ذاكرة التخزين المؤقت لمحلل DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | يستعيد مقياس نافذة RFC 1323 TCP القياسي |
| `rss-on` | `netsh interface tcp set global rss=enabled` | تمكين تحجيم جانب الاستلام لتوزيع معالجة حزم الشبكة عبر مراكز وحدة المعالجة المركزية |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | تعطيل تلقي دمج المقاطع على بطاقات NIC لتقليل زمن الوصول في الألعاب في الوقت الفعلي |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | تعطيل إعلام الازدحام الصريح لمنع إسقاط الحزم على أجهزة التوجيه الأقدم |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | مسح جدول بروتوكول تحليل العنوان المحلي |
| `winsock-reset` | `netsh winsock reset` | يستعيد كتالوج مآخذ شبكة Windows لتنظيف الحالة |

#### ج. إدارة الطاقة والتخزين (`powercfg`، `fsutil`)
| Tweak ID | Tool / Command | غاية |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | يقوم بتنشيط نظام الطاقة Windows High Performance القياسي |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | يفتح وينشط نظام الطاقة Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | تعطيل وضع السبات وحذف `hiberfil.sys`، مما يؤدي إلى تحرير مساحة القرص |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | يوقف تحديث الطوابع الزمنية لآخر وصول إلى NTFS في كل ملف يتم قراءته |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | يضمن تنشيط إشعارات TRIM لصحة SSD وتسوية التآكل |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | يقوم بإنشاء نقطة استعادة نظام Windows كاملة قبل التغييرات |

#### د. القياس عن بعد للنظام ومزامنة الخلفية
| Tweak ID | Hive & Path | Value | غاية |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | تعطيل تحميلات الجدول الزمني لـ Windows وسجل النشاط |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | يقيد Windows Update Delivery Optimization على الشبكة المحلية المحلية فقط (لا توجد تحميلات P2P على الإنترنت) |

---

## القرص غير الآمن المحظور

يحظر Synchro عمدًا العديد من "التعديلات العدوانية" الشائعة الموجودة في البرامج النصية التابعة لجهات خارجية والتي تهدد سلامة النظام أو التوافق مع مكافحة الغش:

- **تعطيل سلامة الذاكرة (HVCI) / الأمان القائم على المحاكاة الافتراضية**: مرفوض.
- **تعطيل Windows Defender / جدار الحماية بالكامل**: مرفوض.
- ** تعطيل ملف ترحيل الصفحات (`pagefile.sys`)**: مرفوض (يتسبب في تعطل اللعبة بسبب ارتفاع الذاكرة).
- **تعطيل العلامة الديناميكية / التلاعب بـ TSC الثابت**: مرفوض (يمكن إلغاء مزامنة الساعات والاستدلال على مكافحة الغش).
- ** اختراق الأداة المساعدة MSI (مقاطعات إشارة الرسائل) **: مرفوض (خطر حجب وحدة معالجة الرسومات / الشاشات الزرقاء).
