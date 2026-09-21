[![](https://img.shields.io/badge/Language-🇷🇺_Перейти_на_русский-0284c7?style=for-the-badge)](README.md)

# Synchro Nova

<div align="center">

[![Telegram](https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white)](https://t.me/synchronova)
[![Releases](https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald)](https://github.com/Arean-Max/synchro-nova/releases)
[![License](https://img.shields.io/badge/License-MIT-white?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square)](https://microsoft.com/windows)

**Desktop utility for hardware display color calibration, input latency reduction, and clean Windows tuning.**  
*Engineered in Rust & Tauri v2. Consumes ~2 MB RAM in tray, 0% CPU at idle, and operates strictly external to game processes.*

<br>

<img src="docs/screenshots/main-frame.png" alt="Synchro Nova Interface" width="880" style="border-radius: 10px; box-shadow: 0 8px 30px rgba(0,0,0,0.5);">

</div>

---

## Overview

Synchro Nova was designed as a lightweight, clean alternative to bloated game overlays and utilities that clutter memory, hook graphics libraries, and cause conflicts with anti-cheat software.

The application combines hardware Digital Vibrance, monitor LUT gamma calibration, GPU driver auditing, and transparent system tweaks in a single offline tool with zero ads, zero telemetry, and zero background services.

---

## 🛡️ Anti-Cheat Safety Architecture (Why Bans Like Tactical Vision Are Impossible)

Many competitive players (Valorant, CS2, Fortnite, Warzone, Apex Legends, Rainbow Six Siege) have suffered bans from third-party tools such as **Tactical Vision**.

### Why did anti-cheats ban Tactical Vision and similar utilities?
1. **Process Memory Injection**: Tactical Vision injected dynamic-link libraries (`CreateRemoteThread`, `LoadLibrary`) directly into the address space of running games.
2. **DirectX / Vulkan SwapChain Interception**: Tactical Vision hooked internal graphics pipeline methods such as `Present` / `Present1` in `dxgi.dll` and `d3d11.dll` to draw over game frames. Kernel-level anti-cheats (Riot Vanguard, Easy Anti-Cheat, BattlEye, Valve Anti-Cheat, Ricochet) flag any DirectX swapchain hooks as Wallhack / ESP / Shader Chams.
3. **Synthetic Input Simulation**: Synthetic keystroke/mouse simulation using `keybd_event`, `mouse_event`, or `SendInput`, which trigger macro and aimbot heuristics.
4. **Foreign Memory Access**: Opening game process handles with `PROCESS_VM_READ` or `PROCESS_VM_WRITE`.

### Why Synchro Nova is 100% Ban-Proof and Safe:
- **Strict Process Isolation (Zero Process Injection)**: Synchro Nova runs exclusively in User Mode as an independent desktop application. It never opens handles to game processes and never touches game memory (0 calls to `PROCESS_VM_READ` or `PROCESS_VM_WRITE`).
- **Zero DirectX / Vulkan Hooks**: Synchro Nova does not hook `dxgi.dll`, `d3d11.dll`, `d3d12.dll`, or `vulkan-1.dll`. The game rendering pipeline remains 100% untouched.
- **Zero Synthetic Input**: The codebase contains zero calls to `keybd_event`, `mouse_event`, or `SendInput`.
- **Operating via Windows DWM & Monitor Hardware LUT**:
  - Color vibrance, saturation, and contrast adjustments are applied via the official Windows Desktop Window Manager compositor API (`MagSetFullscreenColorEffect` in `magnification.dll`).
  - Gamma curves are written directly into GPU hardware tables (`SetDeviceGammaRamp` in `gdi32.dll`).
  - Kernel anti-cheats monitor game memory and do not restrict Windows desktop compositor shaders — exactly like changing Digital Vibrance in NVIDIA Control Panel, AMD Software, or using Windows Night Light / f.lux.
- **Native Process Hardening**: Synchro Nova enables Data Execution Prevention (DEP), Safe Search Mode (preventing CWD DLL hijacking), and heap termination on corruption at launch.

---

## Key Features

### 1. Hardware Display Calibration
- **Windows GDI & Magnification API**: Real-time digital vibrance, saturation, gamma, contrast, and color balance adjustments with zero frame lag and zero input latency.
- **Black Holo Mode**: Smart dynamic range enhancement for competitive shooters (CS2, Rust, Apex, Tarkov), lifting dark shadowy spots without overblowing highlights.
- **Per-Game Color Profiles**: Link custom calibration parameters to specific game titles.
- **Accent Customization**: Full RGB color selection with automatic Rec. 601 perceived luminance calculation and adaptive high-contrast readability.

### 2. System Tweaks & Latency Reduction (System Tweaks)
- **Verified Parameters**: System timer configuration, Multimedia Class Scheduler (MMCSS) priorities, Fullscreen Optimizations (FSO), telemetry suppression, and TCP network stack autotuning.
- **Windows System Restore Points & Registry Backups**: Every backup creation automatically commits a Windows System Restore Point (`SRSetRestorePointW` / `Checkpoint-Computer`) alongside a `.reg` snapshot, ensuring reliable rollback.
- **Total Transparency**: No hidden batch scripts. Every registry key and command is clearly explained in the UI and in the [Tweaks Reference Documentation](docs/TWEAKS_REFERENCE.md).

### 3. Hardware & Driver Inspector
- Detects installed graphics cards, active driver version, and release date.
- Checks driver status for NVIDIA, AMD, Intel with direct official vendor download links.
- Real-time CPU and memory monitoring via native Windows Performance Data Helper (PDH).

---

## Performance and Resources

| Metric | Synchro Nova v2.1 | Typical Electron Utility |
|---|:---:|:---:|
| **RAM usage in tray** | **~1.5 – 3 MB** | 120 – 350 MB |
| **CPU usage at idle** | **0.0% (0 timer interrupts)** | 0.5 – 2.5% |
| **Cold start time** | **< 0.3 sec** | 2.5 – 6.0 sec |
| **External network traffic** | **0 bytes (Chromium telemetry stripped)** | Continuous analytic telemetry |

---

## Download & Installation

Latest builds are always available on the [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) page:

- **Portable Version (`Synchro-Nova-2.1.0-Portable.zip`)** — standalone `synchro.exe` executable, runs immediately from any folder or USB stick without installation.
- **Installer (`Synchro.Nova_2.1.0_x64-setup.exe`)** — standard Windows installer with desktop shortcut, Windows search indexing, and a clean uninstaller.

### Verify SHA-256 Checksum
To verify the integrity of the downloaded file in PowerShell:
```powershell
Get-FileHash .\synchro.exe -Algorithm SHA256
```

---

## Building from Source

### Prerequisites
- [Node.js](https://nodejs.org/) (version 18+)
- [Rust](https://www.rust-lang.org/) (stable `x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc`)
- [Tauri CLI](https://tauri.app/)

### Build Commands
```bash
# 1. Clone repository
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# 2. Install dependencies
npm install

# 3. Launch in development mode
npm run dev

# 4. Build portable executable
npm run build:portable

# 5. Build release installer
npm run build
```

The compiled binary will be placed in `src-tauri/target/release/synchro.exe`, and the NSIS installer in `src-tauri/target/release/bundle/nsis/`.

---

## Community & Support

- **Official Telegram**: [t.me/synchronova](https://t.me/synchronova) — announcements, discussions, and direct support.
- **GitHub Issues**: found a bug or have a suggestion? [Open an issue](https://github.com/Arean-Max/synchro-nova/issues).

---

## License

This project is licensed under the [MIT License](LICENSE).
