# Fluxheim Website

The official localized website for
[Fluxheim](https://github.com/valkyoth/fluxheim), built as a Rust 1.96.0
server-rendered application.

The app uses Axum for HTTP routing, TOML for project/locale metadata, and a
small hardened container runtime. It currently serves the original HTML site
unchanged, with only centralized version replacement and a small bottom language
selector injected at response time. It listens on port `8080` and is intended to
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
are still served directly through the same locale-prefixed paths.
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
fluxheim_version = "1.6.28"
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

Do not reintroduce legacy phrase bundles such as `config/i18n-de.toml` or
`config/i18n/de/*.toml`. `scripts/check_i18n_keys.py` fails if those removed
runtime translation files come back.

Measure translation coverage for the main public pages:

```bash
scripts/i18n_coverage.py --locale de
scripts/i18n_coverage.py --locale fr
```

`scripts/checks.sh` enforces 100% visible-phrase coverage for German and French
on the tracked public-page set.

## Running Locally

```bash
cargo +1.96.0 run
```

Open <http://127.0.0.1:8080/>.

## Verification

Run the full local gate:

```bash
scripts/checks.sh
```

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

- Rust 1.96.0 is pinned in `rust-toolchain.toml`.
- Dependencies are pinned through `Cargo.lock`.
- `cargo-deny` policy lives in `deny.toml`.
- Locale routing uses fixed configured locale prefixes.
- Legacy page paths are normalized and reject traversal.
- Runtime translation uses fixed TOML stable keys; no user-controlled template
  or file path is evaluated.
- Security headers are applied by the Axum router.
- Runtime container uses a non-root user.
- Secret-shaped future runtime values should use the `sanitization` crate.

## License

This website is licensed under the **EUPL-1.2**.
