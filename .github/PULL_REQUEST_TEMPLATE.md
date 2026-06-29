# Pull Request

## Summary

Describe what changed and why.

## Type

- [ ] Content / copy fix
- [ ] New or updated docs page
- [ ] Translation / locale update
- [ ] Release update (new Fluxheim version)
- [ ] Rust app / routing change
- [ ] Observability / telemetry change
- [ ] Container or config change
- [ ] Bug fix

## Checklist

- [ ] `scripts/checks.sh` passes
- [ ] `scripts/smoke_local.sh` passes when route, locale, or rendering behavior changes
- [ ] `scripts/podman_smoke.sh` passes when container/deployment behavior changes
- [ ] `scripts/check_i18n_keys.py --progress` shows complete locale coverage
- [ ] `scripts/i18n_coverage.py --all-configured --summary-only --fail-under 100` passes
- [ ] No external CDN URLs or unreviewed third-party scripts introduced
- [ ] Version strings are consistent across all affected pages
- [ ] `cargo deny check` and `cargo audit` pass, or any allowed advisory is explained

## Notes

Any follow-up work, known gaps, or browser-specific considerations.
