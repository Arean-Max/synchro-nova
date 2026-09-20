# Synchro Nova

<div align="center">

[![Русская версия](https://img.shields.io/badge/Language-Русский-red?style=flat-square)](README.md)
[![Telegram](https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white)](https://t.me/synchronova)
[![Releases](https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald)](https://github.com/Arean-Max/synchro-nova/releases)
[![License](https://img.shields.io/badge/License-MIT-white?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square)](https://microsoft.com/windows)

**Desktop utility for hardware display color calibration, input latency reduction, and clean Windows tuning.**  
*Engineered in Rust & Tauri v2. Consumes ~2 MB RAM in tray, 0% CPU at idle, and is 100% anti-cheat compliant.*

<br>

<img src="docs/screenshots/main-frame.png" alt="Synchro Nova Interface" width="880" style="border-radius: 10px; box-shadow: 0 8px 30px rgba(0,0,0,0.5);">

</div>

---

## Overview

Synchro Nova was designed as a lightweight, clean alternative to bloated game overlays and questionable "FPS boosters" that clutter memory, inject into game threads, and trigger competitive anti-cheat bans.

We brought together hardware Digital Vibrance, precise gamma ramp calibration, GPU driver auditing, and transparent system tweaks in a single offline application with zero ads, zero telemetry, and zero background services.

---

## Key Features

### 1. Hardware Display Calibration
- **Native Win32 GDI & Magnification API**: Real-time digital vibrance, saturation, gamma, contrast, and color balance adjustments with zero frame lag and zero input latency.
- **Black Holo Mode**: Smart dynamic range enhancement for competitive shooters (CS2, Rust, Apex, Tarkov), lifting dark shadowy spots without overblowing highlights.
- **Per-Game Color Presets**: Automatically apply custom profiles when launching specific titles.

### 2. System Tweaks & Input Latency (System Tweaks)
- **41 Tested Optimizations**: System timer configuration, Multimedia Class Scheduler (MMCSS) priorities, Fullscreen Optimizations (FSO), telemetry suppression, and TCP network stack autotuning.
- **Guaranteed Safety (1-Click Rollback)**: Before any modification is made, a timestamped `.reg` backup is created. Revert all applied tweaks with a single click at any time.
- **Total Transparency**: No hidden batch files or detours. Every registry key and command is clearly explained in the UI and in the [Tweaks Reference Documentation](docs/TWEAKS_REFERENCE.md).

### 3. Hardware & Driver Inspector
- Detects installed graphics cards, active driver version, and release date.
- Checks driver status for NVIDIA, AMD, Intel with direct official download links.
- Real-time CPU and memory monitoring via native Windows Performance Data Helper (PDH).

### 4. Anti-Cheat Compliance (Vanguard, EAC, BattlEye)
- **Strict User Mode**: Zero cross-process memory access (`PROCESS_VM_READ`, `PROCESS_VM_WRITE`, `PROCESS_ALL_ACCESS`).
- **No DLL Injection or Detours**: Zero `CreateRemoteThread`, zero keyboard/mouse hooks (`SetWindowsHookEx`), and zero API hooking.
- Fully compatible alongside **Riot Vanguard, Easy Anti-Cheat (EAC), BattlEye, Valve Anti-Cheat (VAC), and Ricochet**.

---

## Resource Efficiency

| Metric | Synchro Nova v2.0 | Typical Electron Utility |
|---|:---:|:---:|
| **RAM Footprint in Tray** | **~1.5 – 3 MB** | 120 – 350 MB |
| **Idle CPU Utilization** | **0.0% (0 timer wakeups)** | 0.5 – 2.5% |
| **Cold Start Time** | **< 0.3 s** | 2.5 – 6.0 s |
| **External Network Activity** | **0 bytes (Chromium telemetry stripped)** | Continuous background analytics |

> [!NOTE]
> In v2.0, the color calibration guard uses Windows `Condvar` synchronization. When calibration is neutral, the thread sleeps in the OS kernel with 0 wakeups, allowing CPU cores to drop into deep C-states.

---

## Download & Install

Official releases are published on the [**Releases Page**](https://github.com/Arean-Max/synchro-nova/releases):

- **Portable Version (`synchro.exe`)** — Standalone executable, runs immediately from any folder or USB drive with zero installation and no leftover system traces.
- **Installer (`Synchro.Nova_2.0.0_x64-setup.exe`)** — Standard Windows setup wizard with Desktop shortcut and clean uninstaller via Windows Settings.

### Verify Checksums (SHA-256)
Open PowerShell in the download folder:
```powershell
Get-FileHash .\synchro.exe -Algorithm SHA256
```
Compare the output hash against `SHA256SUMS.txt` on the release page.

---

## Building from Source

### Prerequisites
- [Node.js](https://nodejs.org/) (version 18+)
- [Rust](https://www.rust-lang.org/) (stable `x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc`)
- [Tauri CLI](https://tauri.app/)

### Build Commands
```bash
# 1. Clone the repository
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# 2. Install frontend dependencies
npm install

# 3. Start development mode
npm run dev

# 4. Build release binaries and installer
npm run build
```

The compiled standalone executable will be located at `src-tauri/target/release/synchro.exe`, and the NSIS installer at `src-tauri/target/release/bundle/nsis/`.

---

## Community & Support

- **Official Telegram**: [t.me/synchronova](https://t.me/synchronova) — Releases, discussions, feature requests, and help.
- **GitHub Issues**: Found a bug or have a suggestion? [Open an issue](https://github.com/Arean-Max/synchro-nova/issues).

---

## License

Released under the [MIT License](LICENSE).
