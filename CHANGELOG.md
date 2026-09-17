# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-17

### Added
- **Display Calibration Engine**: Direct Windows GDI gamma ramp control for saturation, vibrance, contrast, and color balance.
- **System Tweaks Module**: Granular system optimizations categorized by privilege level (HKCU user tweaks and HKLM system tweaks).
- **Safety Snapshot & Rollback**: Automated registry backup creation (`.reg`) before any tweaks are applied, with 1-click rollback support.
- **Hardware Telemetry**: Real-time CPU and memory metrics via Windows Performance Data Helper (PDH).
- **GPU & Driver Inspector**: Automatic detection of active display adapter, driver version, and direct search link.
- **Game Library Integration**: Auto-discovery of installed Steam and Epic Games titles with custom color profile linking.
- **CI/CD Infrastructure**: GitHub Actions workflows for continuous integration testing and automated release artifact compilation with SHA-256 checksums.

### Security
- Added permanent Data Execution Prevention (DEP) enforcement on startup.
- Implemented `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)` and `SetSearchPathMode` to block DLL search order hijacking attacks.
- Enabled heap corruption termination (`HeapEnableTerminationOnCorruption`) on default process heap.
- Hardened HTML rendering engine with full entity escaping (`&`, `<`, `>`, `"`, `'`).
- Restricted driver search query URLs strictly to `https://` schemes.
- Added strict privilege verification before executing system-level modifications.

### Performance
- Restricted WebView2 working set memory via targeted process tree trimming, reducing idle memory footprint to ~8 MB RAM.
- Added in-memory caching for game library scans with 60-second TTL to eliminate redundant disk I/O.
- Recycled thread-local PDH query buffers to avoid heap reallocations during telemetry collection.
