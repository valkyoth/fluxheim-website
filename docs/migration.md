# Legacy-to-Template Migration

The current Rust app intentionally serves the original HTML files so the public
site keeps the same visual design and content while the Rust runtime, container,
security headers, and locale routing are introduced.

When migrating a page to structured content, follow this order:

1. Keep the original HTML route working.
2. Move one repeated area into a template partial.
3. Add or update a test that proves the served page is still identical to the
   legacy HTML except for centralized version replacement, runtime phrase
   translation, locale metadata, and the injected bottom language selector.
4. Only then remove duplicated HTML from that page.

The `tests/legacy_routes.rs` suite exists to prevent accidental visual or
content drift. A structured-content migration must strengthen those checks, not
weaken them.

Routine localization should use phrase maps:

```text
config/i18n-de.toml
config/i18n-fr.toml
config/i18n/de/*.toml
config/i18n/fr/*.toml
```

Localized 1:1 page overrides are still supported only for exceptional cases
where a locale genuinely needs different markup:

```text
download.html
localized/de/download.html
localized/fr/download.html
```

If a localized file is missing, the app uses the English legacy file and applies
the configured runtime phrase map.
