# Content Extraction Area

This directory is reserved for 1:1 extraction of the legacy HTML site into
structured TOML or Markdown-backed content.

Do not add simplified replacement pages here. Any extracted content must render
to the same legacy HTML output, except for centralized version replacement,
runtime phrase translation, and the injected bottom language selector.

German and French phrase maps live in `config/i18n-de.toml`,
`config/i18n-fr.toml`, and page-specific bundles under `config/i18n/`; do not
clone complete HTML pages for routine translation work.

`content/releases.toml` is the first structured release inventory. Keep it in
sync with `config/site.toml`, `download.html`, `changelog.html`, and
`docs/releases/` until release rendering is fully generated from data. The Rust
test suite already loads this file through `src/releases.rs`.
