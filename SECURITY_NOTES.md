# Security Notes & Hardening Architecture

Synchro Nova is a local, privacy-first desktop application designed with zero telemetry, complete offline operation, and rigorous defensive security. The frontend is treated as untrusted UI: every sensitive decision and system operation is strictly validated and executed in Rust.

## Current Hardening & Protection Measures

### 1. Process & OS Security Mitigations
- **Data Execution Prevention (DEP)**: Enforced via `SetProcessDEPPolicy(PROCESS_DEP_ENABLE)` at process startup.
- **Safe DLL Search Order (Anti-DLL Hijacking)**:
  - Enforced permanently via `SetSearchPathMode(BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT)`.
  - Enforced via `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)`: strictly restricts DLL resolution to `%SystemRoot%\System32`, completely blocking DLL preloading attacks from current working directories or user folders.
- **Heap Corruption Defense**: Enforced via `HeapSetInformation(HeapEnableTerminationOnCorruption)` on the default process heap; immediately terminates if heap tampering or overflow is detected.
- **Panic Policy**: Release builds use `panic = "abort"` to prevent unwind-based state inconsistency or information leakage.
- **Compiler Hardening**: Built with `overflow-checks = true`, `opt-level = "z"`, and `lto = true`.

### 2. User Protection & Fail-Safe Rollback Engine
- **Automatic Pre-Tweak Safety Snapshots**:
  - Before applying any system or registry tweak, Synchro automatically creates a timestamped safety snapshot (`Safety backup <timestamp>`) containing all previous registry values, color settings, and app options.
  - Automatic retention policy: automatically prunes older automated snapshots while preserving the last 10, preventing disk bloat.
- **1-Click Rollback (`rollback_last_tweaks`)**:
  - An explicit "Откатить изменения" (Rollback tweaks) button is directly available on the Tweaks page.
  - Instantly restores previous registry values and settings in 1 click, updating checkboxes in real time.
- **Administrator Privilege Verification**:
  - Tweak operations targeting `HKLM` or system services pre-verify elevation via token inspection (`GetTokenInformation(TokenElevation)`).
  - If not elevated, requests are cleanly rejected with clear guidance instead of failing with obscure Windows error codes.
- **Dangerous Tweaks Blocklist**:
  - Aggressive/unsafe tweaks (disabling memory integrity, dynamic tick tampering, bulk MSI mode, disabling paging file) are permanently blocked by the safe apply engine.
- **Full Tweak Audit Trail**:
  - Every tweak attempt, status, and message is permanently logged with timestamps to `%APPDATA%\app.synchro.performance\tweaks_audit.log` (with automatic log rotation at 512 KB).

### 3. Frontend & IPC Defense
- **Strict Content Security Policy (CSP)**:
  - `default-src 'self'`, `script-src 'self'`, `style-src 'self'`, `object-src 'none'`, `frame-ancestors 'none'`, `base-uri 'none'`, `connect-src 'self'`.
  - Remote scripts, inline scripts, remote frames, and external network connections are 100% blocked.
- **Full XML/HTML Entity Escaping**:
  - `escapeHtml` and `escapeAttr` escape all 5 critical entities: `&`, `<`, `>`, `"`, and `'` (`&#39;`).
  - Blocks DOM-based and attribute-based XSS injection.
- **Strict URL Scheme Allowlist**:
  - External driver searches strictly enforce `https://` only (`browser::open_url`).
  - Game launchers only allow validated custom protocols: `steam://`, `com.epicgames.launcher://`, and `riotclient://`.
- **Atomic File Storage & Path Traversal Guard**:
  - Settings, backups, and configs are written atomically using unique temporary files and synced (`sync_all`) before replacement.
  - Storage paths are strictly checked via `canonicalize()` and `starts_with(&root)` to prevent directory traversal (`../`).
  - Windows reserved filenames (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`, `LPT1-9`) are filtered out.

### 4. Anti-Cheat (EAC / BattlEye) Compatibility
- **Zero Process Tampering**:
  - Synchro performs **zero background game scanning**, **zero periodic system snapshots**, and **zero DLL injection**.
  - Memory trimming (`SetProcessWorkingSetSize`) is applied strictly to Synchro's own process tree (`synchro.exe` and its direct `msedgewebview2.exe` children).
  - Zero interference with game hooks, anti-cheat drivers, or protected memory regions.

### 5. Zero Telemetry & 100% Offline Architecture
- No analytics, tracking, daily pings, crash uploaders, or telemetry services exist in the code.
- No local HTTP web servers or listening TCP ports (`devUrl` removed; assets served directly via Tauri's embedded asset protocol).
- Operates 100% offline even without internet or local network adapters.

### 6. Resource & Memory Minimization
- **RAM**: Down to single-digit MBs (~7-8 MB total across `synchro.exe` and all WebView2 processes) via targeted working set management.
- **CPU**: 0.0% idle load; all background timers pause immediately when window is minimized (`visibilitychange` + `document.hidden`).
- **Disk I/O**: Game list is cached in-memory with a 60s TTL, eliminating repeat disk scans on tab navigation.
- **Heap Allocations**: PDH performance counter buffers are recycled across ticks, eliminating heap fragmentation.
