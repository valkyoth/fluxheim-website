use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router;
use tower::ServiceExt;

async fn request(path: &str) -> (StatusCode, http::HeaderMap, String) {
    let site = Arc::new(Site::load().expect("site content loads"));
    let app = build_router(site);
    let response = app
        .oneshot(
            Request::builder()
                .uri(path)
                .body(Body::empty())
                .expect("request builds"),
        )
        .await
        .expect("response");
    let status = response.status();
    let headers = response.headers().clone();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body bytes");

    (
        status,
        headers,
        String::from_utf8(body.to_vec()).expect("utf8 body"),
    )
}

async fn request_with_body(
    method: http::Method,
    path: &str,
    body: impl Into<Body>,
) -> (StatusCode, http::HeaderMap, String) {
    let site = Arc::new(Site::load().expect("site content loads"));
    let app = build_router(site);
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header(header::CONTENT_TYPE, "application/json")
                .body(body.into())
                .expect("request builds"),
        )
        .await
        .expect("response");
    let status = response.status();
    let headers = response.headers().clone();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body bytes");

    (
        status,
        headers,
        String::from_utf8(body.to_vec()).expect("utf8 body"),
    )
}

#[tokio::test]
async fn renders_default_english_home() {
    let (status, _headers, body) = request("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Memory-Safe"));
    assert!(body.contains("Edge Server"));
    assert!(body.contains("Download v1.7.2"));
    assert!(body.contains("English (EU)"));
    assert!(body.contains("English (UK)"));
    assert!(body.contains("English (US)"));
    assert!(body.contains("Deutsch (Schweiz)"));
    assert!(body.contains("Svenska"));
    assert!(body.contains("Norsk"));
    assert!(body.contains("Nederlands"));
    assert!(body.contains("Suomi"));
    assert!(body.contains("Íslenska"));
    assert!(body.contains("Dansk"));
    assert!(body.contains("Español"));
    assert!(body.contains("Português"));
    assert!(body.contains("Eesti"));
    assert!(body.contains("Latviešu"));
    assert!(body.contains("Ελληνικά"));
    assert!(body.contains("Italiano"));
    assert!(body.contains("Lietuvių"));
    assert!(body.contains("Hrvatski"));
    assert!(body.contains("Čeština"));
    assert!(body.contains("Bosanski"));
    assert!(body.contains("Български"));
    assert!(body.contains("Română"));
    assert!(body.contains("Polski"));
    assert!(body.contains("Русский"));
    assert!(body.contains("日本語"));
    assert!(body.contains("Rootless Containers"));
}

#[tokio::test]
async fn locale_prefixes_preserve_legacy_pages() {
    let (de_status, _headers, de_body) = request("/de/download").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Systemd-Dienst"));
    assert!(de_body.contains("Cache-Edge-Build"));
    assert!(de_body.contains("Herunterladen v1.7.2"));
    assert!(de_body.contains("v1.7.2"));
    assert!(de_body.contains("v1.6.0 – v1.6.37"));
    assert!(de_body.contains("Native-Runtime-Cutover- und Bereinigungslinie"));
    assert!(de_body.contains("Alle auf GitHub"));

    let (ch_status, _headers, ch_body) = request("/ch/download").await;
    assert_eq!(ch_status, StatusCode::OK);
    assert!(ch_body.contains(r#"<html lang="de-CH""#));
    assert!(ch_body.contains("Herunterladen v1.7.2"));
    assert!(ch_body.contains("🇨🇭"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/deployment").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(
        fr_body.contains("Systemd &amp; conteneurs") || fr_body.contains("Systemd & conteneurs")
    );
    assert!(fr_body.contains("Checklist de production"));

    let (sv_status, _headers, sv_body) = request("/sv/download").await;
    assert_eq!(sv_status, StatusCode::OK);
    assert!(sv_body.contains(r#"<html lang="sv-SE""#));
    assert!(sv_body.contains("Ladda ner v1.7.2"));
    assert!(sv_body.contains("🇸🇪"));

    let (nb_status, _headers, nb_body) = request("/no/download").await;
    assert_eq!(nb_status, StatusCode::OK);
    assert!(nb_body.contains(r#"<html lang="nb-NO""#));
    assert!(nb_body.contains("Last ned v1.7.2"));
    assert!(nb_body.contains("🇳🇴"));

    let (nl_status, _headers, nl_body) = request("/nl/download").await;
    assert_eq!(nl_status, StatusCode::OK);
    assert!(nl_body.contains(r#"<html lang="nl-NL""#));
    assert!(nl_body.contains("Download versie 1.7.2"));
    assert!(nl_body.contains("🇳🇱"));

    let (fi_status, _headers, fi_body) = request("/fi/download").await;
    assert_eq!(fi_status, StatusCode::OK);
    assert!(fi_body.contains(r#"<html lang="fi-FI""#));
    assert!(fi_body.contains("Lataa v1.7.2"));
    assert!(fi_body.contains("🇫🇮"));

    let (is_status, _headers, is_body) = request("/is/download").await;
    assert_eq!(is_status, StatusCode::OK);
    assert!(is_body.contains(r#"<html lang="is-IS""#));
    assert!(is_body.contains("Sækja v1.7.2"));
    assert!(is_body.contains("🇮🇸"));

    let (da_status, _headers, da_body) = request("/da/download").await;
    assert_eq!(da_status, StatusCode::OK);
    assert!(da_body.contains(r#"<html lang="da-DK""#));
    assert!(da_body.contains("Hent v1.7.2"));
    assert!(da_body.contains("🇩🇰"));

    let (es_status, _headers, es_body) = request("/es/download").await;
    assert_eq!(es_status, StatusCode::OK);
    assert!(es_body.contains(r#"<html lang="es-ES""#));
    assert!(es_body.contains("Descargar v1.7.2"));
    assert!(es_body.contains("🇪🇸"));

    let (pt_status, _headers, pt_body) = request("/pt/download").await;
    assert_eq!(pt_status, StatusCode::OK);
    assert!(pt_body.contains(r#"<html lang="pt-PT""#));
    assert!(pt_body.contains("Transferir v1.7.2"));
    assert!(pt_body.contains("🇵🇹"));

    let (et_status, _headers, et_body) = request("/et/download").await;
    assert_eq!(et_status, StatusCode::OK);
    assert!(et_body.contains(r#"<html lang="et-EE""#));
    assert!(et_body.contains("Laadi alla v1.7.2"));
    assert!(et_body.contains("🇪🇪"));

    let (lv_status, _headers, lv_body) = request("/lv/download").await;
    assert_eq!(lv_status, StatusCode::OK);
    assert!(lv_body.contains(r#"<html lang="lv-LV""#));
    assert!(lv_body.contains("Lejupielādēt v1.7.2"));
    assert!(lv_body.contains("🇱🇻"));

    let (el_status, _headers, el_body) = request("/el/download").await;
    assert_eq!(el_status, StatusCode::OK);
    assert!(el_body.contains(r#"<html lang="el-GR""#));
    assert!(el_body.contains("Λήψη v1.7.2"));
    assert!(el_body.contains("🇬🇷"));

    let (it_status, _headers, it_body) = request("/it/download").await;
    assert_eq!(it_status, StatusCode::OK);
    assert!(it_body.contains(r#"<html lang="it-IT""#));
    assert!(it_body.contains("Scarica v1.7.2"));
    assert!(it_body.contains("🇮🇹"));

    let (lt_status, _headers, lt_body) = request("/lt/download").await;
    assert_eq!(lt_status, StatusCode::OK);
    assert!(lt_body.contains(r#"<html lang="lt-LT""#));
    assert!(lt_body.contains("Atsisiųsti v1.7.2"));
    assert!(lt_body.contains("🇱🇹"));

    let (hr_status, _headers, hr_body) = request("/hr/download").await;
    assert_eq!(hr_status, StatusCode::OK);
    assert!(hr_body.contains(r#"<html lang="hr-HR""#));
    assert!(hr_body.contains("Preuzmi v1.7.2"));
    assert!(hr_body.contains("🇭🇷"));

    let (cs_status, _headers, cs_body) = request("/cs/download").await;
    assert_eq!(cs_status, StatusCode::OK);
    assert!(cs_body.contains(r#"<html lang="cs-CZ""#));
    assert!(cs_body.contains("Stáhnout v1.7.2"));
    assert!(cs_body.contains("🇨🇿"));

    let (bs_status, _headers, bs_body) = request("/bs/download").await;
    assert_eq!(bs_status, StatusCode::OK);
    assert!(bs_body.contains(r#"<html lang="bs-BA""#));
    assert!(bs_body.contains("Preuzmi v1.7.2"));
    assert!(bs_body.contains("🇧🇦"));

    let (bg_status, _headers, bg_body) = request("/bg/download").await;
    assert_eq!(bg_status, StatusCode::OK);
    assert!(bg_body.contains(r#"<html lang="bg-BG""#));
    assert!(bg_body.contains("Изтегли v1.7.2"));
    assert!(bg_body.contains("🇧🇬"));

    let (ro_status, _headers, ro_body) = request("/ro/download").await;
    assert_eq!(ro_status, StatusCode::OK);
    assert!(ro_body.contains(r#"<html lang="ro-RO""#));
    assert!(ro_body.contains("Descarcă v1.7.2"));
    assert!(ro_body.contains("🇷🇴"));

    let (pl_status, _headers, pl_body) = request("/pl/download").await;
    assert_eq!(pl_status, StatusCode::OK);
    assert!(pl_body.contains(r#"<html lang="pl-PL""#));
    assert!(pl_body.contains("Pobierz v1.7.2"));
    assert!(pl_body.contains("🇵🇱"));

    let (ru_status, _headers, ru_body) = request("/ru/download").await;
    assert_eq!(ru_status, StatusCode::OK);
    assert!(ru_body.contains(r#"<html lang="ru-RU""#));
    assert!(ru_body.contains("Скачать v1.7.2"));
    assert!(ru_body.contains("🇷🇺"));

    let (ja_status, _headers, ja_body) = request("/ja/download").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains(r#"<html lang="ja-JP""#));
    assert!(ja_body.contains("ダウンロード v1.7.2"));
    assert!(ja_body.contains("🇯🇵"));

    let (ko_status, _headers, ko_body) = request("/ko/download").await;
    assert_eq!(ko_status, StatusCode::OK);
    assert!(ko_body.contains(r#"<html lang="ko-KR""#));
    assert!(ko_body.contains("v1.7.2 다운로드"));
    assert!(ko_body.contains("🇰🇷"));

    let (hu_status, _headers, hu_body) = request("/hu/download").await;
    assert_eq!(hu_status, StatusCode::OK);
    assert!(hu_body.contains(r#"<html lang="hu-HU""#));
    assert!(hu_body.contains("v1.7.2 letöltése"));
    assert!(hu_body.contains("🇭🇺"));
}

#[tokio::test]
async fn english_variant_prefixes_preserve_english_content() {
    let (gb_status, _headers, gb_body) = request("/en-gb/download").await;
    assert_eq!(gb_status, StatusCode::OK);
    assert!(gb_body.contains(r#"<html lang="en-GB""#));
    assert!(gb_body.contains("Download v1.7.2"));
    assert!(gb_body.contains("Pre-built Linux binaries"));
    assert!(gb_body.contains(r#"<a href="/en-gb/download" aria-current="true">"#));
    assert!(gb_body.contains("🇬🇧"));
    assert!(gb_body.contains("<span>English (UK)</span>"));

    let (us_status, _headers, us_body) = request("/en-us/docs").await;
    assert_eq!(us_status, StatusCode::OK);
    assert!(us_body.contains(r#"<html lang="en-US""#));
    assert!(us_body.contains("Fluxheim Docs"));
    assert!(us_body.contains(r#"<a href="/en-us/docs" aria-current="true">"#));
    assert!(us_body.contains("🇺🇸"));
    assert!(us_body.contains("<span>English (US)</span>"));
}

#[tokio::test]
async fn locale_prefixes_apply_runtime_translations() {
    let (de_status, _headers, de_body) = request("/de/").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains(r#"<html lang="de-DE""#));
    assert!(de_body.contains("Speichersicher"));
    assert!(de_body.contains("Herunterladen v1.7.2"));

    let (fr_status, _headers, fr_body) = request("/fr/").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains(r#"<html lang="fr-FR""#));
    assert!(fr_body.contains("Sûr pour la mémoire"));
    assert!(fr_body.contains("Télécharger v1.7.2"));

    let (sv_status, _headers, sv_body) = request("/sv/").await;
    assert_eq!(sv_status, StatusCode::OK);
    assert!(sv_body.contains(r#"<html lang="sv-SE""#));
    assert!(sv_body.contains("Minnessäker"));
    assert!(sv_body.contains("Ladda ner v1.7.2"));

    let (nb_status, _headers, nb_body) = request("/no/").await;
    assert_eq!(nb_status, StatusCode::OK);
    assert!(nb_body.contains(r#"<html lang="nb-NO""#));
    assert!(nb_body.contains("Minnesikker"));
    assert!(nb_body.contains("Last ned v1.7.2"));

    let (nl_status, _headers, nl_body) = request("/nl/").await;
    assert_eq!(nl_status, StatusCode::OK);
    assert!(nl_body.contains(r#"<html lang="nl-NL""#));
    assert!(nl_body.contains("Memory-safe"));
    assert!(nl_body.contains("Download versie 1.7.2"));

    let (fi_status, _headers, fi_body) = request("/fi/").await;
    assert_eq!(fi_status, StatusCode::OK);
    assert!(fi_body.contains(r#"<html lang="fi-FI""#));
    assert!(fi_body.contains("Muistiturvallinen"));
    assert!(fi_body.contains("Lataa v1.7.2"));

    let (is_status, _headers, is_body) = request("/is/").await;
    assert_eq!(is_status, StatusCode::OK);
    assert!(is_body.contains(r#"<html lang="is-IS""#));
    assert!(is_body.contains("Minnisöruggur"));
    assert!(is_body.contains("Sækja v1.7.2"));

    let (da_status, _headers, da_body) = request("/da/").await;
    assert_eq!(da_status, StatusCode::OK);
    assert!(da_body.contains(r#"<html lang="da-DK""#));
    assert!(da_body.contains("Hukommelsessikker"));
    assert!(da_body.contains("Hent v1.7.2"));

    let (es_status, _headers, es_body) = request("/es/").await;
    assert_eq!(es_status, StatusCode::OK);
    assert!(es_body.contains(r#"<html lang="es-ES""#));
    assert!(es_body.contains("Seguro para memoria"));
    assert!(es_body.contains("Descargar v1.7.2"));

    let (pt_status, _headers, pt_body) = request("/pt/").await;
    assert_eq!(pt_status, StatusCode::OK);
    assert!(pt_body.contains(r#"<html lang="pt-PT""#));
    assert!(pt_body.contains("Seguro para memória"));
    assert!(pt_body.contains("Transferir v1.7.2"));

    let (et_status, _headers, et_body) = request("/et/").await;
    assert_eq!(et_status, StatusCode::OK);
    assert!(et_body.contains(r#"<html lang="et-EE""#));
    assert!(et_body.contains("Mäluturvaline"));
    assert!(et_body.contains("Laadi alla v1.7.2"));

    let (lv_status, _headers, lv_body) = request("/lv/").await;
    assert_eq!(lv_status, StatusCode::OK);
    assert!(lv_body.contains(r#"<html lang="lv-LV""#));
    assert!(lv_body.contains("Atmiņdrošs"));
    assert!(lv_body.contains("Lejupielādēt v1.7.2"));

    let (el_status, _headers, el_body) = request("/el/").await;
    assert_eq!(el_status, StatusCode::OK);
    assert!(el_body.contains(r#"<html lang="el-GR""#));
    assert!(el_body.contains("Memory-safe"));
    assert!(el_body.contains("Λήψη v1.7.2"));

    let (it_status, _headers, it_body) = request("/it/").await;
    assert_eq!(it_status, StatusCode::OK);
    assert!(it_body.contains(r#"<html lang="it-IT""#));
    assert!(it_body.contains("Memory-safe"));
    assert!(it_body.contains("Scarica v1.7.2"));

    let (lt_status, _headers, lt_body) = request("/lt/").await;
    assert_eq!(lt_status, StatusCode::OK);
    assert!(lt_body.contains(r#"<html lang="lt-LT""#));
    assert!(lt_body.contains("Atminčiai saugus"));
    assert!(lt_body.contains("Atsisiųsti v1.7.2"));

    let (hr_status, _headers, hr_body) = request("/hr/").await;
    assert_eq!(hr_status, StatusCode::OK);
    assert!(hr_body.contains(r#"<html lang="hr-HR""#));
    assert!(hr_body.contains("Memorijski siguran"));
    assert!(hr_body.contains("Preuzmi v1.7.2"));

    let (cs_status, _headers, cs_body) = request("/cs/").await;
    assert_eq!(cs_status, StatusCode::OK);
    assert!(cs_body.contains(r#"<html lang="cs-CZ""#));
    assert!(cs_body.contains("Paměťově bezpečný"));
    assert!(cs_body.contains("Stáhnout v1.7.2"));

    let (bs_status, _headers, bs_body) = request("/bs/").await;
    assert_eq!(bs_status, StatusCode::OK);
    assert!(bs_body.contains(r#"<html lang="bs-BA""#));
    assert!(bs_body.contains("memorijski siguran") || bs_body.contains("Memorijski siguran"));
    assert!(bs_body.contains("Preuzmi v1.7.2"));

    let (ch_status, _headers, ch_body) = request("/ch/").await;
    assert_eq!(ch_status, StatusCode::OK);
    assert!(ch_body.contains(r#"<html lang="de-CH""#));
    assert!(ch_body.contains("Speichersicher"));
    assert!(ch_body.contains("Herunterladen v1.7.2"));

    let (bg_status, _headers, bg_body) = request("/bg/").await;
    assert_eq!(bg_status, StatusCode::OK);
    assert!(bg_body.contains(r#"<html lang="bg-BG""#));
    assert!(bg_body.contains("Memory-safe"));
    assert!(bg_body.contains("Изтегли v1.7.2"));

    let (ro_status, _headers, ro_body) = request("/ro/").await;
    assert_eq!(ro_status, StatusCode::OK);
    assert!(ro_body.contains(r#"<html lang="ro-RO""#));
    assert!(ro_body.contains("Memory-safe"));
    assert!(ro_body.contains("Descarcă v1.7.2"));

    let (pl_status, _headers, pl_body) = request("/pl/").await;
    assert_eq!(pl_status, StatusCode::OK);
    assert!(pl_body.contains(r#"<html lang="pl-PL""#));
    assert!(pl_body.contains("Memory-safe"));
    assert!(pl_body.contains("Pobierz v1.7.2"));

    let (ru_status, _headers, ru_body) = request("/ru/").await;
    assert_eq!(ru_status, StatusCode::OK);
    assert!(ru_body.contains(r#"<html lang="ru-RU""#));
    assert!(ru_body.contains("Memory-safe"));
    assert!(ru_body.contains("Скачать v1.7.2"));

    let (ja_status, _headers, ja_body) = request("/ja/").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains(r#"<html lang="ja-JP""#));
    assert!(ja_body.contains("メモリ安全"));
    assert!(ja_body.contains("ダウンロード v1.7.2"));

    let (ko_status, _headers, ko_body) = request("/ko/").await;
    assert_eq!(ko_status, StatusCode::OK);
    assert!(ko_body.contains(r#"<html lang="ko-KR""#));
    assert!(ko_body.contains("메모리 안전"));
    assert!(ko_body.contains("v1.7.2 다운로드"));

    let (hu_status, _headers, hu_body) = request("/hu/").await;
    assert_eq!(hu_status, StatusCode::OK);
    assert!(hu_body.contains(r#"<html lang="hu-HU""#));
    assert!(hu_body.contains("Memóriabiztos"));
    assert!(hu_body.contains("v1.7.2 letöltése"));
}

#[tokio::test]
async fn changelog_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/changelog").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Veröffentlicht am 19. Juni 2026"));
    assert!(de_body.contains("Auf GitHub ansehen"));

    let (fr_status, _headers, fr_body) = request("/fr/changelog").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Publié le 19 juin 2026"));
    assert!(fr_body.contains("Voir sur GitHub"));
}

#[tokio::test]
async fn docs_index_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Anleitungen"));
    assert!(de_body.contains("Statische Sites"));

    let (fr_status, _headers, fr_body) = request("/fr/docs").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Sites statiques"));
    assert!(fr_body.contains("Bon premier parcours"));
}

#[tokio::test]
async fn getting_started_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/getting-started").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Anleitungen"));
    assert!(de_body.contains("Installation"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/getting-started").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Guides"));
    assert!(fr_body.contains("Installation"));
}

#[tokio::test]
async fn configuration_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/configuration").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Konfiguration"));
    assert!(de_body.contains("Sichere Gewohnheiten"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/configuration").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Configuration"));
    assert!(fr_body.contains("Bonnes habitudes sûres"));
}

#[tokio::test]
async fn features_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/features").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Häufige Builds"));
    assert!(de_body.contains("Zukünftige Module"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/features").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Builds courants"));
    assert!(fr_body.contains("Modules futurs"));
}

#[tokio::test]
async fn deployment_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/deployment").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Rootless Podman"));
    assert!(de_body.contains("Produktions-Checkliste"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/deployment").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Podman rootless"));
    assert!(fr_body.contains("Checklist de production"));
}

#[tokio::test]
async fn tls_acme_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/tls-acme").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Verwaltete Zertifikate"));
    assert!(de_body.contains("Detailreferenz"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/tls-acme").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Certificats gérés"));
    assert!(fr_body.contains("Référence détaillée"));
}

#[tokio::test]
async fn cache_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/cache").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Anleitungen"));
    assert!(de_body.contains("Detailreferenz"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/cache").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Guides"));
    assert!(fr_body.contains("Référence détaillée"));
}

#[tokio::test]
async fn observability_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/observability").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Anleitungen"));
    assert!(de_body.contains("Detailreferenz"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/observability").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Guides"));
    assert!(fr_body.contains("Référence détaillée"));
}

#[tokio::test]
async fn advanced_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/advanced").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Zukünftige Module"));
    assert!(de_body.contains("WASM-Erweiterungen"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/advanced").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Modules futurs"));
    assert!(fr_body.contains("Extensions WASM"));
}

#[tokio::test]
async fn reference_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/reference").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Vollständige Referenz"));
    assert!(de_body.contains("Wo die Detaildokumentation liegt"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/reference").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Référence complète"));
    assert!(fr_body.contains("Où se trouve la documentation détaillée"));
}

#[tokio::test]
async fn clean_directory_routes_use_legacy_index_pages() {
    let (status, _headers, body) = request("/docs").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Fluxheim Docs"));
    assert!(body.contains("getting-started.html") || body.contains("Get Fluxheim Running"));

    let (de_status, _headers, de_body) = request("/de/docs").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Dokumentation"));
    assert!(de_body.contains(r#"<a href="/docs">"#));
    assert!(de_body.contains(r#"<a href="/en-gb/docs">"#));
    assert!(de_body.contains(r#"<a href="/en-us/docs">"#));
    assert!(de_body.contains(r#"<a href="/de/docs" aria-current="true">"#));
    assert!(de_body.contains(r#"<a href="/ch/docs">"#));
    assert!(de_body.contains(r#"<a href="/no/docs">"#));
    assert!(de_body.contains(r#"<a href="/nl/docs">"#));
    assert!(de_body.contains(r#"<a href="/fi/docs">"#));
    assert!(de_body.contains(r#"<a href="/is/docs">"#));
    assert!(de_body.contains(r#"<a href="/da/docs">"#));
    assert!(de_body.contains(r#"<a href="/es/docs">"#));
    assert!(de_body.contains(r#"<a href="/pt/docs">"#));
    assert!(de_body.contains(r#"<a href="/et/docs">"#));
    assert!(de_body.contains(r#"<a href="/lv/docs">"#));
    assert!(de_body.contains(r#"<a href="/el/docs">"#));
    assert!(de_body.contains(r#"<a href="/it/docs">"#));
    assert!(de_body.contains(r#"<a href="/lt/docs">"#));
    assert!(de_body.contains(r#"<a href="/hr/docs">"#));
    assert!(de_body.contains(r#"<a href="/cs/docs">"#));
    assert!(de_body.contains(r#"<a href="/bs/docs">"#));
    assert!(de_body.contains(r#"<a href="/bg/docs">"#));
    assert!(de_body.contains(r#"<a href="/ro/docs">"#));
    assert!(de_body.contains(r#"<a href="/pl/docs">"#));
    assert!(de_body.contains(r#"<a href="/ru/docs">"#));
    assert!(de_body.contains(r#"<a href="/ja/docs">"#));
    assert!(de_body.contains(r#"<a href="/ko/docs">"#));
    assert!(de_body.contains(r#"<a href="/hu/docs">"#));
    assert!(de_body.contains("🇪🇺"));
    assert!(de_body.contains("🇩🇪"));
    assert!(de_body.contains("🇨🇭"));
    assert!(de_body.contains("🇫🇷"));
    assert!(de_body.contains("🇳🇴"));
    assert!(de_body.contains("🇳🇱"));
    assert!(de_body.contains("🇫🇮"));
    assert!(de_body.contains("🇮🇸"));
    assert!(de_body.contains("🇩🇰"));
    assert!(de_body.contains("🇪🇸"));
    assert!(de_body.contains("🇵🇹"));
    assert!(de_body.contains("🇪🇪"));
    assert!(de_body.contains("🇱🇻"));
    assert!(de_body.contains("🇬🇷"));
    assert!(de_body.contains("🇮🇹"));
    assert!(de_body.contains("🇱🇹"));
    assert!(de_body.contains("🇭🇷"));
    assert!(de_body.contains("🇨🇿"));
    assert!(de_body.contains("🇧🇦"));
    assert!(de_body.contains("🇧🇬"));
    assert!(de_body.contains("🇷🇴"));
    assert!(de_body.contains("🇵🇱"));
    assert!(de_body.contains("🇷🇺"));
    assert!(de_body.contains("🇯🇵"));
    assert!(de_body.contains("🇰🇷"));
    assert!(de_body.contains("🇭🇺"));
    assert!(de_body.contains("<span>English (EU)</span>"));
    assert!(de_body.contains("<span>Deutsch</span>"));
    assert!(de_body.contains("<span>Deutsch (Schweiz)</span>"));
    assert!(de_body.contains("<span>Français</span>"));
    assert!(de_body.contains("<span>Norsk</span>"));
    assert!(de_body.contains("<span>Nederlands</span>"));
    assert!(de_body.contains("<span>Suomi</span>"));
    assert!(de_body.contains("<span>Íslenska</span>"));
    assert!(de_body.contains("<span>Dansk</span>"));
    assert!(de_body.contains("<span>Español</span>"));
    assert!(de_body.contains("<span>Português</span>"));
    assert!(de_body.contains("<span>Eesti</span>"));
    assert!(de_body.contains("<span>Latviešu</span>"));
    assert!(de_body.contains("<span>Ελληνικά</span>"));
    assert!(de_body.contains("<span>Italiano</span>"));
    assert!(de_body.contains("<span>Lietuvių</span>"));
    assert!(de_body.contains("<span>Hrvatski</span>"));
    assert!(de_body.contains("<span>Čeština</span>"));
    assert!(de_body.contains("<span>Bosanski</span>"));
    assert!(de_body.contains("<span>Български</span>"));
    assert!(de_body.contains("<span>Română</span>"));
    assert!(de_body.contains("<span>Polski</span>"));
    assert!(de_body.contains("<span>Русский</span>"));
    assert!(de_body.contains("<span>日本語</span>"));
    assert!(de_body.contains("<span>한국어</span>"));
    assert!(de_body.contains("<span>Magyar</span>"));
}

#[tokio::test]
async fn html_suffix_routes_still_work_with_locale_prefixes() {
    let (status, _headers, body) = request("/de/download.html").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Systemd-Dienst"));
    assert!(body.contains(r#"<a href="/download.html">"#));
    assert!(body.contains("<span>English (EU)</span>"));
}

#[tokio::test]
async fn source_markdown_artifacts_are_served() {
    let (status, headers, body) = request("/de/docs/source/systemd.md").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-type"], "text/markdown; charset=utf-8");
    assert!(body.contains("# systemd Deployment"));
}

#[tokio::test]
async fn release_note_artifacts_are_served() {
    let (status, headers, body) = request("/fr/docs/releases/RELEASE_NOTES_1.6.28.md").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-type"], "text/markdown; charset=utf-8");
    assert!(body.contains("# Fluxheim 1.6.28 Release Notes"));
}

#[tokio::test]
async fn source_tsv_artifacts_are_served() {
    let (status, headers, body) = request("/fr/docs/source/runtime-parity-fixtures.tsv").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        headers["content-type"],
        "text/tab-separated-values; charset=utf-8"
    );
    assert!(body.contains("scripts/smoke_static_local.sh"));
}

#[tokio::test]
async fn legacy_fluxheim_config_is_served() {
    let (status, headers, body) = request("/de/conf/fluxheim.toml").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-type"], "application/toml; charset=utf-8");
    assert!(body.contains("hosts = [\"fluxheim.eu\"]"));
}

#[tokio::test]
async fn language_selector_targets_same_page() {
    let (_status, _headers, body) = request("/de/download").await;
    assert!(body.contains(r#"<a href="/download""#));
    assert!(body.contains(r#"<a href="/en-gb/download""#));
    assert!(body.contains(r#"<a href="/en-us/download""#));
    assert!(body.contains(r#"<a href="/de/download" aria-current="true""#));
    assert!(body.contains(r#"<a href="/ch/download""#));
    assert!(body.contains(r#"<a href="/fr/download""#));
    assert!(body.contains(r#"<a href="/no/download""#));
    assert!(body.contains(r#"<a href="/nl/download""#));
    assert!(body.contains(r#"<a href="/fi/download""#));
    assert!(body.contains(r#"<a href="/is/download""#));
    assert!(body.contains(r#"<a href="/da/download""#));
    assert!(body.contains(r#"<a href="/es/download""#));
    assert!(body.contains(r#"<a href="/pt/download""#));
    assert!(body.contains(r#"<a href="/et/download""#));
    assert!(body.contains(r#"<a href="/lv/download""#));
    assert!(body.contains(r#"<a href="/el/download""#));
    assert!(body.contains(r#"<a href="/it/download""#));
    assert!(body.contains(r#"<a href="/lt/download""#));
    assert!(body.contains(r#"<a href="/hr/download""#));
    assert!(body.contains(r#"<a href="/cs/download""#));
    assert!(body.contains(r#"<a href="/bs/download""#));
    assert!(body.contains(r#"<a href="/bg/download""#));
    assert!(body.contains(r#"<a href="/ro/download""#));
    assert!(body.contains(r#"<a href="/pl/download""#));
    assert!(body.contains(r#"<a href="/ru/download""#));
    assert!(body.contains(r#"<a href="/ja/download""#));
    assert!(body.contains(r#"<summary aria-label="Sprache">"#));
    assert!(body.contains("<span>Deutsch</span>"));
    assert!(body.contains("<span>Deutsch (Schweiz)</span>"));
    assert!(body.contains("<span>Nederlands</span>"));
    assert!(body.contains("<span>Suomi</span>"));
    assert!(body.contains("<span>Íslenska</span>"));
    assert!(body.contains("<span>Dansk</span>"));
    assert!(body.contains("<span>Español</span>"));
    assert!(body.contains("<span>Português</span>"));
    assert!(body.contains("<span>Eesti</span>"));
    assert!(body.contains("<span>Latviešu</span>"));
    assert!(body.contains("<span>Ελληνικά</span>"));
    assert!(body.contains("<span>Italiano</span>"));
    assert!(body.contains("<span>Lietuvių</span>"));
    assert!(body.contains("<span>Hrvatski</span>"));
    assert!(body.contains("<span>Čeština</span>"));
    assert!(body.contains("<span>Bosanski</span>"));
    assert!(body.contains("<span>Български</span>"));
    assert!(body.contains("<span>Română</span>"));
    assert!(body.contains("<span>Polski</span>"));
    assert!(body.contains("<span>Русский</span>"));
    assert!(body.contains("<span>日本語</span>"));
}

#[tokio::test]
async fn github_outbound_redirects_only_known_targets() {
    let (status, headers, _body) = request("/out/github/repo?locale=de-DE").await;
    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        headers[header::LOCATION],
        "https://github.com/valkyoth/fluxheim"
    );

    let (unknown_status, _headers, body) = request("/out/github/raw-private-target").await;
    assert_eq!(unknown_status, StatusCode::NOT_FOUND);
    assert!(body.contains("Unknown outbound target"));
}

#[tokio::test]
async fn download_outbound_redirects_only_known_artifacts() {
    let artifact = "fluxheim-1.7.2-full-x86_64-linux.tar.gz";
    let (status, headers, _body) = request(&format!("/out/download/{artifact}?locale=en-EU")).await;
    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        headers[header::LOCATION],
        format!("https://github.com/valkyoth/fluxheim/releases/download/v1.7.2/{artifact}")
    );

    let (unknown_status, _headers, body) =
        request("/out/download/fluxheim-1.7.2-private-token.tar.gz").await;
    assert_eq!(unknown_status, StatusCode::NOT_FOUND);
    assert!(body.contains("Unknown download artifact"));
}

#[tokio::test]
async fn page_visible_accepts_bounded_events() {
    let valid = r#"{"locale":"fr-FR","route":"/docs/cache","section":"docs","seconds":42}"#;
    let (status, _headers, body) =
        request_with_body(http::Method::POST, "/telemetry/page-visible", valid).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body, "ok");

    let invalid = r#"{"locale":"fr-FR","route":"/private/raw","section":"docs","seconds":42}"#;
    let (invalid_status, _headers, invalid_body) =
        request_with_body(http::Method::POST, "/telemetry/page-visible", invalid).await;
    assert_eq!(invalid_status, StatusCode::BAD_REQUEST);
    assert!(invalid_body.contains("invalid page-visible event"));
}

#[tokio::test]
async fn telemetry_page_visible_rejects_large_bodies() {
    let large_route = "a".repeat(5000);
    let large = format!(
        r#"{{"locale":"fr-FR","route":"/docs/cache","section":"docs","seconds":42,"padding":"{large_route}"}}"#
    );
    let (status, _headers, body) =
        request_with_body(http::Method::POST, "/telemetry/page-visible", large).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert!(body.contains("length limit"));
}

#[tokio::test]
async fn telemetry_click_accepts_only_bounded_events() {
    let github = r#"{"kind":"github","locale":"en-EU","target":"repo"}"#;
    let (status, _headers, body) =
        request_with_body(http::Method::POST, "/telemetry/click", github).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body, "ok");

    let artifact = "fluxheim-1.7.2-full-x86_64-linux.tar.gz";
    let download = format!(r#"{{"kind":"download","locale":"de-DE","artifact":"{artifact}"}}"#);
    let (download_status, _headers, download_body) =
        request_with_body(http::Method::POST, "/telemetry/click", download).await;
    assert_eq!(download_status, StatusCode::ACCEPTED);
    assert_eq!(download_body, "ok");

    let invalid = r#"{"kind":"download","locale":"de-DE","artifact":"fluxheim-private.tar.gz"}"#;
    let (invalid_status, _headers, invalid_body) =
        request_with_body(http::Method::POST, "/telemetry/click", invalid).await;
    assert_eq!(invalid_status, StatusCode::BAD_REQUEST);
    assert!(invalid_body.contains("invalid click event"));
}

#[tokio::test]
async fn legal_pages_render_and_translate() {
    let (privacy_status, _headers, privacy_body) = request("/privacy").await;
    assert_eq!(privacy_status, StatusCode::OK);
    assert!(privacy_body.contains("Privacy Policy"));
    assert!(privacy_body.contains("raw IP addresses"));
    assert!(privacy_body.contains("Website translations are AI-assisted"));
    assert!(privacy_body.contains(
        r#"href="https://github.com/valkyoth/fluxheim-website/tree/main/config/i18n/keys""#
    ));
    assert!(privacy_body.contains(r#"<a href="/cookies">Cookies</a>"#));
    assert!(privacy_body.contains("navigator.sendBeacon"));

    let (de_status, _headers, de_body) = request("/de/privacy").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Datenschutzerklärung"));
    assert!(de_body.contains("Hinweis zu Übersetzungen"));
    assert!(de_body.contains("i18n-Keys der Fluxheim-Website"));
    assert!(de_body.contains("Was wir nicht erfassen"));
    assert!(de_body.contains(r#"<a href="/de/cookies">Cookies</a>"#));

    let (fr_status, _headers, fr_body) = request("/fr/gdpr").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Informations RGPD"));
    assert!(fr_body.contains("Avis sur les traductions"));
    assert!(fr_body.contains("clés i18n du site Fluxheim"));
    assert!(fr_body.contains("Minimisation des données"));
    assert!(fr_body.contains(r#"<a href="/fr/privacy">Politique de confidentialité</a>"#));
}

#[tokio::test]
async fn rendered_pages_keep_links_and_inject_click_beacon() {
    let (status, _headers, body) = request("/download").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#"href="https://github.com/valkyoth/fluxheim""#));
    assert!(
        body.contains(
            r#"href="https://github.com/valkyoth/fluxheim/releases/download/v1.7.2/fluxheim-1.7.2-full-x86_64-linux.tar.gz""#
        )
    );
    assert!(body.contains("navigator.sendBeacon"));
    assert!(body.contains("/telemetry/click"));
}

#[tokio::test]
async fn sets_security_headers() {
    let (_status, headers, _body) = request("/").await;
    assert_eq!(headers["x-content-type-options"], "nosniff");
    assert_eq!(headers["x-frame-options"], "DENY");
    assert_eq!(
        headers["strict-transport-security"],
        "max-age=31536000; includeSubDomains; preload"
    );
    let csp = headers["content-security-policy"].to_str().unwrap();
    assert!(csp.contains("script-src 'self' 'unsafe-inline' 'unsafe-eval'"));
    assert!(csp.contains("object-src 'none'"));
    assert!(csp.contains("base-uri 'self'"));
    assert!(csp.contains("frame-ancestors 'none'"));
}

#[tokio::test]
async fn returns_404_for_unknown_page() {
    let (status, _headers, body) = request("/de/no-such-page").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Page not found"));
}
