#!/usr/bin/env sh
set -eu

port="${FLUXHEIM_WEBSITE_PORT:-18080}"
base="http://127.0.0.1:${port}"

FLUXHEIM_WEBSITE_PORT="${port}" cargo +1.96.0 run --quiet &
pid="$!"
trap 'kill "$pid" >/dev/null 2>&1 || true' EXIT

for _ in 1 2 3 4 5 6 7 8 9 10; do
  if curl -fsS "${base}/healthz" >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

curl -fsS "${base}/" | grep -q "Memory-safe edge server"
curl -fsS "${base}/en-gb/" | grep -q 'html lang="en-GB"'
curl -fsS "${base}/en-us/" | grep -q 'html lang="en-US"'
curl -fsS "${base}/de/" | grep -q "Herunterladen v1.6.30"
curl -fsS "${base}/ch/" | grep -q "Herunterladen v1.6.30"
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
curl -fsS "${base}/el/" | grep -q "Λήψη v1.6.30"
curl -fsS "${base}/it/" | grep -q "Scarica v1.6.30"
curl -fsS "${base}/lt/" | grep -q "Atsisiųsti v1.6.30"
curl -fsS "${base}/hr/" | grep -q "Preuzmi v1.6.30"
curl -fsS "${base}/cs/" | grep -q "Stáhnout v1.6.30"
curl -fsS "${base}/bs/" | grep -q "Preuzmi v1.6.30"
curl -fsS "${base}/bg/" | grep -q "Изтегли v1.6.30"
curl -fsS "${base}/ro/" | grep -q "Descarcă v1.6.30"
curl -fsS "${base}/pl/" | grep -q "Pobierz v1.6.30"
curl -fsS "${base}/ru/" | grep -q "Скачать v1.6.30"
curl -fsS "${base}/ja/" | grep -q "ダウンロード v1.6.30"
curl -fsS "${base}/de/download" | grep -q "Systemd-Dienst"

curl -fsS "${base}/de/docs" | grep -q "Anleitungen"
curl -fsS "${base}/de/docs/static-sites" | grep -q "Statische Sites"
curl -fsS "${base}/de/docs/reverse-proxy" | grep -q "Produktionshinweise"
curl -fsS "${base}/de/docs/php-fpm" | grep -q "Externer PHP-FPM-Pool"
curl -fsS "${base}/de/docs/wordpress" | grep -q "Häufige Prüfungen"
curl -fsS "${base}/de/docs/features" | grep -q "Häufige Builds"
curl -fsS "${base}/ch/docs/features" | grep -q "Häufige Builds"
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
curl -fsS "${base}/el/docs/features" | grep -q "Συνηθισμένα builds"
curl -fsS "${base}/it/docs/features" | grep -q "Build comuni"
curl -fsS "${base}/lt/docs/features" | grep -q "Dažni build"
curl -fsS "${base}/hr/docs/features" | grep -q "Uobičajeni buildovi"
curl -fsS "${base}/cs/docs/features" | grep -q "Běžné buildy"
curl -fsS "${base}/bs/docs/features" | grep -q "Uobičajeni buildovi"
curl -fsS "${base}/bg/docs/features" | grep -q "Обичайни build-и"
curl -fsS "${base}/ro/docs/features" | grep -q "Build-uri uzuale"
curl -fsS "${base}/pl/docs/features" | grep -q "Typowe buildy"
curl -fsS "${base}/ru/docs/features" | grep -q "Типовые сборки"
curl -fsS "${base}/ja/docs/features" | grep -q "一般的なビルド"

curl -fsS "${base}/de/docs/source/systemd.md" | grep -q "# systemd Deployment"
curl -fsS "${base}/de/conf/fluxheim.toml" | grep -q 'hosts = \["fluxheim.eu"\]'
