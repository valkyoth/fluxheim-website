#!/usr/bin/env sh
set -eu

cargo +1.96.0 fmt --all --check
cargo +1.96.0 clippy --all-targets -- -D warnings
cargo +1.96.0 test
scripts/check_i18n_keys.py
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
