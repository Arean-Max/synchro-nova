# Security Notes

Synchro is a local Tauri application. Treat the frontend as untrusted UI: every sensitive decision must be made in Rust or by a server that the user cannot modify.

## Current hardening

- Release builds use `panic = "abort"`, `debug = false`, no incremental release artifacts, and a low-memory profile that can be stripped after build.
- The production CSP allows only local assets and blocks remote scripts, objects, frames, forms, and network connections.
- Tauri plugin permissions are empty; native access is exposed only through explicit Rust commands.
- Devtools are disabled in the Tauri window config.
- The window is fixed-size in config and again at runtime.
- Settings, backups, and configs are stored in the app data directory, not beside the executable.
- JSON reads reject oversized files and non-regular files.
- JSON writes use a temp file and replace the target only after serialization succeeds.
- File names are allowlisted through safe ASCII stems; display names are length-limited and stripped of control characters.
- Color parameters are clamped and reject non-finite values before reaching Windows APIs.
- Unknown storage-folder requests are rejected.
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
- Keep telemetry off by default unless the user opts in.
- Never apply boot or driver tweaks silently.

## Release checklist

- Run `node --check src/main.js`.
- Run `cargo check` from `src-tauri`.
- Build with `Build Synchro.bat`.
- Distribute only `dist\Synchro.exe` unless building an installer.
- Test on a clean Windows user account.
- Code-sign the final executable before release.
