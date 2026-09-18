export const tweakCatalog = [
  {
    id: "capture",
    groupKey: "tweakGroup_capture",
    icon: "video",
    title: "Capture",
    tweaks: [
      { id: "game-mode-on", title: "Enable Game Mode", description: "Enables Windows Game Mode hints for active games.", badges: ["SAFE", "VERIFIED"] },
      { id: "disable-gamedvr", title: "Disable GameDVR", description: "Disables Windows GameDVR capture policy and user capture toggles.", badges: ["ADMIN", "VERIFIED"], note: "Xbox Game Bar recording and Win+Alt+R capture can stop working." },
      { id: "disable-bg-recording", title: "Disable background recording", description: "Stops passive clip recording for the current Windows user.", badges: ["SAFE", "VERIFIED"], note: "Background clips and instant replay in Xbox Game Bar will be unavailable." },
      { id: "gamebar-startup-off", title: "Game Bar startup off", description: "Keeps Game Bar startup prompts and controller launch hooks quiet.", badges: ["SAFE"], note: "The Xbox button may stop opening Game Bar automatically." },
      { id: "fullscreen-latency", title: "Fullscreen latency profile", description: "Advisory profile for games that expose their own fullscreen latency path.", badges: ["EXPERIMENTAL"] },
      { id: "overlay-audit", title: "Overlay audit", description: "Flags capture and chat overlays for manual benchmark review.", badges: ["SAFE"] }
    ]
  },
  {
    id: "scheduler",
    groupKey: "tweakGroup_scheduler",
    icon: "cpu",
    title: "Scheduler & Power",
    tweaks: [
      { id: "mmcss-games-priority", title: "MMCSS game priority", description: "Raises the Games multimedia task priority hints used by MMCSS.", badges: ["ADMIN", "REBOOT", "ADVANCED"] },
      { id: "system-responsiveness-10", title: "Minimal system reserve", description: "Reduces the MMCSS low-priority CPU reserve from the default desktop profile.", badges: ["ADMIN", "REBOOT", "ADVANCED"] },
      { id: "power-plan-high", title: "High performance plan", description: "Switches to the supported Windows high performance power scheme.", badges: ["VERIFIED"] },
      { id: "ultimate-performance-plan", title: "Ultimate performance plan", description: "Attempts to enable the hidden Windows Ultimate Performance scheme.", badges: ["EXPERIMENTAL"] },
      { id: "power-throttle-audit", title: "Power throttling audit", description: "Reviews throttling candidates before disabling anything globally.", badges: ["SAFE"] },
      { id: "core-parking-off", title: "Disable core parking", description: "Keeps CPU cores awake; useful only for specific desktops with thermal headroom.", badges: ["ADMIN", "AGGRESSIVE", "EXPERIMENTAL"] },
      { id: "processor-idle-min", title: "Processor idle minimum", description: "Avoids deep idle states at the cost of battery, heat and noise.", badges: ["ADMIN", "AGGRESSIVE", "EXPERIMENTAL"] }
    ]
  },
  {
    id: "gpu",
    groupKey: "tweakGroup_gpu",
    icon: "monitor",
    title: "GPU & Display",
    tweaks: [
      { id: "hags-on", title: "Hardware GPU scheduling", description: "Enables Windows Hardware-accelerated GPU Scheduling when the driver supports it.", badges: ["ADMIN", "REBOOT", "VERIFIED"] },
      { id: "modern-flip-model-on", title: "Modern Flip Model upgrade", description: "Forces Modern Flip presentation model for windowed and borderless games to prevent gamma resets and reduce latency.", badges: ["SAFE", "VERIFIED"] },
      { id: "shader-cache-on", title: "Keep shader cache", description: "Keeps driver shader caches enabled to reduce repeat stutter after first run.", badges: ["SAFE", "VERIFIED"] },
      { id: "mpo-disable", title: "Disable MPO", description: "Disables Multiplane Overlay for systems with flicker, black screens or overlay stutter.", badges: ["ADMIN", "REBOOT", "ADVANCED", "EXPERIMENTAL"] },
      { id: "display-refresh-verify", title: "Refresh rate verify", description: "Checks that Windows is using the monitor's intended refresh rate.", badges: ["SAFE"] },
      { id: "hdr-profile-audit", title: "HDR profile audit", description: "Reviews HDR and color profile state before applying color correction.", badges: ["SAFE"] },
      { id: "gpu-msi-mode", title: "Force GPU MSI mode", description: "Only safe per device after driver validation; bulk mode is blocked.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE", "EXPERIMENTAL"] }
    ]
  },
  {
    id: "input",
    groupKey: "tweakGroup_input",
    icon: "settings",
    title: "Input & Smoothness",
    tweaks: [
      { id: "pointer-precision-off", title: "Disable pointer precision", description: "Turns off Windows mouse acceleration for consistent input feel.", badges: ["SAFE", "VERIFIED"] },
      { id: "usb-selective-suspend-off", title: "USB selective suspend off", description: "Prevents USB devices from entering selective suspend on the current power plan.", badges: ["ADVANCED"] },
      { id: "visual-effects-performance", title: "Visual effects performance", description: "Switches Explorer visual effects to the Windows performance profile.", badges: ["SAFE", "VERIFIED"], note: "Some window animations and visual polish will be reduced." },
      { id: "transparency-off", title: "Transparency off", description: "Disables Windows transparency effects for a lighter desktop compositor path.", badges: ["SAFE", "VERIFIED"], note: "Acrylic and translucent shell surfaces will become solid." },
      { id: "startup-delay-off", title: "Startup delay off", description: "Removes Explorer's startup app launch delay for the current user.", badges: ["SAFE"], note: "Many startup apps can open at once after sign-in." },
      { id: "menu-show-delay-low", title: "Fast desktop menus", description: "Reduces classic desktop menu delay to make shell interactions feel sharper.", badges: ["SAFE"], note: "Classic menus may feel too sensitive on touchpads." },
      { id: "timer-resolution-guard", title: "Timer resolution guard", description: "Avoids forcing global timer resolution outside active game sessions.", badges: ["SAFE", "VERIFIED"] },
      { id: "animations-reduced", title: "Reduce UI animation cost", description: "Reduces desktop animation overhead for maximum responsiveness.", badges: ["SAFE"] },
      { id: "focus-assist-game", title: "Focus Assist game profile", description: "Suppresses notification spikes while a game is active.", badges: ["SAFE"] },
      { id: "bluetooth-power-save-off", title: "Bluetooth power save off", description: "Keeps Bluetooth input devices from sleeping during desktop play sessions.", badges: ["ADMIN", "ADVANCED", "EXPERIMENTAL"] }
    ]
  },
  {
    id: "privacy",
    groupKey: "tweakGroup_privacy",
    icon: "shield",
    title: "Privacy & Telemetry",
    tweaks: [
      { id: "advertising-id-off", title: "Advertising ID off", description: "Disables the Windows per-user advertising identifier.", badges: ["PRIVACY", "SAFE"] },
      { id: "tailored-experiences-off", title: "Tailored experiences off", description: "Turns off recommendations based on diagnostic data.", badges: ["PRIVACY", "SAFE"] },
      { id: "activity-history-off", title: "Activity history off", description: "Stops activity history feed, publishing and upload policies.", badges: ["ADMIN", "PRIVACY"] },
      { id: "delivery-optimization-lan", title: "Delivery Optimization LAN only", description: "Prevents update sharing outside the local network policy.", badges: ["ADMIN", "PRIVACY"] },
      { id: "clipboard-cloud-off", title: "Cloud clipboard off", description: "Keeps clipboard history and sync disabled for sensitive systems.", badges: ["PRIVACY", "SAFE"] },
      { id: "diagtrack-manual", title: "Telemetry service manual", description: "Service-level telemetry tuning remains manual until rollback flow is complete.", badges: ["ADMIN", "ADVANCED", "EXPERIMENTAL"] },
      { id: "feedback-tasks-off", title: "Feedback tasks off", description: "Scheduled feedback tuning remains manual until task rollback is complete.", badges: ["ADMIN", "PRIVACY", "EXPERIMENTAL"] }
    ]
  },
  {
    id: "storage",
    groupKey: "tweakGroup_storage",
    icon: "archive",
    title: "Storage & Memory",
    tweaks: [
      { id: "memory-compression-keep", title: "Keep memory compression", description: "Avoids old scripts that disable compression and hurt low-memory systems.", badges: ["SAFE", "VERIFIED"] },
      { id: "storage-sense-clean", title: "Storage Sense cleanup", description: "Uses Windows cleanup flows instead of deleting unknown files directly.", badges: ["SAFE"] },
      { id: "search-indexer-light", title: "Indexer light profile", description: "Reduces Search indexing activity without deleting the index.", badges: ["ADMIN", "EXPERIMENTAL"] },
      { id: "sysmain-manual", title: "SysMain manual", description: "Can help SSD-only desktops but may hurt slow disks and low-memory laptops.", badges: ["ADMIN", "ADVANCED", "EXPERIMENTAL"] },
      { id: "standby-list-monitor", title: "Standby list monitor", description: "Tracks standby memory pressure instead of clearing it on a blind timer.", badges: ["SAFE", "VERIFIED"] },
      { id: "trim-enable", title: "Ensure TRIM enabled", description: "Enables Windows delete notifications for SSD/NVMe cleanup and write consistency.", badges: ["ADMIN", "VERIFIED"], note: "Useful for SSDs; irrelevant for old HDD-only systems." },
      { id: "ntfs-last-access-off", title: "NTFS last access off", description: "Disables last-access timestamp updates to reduce metadata writes.", badges: ["ADMIN", "ADVANCED"], note: "Old backup or audit tools that rely on access time can behave differently." },
      { id: "hibernate-off", title: "Hibernate off", description: "Disables hibernation to remove hiberfil.sys and reduce resume state writes.", badges: ["ADMIN", "REBOOT"], note: "Hibernate and Windows Fast Startup will stop working." },
      { id: "pagefile-static", title: "Static pagefile", description: "Stabilizes pagefile growth only after RAM and workload sizing.", badges: ["ADMIN", "REBOOT", "ADVANCED"] },
      { id: "pagefile-off", title: "Disable pagefile", description: "Blocked because it can crash apps and drivers under memory pressure.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE"] }
    ]
  },
  {
    id: "network",
    groupKey: "tweakGroup_network",
    icon: "cpu",
    title: "Network Latency",
    tweaks: [
      { id: "dns-cache-flush", title: "DNS cache refresh", description: "Refreshes stale resolver state before network troubleshooting.", badges: ["SAFE", "VERIFIED"] },
      { id: "flush-arp-cache", title: "ARP cache refresh", description: "Clears stale local address mappings for network troubleshooting.", badges: ["SAFE", "VERIFIED"], note: "Network discovery can pause briefly while entries rebuild." },
      { id: "tcp-autotune-normal", title: "TCP autotuning normal", description: "Restores supported TCP receive window autotuning.", badges: ["SAFE", "VERIFIED"] },
      { id: "rss-on", title: "Receive-side scaling on", description: "Keeps network receive processing distributed across CPU cores.", badges: ["ADMIN", "REBOOT"] },
      { id: "rsc-off", title: "Receive segment coalescing off", description: "Can reduce packet batching latency on some adapters at the cost of CPU usage.", badges: ["ADMIN", "ADVANCED"] },
      { id: "ecn-off", title: "ECN off", description: "Disables ECN for networks that spike latency during ECN negotiation.", badges: ["ADMIN", "ADVANCED"] },
      { id: "winsock-reset", title: "Winsock reset", description: "Resets the Windows socket catalog when networking is corrupted.", badges: ["ADMIN", "REBOOT", "VERIFIED"], note: "Some VPN, proxy, capture or filter drivers may need reconfiguration." },
      { id: "network-throttle-off", title: "Network throttling off", description: "Removes MMCSS network throttling for latency-sensitive games and streams.", badges: ["ADMIN", "REBOOT", "ADVANCED"] },
      { id: "wifi-roaming-medium", title: "Wi-Fi roaming medium", description: "Avoids aggressive roaming scans on stable home networks.", badges: ["ADMIN", "ADVANCED", "EXPERIMENTAL"] }
    ]
  },
  {
    id: "restore",
    groupKey: "tweakGroup_restore",
    icon: "shield",
    title: "Restore & Guardrails",
    tweaks: [
      { id: "restore-point-first", title: "Create restore point", description: "Requests a Windows restore point before risky service, driver or boot changes.", badges: ["ADMIN", "SAFE"] },
      { id: "firewall-keep-on", title: "Keep firewall enabled", description: "Protects the default firewall profile while tuning latency settings.", badges: ["SAFE", "VERIFIED"] },
      { id: "signed-driver-only", title: "Signed driver check", description: "Keeps kernel driver signature policy intact.", badges: ["SAFE", "VERIFIED"] },
      { id: "platform-clock-off", title: "Delete platform clock override", description: "Returns timer source selection to Windows; apply manually only after audit.", badges: ["ADMIN", "REBOOT", "ADVANCED"] },
      { id: "dynamic-tick-off", title: "Disable Dynamic Tick", description: "Blocked because Microsoft documents the timer option as debug-oriented.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE", "EXPERIMENTAL"] },
      { id: "platform-tick-force", title: "Force platform tick", description: "Blocked because platform timer forcing is debug-oriented.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE", "EXPERIMENTAL"] },
      { id: "driver-msi-bulk", title: "Bulk driver MSI mode", description: "Blocked because unsupported drivers can crash the system.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE", "EXPERIMENTAL"] },
      { id: "memory-integrity-off", title: "Memory integrity off", description: "Blocked because it lowers exploit protection for the user.", badges: ["ADMIN", "REBOOT", "AGGRESSIVE"] }
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
  if (badges.includes("AGGRESSIVE") || ["pagefile-off", "dynamic-tick-off", "platform-tick-force", "driver-msi-bulk", "memory-integrity-off"].includes(tweak.id)) {
    return "risk";
  }
  if (badges.includes("EXPERIMENTAL")) {
    return "experimental";
  }
  if (badges.includes("ADMIN")) {
    return "admin";
  }
  return "safe";
}

export function isSafeTweak(tweak) {
  const risky = new Set(["ADMIN", "REBOOT", "ADVANCED", "AGGRESSIVE", "EXPERIMENTAL"]);
  return tweakBadges(tweak).every((badge) => !risky.has(badge));
}

export function allTweaks() {
  return tweakCatalog.flatMap((group) => group.tweaks);
}

export const tweakAppImpacts = {
  "disable-gamedvr": {
    appId: "xbox_game_bar",
    appName: "Xbox Game Bar",
    impactEn: "Win+Alt+R clip capture, background recording, and Game Bar screen capture will stop working.",
    impactRu: "Запись клипов по Win+Alt+R, фоновый повтор и оверлей захвата Xbox Game Bar перестанут работать."
  },
  "disable-bg-recording": {
    appId: "xbox_game_bar",
    appName: "Xbox Game Bar",
    impactEn: "Instant replay and passive background clip recording will be disabled.",
    impactRu: "Фоновая запись последних минут геймплея и мгновенный повтор в Game Bar будут выключены."
  },
  "gamebar-startup-off": {
    appId: "xbox_game_bar",
    appName: "Xbox Game Bar / Controller",
    impactEn: "Controller Xbox button guide hook disabled; Game Bar will not open on controller button press.",
    impactRu: "Кнопка Xbox на геймпаде больше не будет вызывать оверлей Game Bar."
  },
  "overlay-audit": {
    appId: "discord",
    appName: "Discord / RTSS / Steam / NVIDIA",
    impactEn: "Capture and chat overlays (Discord, ShadowPlay, Steam, RTSS) can cause frametime microstutters.",
    impactRu: "Оверлеи Discord, ShadowPlay, Steam и RTSS могут вызывать микростаттеры и колебания frametime."
  },
  "mpo-disable": {
    appId: "discord",
    appName: "Discord / Chrome / Edge",
    impactEn: "Multiplane Overlay disabled: eliminates screen flicker and stutter, but GPU video playback usage may increase slightly.",
    impactRu: "MPO отключён: устраняет мерцания экрана и статтеры в играх, но нагрузка GPU при видео в браузере и Discord может вырасти."
  },
  "startup-delay-off": {
    appId: "steam",
    appName: "Steam / Discord / Telegram",
    impactEn: "All startup programs will launch simultaneously upon Windows sign-in without 10-second pause.",
    impactRu: "Все автозагружаемые программы (Steam, Discord, Telegram) запустятся одновременно сразу после входа."
  },
  "visual-effects-performance": {
    appId: "windows_shell",
    appName: "Windows Explorer",
    impactEn: "Window opening/minimizing animations and drop shadows will be turned off.",
    impactRu: "Анимации открытия и сворачивания окон, а также тени проводника Windows будут отключены."
  },
  "transparency-off": {
    appId: "windows_shell",
    appName: "Windows Shell / Acrylic",
    impactEn: "Acrylic and translucent taskbar / window surfaces will become solid dark.",
    impactRu: "Эффекты полупрозрачности Acrylic в меню «Пуск» и окнах станут сплошными."
  },
  "bluetooth-power-save-off": {
    appId: "bluetooth",
    appName: "Bluetooth Controller / Audio",
    impactEn: "Bluetooth radio won't enter low-power sleep; increases battery drain on laptops.",
    impactRu: "Bluetooth-модуль не будет уходить в сон; на ноутбуках может чуть быстрее расходовать батарею."
  },
  "hibernate-off": {
    appId: "fast_startup",
    appName: "Быстрый запуск Windows (Fast Startup)",
    impactEn: "Windows Fast Startup and Hibernate will stop working; PC will perform a clean boot every time.",
    impactRu: "«Быстрый запуск» и гибернация Windows перестанут работать; каждый запуск ПК будет чистым."
  },
  "winsock-reset": {
    appId: "vpn",
    appName: "VPN / Proxy / Anti-Cheat",
    impactEn: "Resets TCP/IP and Winsock stack; VPN adapters (WireGuard/OpenVPN) and proxy tools may need restart.",
    impactRu: "Сбрасывает стек Winsock и TCP/IP; виртуальные сетевые адаптеры VPN и прокси могут потребовать перезапуска."
  },
  "search-indexer-light": {
    appId: "windows_search",
    appName: "Windows Search / Индексация",
    impactEn: "Windows Search indexing will run in low priority during desktop usage.",
    impactRu: "Индексация поиска Windows Search будет работать с пониженным приоритетом."
  },
  "sysmain-manual": {
    appId: "sysmain",
    appName: "SysMain (Superfetch)",
    impactEn: "SysMain service disabled: optimal for fast NVMe SSDs; not recommended for mechanical HDDs.",
    impactRu: "Служба SysMain переведена в ручной режим: идеально для быстрых NVMe SSD, не рекомендуется для HDD."
  },
  "hags-on": {
    appId: "geforce_experience",
    appName: "OBS / NVIDIA GPU",
    impactEn: "Hardware-accelerated GPU Scheduling requires PC reboot to take full effect.",
    impactRu: "Аппаратное планирование HAGS требует перезагрузки ПК для вступления в силу."
  },
  "modern-flip-model-on": {
    appId: "windows_shell",
    appName: "DirectX / Modern Flip Model",
    impactEn: "Upgrades legacy DX9/DX11 presentation to Modern Flip Model; prevents gamma and color correction reset in borderless games.",
    impactRu: "Переводит игры DX9/DX11 на Modern Flip Model; устраняет задержку DWM и защищает от сброса калибровки цвета в играх."
  }
};

