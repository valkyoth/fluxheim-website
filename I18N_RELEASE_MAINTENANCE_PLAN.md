# I18n and Release Maintenance Plan

## Goal

Keep the website visually identical while making future Fluxheim release updates
and language additions predictable, testable, and low-churn.

## Target State

- English (EU) remains the default root locale.
- English (UK), English (US), German, and French are selectable locales.
- Locale routes preserve the current page.
- Release data is centralized so a new Fluxheim release does not require manual
  edits across repeated HTML sections.
- Translations are maintained through stable content keys.
- Tests fail when release data, locale coverage, or language routes drift.

## Recommended Phases

### Phase 1: Locale Foundation

- Add English (UK) and English (US) locale entries.
- Render English variants as pass-through locales with correct `html lang`.
- Keep `/` as English (EU), and add `/en-gb/` and `/en-us/`.
- Expand language selector and route tests to cover all configured locales.

### Phase 2: Release Update Structure

- Add structured release metadata under `content/releases/`.
- Add validation that release files are ordered and match `config/site.toml`.
- Keep historical release versions stable during automated version updates.
- Add tooling that compares website metadata with the local Fluxheim checkout.

### Phase 3: Stable Translation Keys

- Add canonical English source keys under `config/i18n/keys/en-EU/`.
- Keep German and French translated through keyed translations.
- Keep English (EU), English (UK), and English (US) complete as pass-through
  English key sets until variant wording is intentionally changed.
- Enforce complete keys per locale.

Initial key files now live under `config/i18n/keys/` and are validated by
`scripts/check_i18n_keys.py`. They cover shared language-selector, shell,
navigation, release, footer, homepage, documentation-index, documentation pages,
source documentation, code comments, and common label copy. Legacy phrase
bundles have been removed; future translation work should add stable keys only.

`content/releases.toml` is also loaded by typed Rust code in `src/releases.rs`.
The current tests validate ordering, the latest version, release-note presence,
and whether listed versions are still rendered on the public release pages.

`scripts/plan_fluxheim_update.py` is the read-only entry point for future
Fluxheim version bumps. It compares the website with a local Fluxheim checkout
and prints the release notes, source docs, public pages, and verification steps
that need attention.

### Phase 4: Template Rendering

- Replace repeated release-table HTML with generated output from release data.
- Move shared layout strings to stable keys.
- Split stable keys by section before any locale key file approaches the
  500-line project limit.
- Keep legacy page HTML layout, CSS, URLs, and visible design unchanged.

### Phase 5: European Language Expansion

- Add new European languages only after key validation is strict.
- Generate new locale folders from English (EU) keys.
- Require every new language to pass key coverage and route tests before merge.

## Stop Outcome

- The site looks the same.
- The language selector includes English (EU), English (UK), English (US),
  German, and French.
- German and French still render translated pages.
- Future Fluxheim releases can be checked through scripts before editing.
- `scripts/checks.sh`, `scripts/smoke_local.sh`, and
  `scripts/podman_smoke.sh` pass.
