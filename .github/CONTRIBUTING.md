# Contributing to fluxheim-website

Fluxheim Website is a Rust 1.98.0 Axum application that serves the original
HTML site through centralized versioning and stable TOML i18n keys. The app
runs on port `8080` and is intended to sit behind Fluxheim or another TLS edge
proxy.

Contributions that keep the site lightweight, accessible, localized, and
secure are welcome.

## License

This project is licensed under the **European Union Public Licence 1.2
(EUPL-1.2)**. By contributing, you agree that your contribution is provided
under the same licence.

## Development Setup

Install the pinned Rust toolchain and run the app from the repository root:

```bash
cargo +1.98.0 run
```

Then visit `http://127.0.0.1:8080`.

Run the full local verification gate before opening a pull request:

```bash
scripts/checks.sh
```

For fast route checks, use the local smoke test:

```bash
scripts/smoke_local.sh
```

To test the container build locally:

```bash
scripts/podman_smoke.sh
```

## What Lives Where

| Path | Purpose |
|------|---------|
| `src/` | Axum app, routing, i18n, observability, and tests |
| `index.html`, `download.html`, `changelog.html` | Preserved legacy HTML page sources |
| `docs/` | Public documentation pages and mirrored source docs |
| `content/` | Release catalog and small reusable content snippets |
| `config/site.toml` | Shared site metadata, including Fluxheim version |
| `config/locales.toml` | Configured locale IDs, URL prefixes, and display names |
| `config/i18n/keys/` | Stable TOML translation keys for every configured locale |
| `assets/` | Vendored CSS, JavaScript, images, and flag assets |
| `container/` | Dockerfile, Podman compose files, and observability stack |
| `scripts/` | Local verification, i18n, release, and smoke-test tooling |

## Making Changes

**Updating copy:** Add shared visible text to the stable key files under
`config/i18n/keys/`. Every configured locale must have the same key shape.
Run `scripts/check_i18n_keys.py --progress` and
`scripts/i18n_coverage.py --all-configured --summary-only --fail-under 100`.

**Adding a locale:** Add the locale to `config/locales.toml`, scaffold keys with
`scripts/scaffold_i18n_locale.py`, translate the new TOML files, then run the
full local gate.

**Updating for a Fluxheim release:** Compare against the local Fluxheim checkout
with `scripts/plan_fluxheim_update.py --fluxheim ../fluxheim`, mirror missing
release notes and changed source docs, update the public release surfaces, and
keep all language keys aligned.

**Editing HTML:** The legacy HTML files remain the structural source for the
public pages. Keep navigation, footer, security headers, language selector
behavior, and static asset paths intact.

**Adding assets:** Keep assets local under `assets/`. Do not add external CDN
script or stylesheet references.

## Checks Before Opening a PR

- [ ] `scripts/checks.sh` passes
- [ ] `scripts/smoke_local.sh` passes for route and locale rendering changes
- [ ] `scripts/podman_smoke.sh` passes for container or deployment changes
- [ ] `scripts/check_i18n_keys.py --progress` shows complete locale coverage
- [ ] `scripts/i18n_coverage.py --all-configured --summary-only --fail-under 100` passes
- [ ] Version strings and release notes are consistent for release updates
- [ ] No external CDN URLs or unreviewed third-party scripts are introduced

## Security-Sensitive Areas

- **Routing:** Locale prefixes and legacy paths must remain allow-listed and
  traversal-safe.
- **Translations:** TOML i18n keys are trusted project data. Do not introduce
  user-controlled template paths or dynamic template evaluation.
- **Telemetry:** Keep observability aggregate and privacy-preserving. Do not log
  raw visitor IPs, raw user agents, or secret-bearing query strings.
- **Secrets:** Use `sanitization` for secret-shaped runtime values that need
  memory wiping.
- **Container:** Preserve the non-root `fluxheim` user and hardened runtime
  defaults.
- **Dependencies:** Run `cargo deny check` and `cargo audit`; explain any
  allowed advisory.

Do not post vulnerability details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Pull Requests

Keep PRs scoped to a single change. Include a clear summary of what changed,
why it changed, and which verification commands were run.
