#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="${ROOT}/container/observability/podman-compose.yml"
BASE_URL="${BASE_URL:-http://127.0.0.1:8080}"
PROM_URL="${PROM_URL:-http://127.0.0.1:9090}"
GRAFANA_URL="${GRAFANA_URL:-http://127.0.0.1:3000}"
JAEGER_URL="${JAEGER_URL:-http://127.0.0.1:16686}"
project_name="${FLUXHEIM_OBSERVABILITY_PROJECT_NAME:-fluxheim_website_observability_smoke}"
container_prefix="${FLUXHEIM_OBSERVABILITY_CONTAINER_PREFIX:-fluxheim-website-smoke}"
smoke_compose_file="$(mktemp "${ROOT}/container/observability/.smoke-compose.XXXXXX.yml")"
export FLUXHEIM_OBSERVABILITY_PROJECT_NAME="${project_name}"
export FLUXHEIM_OBSERVABILITY_CONTAINER_PREFIX="${container_prefix}"

compose() {
  podman compose -p "${project_name}" -f "${smoke_compose_file}" "$@"
}

cleanup() {
  if [[ -s "${smoke_compose_file}" && "${OBSERVABILITY_SMOKE_DOWN:-1}" == "1" ]]; then
    compose down --volumes
  fi
  rm -f "${smoke_compose_file}"
}
trap cleanup EXIT

sed '1{/^name: /d;}' "${COMPOSE_FILE}" >"${smoke_compose_file}"
compose up -d --build --force-recreate

wait_for() {
  local url="$1"
  local name="$2"
  for _ in $(seq 1 90); do
    if curl -fsS "${url}" >/dev/null; then
      return 0
    fi
    sleep 1
  done
  echo "observability smoke failed waiting for ${name}: ${url}" >&2
  return 1
}

wait_for "${BASE_URL}/healthz" "website"
wait_for "${PROM_URL}/-/ready" "prometheus"
wait_for "${GRAFANA_URL}/api/health" "grafana"
wait_for "${JAEGER_URL}/" "jaeger"

grafana_password="${GRAFANA_ADMIN_PASSWORD:-admin}"
if [[ -n "${GRAFANA_ADMIN_PASSWORD:-}" ]]; then
  podman exec "${container_prefix}-grafana" \
    grafana cli admin reset-admin-password "${grafana_password}" >/dev/null
fi

curl -fsS "${BASE_URL}/" >/dev/null
curl -fsS "${BASE_URL}/en-gb/index.html" >/dev/null
curl -fsS "${BASE_URL}/en-us/index.html" >/dev/null
curl -fsS "${BASE_URL}/sv/" >/dev/null
curl -fsS "${BASE_URL}/en-gb/assets/css/theme.css?v=20260519" >/dev/null
curl -fsS "${BASE_URL}/en-us/assets/css/theme.css?v=20260519" >/dev/null
curl -fsS "${BASE_URL}/sv/assets/css/theme.css?v=20260519" >/dev/null
curl -fsS "${BASE_URL}/de/docs" >/dev/null
curl -fsS "${BASE_URL}/fr/download" >/dev/null
curl -fsS "${BASE_URL}/privacy" >/dev/null
curl -fsS -X POST "${BASE_URL}/telemetry/page-visible" \
  -H "content-type: application/json" \
  --data '{"locale":"en-EU","route":"/","section":"home","seconds":7}' >/dev/null
curl -fsS "${BASE_URL}/out/github/repo?locale=en-EU" >/dev/null
curl -fsS "${BASE_URL}/out/download/fluxheim-1.8.0-full-x86_64-linux.tar.gz?locale=en-EU" >/dev/null

sleep 12

metrics="$(curl -fsS "${PROM_URL}/api/v1/label/__name__/values")"
for metric in \
  fluxheim_website_requests_total \
  fluxheim_website_page_views_total \
  fluxheim_website_outbound_clicks_total \
  fluxheim_website_download_clicks_total \
  fluxheim_website_page_visible_seconds_bucket
do
  if ! grep -q "${metric}" <<<"${metrics}"; then
    echo "observability smoke missing metric ${metric}" >&2
    exit 1
  fi
done

curl -fsS "${GRAFANA_URL}/api/search?query=Fluxheim%20Website" \
  -u "admin:${grafana_password}" | grep -q "Fluxheim Website"

echo "observability stack smoke ok"
