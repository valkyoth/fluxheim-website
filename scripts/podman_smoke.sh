#!/usr/bin/env sh
set -eu

image="fluxheim-website:1.6.30"
name="fluxheim-website-smoke"
port="${FLUXHEIM_WEBSITE_PORT:-8080}"
base="http://127.0.0.1:${port}"

podman rm -f "${name}" >/dev/null 2>&1 || true
podman build -f container/Dockerfile -t "${image}" .
podman run -d --rm --name "${name}" -p "${port}:8080" "${image}" >/dev/null
trap 'podman rm -f "${name}" >/dev/null 2>&1 || true' EXIT

for _ in 1 2 3 4 5 6 7 8 9 10; do
  if curl -fsS "${base}/healthz" >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

curl -fsS "${base}/healthz" | grep -q "ok"
curl -fsS "${base}/" | grep -q "Memory-safe edge server"
curl -fsS "${base}/en-gb/" | grep -q 'html lang="en-GB"'
curl -fsS "${base}/en-us/" | grep -q 'html lang="en-US"'
curl -fsS "${base}/de/" | grep -q "Herunterladen v1.6.30"
curl -fsS "${base}/fr/" | grep -q "Télécharger v1.6.30"
curl -fsS "${base}/no/" | grep -q "Last ned v1.6.30"
curl -fsS "${base}/nl/" | grep -q "Download versie 1.6.30"
curl -fsS "${base}/fi/" | grep -q "Lataa v1.6.30"
curl -fsS "${base}/is/" | grep -q "Sækja v1.6.30"
curl -fsS "${base}/da/" | grep -q "Hent v1.6.30"
curl -fsS "${base}/es/" | grep -q "Descargar v1.6.30"
curl -fsS "${base}/pt/" | grep -q "Transferir v1.6.30"
curl -fsS "${base}/et/" | grep -q "Laadi alla v1.6.30"
curl -fsS "${base}/lv/" | grep -q "Lejupielādēt v1.6.30"
curl -fsS "${base}/de/download" | grep -q "Systemd-Dienst"

curl -fsS "${base}/de/docs" | grep -q "Anleitungen"
curl -fsS "${base}/de/docs/static-sites" | grep -q "Statische Sites"
curl -fsS "${base}/de/docs/reverse-proxy" | grep -q "Produktionshinweise"
curl -fsS "${base}/de/docs/php-fpm" | grep -q "Externer PHP-FPM-Pool"
curl -fsS "${base}/de/docs/wordpress" | grep -q "Häufige Prüfungen"
curl -fsS "${base}/de/docs/features" | grep -q "Häufige Builds"
curl -fsS "${base}/de/docs/reference" | grep -q "Wo die Detaildokumentation liegt"
curl -fsS "${base}/fr/docs/static-sites" | grep -q "Sites statiques"
curl -fsS "${base}/fr/docs/features" | grep -q "Builds courants"
curl -fsS "${base}/sv/docs/features" | grep -q "Vanliga byggen"
curl -fsS "${base}/no/docs/features" | grep -q "Vanlige bygg"
curl -fsS "${base}/nl/docs/features" | grep -q "Veelgebruikte builds"
curl -fsS "${base}/fi/docs/features" | grep -q "Yleiset buildit"
curl -fsS "${base}/is/docs/features" | grep -q "Algeng build"
curl -fsS "${base}/da/docs/features" | grep -q "Almindelige builds"
curl -fsS "${base}/es/docs/features" | grep -q "Builds comunes"
curl -fsS "${base}/pt/docs/features" | grep -q "Builds comuns"
curl -fsS "${base}/et/docs/features" | grep -q "Tavalised buildid"
curl -fsS "${base}/lv/docs/features" | grep -q "Biežākie buildi"

curl -fsS "${base}/de/docs/source/systemd.md" | grep -q "# systemd Deployment"
curl -fsS "${base}/de/conf/fluxheim.toml" | grep -q 'hosts = \["fluxheim.eu"\]'
