# Fluxheim Website

The official localized website for
[Fluxheim](https://github.com/valkyoth/fluxheim), built as a Rust 1.96.1
server-rendered application.

The app uses Axum for HTTP routing, TOML for project/locale metadata, and a
small hardened container runtime. It currently serves the original HTML site
unchanged, with only centralized version replacement and a small bottom language
selector injected when an immutable page cache is built at startup. It listens on port `8080` and is intended to
run behind Fluxheim or another TLS edge proxy.

## Locales

English (EU) is the default locale and renders at the root paths:

- `/`
- `/download`
- `/docs`

English (UK), English (US), German, and French render through stable locale
prefixes:

- `/en-gb/`
- `/en-gb/download`
- `/en-us/`
- `/en-us/download`
- `/de/`
- `/de/download`
- `/fr/`
- `/fr/download`

The application also accepts `/en-eu/...` as an explicit English (EU) prefix,
while generated links prefer the default root paths.

English (UK) and English (US) are currently pass-through English variants with
their own `html lang` values. All locale text is keyed under
`config/i18n/keys/`. The legacy HTML files remain the structural source of
truth, so updating the site does not require editing cloned HTML trees.

Legacy static artifacts under `docs/source/`, such as Markdown and TSV files,
are served through bounded, no-follow reads on the same locale-prefixed paths.
The legacy `conf/fluxheim.toml` example is also preserved.

## Project Layout

```text
fluxheim-website/
├── src/                  # Axum app, legacy routing, locale loading, tests helpers
├── content/              # Reserved for future 1:1 content extraction
├── config/               # Shared project metadata, including Fluxheim version
├── config/i18n/keys/     # Stable shared locale keys for all configured locales
├── conf/                 # Preserved Fluxheim example configuration
├── localized/            # Optional 1:1 localized HTML overrides
├── assets/               # Vendored CSS, JavaScript, and images
├── container/            # Podman/Docker deployment files
├── scripts/              # Local verification and smoke tests
├── docs/releases/        # Release-note process
└── security/pentest/     # Security testing notes
```

## Version Updates

Update the Fluxheim version once in `config/site.toml`:

```toml
fluxheim_version = "1.7.8"
```

The Rust response renderer injects that value into download buttons, footers,
release text, and localized pages while preserving the original HTML source.

## Translation Updates

Add or adjust shared site text in the stable key files:

```toml
[common]
download = "Herunterladen"
```

Every configured locale must contain the same stable key set. Validate that with:

```bash
scripts/check_i18n_keys.py
```

To see non-English translation progress against English (EU), run:

```bash
scripts/check_i18n_keys.py --progress
```

To list stable keys that still match English for one locale, with the TOML file
to edit, run:

```bash
scripts/check_i18n_keys.py --list-untranslated de-DE
scripts/check_i18n_keys.py --list-untranslated all --untranslated-limit 20
scripts/check_i18n_keys.py --list-untranslated all --untranslated-format tsv
scripts/check_i18n_keys.py --list-untranslated fr-FR --untranslated-limit 0
```

When adding a locale, add it to `config/locales.toml`, create matching
`config/i18n/keys/{locale}.toml` and `config/i18n/keys/{locale}/` files from
English (EU), and add every configured locale display name under
`[language.names]`. The scaffold script handles the mechanical part:

```bash
scripts/scaffold_i18n_locale.py \
  --locale-id it-IT \
  --html-lang it-IT \
  --url-prefix it \
  --display-name Italiano
```

The new locale starts with English (EU) key values so translation can proceed
directly in TOML while `scripts/check_i18n_keys.py` keeps the shape complete.
The Rust build generates the runtime key registry from `config/locales.toml`.
Once the locale is configured in the real site, `scripts/check_i18n_keys.py`
also rejects under-translated non-English locales when more than 20% of
comparable text values still match English (EU).

Do not reintroduce legacy phrase bundles such as `config/i18n-de.toml` or
`config/i18n/de/*.toml`. `scripts/check_i18n_keys.py` fails if those removed
runtime translation files come back.

Measure source-page coverage against the English (EU) stable keys:

```bash
scripts/i18n_coverage.py --all-configured
```

`scripts/checks.sh` enforces 100% source visible-phrase coverage while
`scripts/check_i18n_keys.py` keeps every configured locale aligned to that same
stable key shape.

## Running Locally

```bash
cargo +1.96.1 run
```

Open <http://127.0.0.1:8080/>.

## Verification

Run the full local gate:

```bash
scripts/checks.sh
```

The gate includes a release-mode startup memory check with a default maximum
RSS of 256 MiB. Its standard-library measurement supports Linux and macOS RSS
units. Override `FLUXHEIM_STARTUP_RSS_LIMIT_KIB` only for a documented
platform-specific reason.

Compare the website release metadata with a local Fluxheim checkout before a
version update:

```bash
scripts/plan_fluxheim_update.py --fluxheim ../fluxheim
scripts/check_fluxheim_source.py --fluxheim ../fluxheim
```

Run a local smoke test:

```bash
scripts/smoke_local.sh
```

The test suite checks localized routes, security headers, version injection,
language-selector links, exact legacy HTML preservation, translation coverage,
and the 500-line authored file limit.

## Container

Build and run with Podman Compose:

```bash
podman compose -f container/podman-compose.yml up --build
```

OTLP export is runtime-controlled. The regular compose file defaults it to
disabled; the observability stack defaults it to enabled:

```bash
FLUXHEIM_OTLP=enabled podman compose -f container/podman-compose.yml up --build
OTEL_COLLECTOR_BEARER_TOKEN="$(openssl rand -hex 32)" podman compose -f container/observability/podman-compose.yml up --build
FLUXHEIM_OTLP=disabled podman compose -f container/observability/podman-compose.yml up --build
```

The app emits HSTS by default. Set `FLUXHEIM_HSTS=disabled` only when TLS is
not terminated before responses reach browsers.
The observability stack binds local UI ports to `127.0.0.1` and keeps OTLP
receiver ports internal to the compose network.
Set `GRAFANA_ADMIN_PASSWORD` to a unique secret outside local development.
Browser-reported visible time is privacy-preserving, untrusted aggregate input;
server-validated redirects record GitHub and download clicks without accepting
client-supplied click events.

Or build manually:

```bash
podman build -f container/Dockerfile -t fluxheim-website:0.1.0 .
podman run --rm -p 8080:8080 fluxheim-website:0.1.0
```

Run the container smoke test:

```bash
scripts/podman_smoke.sh
```

Production topology:

```text
browser -> Fluxheim edge/TLS proxy -> fluxheim-website:8080
```

## Security

Security is part of the default development path:

- Rust 1.96.1 is pinned in `rust-toolchain.toml`.
- Dependencies are pinned through `Cargo.lock`.
- `cargo-deny` policy lives in `deny.toml`.
- Locale routing uses fixed configured locale prefixes.
- Legacy page paths are normalized and reject traversal.
- Runtime translation uses fixed TOML stable keys; no user-controlled template
  or file path is evaluated.
- Security headers are applied by the Axum router.
- Runtime container uses a non-root user, read-only filesystem, dropped
  capabilities, a file-descriptor limit, and root-owned content. CPU, PID,
  memory, connection, and request-rate ceilings belong at the trusted Fluxheim
  edge or orchestrator because rootless cgroup delegation varies by host.
- Secret-shaped future runtime values should use the `sanitization` crate.

## License

This website is licensed under the **EUPL-1.2**.
