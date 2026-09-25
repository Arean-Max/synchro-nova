# Contributing to Synchro Nova

Thank you for your interest in contributing to Synchro Nova! We welcome bug reports, feature suggestions, documentation improvements, and pull requests.

---

## Code of Conduct & Ground Rules

1. **Safety First**: Any modifications to the Windows registry, system settings, or hardware controls must include explicit verification, sanitization, and automated rollback support.
2. **Anti-Cheat Integrity**: Synchro Nova must never inject code, hook third-party processes, or tamper with external memory. Contributions that violate this rule will be rejected.
3. **100% Offline & Private**: Zero telemetry, analytics, or unsolicited network calls. The application functions in completely air-gapped environments; release updates are strictly opt-in and check GitHub only when manually clicked or when the auto-update setting is explicitly enabled.

---

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or v20 LTS recommended)
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (stable `x86_64-pc-windows-msvc` toolchain)
- Microsoft Visual Studio C++ Build Tools (with Windows SDK)

### Getting Started

```bash
# Fork and clone the repository
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# Install frontend dependencies
npm install

# Run application in development mode
npm run dev
```

---

## Verification & Testing

Before submitting a Pull Request, ensure all tests and lint checks pass:

```bash
# 1. Run Rust test suite
cargo test --manifest-path src-tauri/Cargo.toml

# 2. Run Clippy checks
cargo clippy --manifest-path src-tauri/Cargo.toml --no-deps

# 3. Check JavaScript syntax
node --check src/app/main.js
node --check scripts/app-server.mjs
```

---

## Pull Request Guidelines

1. **Branch Naming**: Use descriptive names such as `feat/custom-gamma-curve` or `fix/display-detection-fallback`.
2. **Commit Messages**: Follow standard conventional commits format:
   - `feat(category): description`
   - `fix(module): description`
   - `docs(reference): description`
   - `perf(metrics): description`
3. **Documentation**: If adding new system tweaks or modifying registry keys, update [`docs/TWEAKS_REFERENCE.md`](docs/TWEAKS_REFERENCE.md).
4. **Unit Tests**: Include corresponding tests in `src-tauri/src/lib.rs` for new backend logic or parser functions.
