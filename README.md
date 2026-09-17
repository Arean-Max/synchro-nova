# Synchro Nova

Windows display calibration, color profiles, and system performance management utility built with Tauri v2 and Rust.

[![CI](https://github.com/Arean-Max/synchro-nova/actions/workflows/ci.yml/badge.svg)](https://github.com/Arean-Max/synchro-nova/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0078D6.svg)](https://microsoft.com/windows)
[![Tauri](https://img.shields.io/badge/Tauri-v2.0-24C8DB.svg)](https://tauri.app/)

Synchro Nova provides desktop gamers and power users with hardware-level display color adjustments (vibrance, gamma, saturation), safe system and registry optimizations, and real-time hardware telemetry in a single lightweight, offline-first application.

---

## Screenshots

| Display Calibration | System Tweaks & Optimization |
| :---: | :---: |
| ![Color Correction](docs/screenshots/color-correction.png) | ![System Tweaks](docs/screenshots/tweaks.png) |

---

## Key Features

- **Display Color Calibration**: Direct GDI gamma ramp control for vibrance, saturation, contrast, gamma, and color balance with per-game profile associations.
- **System & Registry Optimizations**: Curated system settings split into user-level (`HKCU`) and administrative (`HKLM`) tweaks with automated safety backups and 1-click rollback.
- **Hardware Telemetry**: Real-time CPU utilization, system specs, and display driver information via Windows PDH counters.
- **Game Library Integration**: Auto-detects installed Steam and Epic Games titles to easily map individual color presets to executables.
- **Minimal Resource Footprint**: Processes run with targeted working set trimming (~8 MB total RAM in Task Manager) and zero CPU usage when minimized.
- **Offline & Private**: Zero telemetry, zero external network connections, no listening ports, and no background services.

---

## Administrative Privileges & Transparency

Synchro Nova operates under a strict principle of transparency. Many features (display calibration, user tweaks, game launcher integration) run entirely with standard user privileges.

Certain system optimizations require elevated administrator rights because they modify machine-wide policies (`HKEY_LOCAL_MACHINE`) or execute standard Windows administrative tools (`powercfg`, `netsh`, `fsutil`).

### Why Admin is Requested:
- **Multimedia Class Scheduler (MMCSS)**: Configuring `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` to prioritize gaming network packets.
- **Hardware-Accelerated GPU Scheduling (HAGS)**: Enabling `HwSchMode` in `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers`.
- **Power Schemes**: Activating High Performance or Ultimate Performance power plans via `powercfg`.
- **Network Stack**: Applying TCP window autotuning and Receive-Side Scaling (RSS) via `netsh`.

### Safety & Rollback:
- **Pre-Tweak Snapshots**: Before any registry change is written, Synchro creates a timestamped `.reg` backup in `%APPDATA%\app.synchro.performance\backups\`.
- **1-Click Rollback**: You can revert all applied tweaks at any time by clicking **"Откатить изменения"** on the Tweaks page.
- **Audit Log**: Every applied setting and status code is recorded locally in `%APPDATA%\app.synchro.performance\tweaks_audit.log`.

For a full list of all registry keys and commands executed by the application, see the [Tweaks Reference Documentation](docs/TWEAKS_REFERENCE.md).

---

## Security & Anti-Cheat Compatibility

Synchro Nova is engineered to coexist safely with kernel-level and user-mode anti-cheat systems (Easy Anti-Cheat, BattlEye, Vanguard):

- **No Memory Injection**: The application never reads or writes to the memory of other processes (`WriteProcessMemory`, `CreateRemoteThread`, etc. are not used).
- **No Global Hooks**: No keyboard, mouse, or graphics API hooks (`SetWindowsHookEx`, DirectX hooks) are installed.
- **Process Hardening**: Built with permanent Data Execution Prevention (DEP), Safe DLL Search Mode (`LOAD_LIBRARY_SEARCH_SYSTEM32` to prevent DLL hijacking), and Heap Corruption Termination.
- **Network Isolation**: The application contains no analytics, telemetry, or remote command execution capabilities.

Detailed technical threat models and architecture notes are documented in [SECURITY_NOTES.md](SECURITY_NOTES.md).

---

## Verifying Downloads & Checksums

All official releases include a `SHA256SUMS.txt` file generated directly in GitHub Actions. To verify your downloaded binary:

```powershell
Get-FileHash -Path .\synchro.exe -Algorithm SHA256
```

Compare the output hash against `SHA256SUMS.txt`. For details on code signing and verifying binaries, see [docs/CODE_SIGNING.md](docs/CODE_SIGNING.md).

---

## Building from Source

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or newer)
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (stable `x86_64-pc-windows-msvc` toolchain)
- Microsoft Visual Studio C++ Build Tools

### Build Steps

```bash
# Clone the repository
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build standalone portable executable
npm run build:portable
# Output: src-tauri/target/release/synchro.exe

# Build NSIS installer
npm run build
# Output: src-tauri/target/release/bundle/nsis/Synchro_0.1.0_x64-setup.exe
```

---

## Automated Testing

Run the Rust backend test suite:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

Verify frontend syntax:

```bash
node --check src/app/main.js
node --check scripts/app-server.mjs
```

---

## License

This project is licensed under the [MIT License](LICENSE).
