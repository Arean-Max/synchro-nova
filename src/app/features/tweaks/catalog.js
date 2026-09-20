export const tweakCatalog = [
  {
    id: "gaming_latency",
    groupKey: "tweakGroup_gaming_latency",
    icon: "zap",
    title: "Gaming & Latency",
    tweaks: [
      { id: "game-mode-on", title: "Enable Game Mode", description: "Enables Windows Game Mode hints and priority for active games.", badges: ["SAFE", "VERIFIED"] },
      { id: "modern-flip-model-on", title: "Modern Flip Model upgrade", description: "Forces Modern Flip presentation model for windowed and borderless games to prevent gamma resets and reduce latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "gamedvr-fse-mode", title: "DirectX FSE mode", description: "Enforces true Full Screen Exclusive presentation for games to minimize latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "disable-fso-globally", title: "Disable Fullscreen Optimizations", description: "Globally disables Windows FSO (Fullscreen Optimizations) for pure exclusive fullscreen and minimal display latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "hags-on", title: "Hardware GPU scheduling", description: "Enables Windows Hardware-accelerated GPU Scheduling when the driver supports it.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "mpo-disable", title: "Disable MPO", description: "Disables Multiplane Overlay for systems with flicker, black screens or overlay stutter.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "pcie-aspm-off", title: "Disable PCIe ASPM", description: "Disables PCIe Active State Power Management to lock PCI Express link at maximum throughput and eliminate bus latency.", badges: ["SAFE", "VERIFIED"] }
    ]
  },
  {
    id: "scheduler",
    groupKey: "tweakGroup_scheduler",
    icon: "cpu",
    title: "CPU & Performance",
    tweaks: [
      { id: "power-plan-high", title: "High performance plan", description: "Switches to the supported Windows high performance power scheme.", badges: ["SAFE", "VERIFIED"] },
      { id: "ultimate-performance-plan", title: "Ultimate performance plan", description: "Activates the hidden Windows Ultimate Performance power scheme.", badges: ["SAFE", "VERIFIED"] },
      { id: "cpu-unpark-cores", title: "Unpark CPU Cores", description: "Disables CPU core parking so 100% of logical cores remain active, eliminating core wake-up micro-stutters in games.", badges: ["ADMIN", "VERIFIED"] },
      { id: "power-throttling-off", title: "Disable Power Throttling", description: "Globally disables Windows Power Throttling to prevent Windows from downclocking game threads.", badges: ["ADMIN", "VERIFIED"] },
      { id: "mmcss-games-priority", title: "MMCSS game priority", description: "Raises the Games multimedia task priority hints used by MMCSS.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "system-responsiveness-10", title: "Minimal system reserve", description: "Reduces the MMCSS low-priority CPU reserve to give up to 100% CPU to games.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "network-throttle-off", title: "Network throttling off", description: "Removes MMCSS network throttling for latency-sensitive games and streams.", badges: ["ADMIN", "REBOOT", "VERIFIED"] }
    ]
  },
  {
    id: "input",
    groupKey: "tweakGroup_input",
    icon: "settings",
    title: "Input & Responsiveness",
    tweaks: [
      { id: "pointer-precision-off", title: "Disable pointer precision", description: "Turns off Windows mouse acceleration for consistent 1:1 raw aim feel.", badges: ["SAFE", "VERIFIED"] },
      { id: "input-response-fast", title: "Ultra-fast Input Queue", description: "Minimizes keyboard repeat delay, sets instant mouse hover time, and expands mouse/keyboard driver buffer queues.", badges: ["ADMIN", "VERIFIED"] },
      { id: "csrss-high-priority", title: "csrss.exe High Priority", description: "Sets csrss.exe (Client Server Runtime Subsystem) priority to High for instant input event dispatching even under 100% CPU load.", badges: ["ADMIN", "VERIFIED"] },
      { id: "sticky-keys-off", title: "Disable Sticky Keys", description: "Disables Shift x5 Sticky Keys and Filter Keys shortcut popups during gaming.", badges: ["SAFE", "VERIFIED"] },
      { id: "usb-selective-suspend-off", title: "USB selective suspend off", description: "Prevents USB devices from entering selective suspend to eliminate wake latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "visual-effects-performance", title: "Visual effects performance", description: "Switches Explorer visual effects to the Windows performance profile.", badges: ["SAFE", "VERIFIED"], note: "Some window animations and visual polish will be reduced." },
      { id: "transparency-off", title: "Transparency off", description: "Disables Windows transparency effects for a lighter desktop compositor path.", badges: ["SAFE", "VERIFIED"], note: "Acrylic and translucent shell surfaces will become solid." },
      { id: "menu-show-delay-low", title: "Fast desktop menus", description: "Reduces classic desktop menu delay to make shell interactions feel sharper.", badges: ["SAFE", "VERIFIED"] }
    ]
  },
  {
    id: "storage",
    groupKey: "tweakGroup_storage",
    icon: "archive",
    title: "Storage & Debloat",
    tweaks: [
      { id: "clean-temp-junk", title: "Clean Windows & Shader Junk", description: "Cleans temporary junk files, crash dumps, and DirectX shader cache to free gigabytes and eliminate stutters.", badges: ["SAFE", "VERIFIED"] },
      { id: "hibernate-off", title: "Hibernate off", description: "Disables hibernation to remove hiberfil.sys (freeing 8-32GB) and eliminates Fast Startup state corruption.", badges: ["ADMIN", "REBOOT", "VERIFIED"], note: "Hibernate and Windows Fast Startup will stop working." },
      { id: "ntfs-last-access-off", title: "NTFS last access off", description: "Disables last-access timestamp updates to reduce metadata writes and disk I/O.", badges: ["ADMIN", "VERIFIED"] },
      { id: "trim-enable", title: "Ensure TRIM enabled", description: "Enables Windows delete notifications for SSD/NVMe cleanup and write consistency.", badges: ["ADMIN", "VERIFIED"], note: "Useful for SSDs; preserves flash cell lifespan and write speed." }
    ]
  },
  {
    id: "network",
    groupKey: "tweakGroup_network",
    icon: "monitor",
    title: "Network Latency",
    tweaks: [
      { id: "tcp-nodelay-ack", title: "TCP NoDelay & AckFrequency", description: "Disables Nagle's algorithm and forces immediate TCP ACK without delay, cutting 40-200ms ping latency in online games.", badges: ["ADMIN", "VERIFIED"] },
      { id: "nic-energy-saving-off", title: "Disable NIC Energy Saving", description: "Disables Energy-Efficient Ethernet (EEE), Green Ethernet, and Flow Control on network cards to prevent ping spikes.", badges: ["ADMIN", "VERIFIED"] },
      { id: "tcp-heuristics-off", title: "Optimize TCP Windows & Heuristics", description: "Disables TCP heuristics and timestamps, setting autotuning to normal to eliminate packet loss and jitter.", badges: ["ADMIN", "VERIFIED"] },
      { id: "rss-on", title: "Receive-side scaling on", description: "Keeps network receive processing distributed across CPU cores, eliminating Core 0 bottleneck.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "rsc-off", title: "Receive segment coalescing off", description: "Reduces packet batching latency on network adapters for online multiplayer games.", badges: ["ADMIN", "VERIFIED"] },
      { id: "ecn-off", title: "ECN off", description: "Disables ECN for networks that drop packets or spike ping during ECN negotiation.", badges: ["ADMIN", "VERIFIED"] },
      { id: "dns-cache-flush", title: "DNS cache refresh", description: "Refreshes stale resolver state and address mappings for clean network resolution.", badges: ["SAFE", "VERIFIED"] }
    ]
  },
  {
    id: "background",
    groupKey: "tweakGroup_background",
    icon: "shield",
    title: "Background & Telemetry",
    tweaks: [
      { id: "disable-gamedvr", title: "Disable GameDVR", description: "Disables Windows GameDVR capture policy to free GPU video encoder resources.", badges: ["ADMIN", "VERIFIED"], note: "Xbox Game Bar recording and Win+Alt+R capture will stop working." },
      { id: "disable-bg-recording", title: "Disable background recording", description: "Stops passive clip recording and continuous video writing to disk.", badges: ["SAFE", "VERIFIED"], note: "Background clips and instant replay in Xbox Game Bar will be unavailable." },
      { id: "gamebar-startup-off", title: "Game Bar startup off", description: "Keeps Game Bar startup prompts and controller launch hooks quiet.", badges: ["SAFE", "VERIFIED"], note: "The Xbox button will no longer open Game Bar automatically." },
      { id: "wer-off", title: "Disable Error Reporting", description: "Disables Windows Error Reporting (WerFault) to eliminate crash lag spikes.", badges: ["SAFE", "VERIFIED"], note: "WerFault background diagnostics will be turned off." },
      { id: "start-bing-search-off", title: "Disable Start web search", description: "Disables Bing web search in Start menu for instant, 100% offline local search.", badges: ["SAFE", "PRIVACY", "VERIFIED"] },
      { id: "delivery-optimization-lan", title: "Delivery Optimization LAN only", description: "Prevents Windows Update from seeding updates across the internet during gaming.", badges: ["ADMIN", "PRIVACY", "VERIFIED"] },
      { id: "activity-history-off", title: "Activity history off", description: "Stops activity history feed, timeline publishing and cloud upload policies.", badges: ["ADMIN", "PRIVACY", "VERIFIED"] },
      { id: "advertising-id-off", title: "Advertising ID & tracking off", description: "Disables the Windows per-user advertising identifier and tailored diagnostic profiling.", badges: ["SAFE", "PRIVACY", "VERIFIED"] }
    ]
  }
];

export function tweakBadges(tweak) {
  return Array.isArray(tweak.badges) ? tweak.badges : tweak.badges ? [tweak.badges] : [];
}

export function tweakKey(tweak) {
  return String(tweak?.id || "").replace(/-/g, "_");
}

export function tweakTitle(tweak, t) {
  const key = `tweak_${tweakKey(tweak)}_title`;
  const val = t(key);
  return val && val !== key ? val : tweak.title;
}

export function tweakDescription(tweak, t) {
  const key = `tweak_${tweakKey(tweak)}_desc`;
  const val = t(key);
  return val && val !== key ? val : tweak.description;
}

export function tweakNote(tweak, t) {
  const key = `tweak_${tweakKey(tweak)}_note`;
  const val = t(key);
  if (val && val !== key) return val;
  return tweak.note || "";
}

export function tweakGroupTitle(group, t) {
  const key = group.groupKey || `tweakGroup_${group.id || ""}`;
  const val = t(key);
  return val && val !== key ? val : group.title;
}

export function tweakCategory(tweak) {
  const badges = tweakBadges(tweak);
  if (badges.includes("ADMIN")) {
    return "admin";
  }
  return "safe";
}

export function isSafeTweak(tweak) {
  return !tweakBadges(tweak).includes("ADMIN");
}

export function allTweaks() {
  return tweakCatalog.flatMap((group) => group.tweaks);
}

export const tweakAppImpacts = {
  "game-mode-on": {
    appId: "windows_game_mode",
    appName: "Windows Game Mode",
    impactEn: "Windows prioritizes GPU and CPU resources for the active game and suppresses background tasks.",
    impactRu: "Windows выделяет максимум ресурсов CPU и GPU активной игре, приостанавливая фоновые обновления и задачи."
  },
  "modern-flip-model-on": {
    appId: "directx_flip",
    appName: "DirectX / Modern Flip",
    impactEn: "Upgrades DX9/DX11 presentation to Modern Flip, eliminating DWM compositing latency and ICC color profile resets.",
    impactRu: "Переводит игры DX9/DX11 на Modern Flip: устраняет задержку DWM в оконном режиме и защищает от сброса калибровки цвета."
  },
  "gamedvr-fse-mode": {
    appId: "directx_fse",
    appName: "DirectX / FSE Mode",
    impactEn: "DirectX Full Screen Exclusive mode prioritizes raw frame delivery directly to display output.",
    impactRu: "Режим DirectX FSE отдает кадровый буфер напрямую дисплею в обход композитора рабочего стола."
  },
  "hags-on": {
    appId: "gpu_driver",
    appName: "GPU Driver / HAGS",
    impactEn: "Hardware-accelerated GPU Scheduling offloads frame scheduling to GPU, improving 1% low FPS in DX12 games.",
    impactRu: "Аппаратное планирование GPU разгружает процессор, снижает задержку кадров и повышает редкие события (1% Low FPS)."
  },
  "mpo-disable": {
    appId: "display_mpo",
    appName: "Multiplane Overlay (MPO)",
    impactEn: "Disables Multiplane Overlays, fixing black screens, flickering, and stuttering with Discord, browser, or multi-monitor setups.",
    impactRu: "Отключает Multiplane Overlay (MPO): устраняет мерцания экрана, микростаттеры и чёрные экраны при запущенном Discord и браузере."
  },
  "power-plan-high": {
    appId: "cpu_power",
    appName: "Схема электропитания",
    impactEn: "Switches Windows power scheme to High Performance, locking CPU core clocks to eliminate frequency ramp-up lag.",
    impactRu: "Включает схему высокой производительности: фиксирует частоты ядер процессора, исключая задержки при резкой нагрузке в играх."
  },
  "ultimate-performance-plan": {
    appId: "cpu_power",
    appName: "Схема электропитания Ultimate",
    impactEn: "Activates the hidden Windows Ultimate Performance power scheme, eliminating micro-latencies and aggressive core sleep.",
    impactRu: "Активирует скрытую схему «Максимальная производительность»: полностью исключает микрозадержки сна ядер процессора."
  },
  "mmcss-games-priority": {
    appId: "multimedia_scheduler",
    appName: "Windows MMCSS Scheduler",
    impactEn: "Assigns maximum scheduling and disk I/O priority to game threads in Windows Multimedia Class Scheduler.",
    impactRu: "Задаёт наивысший приоритет потоков и дискового ввода-вывода для игровых задач в планировщике MMCSS."
  },
  "system-responsiveness-10": {
    appId: "multimedia_scheduler",
    appName: "Резерв процессора MMCSS",
    impactEn: "Reduces background CPU reservation from 20% to 10%, giving up to 100% CPU capacity to games.",
    impactRu: "Снижает резерв процессора для фоновых служб с 20% до 10%, отдавая до 100% мощности процессора активной игре."
  },
  "network-throttle-off": {
    appId: "network_mmcss",
    appName: "Сетевой троттлинг MMCSS",
    impactEn: "Disables legacy MMCSS network packet throttling, eliminating gaming packet delays during audio playback.",
    impactRu: "Отключает древний лимит сетевых пакетов Windows при воспроизведении звука/музыки, убирая сетевой джиттер в играх."
  },
  "pointer-precision-off": {
    appId: "mouse_input",
    appName: "Курсор мыши / Raw Input",
    impactEn: "Disables Windows mouse acceleration for 1:1 true raw mouse input and consistent muscle memory aim in shooters.",
    impactRu: "Отключает нелинейное ускорение мыши («Повышенная точность»): даёт чистый ввод 1:1 Raw Input для стабильного аима в шутерах."
  },
  "sticky-keys-off": {
    appId: "keyboard_accessibility",
    appName: "Залипание клавиш (Shift x5)",
    impactEn: "Disables Shift x5 Sticky Keys and Filter Keys popups that can minimize or freeze fullscreen games.",
    impactRu: "Отключает диалоги залипания клавиш: частое нажатие Shift больше не свернёт полноэкранную игру в разгар боя."
  },
  "usb-selective-suspend-off": {
    appId: "usb_power",
    appName: "Энергосбережение USB",
    impactEn: "Prevents USB controllers from putting mice, keyboards, and DACs to sleep, eliminating initial movement lag.",
    impactRu: "Запрещает Windows переводить USB-порты в спящий режим: устраняет задержку отклика мыши, клавиатуры и звуковой карты."
  },
  "visual-effects-performance": {
    appId: "windows_shell",
    appName: "Эффекты проводника Windows",
    impactEn: "Disables heavy desktop animations, drop shadows, and window transition delays for a snappy UI.",
    impactRu: "Оптимизирует эффекты проводника под максимальное быстродействие: убирает тяжелые анимации окон и тени."
  },
  "transparency-off": {
    appId: "windows_shell",
    appName: "Прозрачность DWM / Acrylic",
    impactEn: "Disables Acrylic and transparent shell surfaces, freeing GPU VRAM and compositor rendering cycles.",
    impactRu: "Отключает полупрозрачность интерфейса Windows: разгружает видеопамять и ускоряет работу композитора рабочего стола."
  },
  "menu-show-delay-low": {
    appId: "windows_shell",
    appName: "Контекстные меню Windows",
    impactEn: "Reduces classic desktop menu delay to 100ms, making context menus and shell navigation instantaneous.",
    impactRu: "Снижает задержку контекстных меню Windows до 100 мс: интерфейс открывается мгновенно."
  },
  "clean-temp-junk": {
    appId: "storage_cleaner",
    appName: "Очистка мусора и кэша шейдеров",
    impactEn: "Safely cleans %TEMP%, Windows temp files, DirectX D3D shader cache, and crash dumps, freeing gigabytes and fixing stutters.",
    impactRu: "Безопасно удаляет временные файлы (%TEMP%), кэш шейдеров DirectX и дампы ошибок: освобождает гигабайты и убирает статтеры."
  },
  "hibernate-off": {
    appId: "fast_startup",
    appName: "Быстрый запуск и hiberfil.sys",
    impactEn: "Disables Windows hibernation, deletes hiberfil.sys (freeing 8-32GB of SSD space), and disables Fast Startup.",
    impactRu: "Отключает гибернацию и удаляет hiberfil.sys: освобождает от 8 до 32 ГБ на SSD и обеспечивает чистую загрузку Windows без накопленных ошибок."
  },
  "ntfs-last-access-off": {
    appId: "ntfs_filesystem",
    appName: "Файловая система NTFS",
    impactEn: "Stops NTFS from updating last-access timestamps on every file read, reducing disk write load during game loading.",
    impactRu: "Запрещает запись меток последнего доступа при чтении файлов: снижает лишнюю нагрузку на SSD при подгрузке игровых локаций."
  },
  "trim-enable": {
    appId: "ssd_trim",
    appName: "Команда TRIM для SSD",
    impactEn: "Ensures TRIM command is active for NVMe and SATA SSDs, preserving peak read/write speeds over time.",
    impactRu: "Проверяет и включает TRIM для SSD: гарантирует своевременную очистку ячеек памяти, предотвращая падение скорости со временем."
  },
  "rss-on": {
    appId: "network_adapter",
    appName: "Receive-Side Scaling (RSS)",
    impactEn: "Enables Receive-Side Scaling, spreading incoming network packet processing across multiple CPU cores.",
    impactRu: "Включает Receive-Side Scaling (RSS): распределяет обработку сетевых пакетов между ядрами CPU, устраняя перегрузку первого ядра."
  },
  "rsc-off": {
    appId: "network_adapter",
    appName: "Receive Segment Coalescing (RSC)",
    impactEn: "Disables Receive Segment Coalescing on network adapters, eliminating packet batching latency in online games.",
    impactRu: "Отключает Receive Segment Coalescing (RSC): исключает буферизацию и склейку пакетов, снижая сетевую задержку в шутерах."
  },
  "ecn-off": {
    appId: "network_tcp",
    appName: "ECN Capability",
    impactEn: "Disables Explicit Congestion Notification to prevent dropped packets with older routers and game servers.",
    impactRu: "Отключает ECN: устраняет внезапные потери пакетов и скачки пинга при игре на серверах, не поддерживающих ECN."
  },
  "dns-cache-flush": {
    appId: "network_dns",
    appName: "Кэш DNS",
    impactEn: "Flushes stale DNS resolver cache and address mappings for clean network server resolution.",
    impactRu: "Очищает кэш сопоставления DNS-имён: полезно при сбоях подключения к игровым серверам и для сброса устаревших маршрутов."
  },
  "disable-gamedvr": {
    appId: "xbox_game_bar",
    appName: "Xbox GameDVR",
    impactEn: "Disables Xbox GameDVR background recording service, freeing GPU hardware video encoder and memory.",
    impactRu: "Полностью отключает службу записи GameDVR: освобождает аппаратный видеокодер видеокарты и устраняет просадки кадров."
  },
  "disable-bg-recording": {
    appId: "xbox_game_bar",
    appName: "Фоновая запись клипов",
    impactEn: "Stops continuous background gameplay clip buffering, eliminating constant SSD writes and encoder usage.",
    impactRu: "Отключает фоновую непрерывную запись клипов: прекращает постоянную запись видео на накопитель и нагрузку на GPU."
  },
  "gamebar-startup-off": {
    appId: "xbox_game_bar",
    appName: "Автозапуск Game Bar",
    impactEn: "Disables Game Bar startup hooks and controller guide button intercepts.",
    impactRu: "Отключает автозапуск Game Bar и перехват кнопки Guide на контроллерах Xbox."
  },
  "wer-off": {
    appId: "windows_wer",
    appName: "Windows Error Reporting (WER)",
    impactEn: "Disables Windows Error Reporting (WerFault) to prevent system freeze spikes when background processes crash.",
    impactRu: "Отключает службу отчётов об ошибках WerFault: исключает микрофризы и зависания игр при сбоях фоновых программ."
  },
  "start-bing-search-off": {
    appId: "windows_search",
    appName: "Поиск Bing в меню «Пуск»",
    impactEn: "Disables Bing web search in Start menu, making desktop search 100% offline, private, and instant.",
    impactRu: "Отключает поиск Bing в меню «Пуск»: делает поиск локальным, мгновенным и прекращает отправку поисковых запросов в Microsoft."
  },
  "delivery-optimization-lan": {
    appId: "windows_update",
    appName: "Delivery Optimization",
    impactEn: "Restricts Windows Delivery Optimization to LAN only, preventing P2P update seeding from using your upload bandwidth during games.",
    impactRu: "Ограничивает раздачу обновлений только локальной сетью: запрещает Windows отдавать обновления в интернет во время игр."
  },
  "activity-history-off": {
    appId: "windows_privacy",
    appName: "История активности (Timeline)",
    impactEn: "Disables Windows user activity history tracking, timeline publishing, and cloud synchronization.",
    impactRu: "Отключает сбор истории активности пользователя (Timeline) и передачу логов в облако Microsoft."
  },
  "advertising-id-off": {
    appId: "windows_privacy",
    appName: "Рекламный идентификатор",
    impactEn: "Disables the Windows per-user advertising identifier and tailored diagnostic profiling.",
    impactRu: "Отключает персонализированный рекламный идентификатор Windows, предотвращая фоновый трекинг."
  },
  "disable-fso-globally": {
    appId: "directx_fso",
    appName: "Полноэкранная оптимизация (FSO)",
    impactEn: "Globally disables Windows Fullscreen Optimizations, unlocking true Exclusive Fullscreen with lowest possible render-to-display latency.",
    impactRu: "Глобально отключает оптимизацию во весь экран (FSO): даёт настоящий чистый Exclusive Fullscreen и минимальную задержку кадра."
  },
  "pcie-aspm-off": {
    appId: "pcie_bus",
    appName: "PCI Express ASPM",
    impactEn: "Disables PCIe Link State Power Management, keeping the GPU bus at peak speed and eliminating frame drop latency.",
    impactRu: "Отключает энергосбережение шины PCI Express: удерживает шину видеокарты на максимальной пропускной способности без задержек пробуждения."
  },
  "cpu-unpark-cores": {
    appId: "cpu_parking",
    appName: "Парковка ядер CPU (Core Parking)",
    impactEn: "Unparks 100% of CPU logical cores, keeping all cores instantly ready to execute game render threads without sleep latency.",
    impactRu: "Распарковывает все ядра процессора: удерживает 100% ядер в активном состоянии, устраняя микрофризы от пробуждения спящих ядер."
  },
  "power-throttling-off": {
    appId: "cpu_power",
    appName: "Windows Power Throttling",
    impactEn: "Disables Windows Power Throttling globally to prevent the scheduler from downclocking high-performance game threads.",
    impactRu: "Отключает механизм Power Throttling в Windows: исключает сброс частот процессора под игровой нагрузкой."
  },
  "input-response-fast": {
    appId: "mouse_keyboard_input",
    appName: "Буферы ввода мыши и клавиатуры",
    impactEn: "Sets zero repeat delay for keys, 8ms mouse hover responsiveness, and enlarges mouse/keyboard buffer queues for high polling rate devices (1000-8000Hz).",
    impactRu: "Убирает задержку повтора клавиш (0 мс), снижает задержку мыши до 8 мс и увеличивает буфер пакетов ввода для мышей с частотой 1000-8000 Гц."
  },
  "csrss-high-priority": {
    appId: "csrss_subsystem",
    appName: "Подсистема ввода csrss.exe",
    impactEn: "Raises csrss.exe process priority to High so raw mouse and keyboard events are dispatched immediately without hitching even under 100% CPU load.",
    impactRu: "Повышает приоритет процесса csrss.exe до Высокого: события мыши и клавиатуры обрабатываются мгновенно даже при 100% нагрузке на процессор."
  },
  "tcp-nodelay-ack": {
    appId: "network_nodelay",
    appName: "Алгоритм Нагла (TCP NoDelay)",
    impactEn: "Enables TCP NoDelay and AckFrequency=1 across all network adapters, eliminating the 40-200ms delayed ACK buffering and lowering multiplayer ping.",
    impactRu: "Отключает алгоритм Нагла и задержку подтверждения (AckFrequency=1): сетевые пакеты отправляются мгновенно, снижая пинг в онлайн-играх на 40-200 мс."
  },
  "nic-energy-saving-off": {
    appId: "network_adapter",
    appName: "Энергосбережение сетевой карты (EEE)",
    impactEn: "Disables Energy-Efficient Ethernet, Green Ethernet, and Flow Control on network cards, stopping packet throttling and sudden ping spikes.",
    impactRu: "Отключает энергосбережение сетевой карты (Green Ethernet, EEE) и Flow Control: предотвращает задержки и внезапные скачки пинга."
  },
  "tcp-heuristics-off": {
    appId: "network_tcp",
    appName: "Эвристика TCP и Timestamps",
    impactEn: "Disables TCP heuristics and timestamps while enforcing normal autotuning, preventing bufferbloat and packet drop spikes in online shooters.",
    impactRu: "Отключает TCP heuristics и timestamps, стабилизируя автоподстройку окна: предотвращает потерю пакетов и джиттер в шутерах."
  }
};
