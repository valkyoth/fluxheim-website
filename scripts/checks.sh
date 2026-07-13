#!/usr/bin/env sh
set -eu

cargo +1.97.0 fmt --all --check
cargo +1.97.0 clippy --all-targets -- -D warnings
cargo +1.97.0 test
scripts/cargo_binary_path.py --self-test
scripts/measure_startup_memory.py --self-test
scripts/check_startup_memory.sh
python3 -m py_compile scripts/check_i18n_keys.py scripts/i18n_coverage.py scripts/scaffold_i18n_locale.py scripts/check_english_variants.py scripts/check_i18n_fallback_prefixes.py scripts/check_i18n_english_fragments.py scripts/check_i18n_collapsed_values.py scripts/check_i18n_integrity.py scripts/cargo_binary_path.py scripts/measure_startup_memory.py
scripts/check_i18n_keys.py
scripts/check_i18n_fallback_prefixes.py
scripts/check_i18n_english_fragments.py
scripts/check_i18n_collapsed_values.py
scripts/check_i18n_integrity.py --self-test
scripts/check_i18n_integrity.py
scripts/check_english_variants.py
for locale in ja-JP ko-KR; do
  unchanged="$(scripts/check_i18n_keys.py --list-untranslated "$locale" --include-intentional --untranslated-limit 1)"
  case "$unchanged" in
    *"$locale: 0 keys still match en-EU"*) ;;
    *)
      echo "$locale still has copied English key values" >&2
      echo "$unchanged" >&2
      exit 1
      ;;
  esac
done
i18n_progress="$(scripts/check_i18n_keys.py --progress)"
case "$i18n_progress" in
  *"de-DE:"*"fr-FR:"*"sv-SE:"*"nb-NO:"*"nl-NL:"*"fi-FI:"*"is-IS:"*"da-DK:"*"es-ES:"*"pt-PT:"*"et-EE:"*"lv-LV:"*"el-GR:"*"it-IT:"*"lt-LT:"*"hr-HR:"*"cs-CZ:"*"bs-BA:"*"bg-BG:"*"de-CH:"*"ro-RO:"*"pl-PL:"*"ru-RU:"*"ja-JP:"*"ko-KR:"*"hu-HU:"*) ;;
  *)
    echo "i18n progress report missing configured non-English locales" >&2
    exit 1
    ;;
esac
i18n_untranslated="$(scripts/check_i18n_keys.py --list-untranslated all --untranslated-limit 1 --include-intentional)"
case "$i18n_untranslated" in
  *"config/i18n/keys/de-DE/"*"config/i18n/keys/fr-FR/"*"config/i18n/keys/sv-SE/"*) ;;
  *)
    echo "i18n untranslated report missing locale edit paths" >&2
    exit 1
    ;;
esac
i18n_untranslated_tsv="$(
  scripts/check_i18n_keys.py \
    --list-untranslated all \
    --untranslated-limit 1 \
    --untranslated-format tsv \
    --include-intentional
)"
case "$i18n_untranslated_tsv" in
  *"de-DE	"*"config/i18n/keys/de-DE/"*"fr-FR	"*"config/i18n/keys/fr-FR/"*"sv-SE	"*"config/i18n/keys/sv-SE/"*) ;;
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
  --locale-id zz-ZZ \
  --html-lang zz-ZZ \
  --url-prefix zz-test \
  --display-name "Scaffold Test" >/dev/null
scripts/check_i18n_keys.py --root "$tmp_i18n_root" --allow-untranslated-locales
i18n_scaffold_untranslated="$(
  scripts/check_i18n_keys.py \
    --root "$tmp_i18n_root" \
    --allow-untranslated-locales \
    --list-untranslated all \
    --untranslated-limit 1
)"
case "$i18n_scaffold_untranslated" in
  *"zz-ZZ:"*"config/i18n/keys/zz-ZZ/"*) ;;
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

cargo deny check
cargo audit
