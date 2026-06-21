# Release Notes

Fluxheim Website release notes must cover user-visible website changes,
localized HTML overrides, dependency changes, container changes, and security
impact.

Before a release:

- update `config/site.toml` when the Fluxheim version changes;
- update English (EU), German, and French localized overrides together when
  those files exist;
- run `scripts/checks.sh`;
- run `scripts/smoke_local.sh`;
- run `scripts/podman_smoke.sh`.
