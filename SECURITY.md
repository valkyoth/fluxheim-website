# Security Policy

Security is paramount for Fluxheim Website. Treat route handling, templates,
localized content, dependency updates, and container changes as security-relevant
unless proven otherwise.

## Supported Version

The `main` branch is the active development line. Release branches should define
their support window in `docs/releases/`.

## Reporting a Vulnerability

Please report security issues privately through the project owner or the GitHub
security advisory flow when available. Do not disclose exploitable details in a
public issue before maintainers have had time to assess and fix the problem.

Include:

- affected route or component;
- reproduction steps;
- expected and observed behavior;
- dependency or container version when relevant;
- potential impact and suggested mitigation if known.

## Security Baseline

- Rust toolchain pinned to `1.96.0`.
- Askama templates are compiled and type checked.
- Localized content is parsed from bundled TOML into typed structures.
- Request paths are resolved against a fixed locale/page table.
- Security headers are applied to responses.
- Container runtime uses a non-root user.
- Dependency policy is checked with `cargo-deny`.
- Secrets must not be committed; secret-shaped runtime values should use
  `sanitization`.

## Local Security Checks

Run:

```bash
scripts/checks.sh
scripts/smoke_local.sh
```

Review `security/pentest/README.md` before focused security testing.
