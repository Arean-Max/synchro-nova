export const tweakCatalog = [
  {
    id: "gaming_latency",
    groupKey: "tweakGroup_gaming_latency",
    icon: "zap",
    title: "Gaming & Latency",
    tweaks: [
      { id: "win32-priority-rust", title: "Win32 Priority Separation", description: "Allocates long fixed-length processor quantums to Rust and active games, stabilizing 1% Low FPS.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "system-timer-resolution", title: "System Timer Resolution (0.5 ms)", description: "Forces Windows global timer resolution to 0.5 ms to smooth frame pacing and eliminate micro-stutters during weapon spray.", badges: ["ADMIN", "VERIFIED"] },
      { id: "kill-gamebar-presence", title: "Kill GameBar Presence Writer", description: "Completely disables GameBarPresenceWriter and GameDVR hooks that cause frame time spikes in Unity games.", badges: ["ADMIN", "VERIFIED"] },
      { id: "directx-thread-priority", title: "DirectX GPU Scheduling & TdrDelay", description: "Enables GPU preemption and extends DirectX TDR delay to 10s to eliminate GPU hang spikes and driver timeouts.", badges: ["ADMIN", "VERIFIED"] },
      { id: "gpu-adaptive-power", title: "Hardware-Adaptive GPU Power", description: "Detects GPU vendor: disables PowerMizer downclocking on NVIDIA or disables ULPS sleep state on AMD for locked framerates.", badges: ["ADMIN", "VERIFIED"] },
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
      { id: "cpu-adaptive-scheduling", title: "Hardware-Adaptive CPU Scheduling", description: "Detects Intel Hybrid P/E-cores or AMD Ryzen: pins game threads to high-performance P-cores or locks multi-core quantums.", badges: ["ADMIN", "VERIFIED"] },
      { id: "system-worker-threads", title: "Adaptive System Worker Threads", description: "Dynamically expands Windows kernel executive worker threads based on detected CPU core count to eliminate I/O starvation.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "mmcss-games-priority", title: "MMCSS game priority", description: "Raises the Games multimedia task priority hints used by MMCSS.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "system-responsiveness-0", title: "Dedicated 100% CPU to Games", description: "Sets MMCSS system responsiveness reserve to 0% to allocate full processor capacity to active games.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
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
      { id: "usb-selective-suspend-off", title: "USB selective suspend off", description: "Prevents USB devices from entering selective suspend to eliminate wake latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "visual-effects-performance", title: "Visual effects performance", description: "Switches Explorer visual effects to the Windows performance profile.", badges: ["SAFE", "VERIFIED"], note: "Some window animations and visual polish will be reduced." },
      { id: "transparency-off", title: "Transparency off", description: "Disables Windows transparency effects for a lighter desktop compositor path.", badges: ["SAFE", "VERIFIED"], note: "Acrylic and translucent shell surfaces will become solid." }
    ]
  },
  {
    id: "storage",
    groupKey: "tweakGroup_storage",
    icon: "archive",
    title: "Storage & Debloat",
    tweaks: [
      { id: "clean-temp-junk", title: "Clean Windows & Shader Junk", description: "Cleans temporary junk files, crash dumps, and DirectX shader cache to free gigabytes and eliminate stutters.", badges: ["SAFE", "VERIFIED"] },
      { id: "disable-paging-executive", title: "Lock Kernel & Drivers in RAM", description: "Forces Windows to keep system kernel and drivers in physical RAM instead of paging them to disk, eliminating asset-load freezes.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "disable-memory-compression", title: "Disable Memory Compression", description: "Disables Windows background memory compression to free CPU cores and avoid compression overhead during intense gunfights.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "disable-page-combining", title: "Disable Memory Deduplication", description: "Disables background Windows Page Combining RAM scans that induce micro-stutters during heavy gameplay.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "adaptive-io-page-lock", title: "Hardware-Adaptive IoPageLockLimit", description: "Dynamically allocates 512MB-1GB physical memory I/O transfer limits based on total RAM for zero-hitch monument streaming.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "large-system-cache-off", title: "Prioritize RAM for Game Execution", description: "Sets LargeSystemCache to 0 so Windows dedicates physical RAM to game processes rather than filesystem cache.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
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
      { id: "network-udp-buffers", title: "Hardware-Adaptive RakNet Buffers", description: "Optimizes AFD datagram thresholds and dynamically sizes socket buffers (512KB-1MB) based on RAM for zero packet loss.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "tcp-nodelay-ack", title: "TCP NoDelay & AckFrequency", description: "Disables Nagle's algorithm and forces immediate TCP ACK without delay, cutting 40-200ms ping latency in online games.", badges: ["ADMIN", "VERIFIED"] },
      { id: "nic-energy-saving-off", title: "Disable NIC Energy Saving", description: "Disables Energy-Efficient Ethernet (EEE), Green Ethernet, and Flow Control on network cards to prevent ping spikes.", badges: ["ADMIN", "VERIFIED"] },
      { id: "tcp-heuristics-off", title: "Optimize TCP Windows & Heuristics", description: "Disables TCP heuristics and timestamps, setting autotuning to normal to eliminate packet loss and jitter.", badges: ["ADMIN", "VERIFIED"] },
      { id: "rss-adaptive-cores", title: "Hardware-Adaptive RSS Core Affinity", description: "Enables Receive-Side Scaling with BaseProcessor=2 to steer network interrupts off Core 0 and protect game simulation threads.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "rsc-off", title: "Receive segment coalescing off", description: "Reduces packet batching latency on network adapters for online multiplayer games.", badges: ["ADMIN", "VERIFIED"] }
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
      { id: "wer-off", title: "Disable Error Reporting", description: "Disables Windows Error Reporting (WerFault) to eliminate crash lag spikes.", badges: ["SAFE", "VERIFIED"], note: "WerFault background diagnostics will be turned off." }
    ]
  }
];

function tweakBadges(tweak) {
  return Array.isArray(tweak.badges) ? tweak.badges : tweak.badges ? [tweak.badges] : [];
}

function tweakKey(tweak) {
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

export function allTweaks() {
  return tweakCatalog.flatMap((group) => group.tweaks);
}

export const tweakAppImpacts = {
  "game-mode-on": {
    appId: "windows_game_mode",
    appName: "Windows Game Mode",
    appNameEn: "Windows Game Mode",
    appNameRu: "Игровой режим Windows",
    impactEn: "Windows prioritizes GPU and CPU resources for the active game and suppresses background tasks.",
    impactRu: "Windows выделяет максимум ресурсов CPU и GPU активной игре, приостанавливая фоновые обновления и задачи."
  },
  "win32-priority-rust": {
    appId: "win32_priority",
    appName: "Win32 Priority Separation",
    appNameEn: "Win32 Priority Separation",
    appNameRu: "Приоритеты Win32 / Кванты процессора",
    impactEn: "Allocates long fixed-length processor quantums (0x26 / 38) to foreground games, dramatically boosting 1% Low FPS stability.",
    impactRu: "Выделяет длинные фиксированные кванты времени процессора (0x26 / 38) активной игре, обеспечивая +5-12% к стабильности 1% Low FPS."
  },
  "gpu-adaptive-power": {
    appId: "gpu_power",
    appName: "Hardware-Adaptive GPU Power",
    appNameEn: "Hardware-Adaptive GPU Power",
    appNameRu: "Адаптивное энергопотребление GPU",
    impactEn: "Detects GPU vendor: disables PowerMizer downclocking on NVIDIA or disables ULPS sleep on AMD to prevent sudden clock drops and stutters in games.",
    impactRu: "Определяет видеокарту: отключает сброс частот PowerMizer на NVIDIA или отключает ULPS на AMD, устраняя просадки частот и микрофризы."
  },
  "system-timer-resolution": {
    appId: "system_timer",
    appName: "System Timer Resolution (0.5 ms)",
    appNameEn: "System Timer Resolution (0.5 ms)",
    appNameRu: "Системный таймер 0.5 мс",
    impactEn: "Enforces 0.5 ms global timer resolution (GlobalTimerResolutionRequests = 1), smoothing frame delivery and spray tracking in Rust.",
    impactRu: "Переводит системный таймер прерываний с 15.6 мс на 0.5 мс: выравнивает фреймтайм и делает спрей оружия идеально плавным."
  },
  "kill-gamebar-presence": {
    appId: "gamebar_presence",
    appName: "GameBarPresenceWriter",
    appNameEn: "GameBarPresenceWriter",
    appNameRu: "GameBarPresenceWriter",
    impactEn: "Completely disables GameBarPresenceWriter and background presence tracking that causes frametime spikes in Unity engine games.",
    impactRu: "Полностью устраняет системный процесс GameBarPresenceWriter, вызывающий скачки задержки кадров в играх на движке Unity."
  },
  "directx-thread-priority": {
    appId: "directx_gpu",
    appName: "DirectX GPU Scheduling & TdrDelay",
    appNameEn: "DirectX GPU Scheduling & TdrDelay",
    appNameRu: "DirectX планирование GPU и TdrDelay",
    impactEn: "Enables GPU preemption and sets TDR recovery delay to 10s, preventing GPU timeout driver crashes and hang spikes.",
    impactRu: "Включает вытесняющую многозадачность GPU и увеличивает задержку TDR до 10 сек, предотвращая вылеты видеодрайвера при тяжелых сценах."
  },
  "disable-paging-executive": {
    appId: "memory_paging",
    appName: "Kernel Memory Paging",
    appNameEn: "Kernel Memory Paging",
    appNameRu: "Блокировка ядра в RAM",
    impactEn: "Forces Windows to keep kernel executive and device drivers locked in physical RAM, eliminating disk page-in stutters during base loading.",
    impactRu: "Запрещает Windows выгружать системные драйверы и ядро в файл подкачки на диск, устраняя фризы при подгрузке баз в Расте."
  },
  "disable-memory-compression": {
    appId: "memory_compression",
    appName: "Memory Compression",
    appNameEn: "Memory Compression",
    appNameRu: "Сжатие памяти Windows",
    impactEn: "Disables Windows background RAM compression, freeing CPU cores from compression work during gunfights.",
    impactRu: "Отключает фоновое сжатие памяти Windows, освобождая ядра процессора для рендеринга и предотвращая микрофризы при стрельбе."
  },
  "large-system-cache-off": {
    appId: "system_cache",
    appName: "Large System Cache",
    appNameEn: "Large System Cache",
    appNameRu: "Файловый кэш LargeSystemCache",
    impactEn: "Sets LargeSystemCache to 0, dedicating physical memory directly to game processes rather than Windows filesystem cache.",
    impactRu: "Отключает LargeSystemCache, отдавая максимум оперативной памяти активным играм вместо дискового кэша."
  },
  "network-udp-buffers": {
    appId: "raknet_udp",
    appName: "RakNet UDP Network Buffers",
    appNameEn: "RakNet UDP Network Buffers",
    appNameRu: "Буферы RakNet UDP в Rust",
    impactEn: "Expands AFD FastSendDatagramThreshold and socket buffers for Rust's RakNet UDP networking, preventing packet loss in 50+ player fights.",
    impactRu: "Оптимизирует буферы сокетов AFD для протокола RakNet в Rust: исключает потерю пакетов в масштабных файтах 50+ человек и рейдах."
  },
  "modern-flip-model-on": {
    appId: "directx_flip",
    appName: "DirectX / Modern Flip",
    appNameEn: "DirectX / Modern Flip",
    appNameRu: "DirectX / Modern Flip",
    impactEn: "Upgrades DX9/DX11 presentation to Modern Flip, eliminating DWM compositing latency and ICC color profile resets.",
    impactRu: "Переводит игры DX9/DX11 на Modern Flip: устраняет задержку DWM в оконном режиме и защищает от сброса калибровки цвета."
  },
  "gamedvr-fse-mode": {
    appId: "directx_fse",
    appName: "DirectX / FSE Mode",
    appNameEn: "DirectX / FSE Mode",
    appNameRu: "DirectX / FSE Mode",
    impactEn: "DirectX Full Screen Exclusive mode prioritizes raw frame delivery directly to display output.",
    impactRu: "Режим DirectX FSE отдает кадровый буфер напрямую дисплею в обход композитора рабочего стола."
  },
  "hags-on": {
    appId: "gpu_driver",
    appName: "GPU Driver / HAGS",
    appNameEn: "GPU Driver / HAGS",
    appNameRu: "Драйвер GPU / HAGS",
    impactEn: "Hardware-accelerated GPU Scheduling offloads frame scheduling to GPU, improving 1% low FPS in DX12 games.",
    impactRu: "Аппаратное планирование GPU разгружает процессор, снижает задержку кадров и повышает редкие события (1% Low FPS)."
  },
  "mpo-disable": {
    appId: "display_mpo",
    appName: "Multiplane Overlay (MPO)",
    appNameEn: "Multiplane Overlay (MPO)",
    appNameRu: "Multiplane Overlay (MPO)",
    impactEn: "Disables Multiplane Overlays, fixing black screens, flickering, and stuttering with Discord, browser, or multi-monitor setups.",
    impactRu: "Отключает Multiplane Overlay (MPO): устраняет мерцания экрана, микростаттеры и чёрные экраны при запущенном Discord и браузере."
  },
  "power-plan-high": {
    appId: "cpu_power",
    appName: "Power Scheme (High Performance)",
    appNameEn: "Power Scheme (High Performance)",
    appNameRu: "Схема электропитания",
    impactEn: "Switches Windows power scheme to High Performance, locking CPU core clocks to eliminate frequency ramp-up lag.",
    impactRu: "Включает схему высокой производительности: фиксирует частоты ядер процессора, исключая задержки при резкой нагрузке в играх."
  },
  "ultimate-performance-plan": {
    appId: "cpu_power",
    appName: "Power Scheme (Ultimate Performance)",
    appNameEn: "Power Scheme (Ultimate Performance)",
    appNameRu: "Схема электропитания Ultimate",
    impactEn: "Activates the hidden Windows Ultimate Performance power scheme, eliminating micro-latencies and aggressive core sleep.",
    impactRu: "Активирует скрытую схему «Максимальная производительность»: полностью исключает микрозадержки сна ядер процессора."
  },
  "mmcss-games-priority": {
    appId: "multimedia_scheduler",
    appName: "Windows MMCSS Scheduler",
    appNameEn: "Windows MMCSS Scheduler",
    appNameRu: "Планировщик MMCSS",
    impactEn: "Assigns maximum scheduling and disk I/O priority to game threads in Windows Multimedia Class Scheduler.",
    impactRu: "Задаёт наивысший приоритет потоков и дискового ввода-вывода для игровых задач в планировщике MMCSS."
  },
  "cpu-adaptive-scheduling": {
    appId: "cpu_scheduling",
    appName: "Hardware-Adaptive CPU Scheduling",
    appNameEn: "Hardware-Adaptive CPU Scheduling",
    appNameRu: "Адаптивное планирование CPU",
    impactEn: "Detects Intel Hybrid architecture to force game threads onto performance P-cores, or pins cores and quantums on AMD Ryzen for peak FPS.",
    impactRu: "Определяет процессор: фиксирует игровые потоки на мощных P-ядрах на Intel Hybrid или оптимизирует кванты ядер на AMD Ryzen для максимального FPS."
  },
  "system-worker-threads": {
    appId: "kernel_workers",
    appName: "Adaptive System Worker Threads",
    appNameEn: "Adaptive System Worker Threads",
    appNameRu: "Системные рабочие потоки ядра",
    impactEn: "Expands Windows kernel executive worker threads dynamically based on CPU core count, eliminating I/O starvation during intense gaming.",
    impactRu: "Динамически расширяет пул системных потоков ядра под количество ядер CPU, предотвращая задержки очередей ввода-вывода в Расте."
  },
  "system-responsiveness-0": {
    appId: "multimedia_scheduler",
    appName: "MMCSS CPU Reserve (0%)",
    appNameEn: "MMCSS CPU Reserve (0%)",
    appNameRu: "Резерв процессора MMCSS (0%)",
    impactEn: "Sets MMCSS system responsiveness reserve to 0%, dedicating 100% of CPU capacity exclusively to the active game process.",
    impactRu: "Устанавливает резерв процессора MMCSS на 0%: отдает 100% мощности процессора активной игре без резервирования под фоновые службы."
  },
  "network-throttle-off": {
    appId: "network_mmcss",
    appName: "MMCSS Network Throttling",
    appNameEn: "MMCSS Network Throttling",
    appNameRu: "Сетевой троттлинг MMCSS",
    impactEn: "Disables legacy MMCSS network packet throttling, eliminating gaming packet delays during audio playback.",
    impactRu: "Отключает древний лимит сетевых пакетов Windows при воспроизведении звука/музыки, убирая сетевой джиттер в играх."
  },
  "pointer-precision-off": {
    appId: "mouse_input",
    appName: "Mouse Cursor / Raw Input",
    appNameEn: "Mouse Cursor / Raw Input",
    appNameRu: "Курсор мыши / Raw Input",
    impactEn: "Disables Windows mouse acceleration for 1:1 true raw mouse input and consistent muscle memory aim in shooters.",
    impactRu: "Отключает нелинейное ускорение мыши («Повышенная точность»): даёт чистый ввод 1:1 Raw Input для стабильного аима в шутерах."
  },
  "usb-selective-suspend-off": {
    appId: "usb_power",
    appName: "USB Power Saving",
    appNameEn: "USB Power Saving",
    appNameRu: "Энергосбережение USB",
    impactEn: "Prevents USB controllers from putting mice, keyboards, and DACs to sleep, eliminating initial movement lag.",
    impactRu: "Запрещает Windows переводить USB-порты в спящий режим: устраняет задержку отклика мыши, клавиатуры и звуковой карты."
  },
  "visual-effects-performance": {
    appId: "windows_shell",
    appName: "Windows Shell Visual Effects",
    appNameEn: "Windows Shell Visual Effects",
    appNameRu: "Эффекты проводника Windows",
    impactEn: "Disables heavy desktop animations, drop shadows, and window transition delays for a snappy UI.",
    impactRu: "Оптимизирует эффекты проводника под максимальное быстродействие: убирает тяжелые анимации окон и тени."
  },
  "transparency-off": {
    appId: "windows_shell",
    appName: "DWM Transparency / Acrylic",
    appNameEn: "DWM Transparency / Acrylic",
    appNameRu: "Прозрачность DWM / Acrylic",
    impactEn: "Disables Acrylic and transparent shell surfaces, freeing GPU VRAM and compositor rendering cycles.",
    impactRu: "Отключает полупрозрачность интерфейса Windows: разгружает видеопамять и ускоряет работу композитора рабочего стола."
  },
  "clean-temp-junk": {
    appId: "storage_cleaner",
    appName: "Junk & Shader Cache Cleaner",
    appNameEn: "Junk & Shader Cache Cleaner",
    appNameRu: "Очистка мусора и кэша шейдеров",
    impactEn: "Safely cleans %TEMP%, Windows temp files, DirectX D3D shader cache, and crash dumps, freeing gigabytes and fixing stutters.",
    impactRu: "Безопасно удаляет временные файлы (%TEMP%), кэш шейдеров DirectX и дампы ошибок: освобождает гигабайты и убирает статтеры."
  },
  "disable-page-combining": {
    appId: "memory_dedup",
    appName: "Disable Page Combining",
    appNameEn: "Disable Page Combining",
    appNameRu: "Отключение Page Combining",
    impactEn: "Disables Windows kernel memory deduplication scans that induce periodic CPU spikes and TLB invalidation stutters during gunfights.",
    impactRu: "Отключает фоновое объединение страниц памяти Windows: исключает периодические подергивания и задержки TLB во время перестрелок."
  },
  "adaptive-io-page-lock": {
    appId: "io_page_lock",
    appName: "Hardware-Adaptive IoPageLockLimit",
    appNameEn: "Hardware-Adaptive IoPageLockLimit",
    appNameRu: "Адаптивный буфер ввода-вывода (IoPageLockLimit)",
    impactEn: "Allocates 512MB-1GB physical RAM for direct I/O transfers based on total memory, eliminating hitching when streaming monuments and clan bases.",
    impactRu: "Выделяет 512–1024 МБ памяти под прямой ввод-вывод в зависимости от объема RAM: полностью устраняет фризы при подгрузке РТ и огромных баз."
  },
  "hibernate-off": {
    appId: "fast_startup",
    appName: "Fast Startup & hiberfil.sys",
    appNameEn: "Fast Startup & hiberfil.sys",
    appNameRu: "Быстрый запуск и hiberfil.sys",
    impactEn: "Disables Windows hibernation, deletes hiberfil.sys (freeing 8-32GB of SSD space), and disables Fast Startup.",
    impactRu: "Отключает гибернацию и удаляет hiberfil.sys: освобождает от 8 до 32 ГБ на SSD и обеспечивает чистую загрузку Windows без накопленных ошибок."
  },
  "ntfs-last-access-off": {
    appId: "ntfs_filesystem",
    appName: "NTFS File System",
    appNameEn: "NTFS File System",
    appNameRu: "Файловая система NTFS",
    impactEn: "Stops NTFS from updating last-access timestamps on every file read, reducing disk write load during game loading.",
    impactRu: "Запрещает запись меток последнего доступа при чтении файлов: снижает лишнюю нагрузку на SSD при подгрузке игровых локаций."
  },
  "trim-enable": {
    appId: "ssd_trim",
    appName: "TRIM Command for SSD",
    appNameEn: "TRIM Command for SSD",
    appNameRu: "Команда TRIM для SSD",
    impactEn: "Ensures TRIM command is active for NVMe and SATA SSDs, preserving peak read/write speeds over time.",
    impactRu: "Проверяет и включает TRIM для SSD: гарантирует своевременную очистку ячеек памяти, предотвращая падение скорости со временем."
  },
  "rss-adaptive-cores": {
    appId: "network_rss",
    appName: "Hardware-Adaptive Receive-Side Scaling",
    appNameEn: "Hardware-Adaptive Receive-Side Scaling",
    appNameRu: "Адаптивный Receive-Side Scaling (RSS)",
    impactEn: "Enables RSS and steers network packet processing off Core 0 to protect Rust's main game simulation thread from DPC interrupt lag.",
    impactRu: "Включает RSS со смещением BaseProcessor=2: убирает сетевые прерывания с ядра 0, освобождая его для главного потока рендеринга Rust."
  },
  "rsc-off": {
    appId: "network_adapter",
    appName: "Receive Segment Coalescing (RSC)",
    appNameEn: "Receive Segment Coalescing (RSC)",
    appNameRu: "Receive Segment Coalescing (RSC)",
    impactEn: "Disables Receive Segment Coalescing on network adapters, eliminating packet batching latency in online games.",
    impactRu: "Отключает Receive Segment Coalescing (RSC): исключает буферизацию и склейку пакетов, снижая сетевую задержку в шутерах."
  },
  "disable-gamedvr": {
    appId: "xbox_game_bar",
    appName: "Xbox GameDVR",
    appNameEn: "Xbox GameDVR",
    appNameRu: "Xbox GameDVR",
    impactEn: "Disables Xbox GameDVR background recording service, freeing GPU hardware video encoder and memory.",
    impactRu: "Полностью отключает службу записи GameDVR: освобождает аппаратный видеокодер видеокарты и устраняет просадки кадров."
  },
  "disable-bg-recording": {
    appId: "xbox_game_bar",
    appName: "Background Clip Recording",
    appNameEn: "Background Clip Recording",
    appNameRu: "Фоновая запись клипов",
    impactEn: "Stops continuous background gameplay clip buffering, eliminating constant SSD writes and encoder usage.",
    impactRu: "Отключает фоновую непрерывную запись клипов: прекращает постоянную запись видео на накопитель и нагрузку на GPU."
  },
  "wer-off": {
    appId: "windows_wer",
    appName: "Windows Error Reporting (WER)",
    appNameEn: "Windows Error Reporting (WER)",
    appNameRu: "Отчёты об ошибках (WER)",
    impactEn: "Disables Windows Error Reporting (WerFault) to prevent system freeze spikes when background processes crash.",
    impactRu: "Отключает службу отчётов об ошибках WerFault: исключает микрофризы и зависания игр при сбоях фоновых программ."
  },
  "disable-fso-globally": {
    appId: "directx_fso",
    appName: "Fullscreen Optimization (FSO)",
    appNameEn: "Fullscreen Optimization (FSO)",
    appNameRu: "Полноэкранная оптимизация (FSO)",
    impactEn: "Globally disables Windows Fullscreen Optimizations, unlocking true Exclusive Fullscreen with lowest possible render-to-display latency.",
    impactRu: "Глобально отключает оптимизацию во весь экран (FSO): даёт настоящий чистый Exclusive Fullscreen и минимальную задержку кадра."
  },
  "pcie-aspm-off": {
    appId: "pcie_bus",
    appName: "PCI Express ASPM",
    appNameEn: "PCI Express ASPM",
    appNameRu: "PCI Express ASPM",
    impactEn: "Disables PCIe Link State Power Management, keeping the GPU bus at peak speed and eliminating frame drop latency.",
    impactRu: "Отключает энергосбережение шины PCI Express: удерживает шину видеокарты на максимальной пропускной способности без задержек пробуждения."
  },
  "cpu-unpark-cores": {
    appId: "cpu_parking",
    appName: "CPU Core Parking",
    appNameEn: "CPU Core Parking",
    appNameRu: "Парковка ядер CPU (Core Parking)",
    impactEn: "Unparks 100% of CPU logical cores, keeping all cores instantly ready to execute game render threads without sleep latency.",
    impactRu: "Распарковывает все ядра процессора: удерживает 100% ядер в активном состоянии, устраняя микрофризы от пробуждения спящих ядер."
  },
  "power-throttling-off": {
    appId: "cpu_power",
    appName: "Windows Power Throttling",
    appNameEn: "Windows Power Throttling",
    appNameRu: "Windows Power Throttling",
    impactEn: "Disables Windows Power Throttling globally to prevent the scheduler from downclocking high-performance game threads.",
    impactRu: "Отключает механизм Power Throttling в Windows: исключает сброс частот процессора под игровой нагрузкой."
  },
  "input-response-fast": {
    appId: "mouse_keyboard_input",
    appName: "Mouse & Keyboard Input Buffers",
    appNameEn: "Mouse & Keyboard Input Buffers",
    appNameRu: "Буферы ввода мыши и клавиатуры",
    impactEn: "Sets zero repeat delay for keys, 8ms mouse hover responsiveness, and enlarges mouse/keyboard buffer queues for high polling rate devices (1000-8000Hz).",
    impactRu: "Убирает задержку повтора клавиш (0 мс), снижает задержку мыши до 8 мс и увеличивает буфер пакетов ввода для мышей с частотой 1000-8000 Гц."
  },
  "tcp-nodelay-ack": {
    appId: "network_nodelay",
    appName: "Nagle's Algorithm (TCP NoDelay)",
    appNameEn: "Nagle's Algorithm (TCP NoDelay)",
    appNameRu: "Алгоритм Нагла (TCP NoDelay)",
    impactEn: "Enables TCP NoDelay and AckFrequency=1 across all network adapters, eliminating the 40-200ms delayed ACK buffering and lowering multiplayer ping.",
    impactRu: "Отключает алгоритм Нагла и задержку подтверждения (AckFrequency=1): сетевые пакеты отправляются мгновенно, снижая пинг в онлайн-играх на 40-200 мс."
  },
  "nic-energy-saving-off": {
    appId: "network_adapter",
    appName: "NIC Energy Saving (EEE)",
    appNameEn: "NIC Energy Saving (EEE)",
    appNameRu: "Энергосбережение сетевой карты (EEE)",
    impactEn: "Disables Energy-Efficient Ethernet, Green Ethernet, and Flow Control on network cards, stopping packet throttling and sudden ping spikes.",
    impactRu: "Отключает энергосбережение сетевой карты (Green Ethernet, EEE) и Flow Control: предотвращает задержки и внезапные скачки пинга."
  },
  "tcp-heuristics-off": {
    appId: "network_tcp",
    appName: "TCP Heuristics & Timestamps",
    appNameEn: "TCP Heuristics & Timestamps",
    appNameRu: "Эвристика TCP и Timestamps",
    impactEn: "Disables TCP heuristics and timestamps while enforcing normal autotuning, preventing bufferbloat and packet drop spikes in online shooters.",
    impactRu: "Отключает TCP heuristics и timestamps, стабилизируя автоподстройку окна: предотвращает потерю пакетов и джиттер в шутерах."
  }
};
