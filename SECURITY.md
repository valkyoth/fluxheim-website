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
- Request paths are resolved against an immutable page cache rendered at startup.
- Security headers are applied to responses.
- Static documentation artifacts use no-follow file opens and a fixed size limit.
- Container runtime uses a non-root user, a read-only filesystem, dropped
  capabilities, a file-descriptor limit, and root-owned site content. Apply
  CPU, PID, memory, connection, and request-rate limits at the trusted Fluxheim
  edge or orchestrator because rootless cgroup delegation varies by host.
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

## Browser Policy Boundary

The preserved legacy frontend currently depends on vendored browser Tailwind,
Alpine expressions, and inline page styles. Its CSP therefore retains
`unsafe-eval` for scripts and `unsafe-inline` for scripts and styles. Other CSP
sources are restricted to the same origin. Removing these exceptions requires a
separate migration to precompiled CSS and external JavaScript and must include
full visual and mobile-navigation regression testing.

Browser-reported page-visible duration is untrusted, best-effort aggregate data.
It must not drive security alerts or billing. Outbound and download clicks are
recorded only by server-side, fixed-destination redirect routes.

Container image references include both immutable digests and readable version
tags. Refresh both together after verifying the upstream release and rerunning
the container smoke tests.
