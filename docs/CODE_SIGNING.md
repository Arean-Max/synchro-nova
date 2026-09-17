# Code Signing, Verification & Build Reproducibility

This document outlines how Synchro Nova binaries are compiled, verified, and signed.

---

## 1. Verifying Official Release Binaries

Every GitHub Release published on the [Releases page](https://github.com/Arean-Max/synchro-nova/releases) includes an official `SHA256SUMS.txt` file generated directly in the GitHub Actions runner.

### Verifying SHA-256 Checksums in PowerShell

To verify that your downloaded binary has not been modified or corrupted:

```powershell
# Compute hash of downloaded binary
Get-FileHash -Path .\synchro.exe -Algorithm SHA256

# Or verify against SHA256SUMS.txt directly:
Get-Content .\SHA256SUMS.txt
```

Compare the computed hash with the entry in `SHA256SUMS.txt`. If they match, the binary is identical to the one built by GitHub Actions.

---

## 2. Public CI/CD Builds (No Black-Box Binaries)

All official releases are compiled automatically via GitHub Actions workflows:
- [`.github/workflows/ci.yml`](../.github/workflows/ci.yml): Runs automated tests, Clippy, and syntax checks on all pull requests and commits.
- [`.github/workflows/release.yml`](../.github/workflows/release.yml): Compiles release binaries on tagged commits (`v*`), computes SHA-256 hashes, and uploads the artifacts.

Anyone can inspect the exact commit hash, build logs, and environment used to create each binary.

---

## 3. Code Signing (Authenticode)

### SmartScreen & Unknown Publisher
Windows SmartScreen displays an "Unknown Publisher" warning on binaries that are not signed with a recognized Authenticode certificate (EV or OV). For open-source projects without commercial code signing certificates ($300–$500/year), this is a common occurrence.

### Signing Builds with a Custom / Organizational Certificate
If you maintain a local CA or have an Authenticode certificate, you can sign the output binary using Microsoft `signtool.exe`:

```powershell
# Sign with an EV token or PFX certificate
signtool.exe sign `
    /tr http://timestamp.digicert.com `
    /td sha256 `
    /fd sha256 `
    /f "path\to\certificate.pfx" `
    /p "certificate-password" `
    "src-tauri\target\release\synchro.exe"

# Verify signature
signtool.exe verify /pa /v "src-tauri\target\release\synchro.exe"
```

---

## 4. Independent Local Compilation

If you prefer not to use prebuilt binaries, you can build Synchro Nova directly from source in under two minutes:

```bash
# Clone the repository
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# Install frontend dependencies
npm install

# Build portable executable
npm run build:portable
```

The resulting binary will be located at `src-tauri/target/release/synchro.exe`.
