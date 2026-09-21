# Security Policy

## Supported Versions

Only the latest release branch is currently supported for security updates.

| Version | Supported          |
| ------- | ------------------ |
| 2.1.x   | :white_check_mark: |
| < 2.1.0 | :x:                |

---

## Reporting a Vulnerability

The Synchro Nova team takes the security of our application and our users seriously. If you believe you have discovered a vulnerability, security flaw, or unexpected elevation behavior, please report it responsibly.

### How to Report

1. **GitHub Private Vulnerability Reporting**:
   Use the **"Report a vulnerability"** button under the [Security Advisories](https://github.com/Arean-Max/synchro-nova/security/advisories) tab of the repository. This opens a private discussion thread directly with the maintainers.

2. **Direct Contact**:
   If private vulnerability reporting is unavailable, open an issue labeled `security: confidential` without publishing proof-of-concept exploit details, or contact the project maintainer directly via GitHub profile: [@Arean-Max](https://github.com/Arean-Max).

### What to Include in Your Report

To help us triage and reproduce the issue quickly, please include:
- Affected version or commit hash.
- Step-by-step reproduction instructions.
- Potential impact (e.g., local privilege escalation, unexpected registry write, path traversal).
- Proposed fix or mitigation, if available.

### Response Timeline

- **Initial Response**: Within 48 hours of receiving the report.
- **Status Update**: Regular updates during investigation and patch development.
- **Fix Release**: Security patches will be prioritized and released promptly with public disclosure coordinated upon release.

---

## Security Architecture

For detailed information on the application's threat model, WinAPI mitigation flags, frontend sandboxing, and anti-cheat compatibility, see [SECURITY_NOTES.md](SECURITY_NOTES.md).
