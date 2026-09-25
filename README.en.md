[![](https://img.shields.io/badge/Language-🇷🇺_Перейти_на_русский-0284c7?style=for-the-badge)](README.md)

# Synchro Nova

<div align="center">

[![Telegram](https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white)](https://t.me/synchronova)
[![Releases](https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald)](https://github.com/Arean-Max/synchro-nova/releases)
[![License](https://img.shields.io/badge/License-AGPL--3.0-blue?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square)](https://microsoft.com/windows)

**Desktop utility for hardware display color calibration, input latency reduction, and clean Windows tuning.**  
*Engineered in Rust & Tauri v2. Consumes ~2–8 MB RAM in tray (trimmed working set), 0% CPU at idle, and operates strictly external to game processes.*

<br>

<img src="docs/screenshots/main-frame.png" alt="Synchro Nova Interface" width="880" style="border-radius: 10px; box-shadow: 0 8px 30px rgba(0,0,0,0.5);">

</div>

---

## Overview

Synchro Nova is a high-performance, modern alternative to utilities like VibranceGUI, intrusive overlays, and bloated game optimizers. It combines hardware-accelerated Digital Vibrance and saturation controls, monitor LUT gamma calibration, GPU driver auditing, and transparent Windows latency tweaks (Input Lag reduction) in a single package with zero ads, zero telemetry, and zero background services.

---

## Anti-Cheat Safety & Architecture

Synchro Nova is built from the ground up for full compatibility with competitive gaming and kernel-level anti-cheat engines (Easy Anti-Cheat, BattlEye, Riot Vanguard, Ricochet, VAC):

- **Strict Process Isolation**: Operates entirely in user mode as an independent desktop process. It never opens handles to game processes and never touches external process memory (0 calls to `PROCESS_VM_READ` or `PROCESS_VM_WRITE`).
- **Zero Graphics Hooks**: Synchro Nova does not intercept graphics pipelines (DirectX / Vulkan SwapChain hooks) and does not inject dynamic libraries into running games.
- **No Synthetic Input**: The codebase contains zero keystroke or mouse simulation calls (`SendInput`, `keybd_event`, global window hooks).
- **Native Windows Interfaces**: Display calibration utilizes official Desktop Window Manager compositor shaders (`MagSetFullscreenColorEffect`) and hardware GPU LUT tables (`SetDeviceGammaRamp`), operating identically to standard display control panels (NVIDIA Control Panel, AMD Software).
- **Native Process Hardening**: Permanent Data Execution Prevention (DEP), safe DLL search mode (eliminating CWD DLL preloading), and heap corruption termination are enforced at startup.

---

## Key Features

### 1. Hardware Display Calibration
- **Windows GDI & Magnification API**: Real-time digital vibrance, saturation, gamma, contrast, and color balance adjustments with zero frame lag and zero input latency.
- **Black Holo Mode**: Dynamic range enhancement for competitive titles, lifting dark shadow detail without overblowing highlights.
- **Competitive Game Profiles**: Automatic profile switching for popular titles (Counter-Strike 2 / CS2, Dota 2, Rust, Apex Legends, PUBG, Valorant, Rainbow Six Siege, War Thunder).
- **Adaptive UI Accent**: Full RGB accent color support with real-time text contrast calculation.

### 2. System Tweaks & Latency Optimization
- **Verified Parameters**: System timer configuration, MMCSS task priorities, Fullscreen Optimizations (FSO), telemetry suppression, and TCP network autotuning.
- **Windows System Restore Points & Registry Backups**: Creates a Windows System Restore Point (`SRSetRestorePointW`) alongside a `.reg` snapshot before applying changes, ensuring reliable rollback.
- **Safety & Elevation Guard**: Every setting is documented in the [Tweaks Reference Documentation](docs/TWEAKS_REFERENCE.md). System tweaks are protected by an administrative lock and frosted blur until elevated via UAC.

### 3. Hardware & Driver Inspector
- Detects installed graphics hardware, active driver version, and release date.
- Checks driver status for NVIDIA, AMD, and Intel with direct vendor download links.
- Real-time CPU and memory monitoring via native Windows Performance Data Helper (PDH).

---

## Performance & Resource Comparison

| Metric | Synchro Nova | VibranceGUI | Typical Electron Utility |
|---|:---:|:---:|:---:|
| **RAM usage in tray** | **~2 – 8 MB** | ~15 – 35 MB | 120 – 350 MB |
| **CPU usage at idle** | **0.0% (0 timer interrupts)** | 0.2 – 0.8% | 0.5 – 2.5% |
| **Dynamic Range / Shadow Lift (Black Holo)** | **Supported (Hardware LUT)** | No | Rare (hook-based overlay) |
| **Input Latency Tweaks (MMCSS, TCP, FSO)** | **Supported (with Restore Point)** | No | No / external BAT scripts |
| **Anti-Cheat Safety (EAC, BE, Vanguard)** | **100% (User-Mode WinAPI)** | Compatible | Hook injection risk |
| **Cold start time** | **< 0.3 sec** | ~1.0 – 1.8 sec | 2.5 – 6.0 sec |

---

## Download & Installation

Official release packages are available on the [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) page:

- **Portable Version (`Synchro-Nova-2.2.3-Portable.zip`)** — standalone `synchro.exe` executable, runs immediately from any folder without installation.
- **Installer (`Synchro.Nova_2.2.3_x64-setup.exe`)** — standard Windows installer with desktop shortcuts, Windows search indexing, and a clean uninstaller.

### Verify SHA-256 Checksum
To verify the integrity of the downloaded binary in PowerShell:
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

# 4. Build portable binary
npm run build:portable

# 5. Build release installer
npm run build
```

Compiled executables are located in `src-tauri/target/release/synchro.exe` and `src-tauri/target/release/bundle/nsis/`.

---

## Community & Support

- **Official Telegram**: [t.me/synchronova](https://t.me/synchronova) — announcements, updates, and community discussion.
- **GitHub Issues**: bug reports and feature requests — [Open an issue](https://github.com/Arean-Max/synchro-nova/issues).

---

## License

The source code is licensed under the [GNU Affero General Public License v3 (AGPL-3.0)](LICENSE).

For commercial use, proprietary integrations, or closed-source products without AGPLv3 copyleft requirements, commercial licensing is available. For buyout inquiries: **Telegram: [https://t.me/vkmef](https://t.me/vkmef)**.
