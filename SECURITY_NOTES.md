# Security Architecture & Threat Model

This document outlines the security architecture, threat model, and mitigation mechanisms implemented in Synchro Nova.

---

## 1. Threat Model & Boundaries

Synchro Nova operates under the following security assumptions:

- **Untrusted Frontend Layer**: The webview renderer (HTML/CSS/JS) is considered an untrusted presentation layer. It possesses no direct filesystem access, no raw network access, and cannot execute arbitrary system binaries.
- **Strict IPC Boundary**: Communication between the webview and the native Rust backend occurs exclusively through Tauri's typed command dispatcher (`invoke_handler`). Only explicitly registered commands can be invoked.
- **Least Privilege Execution**: Operations are split into unprivileged (HKCU registry, display gamma ramps) and elevated (HKLM registry, system services). Privileged commands explicitly verify administrator tokens prior to execution.
- **Anti-Cheat Coexistence**: The application must never interfere with anti-cheat software (Easy Anti-Cheat, BattlEye, Vanguard). It performs no process injection, installs no global keyboard/mouse hooks, and never modifies memory of third-party processes.

---

## 2. Process & OS Mitigations

During startup in `src-tauri/src/platform/ffi/security.rs`, the native process configures several Win32 security policies:

### Data Execution Prevention (DEP)
```rust
winapi::SetProcessDEPPolicy(PROCESS_DEP_ENABLE);
```
Enforces permanent DEP for the process lifetime, preventing execution of code in non-executable memory pages (data, heap, stack).

### DLL Search Order Hijacking Protection
```rust
winapi::SetSearchPathMode(BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT);
winapi::SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
```
Portable binaries running from user directories (`Desktop`, `Downloads`) are vulnerable to DLL preloading attacks if an attacker places a rogue DLL in the current working directory. `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS)` (0x00001000) restricts library resolution exclusively to `%SystemRoot%\System32` and the application's own directory, strictly eliminating the current working directory (CWD) and user `PATH` from the search order.

### Authenticode Signature Verification (WinVerifyTrust)
All auxiliary dynamic libraries (such as `WebView2Loader.dll`) and detected Edge WebView2 runtime executables are cryptographically validated prior to loading:
- Validation is performed in-process via `WinVerifyTrust` (`WINTRUST_ACTION_GENERIC_VERIFY_V2`) against the Windows Trust Provider.
- Unsigned, modified, or tampered binaries are rejected before any `LoadLibraryW` call can occur.
- This neutralizes DLL sideloading and local privilege escalation (LPE) vectors when the application restarts as an administrator.

### Heap Corruption Defense
```rust
winapi::HeapSetInformation(heap, HeapEnableTerminationOnCorruption, NULL, 0);
```
Enforces immediate process termination if the Windows heap manager detects corruption in internal metadata structures, neutralizing heap-based buffer overflow exploitation.

### Compiler Flags
Release binaries are built with:
- `panic = "abort"`: Eliminates stack unwinding code, reducing attack surface and binary size.
- `overflow-checks = true`: Prevents integer overflow exploitation in release builds.
- `opt-level = "z"` and `lto = true`: Link-time optimization with aggressive code elimination.

---

## 3. Frontend & IPC Defense

### Content Security Policy (CSP)
The WebView2 container enforces a locked-down CSP:
```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data: asset: asset://localhost http://asset.localhost https://asset.localhost;
font-src 'self';
connect-src 'self';
object-src 'none';
base-uri 'none';
frame-ancestors 'none';
form-action 'none';
```
- **Scripts**: `script-src 'self'` strictly blocks remote scripts (`<script src="https://...">`) and inline executable strings (`eval`, `setTimeout(string)`).
- **Styles**: `style-src 'self' 'unsafe-inline'` is scoped to allow dynamic DOM styling computed in JavaScript (such as user-selected RGB accent colors, theme variables, and gamma ramp slider fills).
- **Boundaries**: Sub-frames, plugins (`<object>`, `<embed>`), and form posts are completely disabled.

### Scoped Asset Protocol
The custom asset protocol (`asset:`) is restricted in `tauri.conf.json` strictly to `$APPDATA/**`, `$LOCALAPPDATA/**`, and `$RESOURCE/**`. Blanket filesystem wildcards (`**/*.png`, etc.) are disallowed. Specific game banners and icons discovered by the backend game detector are dynamically whitelisted via `app.asset_protocol_scope().allow_file()` on demand, preventing arbitrary disk reads.

### HTML and Attribute Escaping
All dynamic content rendered in the DOM passes through `escapeHtml()` in `src/app/core/html.js`, escaping all five XML/HTML entities (`&`, `<`, `>`, `"`, `'`). This prevents DOM-based cross-site scripting when rendering local file paths or game titles.

### URL Scheme Validation
Native URL handlers enforce strict scheme validation:
- External browser searches enforce `https://` only; non-HTTP schemes and local paths are rejected.
- Game launcher shortcuts strictly match `steam://`, `com.epicgames.launcher://`, or `riotclient://`.

---

## 4. System Modifications & Rollback Architecture

### Pre-Execution State Snapshots
Before applying any batch of system tweaks, the backend captures current registry values into a `.reg` file:
- Location: `%APPDATA%\app.synchro.performance\backups\tweak_safety_backup_<timestamp>.reg`
- Storage retention: The last 10 snapshots are preserved; older snapshots are pruned automatically.

### 1-Click Rollback
The `rollback_last_tweaks` command restores the most recent snapshot using Windows `reg import` and refreshes the application state.

### Privilege Verification
Tweaks that require administrative access (e.g. HKLM policies, `powercfg`, `netsh`) verify token elevation via `GetTokenInformation(TokenElevation)` before attempting execution. If not running as administrator, the application returns a clear error instead of generating partial or broken state.

### Audit Logging
All tweak executions, rollbacks, and failures are recorded in `%APPDATA%\app.synchro.performance\tweaks_audit.log`. The log automatically rolls over when reaching 512 KB.

### NSIS Installation Hardening
The official NSIS setup installer specifies `installMode: "perMachine"`, requiring UAC elevation during installation and deploying exclusively into `%ProgramFiles%\Synchro Nova`. This ensures all installed files and directories are governed by Windows Access Control Lists (ACLs), preventing unprivileged local users or malicious processes from modifying application binaries, configurations, or dependencies.

---

## 5. Anti-Cheat & Process Interaction

Synchro Nova is designed to be fully compatible with games running kernel-level or user-mode anti-cheats (Easy Anti-Cheat, BattlEye, Ricochet, Vanguard):

1. **No External Memory Access**: The application never calls `OpenProcess` with `PROCESS_VM_READ`, `PROCESS_VM_WRITE`, or `PROCESS_ALL_ACCESS` on game processes.
2. **No Hooking**: No API hooking (`Detours`, `MinHook`), no Windows message hooks (`SetWindowsHookEx`), and no driver-level filters are installed.
3. **Scoped Working Set Management**: Memory reduction via `SetProcessWorkingSetSize` is applied strictly to Synchro's own process (`synchro.exe`) and its direct child `msedgewebview2.exe` processes. Other processes are never inspected or modified.
4. **Offline Operation**: The application runs completely offline with no background listening ports or remote telemetry, avoiding network heuristic flags.

---

## 6. Verifying Releases

To ensure binary integrity:
1. Every release includes a `SHA256SUMS.txt` file containing cryptographic hashes of all binaries.
2. Official builds are compiled transparently via GitHub Actions from tagged commits.
3. Users can independently build the project using standard Rust toolchains (`cargo build --release`) to verify reproducibility.
