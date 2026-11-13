# Synchro Nova — довідник із модифікацій і налаштувань системи

У цьому документі наведено повну, прозору розбивку кожного параметра системи, розділу реєстру та команди, які виконує Synchro Nova. Усі дії суто локальні, детерміновані та резервні копії перед виконанням.

---

## Гарантія безпеки та повернення

1. **Автоматизоване резервне копіювання перед виконанням**:
Перед зміною будь-якого ключа реєстру Synchro Nova створює точку відновлення системи та зберігає поточний стан усіх цільових ключів у файлі резервної копії `.reg` у `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **1-клацніть Відкат**:
Натискання **"Откатить изменения"** (Відкат) у програмі відновлює попередній знімок `.reg` через Windows `reg.exe` і перечитує поточні значення.
3. **Загартоване виконання команд**:
Усі інструменти адміністрування (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) суворо вирішуються з `%SystemRoot%\System32` (запобігаючи викраденню підвищеного ШЛЯХУ) і запускаються з тайм-аутами сторожового тайм-ауту без блокування (5 с для утиліт, 25 с для відновлення системи).
4. **Жорсткий чорний список і захист від чітів**:
Небезпечні операції (вимкнення цілісності пам’яті/HVCI, виправлення таймерів ядра, вимкнення файлу підкачки або застосування довільних замін драйверів) назавжди блокуються в механізмі. Нульове впровадження процесу або підключення пам’яті забезпечує 100% відповідність Easy Anti-Cheat, BattlEye, Vanguard і VAC.

---

## Повна розбивка налаштувань

### 1. Налаштування на рівні користувача (стандартний користувач — адміністратор не потрібен)

Ці налаштування змінюють лише кущ `HKEY_CURRENT_USER`. Вони не вимагають підвищених привілеїв і впливають лише на активний профіль користувача.

| Tweak ID | Hive & Path | Value / Command | призначення |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Запобігає спливаючому вікну ігрової панелі Xbox під час запуску гри |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Вимикає прискорення миші Windows (необроблений лінійний ввід) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Вимикає рекламний ідентифікатор Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Вимикає персоналізовані підказки на основі даних діагностики |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Зберігає буфер обміну виключно в локальній оперативній пам’яті, вимикає хмарну синхронізацію |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Вимикає прозорість вікна DWM для економії пам’яті GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Налаштовує Windows Explorer для найкращої візуальної продуктивності |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Усуває штучну затримку запуску робочого столу Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Зменшує затримку анімації меню інтерфейсу до 10 мс |

---

### 2. Налаштування системного рівня (потрібні права адміністратора)

Ці налаштування налаштовують загальносистемні політики (`HKEY_LOCAL_MACHINE`) або викликають стандартні засоби адміністрування Windows (`powercfg`, `netsh`, `fsutil`). Перед виконанням Synchro перевіряє висоту.

#### A. Планування ігор і мультимедіа (MMCSS)
| Tweak ID | Hive & Path | Value / Command | призначення |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Вимикає фонове захоплення відео DVR гри для вільного графічного кодувальника |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Вимикає механізм обмеження мережі Windows під час виконання завдань з високим пріоритетом |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Резервує 90% процесорного часу для ігор на передньому плані (за замовчуванням 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Підвищує пріоритет завдань мультимедійного планувальника для активних ігор |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Вмикає планування графічного процесора з апаратним прискоренням (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Вимикає багатоплощинні накладання (вирішує проблеми з чорним екраном/заїканням на деяких графічних процесорах) |

#### B. Оптимізація стеку мережі (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | призначення |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Очищає кеш розпізнавача DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Відновлює стандартне масштабування вікна TCP RFC 1323 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Вмикає масштабування на стороні отримання, щоб розподілити обробку мережевих пакетів між ядрами ЦП |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Вимикає об’єднання сегментів отримання на мережевих картах, щоб зменшити затримку в іграх у реальному часі |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Вимикає явне сповіщення про перевантаження, щоб запобігти випаданню пакетів на старих маршрутизаторах |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Очищає локальну таблицю протоколу розпізнавання адрес |
| `winsock-reset` | `netsh winsock reset` | Відновлює чистий стан каталогу мережевих розеток Windows |

#### C. Керування живленням і накопичувачем (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | призначення |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Активує стандартну високопродуктивну схему живлення Windows |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Розблокує та активує схему живлення Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Вимикає сплячий режим і видаляє `hiberfil.sys`, звільняючи місце на диску |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Зупиняє оновлення міток часу останнього доступу NTFS під час кожного читання файлу |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Забезпечує активність сповіщень TRIM для вирівнювання стану SSD і зносу |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Створює повну точку відновлення системи Windows перед внесенням змін |

#### D. Системна телеметрія та фонова синхронізація
| Tweak ID | Hive & Path | Value | призначення |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Вимикає завантаження часової шкали Windows і історії активності |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Обмежує оптимізацію доставки Windows Update тільки локальною локальною мережею (без завантажень P2P в Інтернет) |

---

## Заблоковані небезпечні налаштування

Synchro навмисно блокує кілька поширених «агресивних налаштувань», знайдених у сценаріях сторонніх розробників, які порушують цілісність системи або сумісність із засобом чітування:

- **Відключення цілісності пам’яті (HVCI)/захист на основі віртуалізації**: відхилено.
- **Повне вимкнення Windows Defender / Firewall**: відхилено.
- **Вимкнення файлу підкачки (`pagefile.sys`)**: відхилено (спричиняє збої в грі через стрибки пам’яті).
- **Вимкнення динамічного тику/інваріантного втручання TSC**: відхилено (може десинхронізувати годинники та спрацьовувати античіт-евристики).
- **Злом утиліти Bulk MSI (Message Signaled Interrupts)**: відхилено (ризик блокування GPU/синіх екранів).
