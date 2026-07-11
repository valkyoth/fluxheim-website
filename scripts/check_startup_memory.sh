#!/usr/bin/env sh
set -eu

limit_kib="${FLUXHEIM_STARTUP_RSS_LIMIT_KIB:-262144}"
report="$(mktemp /tmp/fluxheim-startup-rss.XXXXXX)"
trap 'rm -f "$report"' EXIT

if [ ! -x /usr/bin/time ]; then
  echo "startup memory gate requires /usr/bin/time" >&2
  exit 1
fi

cargo +1.96.1 build --release --locked
/usr/bin/time -f '%M' -o "$report" \
  env FLUXHEIM_OTLP=disabled target/release/fluxheim-website --startup-probe

rss_kib="$(cat "$report")"
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
