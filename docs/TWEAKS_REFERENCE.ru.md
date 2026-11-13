# Synchro Nova — Справочник системных модификаций и настроек

В этом документе представлена ​​полная и прозрачная информация о каждом системном параметре, ключе реестра и командах, выполняемых Synchro Nova. Все действия строго локальны, детерминированы и резервируются перед выполнением.

---

## Гарантия безопасности и отката

1. **Автоматизированное резервное копирование перед выполнением**:
Перед изменением любого раздела реестра Synchro Nova создает точку восстановления системы и сохраняет текущее состояние всех целевых ключей в файле резервной копии `.reg` в `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Откат в 1 клик**:
Нажатие кнопки **"Откатить изменения"** (Откат) в приложении восстанавливает предыдущий снимок `.reg` через `reg.exe` Windows и перечитывает текущие значения.
3. **Усиленное выполнение команд**:
Все административные инструменты (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) строго разрешаются из `%SystemRoot%\System32` (предотвращая перехват повышенного PATH) и запускаются с неблокирующими тайм-аутами сторожевого таймера (5 секунд для утилит, 25 секунд для восстановления системы).
4. **Жесткий черный список и защита от мошенничества**:
Небезопасные операции (отключение целостности памяти/HVCI, исправление таймеров ядра, отключение файла подкачки или применение произвольных переопределений драйвера) навсегда блокируются в движке. Отсутствие внедрения процессов или перехвата памяти обеспечивает 100% соответствие требованиям Easy Anti-Cheat, BattlEye, Vanguard и VAC.

---

## Полная разбивка твиков

### 1. Настройки на уровне пользователя (стандартный пользователь — администратор не требуется)

Эти настройки изменяют только куст HKEY_CURRENT_USER. Они не требуют повышенных привилегий и влияют только на активный профиль пользователя.

| Tweak ID | Hive & Path | Value / Command | Цель |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Предотвращает всплывающее окно Xbox Game Bar при запуске игры. |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Отключает ускорение мыши Windows (необработанный линейный ввод) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Отключает рекламный идентификатор Windows. |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Отключает персональные советы на основе диагностических данных. |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Хранит буфер обмена строго в локальной оперативной памяти, отключает облачную синхронизацию. |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Отключает прозрачность окна DWM для экономии памяти графического процессора. |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Настраивает проводник Windows для наилучшей визуальной производительности. |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Устраняет искусственную задержку запуска рабочего стола Windows. |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Уменьшает задержку анимации меню пользовательского интерфейса до 10 мс. |

---

### 2. Настройки на уровне системы (требуются права администратора)

Эти настройки настраивают общесистемные политики («HKEY_LOCAL_MACHINE») или вызывают стандартные инструменты администрирования Windows («powercfg», «netsh», «fsutil»). Synchro проверяет высоту перед их выполнением.

#### A. Планирование игр и мультимедиа (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Цель |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Отключает фоновый захват видео Game DVR для бесплатного кодирования графического процессора. |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Отключает механизм регулирования сети Windows во время выполнения высокоприоритетных задач. |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Резервирует 90% процессорного времени для игр на переднем плане (по умолчанию 80%). |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Повышает приоритет задач планировщика мультимедиа для активных игр. |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Включает аппаратное ускорение планирования графического процессора (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Отключает многоплоскостные наложения (решает проблемы с черным экраном/заиканием на некоторых графических процессорах) |

#### Б. Оптимизация сетевого стека (netsh, ipconfig)
| Tweak ID | Tool / Command | Цель |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Очищает кэш преобразователя DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Восстанавливает стандартное масштабирование TCP-окна RFC 1323. |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Включает масштабирование на стороне приема для распределения обработки сетевых пакетов по ядрам ЦП. |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Отключает объединение сегментов приема на сетевых картах для уменьшения задержки в играх в реальном времени. |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Отключает явное уведомление о перегрузке, чтобы предотвратить потерю пакетов на старых маршрутизаторах. |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Очищает локальную таблицу протокола разрешения адресов. |
| `winsock-reset` | `netsh winsock reset` | Восстанавливает каталог сетевых сокетов Windows в чистое состояние. |

#### C. Управление питанием и хранилищем (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Цель |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Активирует стандартную схему питания Windows High Performance. |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Разблокирует и активирует схему питания Windows Ultimate Performance. |
| `hibernate-off` | `powercfg /hibernate off` | Отключает спящий режим и удаляет hiberfil.sys, освобождая место на диске. |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Прекращает обновление меток времени последнего доступа NTFS при каждом чтении файла. |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Обеспечивает активность уведомлений TRIM для проверки работоспособности SSD и выравнивания износа. |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Создает полную точку восстановления системы Windows перед изменениями. |

#### D. Системная телеметрия и фоновая синхронизация
| Tweak ID | Hive & Path | Value | Цель |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Отключает загрузку временной шкалы Windows и истории активности. |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Ограничивает оптимизацию доставки обновлений Windows только локальной локальной сетью (без P2P-загрузки в Интернет) |

---

## Небезопасные настройки, занесенные в черный список

Synchro намеренно блокирует несколько распространенных «агрессивных настроек», обнаруженных в сторонних скриптах, которые ставят под угрозу целостность системы или совместимость с античитами:

- **Отключение целостности памяти (HVCI)/безопасности на основе виртуализации**: Отклонено.
- **Полное отключение Защитника Windows/Брандмауэра**: Отклонено.
- **Отключение файла подкачки (`pagefile.sys`)**: Отклонено (приводит к сбою игры при скачках памяти).
- **Отключение динамического тактового/инвариантного вмешательства в TSC**: Отклонено (может привести к десинхронизации часов и отключению античит-эвристики).
- **Массовый взлом утилиты MSI (прерывания с сигнализацией сообщений)**: отклонено (риск блокировки графического процессора или появления синих экранов).
