# Changelog

[🌐 Читать на русском (Russian version)](CHANGELOG.md)

All notable changes to Synchro Nova are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.5.0-beta] - 2026-11-10

### Discord-Style Startup Auto-Updater
- **Integrated Splash Updater**:
  - Full migration of update checking and downloading directly into the startup splash screen animation — Discord-style.
  - Minimalist progress taskbar directly below the logo with status, percentage, and elegant Manual Download and Skip controls.
  - Cryptographic patch detection: automatically discovers patch releases sharing the same version number via SHA-256 hash and build timestamp comparison.
  - Seamless in-place binary replacement with automatic restart and state preservation.

### Deep Architectural & Security Audit Fixes
- **SEC-01 (TOCTOU Elimination in `helper.rs`)**:
  - Payload file is opened once with an exclusive read share lock (`share_mode(1)` / `FILE_SHARE_READ`, blocking modifications or deletion by third-party processes).
  - MZ header validation, streaming SHA-256 calculation, and destination copy are executed directly over this locked file handle.
- **SEC-02 (Elevated PATH Hijacking Prevention)**:
  - System commands (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) are strictly resolved from `%SystemRoot%\System32` (and `System32\WindowsPowerShell\v1.0\powershell.exe`). All relative path fallbacks are rejected.
- **SEC-03 (CLI Parameter Restriction)**:
  - Restricts the `--navigate-to` CLI argument to a strict allowlist of validated pages (`color`, `tweaks`, `binds`, `settings`, `characteristics`).
- **SEC-04 (Template Sanitization)**:
  - Comprehensive HTML escaping across all UI templates via `escapeHtml` and `escapeAttr`.
- **DEP-01**:
  - Unpinned `time = "=0.3.36"` to `time = "0.3"` in `Cargo.toml`.

### Crash Prevention & Memory Hardening
- **CRASH-01 (Thread Stack Overflow Elimination)**:
  - Removed artificial `.stack_size(...)` limits (64KB–256KB) across all 10 background worker threads, eliminating `0xC00000FD` stack overflows caused by third-party DLLs, hooks, and overlays.
- **CRASH-02 (Protected Fast-Boot Cache)**:
  - Bounded `fast_boot.bin` deserialization to 64KB with automatic deletion of corrupted cache files.
- **CRASH-03 & CRASH-04**:
  - Automatic recovery from poisoned mutexes in the color pipeline and atomic `AcqRel` exchange in the single-instance mutex.

### Hangs, Timeouts & Race Condition Resolutions
- **HANG-01 (Process Timeouts)**:
  - Non-blocking 5s timeouts for system utility execution and 25s timeout for PowerShell restore point creation with child termination on hang.
- **HANG-02 (Hotkey Queue Synchronization)**:
  - Guaranteed Windows message queue creation via `PeekMessageW` prior to publishing `LISTENER_THREAD_ID`, synchronized using `sync_channel(1)`.
- **HANG-03 & HANG-04**:
  - Single-instance restart mutex acquisition refactored to `WaitForSingleObject(handle, 5000)`.
  - Removed immediate `trim_process_memory()` from `hide_main_window` to eliminate hard page faults and UI freeze when restoring from tray.

### Color Math & Optimization
- **BUG-02 (Rec.709 Coefficients for Black Holo)**:
  - Replaced symmetric Black Holo matrix weights with Rec.709 luminance-informed coefficients (`r_to_r = 1.0 + 0.15*k; r_to_g = 0.72*k; g_to_g = 1.0 - k; b_to_g = 0.28*k`), eliminating the parasitic green cast on skies, water, and bright highlights.
- **BUG-01**:
  - Added strict validation for baseline display gamma in `ensure_original_gamma_captured()`.
- **Opt-01**:
  - Fullscreen color matrix deduplication ($\epsilon = 10^{-5}$) and event-driven fullscreen monitoring via `SetWinEventHook` with zero timer polling overhead.

### UI, Usability & Binary Footprint Reduction
- **Universal Mouse Wheel Scrolling**: Fixed wheel scrolling across all page containers and sub-panels.
- **Interactive Slider Badges**: Slider value pills (e.g. `200%`) are now clickable with full keyboard navigation and precision value input.
- **57% Binary Size Reduction**: Migrated all UI raster images to optimized WebP format, reducing binary size from 11.4 MB to 4.85 MB.
- **DeepWiki Integration**: Added interactive documentation links to DeepWiki in README and UI.

---

## [2.4.0] - 2026-10-26

### Added
- **Seasonal Themes Engine**: Dynamic seasonal themes (Winter, Spring, Summer, Autumn) with automatic calendar detection and manual selection in Settings.
- **macOS Traffic Lights**: Optional window control buttons styled after macOS traffic lights.
- **Sidebar Sub-Navigation**: Smooth-scrolling sub-links under active tabs with dynamic Scroll Spy tracking.
- **1-Click First Bind Creation**: Direct creation button when keybind list is empty.
- **Interactive Documentation**: Direct links to GitDiagram architecture and DeepWiki.

---

### Performance & Resource Optimization
- **Zero-Footprint System Tray Lifecycle**:
  - Full teardown of WebView2 runtime and working sets when minimized to tray.
  - Background memory consumption reduced from ~130 MB to ~11 MB total (~2 MB for the primary `synchro.exe` host process).
- **Binary Fast-Boot Disk Cache (< 5 ms Boot)**:
  - Compact binary cache for hardware topology and display profile (`bincode`).
  - Instant direct application of display LUT/gamma at `main()` entry before GUI initialization.
- **Runtime Optimization Flags**:
  - Disabled extraneous GPU compositing and JIT overhead in background tasks (`--disable-gpu-compositing`, `--jitless`).

### Interface & Quality of Life
- **Tooltip Streamlining**: removed redundant browser cursor popups over filter tiles and sliders.
- **Codebase & Documentation Refresh**: purged debugging comments and streamlined documentation.

---

## [2.3.3] - 2026-10-14

### Performance & Resource Optimization
- **Background CPU Minimization**:
  - System integrity audit caching with 120s TTL to prevent background process churn.
  - Short-term TTL caching for game detection to eliminate repeated process snapshotting.
  - Registry inspection optimizations during tweak status evaluation.
  - Background monitoring pauses when window is hidden.
- **Release Build Optimization**:
  - Enabled thin LTO and `opt-level = 3` for maximum execution speed.
  - Omitted release overflow checks to streamline binary hot paths.

### Fixed
- **Filter Tab Layout Shift**: stabilized button geometry when toggling active color profiles.
- **Localization**: completed UI translations across preview modes and color toggles.
- **Audit Refresh**: immediate cache invalidation when triggering manual integrity checks.

---

## [2.3.2] - 2026-10-13

### Added
- Comprehensive **Anti-Cheat & EAC Safe Guard** engine: verified safe for Easy Anti-Cheat, BattlEye, Vanguard, and Ricochet.
- Automated system integrity auditing (checking `testsigning`, `debug`, `nointegritychecks`, game IFEO hijackings, and vulnerable driver blocklists).
- 1-click Anti-Cheat integrity recovery tool to proactively prevent kicks and bans in EAC games.
- Real-time Game Shield: automatically silences background process checks and memory trimming during active game sessions.
- Complete localization for the tweaks catalog across all 22 supported languages (group headings, impact ratings, safety badges, descriptions).
- Full translation support for driver and hardware categories on the characteristics page.
- Native translations for "Day", "Night", "Holo" display modes and the "Black Holo" enhancement toggle across all locales.
- Multilingual toast notifications for screenshot triggers, clipboard clearing, profile resets, and system alerts.

### Fixed
- Fixed critical display adapter class GUID bug in `apply_gpu_adaptive_power` and revert logic (replaced Network GUID with Display Adapter GUID `{4d36e968-e325-11ce-bfc1-08002be10318}`).
- Hardened `delete_tree` registry methods against root Windows hive corruption.
- Hardened process trimming to prevent opening handles to game or anti-cheat processes.
- Fixed UI thread hangs and stutter when navigating away from the tweaks page and toggling keybind sidebar items.
- Removed synchronous aggressive working set memory trimming (`EmptyWorkingSet`) that stalled rendering pipelines.
- Added CSS containment (`contain: layout style`, `will-change`) for instant layout recalculation during sidebar transitions.

---

## [2.3.1] - 2026-10-11

### Added
- Automatic Windows system language detection on first boot (Russian or English).
- Custom scrollable language dropdown menu supporting all 22 languages with native display names.
- Expanded i18n dictionaries and graceful English fallback for all 22 supported locales.
- DeepWiki documentation and interactive GitDiagram architecture diagram in README.

### Fixed
- Fixed Rust backend state sanitizer resetting selected languages back to default.
- Added click-outside and Escape key dismissal for the language dropdown menu.

---

## [2.3.0] - 2026-10-04

### Added
- Real driver freshness checks for display adapters (NVIDIA, AMD, Intel) and system hardware (chipsets, Realtek audio, network cards).
- Accurate available version badges and direct official download links for all supported vendors.
- Native Win32 `RegisterHotKey` global message pump for instantaneous hotkey responsiveness.
- New startup splash animation and Rust-themed brand identity lockup.
- Settings toggle to disable UI and splash animations.

### Fixed
- Fixed application icon resolution in Windows taskbar and system notification area.
- Fixed Common-Controls 6.0 TaskDialogIndirect manifest integration under MinGW GCC.
- Isolated asynchronous background driver scanning with zero UI thread contention.

---

## [2.2.4 (fix)] - 2026-09-27

### Fixed
- Black Holosight toggle switch behavior: knob state now reliably reflects active LUT status upon activation.
- Preserved and restored Windows System Restore Point creation frequency policy upon fallback.
- Eliminated redundant frontend action delay.

### Performance & Core
- Replaced custom SHA-256 routines with standard `sha2` (v0.10) crate.
- Enabled Link-Time Optimization (`lto = true`) and binary size optimization (`opt-level = "z"`), cutting executable size in half (to 6.8 MB).
- Codebase refactoring: removed dead tweak handlers, streamlined event dispatch logic, and deduplicated WebView2 initialization arguments.

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
