#!/usr/bin/env sh
set -eu

cargo +1.96.0 fmt --all --check
cargo +1.96.0 clippy --all-targets -- -D warnings
cargo +1.96.0 test
python3 -m py_compile scripts/check_i18n_keys.py scripts/i18n_coverage.py scripts/scaffold_i18n_locale.py
scripts/check_i18n_keys.py
i18n_progress="$(scripts/check_i18n_keys.py --progress)"
case "$i18n_progress" in
  *"de-DE:"*"fr-FR:"*) ;;
  *)
    echo "i18n progress report missing configured non-English locales" >&2
    exit 1
    ;;
esac
i18n_untranslated="$(scripts/check_i18n_keys.py --list-untranslated all --untranslated-limit 1)"
case "$i18n_untranslated" in
  *"config/i18n/keys/de-DE/"*"config/i18n/keys/fr-FR/"*) ;;
  *)
    echo "i18n untranslated report missing locale edit paths" >&2
    exit 1
    ;;
esac
i18n_untranslated_tsv="$(
  scripts/check_i18n_keys.py \
    --list-untranslated all \
    --untranslated-limit 1 \
    --untranslated-format tsv
)"
case "$i18n_untranslated_tsv" in
  *"de-DE	"*"config/i18n/keys/de-DE/"*"fr-FR	"*"config/i18n/keys/fr-FR/"*) ;;
  *)
    echo "i18n untranslated TSV report missing locale edit rows" >&2
    exit 1
    ;;
esac
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
scripts/check_i18n_keys.py --root "$tmp_i18n_root" --allow-untranslated-locales
i18n_scaffold_untranslated="$(
  scripts/check_i18n_keys.py \
    --root "$tmp_i18n_root" \
    --allow-untranslated-locales \
    --list-untranslated all \
    --untranslated-limit 1
)"
case "$i18n_scaffold_untranslated" in
  *"it-IT:"*"config/i18n/keys/it-IT/"*) ;;
  *)
    echo "i18n scaffold untranslated report missing new locale edit path" >&2
    exit 1
    ;;
esac
if scripts/check_i18n_keys.py --root "$tmp_i18n_root" >/dev/null 2>&1; then
  echo "untranslated scaffold locale unexpectedly passed i18n validation" >&2
  exit 1
fi
scripts/check_release_data.py
if [ -d ../fluxheim ]; then
  scripts/plan_fluxheim_update.py --fluxheim ../fluxheim >/dev/null
fi
scripts/i18n_coverage.py --all-configured --summary-only --fail-under 100

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
