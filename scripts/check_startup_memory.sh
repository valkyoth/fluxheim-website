#!/usr/bin/env sh
set -eu

limit_kib="${FLUXHEIM_STARTUP_RSS_LIMIT_KIB:-262144}"

binary="$(
  cargo +1.97.0 build \
    --release \
    --locked \
    --message-format=json-render-diagnostics |
    scripts/cargo_binary_path.py fluxheim-website
)"
if [ ! -x "$binary" ]; then
  echo "Cargo release binary is not executable: $binary" >&2
  exit 1
fi

rss_kib="$(
  FLUXHEIM_OTLP=disabled \
    scripts/measure_startup_memory.py \
    "$binary" \
    --startup-probe
)"
case "$rss_kib" in
  ''|*[!0-9]*)
    echo "invalid startup RSS measurement: $rss_kib" >&2
    exit 1
    ;;
esac

if [ "$rss_kib" -gt "$limit_kib" ]; then
  echo "startup RSS ${rss_kib} KiB exceeds ${limit_kib} KiB" >&2
  exit 1
fi

echo "startup RSS ok: ${rss_kib} KiB <= ${limit_kib} KiB"
