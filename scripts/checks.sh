#!/usr/bin/env sh
set -eu

cargo +1.96.0 fmt --all --check
cargo +1.96.0 clippy --all-targets -- -D warnings
cargo +1.96.0 test
scripts/i18n_coverage.py --locale de --summary-only --fail-under 86.6
scripts/i18n_coverage.py --locale fr --summary-only --fail-under 86.6

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
