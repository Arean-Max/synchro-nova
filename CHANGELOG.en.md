# Changelog

[🌐 Читать на русском (Russian version)](CHANGELOG.md)

All notable changes to Synchro Nova are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.2.4] - 2026-09-26

### Added
- Original hardware GPU vendor indicator (NVIDIA / AMD) dynamically rendered next to the Black Holo toggle switch.
- Modal window attachment (`HWND`) for UAC administrator elevation dialogs via `ShellExecuteExW`.
- Automatic page routing (`--navigate-to tweaks`) to seamlessly restore the active view following an elevated restart.
- Process tree Job Object breakaway flags (`JOB_OBJECT_LIMIT_BREAKAWAY_OK` & `SILENT_BREAKAWAY`) for unhindered WebView2 sandbox lifecycle.

### Fixed
- Application launch failure caused by orphaned single-instance mutex locks when no visible desktop window exists.
- Self-termination issue during elevation requests where parent and child processes closed simultaneously.
- Black Holosight toggle knob sliding animation and optimistic UI responsiveness.
- Isolated WebView2 browser data directory (`EBWebView_Admin`) for elevated instances to eliminate SQLite file contention and Windows integrity level mismatches.
- Removed obsolete keyboard shortcut labeling and redundant hint text.

---

## [2.2.3] - 2026-09-24

### Added
- Tweak page blur effect and interaction lock when running without administrator privileges.
- Persistent administrator elevation notification banner with 1-click UAC restart.
- Multi-monitor support for saturated screenshot capture based on active window and cursor position.
- Dynamic localization for system tray context menu (RU/EN).

### Security
- Pre-execution elevation checks for privileged tweak application and rollback commands.
- Streaming SHA-256 integrity verification against release checksums prior to applying updates.
- PE signature header verification before executing downloaded update packages.

### Fixed
- Mutual exclusion deadlock during elevated restart sequence.
- Desktop color ramp restoration on application exit and system shutdown.

---

## [2.2.2] - 2026-09-23

### Added
- Integrated background updater with GitHub release discovery and progress tracking.
- EAC-safe desktop screenshot capture with embedded vibrance processing.

### Fixed
- Progress reporting edge cases in download stream handler.
- Sizing and alignment of manual download indicator in updater modal.

---

## [2.2.1] - 2026-09-22

### Security
- In-process Authenticode verification via `WinVerifyTrust` for loaded runtime libraries.
- Hardened Tauri custom asset protocol scope to prevent unauthorized directory traversal.
- Enabled Data Execution Prevention (DEP) and restricted DLL search path mode at startup.

---

## [2.2.0] - 2026-09-22

### Changed
- Refactored monolithic backend into domain-driven modules (`app`, `domain`, `infra`, `platform`, `commands`).
- Enforced strict module boundaries and reduced source file complexity.

---

## [2.1.0] - 2026-09-21

### Added
- System Restore Point integration via `SRSetRestorePointW` and PowerShell fallback before applying tweaks.
- Deep uninstaller in NSIS package removing configuration, cache, and registry keys.
- Adaptive luminance contrast algorithm for custom RGB accent selection.

### Fixed
- Window initialization crash (`0xC00000FD`) caused by recursive position updates on frameless window.
- White border artifacts by removing native DWM window shadow and managing border color attributes.
- Standalone portable execution by bundling and dynamically loading `WebView2Loader.dll`.

---

## [2.0.0] - 2026-09-20

### Added
- Event-driven thread synchronization using condition variables to achieve 0% idle CPU usage.
- Graceful process termination with atomic shutdown signaling.
- Black Holo dynamic range mode for enhanced shadow visibility.
- Rust game configuration optimizer for input latency reduction.

### Changed
- Isolated WebView2 user data directory and optimized browser process flags.
- Memory working set compaction reducing idle footprint to ~2 MB RAM.

---

## [0.1.1] - 2026-09-19

### Fixed
- UI responsiveness during tweak execution using protected async blocks.
- Localization coverage across all display presets.
- Input field reset behavior after configuration saves.

---

## [0.1.0] - 2026-09-17

### Added
- Initial release with hardware LUT display calibration.
- Core system tweaks engine with automated `.reg` snapshot backups and 1-click rollback.
- Hardware telemetry for CPU and memory via Windows PDH.
- GPU and driver information inspector.
- Game library detection for Steam and Epic Games.
