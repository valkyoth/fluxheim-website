#!/usr/bin/env sh
set -eu

cargo +1.96.0 fmt --all --check
cargo +1.96.0 clippy --all-targets -- -D warnings
cargo +1.96.0 test
python3 -m py_compile scripts/check_i18n_keys.py scripts/i18n_coverage.py scripts/scaffold_i18n_locale.py
scripts/check_i18n_keys.py
tmp_i18n_root="$(mktemp -d /tmp/fluxheim-i18n-scaffold.XXXXXX)"
trap 'rm -rf "$tmp_i18n_root"' EXIT
mkdir -p "$tmp_i18n_root/config/i18n"
cp -a config/i18n/keys "$tmp_i18n_root/config/i18n/"
cp config/locales.toml "$tmp_i18n_root/config/locales.toml"
scripts/scaffold_i18n_locale.py \
  --root "$tmp_i18n_root" \
  --locale-id it-IT \
  --html-lang it-IT \
  --url-prefix it \
  --display-name Italiano >/dev/null
scripts/check_i18n_keys.py --root "$tmp_i18n_root"
scripts/check_release_data.py
if [ -d ../fluxheim ]; then
  scripts/plan_fluxheim_update.py --fluxheim ../fluxheim >/dev/null
fi
scripts/i18n_coverage.py --locale de --summary-only --fail-under 100
scripts/i18n_coverage.py --locale fr --summary-only --fail-under 100

if cargo deny --version >/dev/null 2>&1; then
  cargo deny check
else
  echo "cargo-deny not installed; skipping dependency policy check" >&2
fi

if cargo audit --version >/dev/null 2>&1; then
  cargo audit
else
  echo "cargo-audit not installed; skipping advisory check" >&2
fi
