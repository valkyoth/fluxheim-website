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

- Rust toolchain pinned to `1.99.1`.
- HTML and locale data are embedded at build time; translation output is escaped
  and rich markup is restricted to source-approved tags.
- Localized content is parsed from bundled TOML into typed structures.
- Request paths are resolved against an immutable page cache rendered at startup.
- Translation matchers are bounded per worker and released after each locale;
  release startup RSS is gated at 256 MiB against the exact executable emitted
  by the locked Cargo build.
- Security headers are applied to responses.
- Static documentation artifacts are allow-listed and embedded during the
  locked build. Build-time traversal rejects symlinks, artifacts are limited
  to 2 MiB, and the runtime serves immutable in-memory bytes without filesystem
  access.
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

These checks are a local maintainer release gate; this repository does not
enforce them through required GitHub Actions or branch protection. CodeQL uses
GitHub Default setup. Deployments requiring enforced merge controls should
configure them separately.

## Observability Deployment Boundary

The optional rootless observability stack binds its published ports to loopback.
Its Grafana password (`admin`) and OTLP token (`fluxheim-local-otlp-token`) are
local-development defaults, not production secrets. Set `GRAFANA_ADMIN_PASSWORD`
and `OTEL_COLLECTOR_BEARER_TOKEN` before using the stack on a shared or production
host. Changing Grafana's bootstrap password does not reset an existing database's
admin password; change that password in Grafana as well.

Auxiliary services have read-only roots, dropped capabilities, no-new-privileges,
PID limits and bounded temporary storage. Prometheus and Grafana retain their
explicit writable data volumes. Internal OTLP uses HTTP on the compose network;
TLS/mTLS and host-specific CPU/memory ceilings are deployment choices. This
local stack does not claim military or high-assurance isolation.

## Browser Policy Boundary

The preserved legacy frontend currently depends on vendored browser Tailwind,
Alpine expressions, and inline page styles. Its CSP therefore retains
`unsafe-eval` for scripts and `unsafe-inline` for scripts and styles. Other CSP
sources are restricted to the same origin. Removing these exceptions requires a
separate migration to precompiled CSS and external JavaScript and must include
full visual and mobile-navigation regression testing.

Browser-reported page-visible duration is untrusted, best-effort aggregate data.
It must not drive security alerts or billing. Outbound and download clicks are
recorded only by server-side, fixed-destination redirect routes. Download
artifacts must also exist in the immutable rendered-site allowlist, keeping
metric labels bounded while preserving published historical releases.

Container image references include both immutable digests and readable version
tags. Refresh both together after verifying the upstream release and rerunning
the container smoke tests. The pinned Wolfi runtime already provides the CA
bundle, so the image build performs no mutable package-repository install.
