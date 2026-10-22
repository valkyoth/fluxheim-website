# Fluxheim 1.8.3 Release Notes

**Released 22 October 2026.** This maintenance release was built from signed
tag `v1.8.3` at commit `14d7b6cfc9b16c39a8dc0d4770f59290c8feea48`.
Pentest/retest, GitHub CI, the local deep release gate, native Windows and
Apple Silicon macOS archive checks, and native Linux ARM64 qualification all
passed before publication.

See the [qualification record](../source/maintenance-1.8.3-qualification.md)
for the completed platform and release gates.

## Maintenance Scope

- Rust 1.99.1 and refreshed stable dependencies, including Tokio 1.54.1,
  Rustls 0.23.46, sanitization 2.1.1 and base64-ng 2.1.0.
- Wasmtime/WASI 50.0.0 for the explicit Wasm profile. Existing host-call,
  memory/fuel/timeout and fail-closed policy contracts remain in force.
  `full` still excludes Wasm. Modules must be trusted and integrity-pinned;
  sandboxing is not a reason to accept arbitrary untrusted deployment files.
- Refreshed immutable Rust and runtime container image digests, pinned Docker
  Actions, and checksum-verified Rustup 1.29.2 installers.
- Windows release-builder startup/evidence fixes. Read-only registry host
  metadata replaces the WMI query, avoiding manual WMI ACL grants.
- Rust 1.99's replacement atomic update API preserves UDP session admission
  ordering and limits without changing the configuration contract.
- Fix the standalone systemd boundary's rustix feature declaration and align
  the RPM compiler requirement with Cargo's full Rust 1.99.1 minimum. Both
  checks are now exercised independently in CI.

## Container Maintenance

The [runtime refresh workflow](../source/container-runtime-refresh.md) rebuilds
the latest published release's containers with refreshed OS/PHP packages and
requires live smoke checks before promoting versioned image aliases. Unique
rebuild tags/digests identify each runtime. It does not rewrite source tags,
archive checksums, or Fluxheim's application version.

Refresh source must be an authorized SSH-signed release tag. Promotion uses
the tested digest recorded in same-run evidence, not a mutable candidate tag.
Operators must configure the protected `container-refresh-release` environment
and its `FLUXHEIM_RELEASE_ALLOWED_SIGNERS` secret before automatic refreshes
can publish. Without that trust anchor the workflow fails closed; normal
release packaging does not depend on this optional refresh setup.

## Compatibility And Qualification

No new protocol, default feature profile, managed Windows PHP, management API
or observability redesign is introduced here. Windows stays x86_64 MSVC;
macOS stays Apple Silicon. Existing configs should remain valid.

The Wasmtime major update requires sandbox/policy regression tests, live
request tests and tests of the extracted Wasm release archive. Native Windows
and macOS archive/runtime checks passed for this release, not merely on
the earlier 1.8.2 release. Linux Wasm archives and live policy execution,
container smokes, PHP image parity and ARM64 profile archives passed too.
Detailed evidence and publication gates are tracked in the
qualification record. See the
[maintenance policy](../source/maintenance-releases.md).

Main stays at 1.9.0 for logging, metrics and operational visibility. HTTP/3
research is deferred there; it is absent from this release's dependency graph
and CI gates. Brynja/Vef are not production dependencies.
