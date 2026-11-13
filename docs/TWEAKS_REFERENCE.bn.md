# সিঙ্ক্রো নোভা — সিস্টেম পরিবর্তন এবং টুইকস রেফারেন্স

এই নথিটি প্রতিটি সিস্টেম সেটিং, রেজিস্ট্রি কী, এবং Synchro Nova দ্বারা নির্বাহিত কমান্ডের সম্পূর্ণ, স্বচ্ছ ভাঙ্গন প্রদান করে। সমস্ত ক্রিয়া কঠোরভাবে স্থানীয়, নির্ধারক এবং কার্যকর করার আগে ব্যাক আপ করা হয়।

---

## নিরাপত্তা এবং রোলব্যাক গ্যারান্টি

1. **স্বয়ংক্রিয় প্রি-এক্সিকিউশন ব্যাকআপ**:
যেকোনো রেজিস্ট্রি কী পরিবর্তন করার আগে, Synchro Nova একটি সিস্টেম পুনরুদ্ধার পয়েন্ট তৈরি করে এবং `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`-এ একটি `.reg` ব্যাকআপ ফাইলে সমস্ত টার্গেট কীগুলির বর্তমান অবস্থা সংরক্ষণ করে।
2. **1- Rollback-এ ক্লিক করুন**:
অ্যাপ্লিকেশনটিতে **"Откатить изменения"** (রোলব্যাক) এ ক্লিক করা উইন্ডোজ `reg.exe` এর মাধ্যমে পূর্ববর্তী `.reg` স্ন্যাপশট পুনরুদ্ধার করে এবং বর্তমান মানগুলি পুনরায় পাঠ করে।
3. **কঠোর কমান্ড এক্সিকিউশন**:
সমস্ত প্রশাসনিক সরঞ্জাম (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) কঠোরভাবে `%SystemRoot%\System32` থেকে সমাধান করা হয়েছে (উন্নত PATH হাইজ্যাকিং প্রতিরোধ করা) এবং নন-ব্লকিং ওয়াচডগ টাইমআউটের জন্য (5 সিস্টেমের বিশ্রামের জন্য 5গুলি)।
4. **হার্ড ব্লকলিস্ট এবং এন্টি-চিট নিরাপদ**:
অনিরাপদ ক্রিয়াকলাপগুলি (মেমরি অখণ্ডতা/এইচভিসিআই নিষ্ক্রিয় করা, কার্নেল টাইমারগুলি প্যাচ করা, পেজিং ফাইল নিষ্ক্রিয় করা, বা নির্বিচারে ড্রাইভার ওভাররাইড প্রয়োগ করা) স্থায়ীভাবে ইঞ্জিনে ব্লক করা হয়। জিরো প্রসেস ইনজেকশন বা মেমরি হুকিং ইজি অ্যান্টি-চিট, ব্যাটলআই, ভ্যানগার্ড এবং ভিএসির সাথে 100% সম্মতি নিশ্চিত করে।

---

## সম্পূর্ণ টুইকস ব্রেকডাউন

### 1. ইউজার-লেভেল টুইকস (স্ট্যান্ডার্ড ইউজার - কোন অ্যাডমিনের প্রয়োজন নেই)

এই টুইকগুলি শুধুমাত্র `HKEY_CURRENT_USER` হাইভকে পরিবর্তন করে। তাদের উন্নত সুবিধার প্রয়োজন হয় না এবং শুধুমাত্র সক্রিয় ব্যবহারকারী প্রোফাইলকে প্রভাবিত করে।

| Tweak ID | Hive & Path | Value / Command | উদ্দেশ্য |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | গেম লঞ্চে এক্সবক্স গেম বার পপআপ প্রতিরোধ করে |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | উইন্ডোজ মাউস ত্বরণ অক্ষম করে (কাঁচা লিনিয়ার ইনপুট) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | উইন্ডোজ বিজ্ঞাপন শনাক্তকারী অক্ষম করে |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | ডায়গনিস্টিক ডেটা-ভিত্তিক ব্যক্তিগতকৃত টিপস অক্ষম করে৷ |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | স্থানীয় র‌্যামে ক্লিপবোর্ডকে কঠোরভাবে রাখে, ক্লাউড সিঙ্ক অক্ষম করে |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | GPU মেমরি সংরক্ষণ করতে DWM উইন্ডো স্বচ্ছতা অক্ষম করে |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | সেরা ভিজ্যুয়াল পারফরম্যান্সের জন্য উইন্ডোজ এক্সপ্লোরার কনফিগার করে |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | কৃত্রিম উইন্ডোজ ডেস্কটপ স্টার্টআপ বিলম্ব দূর করে |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | UI মেনু অ্যানিমেশন বিলম্ব 10ms এ হ্রাস করে |

---

### 2. সিস্টেম-লেভেল টুইকস (প্রশাসকের বিশেষাধিকার প্রয়োজন)

এই পরিবর্তনগুলি সিস্টেম-ওয়াইড নীতিগুলি কনফিগার করে (`HKEY_LOCAL_MACHINE`) বা স্ট্যান্ডার্ড উইন্ডোজ অ্যাডমিনিস্ট্রেটিভ টুল (`powercfg`, `netsh`, `fsutil`) কল করে। সিঙ্ক্রো এগুলি কার্যকর করার আগে উচ্চতা পরীক্ষা করে।

#### A. গেমিং এবং মাল্টিমিডিয়া শিডিউলিং (MMCSS)
| Tweak ID | Hive & Path | Value / Command | উদ্দেশ্য |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | বিনামূল্যে GPU এনকোডারে ব্যাকগ্রাউন্ড গেম DVR ভিডিও ক্যাপচার অক্ষম করে৷ |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | উচ্চ-অগ্রাধিকারমূলক কাজের সময় Windows নেটওয়ার্ক থ্রটলিং প্রক্রিয়া নিষ্ক্রিয় করে |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | ফোরগ্রাউন্ড গেমের জন্য CPU সময়ের 90% সংরক্ষণ করে (ডিফল্ট 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | সক্রিয় গেমগুলির জন্য মাল্টিমিডিয়া শিডিউলারের টাস্ক অগ্রাধিকারকে উন্নত করে৷ |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | হার্ডওয়্যার-অ্যাক্সিলারেটেড GPU শিডিউলিং সক্ষম করে (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | মাল্টি-প্লেন ওভারলে অক্ষম করে (কিছু জিপিইউতে কালো স্ক্রিন / তোতলামি সমস্যা সমাধান করে) |

#### B. নেটওয়ার্ক স্ট্যাক অপ্টিমাইজেশান (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | উদ্দেশ্য |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | DNS সমাধানকারী ক্যাশে সাফ করে |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | স্ট্যান্ডার্ড RFC 1323 TCP উইন্ডো স্কেলিং পুনরুদ্ধার করে |
| `rss-on` | `netsh interface tcp set global rss=enabled` | CPU কোর জুড়ে নেটওয়ার্ক প্যাকেট প্রক্রিয়াকরণ বিতরণ করতে রিসিভ-সাইড স্কেলিং সক্ষম করে |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | রিয়েল-টাইম গেমিং-এ লেটেন্সি কমাতে NIC-তে রিসিভ সেগমেন্ট কোলেসিং অক্ষম করে |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | পুরানো রাউটারে ড্রপ প্যাকেট প্রতিরোধ করতে সুস্পষ্ট কনজেশন বিজ্ঞপ্তি অক্ষম করে |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | স্থানীয় ঠিকানা রেজোলিউশন প্রোটোকল টেবিল সাফ করে |
| `winsock-reset` | `netsh winsock reset` | পরিষ্কার অবস্থায় Windows নেটওয়ার্ক সকেট ক্যাটালগ পুনরুদ্ধার করে |

#### C. পাওয়ার ও স্টোরেজ ম্যানেজমেন্ট (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | উদ্দেশ্য |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | স্ট্যান্ডার্ড উইন্ডোজ হাই পারফরম্যান্স পাওয়ার স্কিম সক্রিয় করে |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | উইন্ডোজ আলটিমেট পারফরম্যান্স পাওয়ার স্কিম আনলক এবং সক্রিয় করে |
| `hibernate-off` | `powercfg /hibernate off` | হাইবারনেশন অক্ষম করে এবং ডিস্কের জায়গা খালি করে `hiberfil.sys` মুছে দেয় |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | পড়া প্রতিটি ফাইলে NTFS শেষ-অ্যাক্সেস টাইমস্ট্যাম্প আপডেট করা বন্ধ করে |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | এসএসডি স্বাস্থ্য এবং পরিধান সমতলকরণের জন্য TRIM বিজ্ঞপ্তিগুলি সক্রিয় রয়েছে তা নিশ্চিত করে |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | পরিবর্তনের আগে একটি সম্পূর্ণ উইন্ডোজ সিস্টেম রিস্টোর পয়েন্ট তৈরি করে |

#### D. সিস্টেম টেলিমেট্রি এবং ব্যাকগ্রাউন্ড সিঙ্ক
| Tweak ID | Hive & Path | Value | উদ্দেশ্য |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Windows টাইমলাইন এবং কার্যকলাপ ইতিহাস আপলোড নিষ্ক্রিয় করে |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | শুধুমাত্র স্থানীয় ল্যানে উইন্ডোজ আপডেট ডেলিভারি অপ্টিমাইজেশান সীমাবদ্ধ করে (ইন্টারনেটে কোন P2P আপলোড নেই) |

---

## অবরুদ্ধ তালিকাভুক্ত অনিরাপদ Tweaks

সিঙ্ক্রো ইচ্ছাকৃতভাবে তৃতীয় পক্ষের স্ক্রিপ্টগুলিতে পাওয়া বেশ কয়েকটি সাধারণ "আক্রমনাত্মক টুইক" ব্লক করে যা সিস্টেমের অখণ্ডতা বা অ্যান্টি-চিট সামঞ্জস্যের সাথে আপস করে:

- **মেমোরি ইন্টিগ্রিটি অক্ষম করা (HVCI) / ভার্চুয়ালাইজেশন-ভিত্তিক নিরাপত্তা**: প্রত্যাখ্যান করা হয়েছে।
- **উইন্ডোজ ডিফেন্ডার/ফায়ারওয়াল সম্পূর্ণরূপে নিষ্ক্রিয় করা**: প্রত্যাখ্যাত।
- **পেজিং ফাইল নিষ্ক্রিয় করা হচ্ছে (`pagefile.sys`)**: প্রত্যাখ্যাত (মেমরি স্পাইকগুলিতে গেম ক্র্যাশের কারণ)।
- **ডাইনামিক টিক/ইনভেরিয়েন্ট টিএসসি টেম্পারিং নিষ্ক্রিয় করা**: প্রত্যাখ্যান করা হয়েছে (ঘড়ি ডিসিঙ্ক্রোনাইজ করতে পারে এবং অ্যান্টি-চিট হিউরিস্টিক ট্রিপ করতে পারে)।
- **বাল্ক MSI (মেসেজ সিগন্যালড ইন্টারাপ্টস) ইউটিলিটি হ্যাকিং**: প্রত্যাখ্যাত (GPU ব্রিকিং/ব্লু স্ক্রীনের ঝুঁকি)।
