# Security Notes

Synchro is a local Tauri application. Treat the frontend as untrusted UI: every sensitive decision must be made in Rust or by a server that the user cannot modify.

## Current hardening

- Release builds use `panic = "abort"`, `debug = false`, no incremental release artifacts, and a low-memory profile that can be stripped after build.
- The production CSP allows only local assets and blocks remote scripts, objects, frames, forms, and network connections.
- Tauri plugin permissions are empty; native access is exposed only through explicit Rust commands.
- Devtools are disabled in the Tauri window config for production.
- The window is resizable with strict minimum boundary constraints (`minWidth: 1100`, `minHeight: 720`).
- Settings, backups, and configs are stored in the app data directory, not beside the executable.
- JSON reads reject oversized files and non-regular files.
- JSON writes use a temp file and replace the target only after serialization succeeds.
- File names are allowlisted through safe ASCII stems with Windows reserved device name filtering (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`, `LPT1-9`); display names are length-limited and stripped of control characters.
- Color parameters are clamped and reject non-finite values before reaching Windows APIs.
- Unknown storage-folder requests are rejected.
- Anti-Cheat (EAC / BattlEye) Compatibility: Synchro performs zero process scanning, zero toolhelp snapshotting, zero external handle opening, and zero DLL injection. Working-set memory trimming is applied exclusively to Synchro's own process (`GetCurrentProcess()`), guaranteeing 100% safety and transparency alongside games protected by Easy Anti-Cheat.
- `assetProtocol.scope` permits loading local game artwork and icons while the CSP strictly enforces offline operation (`connect-src 'self'`, no remote assets/scripts).
- WinAPI invocations are centralized in `ffi.rs` with safe boundary wrappers, process-local memory trimming, and startup process DEP/safe search path mitigation.
- All tweak executions are logged to `%APPDATA%\app.synchro.performance\tweaks_audit.log` for traceability and accountability.
- Zero telemetry policy: all analytics, daily pings, and crash reporting have been 100% purged from both backend and frontend. The application operates strictly offline.
- The release batch copies only `dist\Synchro.exe` and removes PDB/debug side files.

## Reverse engineering

No local `.exe` can be made impossible to reverse. The goal is to raise effort and avoid shipping secrets.

- Do not put license secrets, payment logic, private algorithms, registry scripts, or privileged tweak decisions in JS.
- Keep Rust commands narrow and allowlisted.
- Do not expose broad shell execution to the frontend.
- Sign production binaries before public distribution.
- Keep `Cargo.lock` and `package-lock.json` committed and review dependency changes.
- Obfuscation can be added later, but it is not a security boundary.
- Residual risk: `withGlobalTauri` is enabled because the current frontend is static HTML/CSS/JS without a bundler import step. Keep CSP strict and never load remote HTML or scripts. If a bundler is added later, import only the exact Tauri APIs needed by the app.

## User safety

- Make a backup before any registry, service, BCDEdit, driver, or power-plan tweak.
- Mark risky changes as `AGGRESSIVE` and keep them out of safe presets.
- Require elevation only for commands that need it.
- Do not disable OS security features without an explicit warning and restore path.
- Keep telemetry completely absent: zero data collection, zero network tracking.
- Never apply boot or driver tweaks silently.
- Inspect `%APPDATA%\app.synchro.performance\tweaks_audit.log` if troubleshooting system changes.

## Release checklist

- Run `node --check src/main.js`.
- Run `cargo check` from `src-tauri`.
- Run `cargo test` from `src-tauri`.
- Build with `Build Synchro.bat`.
- Distribute only `dist\Synchro.exe` unless building an installer.
- Test on a clean Windows user account.
- Code-sign the final executable before release.
