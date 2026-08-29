use std::sync::{Arc, OnceLock};

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router;
use tower::ServiceExt;

fn app() -> axum::Router {
    static APP: OnceLock<axum::Router> = OnceLock::new();
    APP.get_or_init(|| {
        let site = Arc::new(Site::load().expect("site content loads"));
        build_router(site)
    })
    .clone()
}

pub(super) async fn request(path: &str) -> (StatusCode, http::HeaderMap, String) {
    let response = app()
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

pub(super) async fn request_with_body(
    method: http::Method,
    path: &str,
    body: impl Into<Body>,
) -> (StatusCode, http::HeaderMap, String) {
    let response = app()
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
    assert!(body.contains("Download v1.8.1"));
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
    assert!(de_body.contains("Herunterladen v1.8.1"));
    assert!(de_body.contains("Fluxheim-1.7-Reihe"));
    assert!(de_body.contains("Prozess-Upgrades ohne Ausfallzeit"));
    assert!(de_body.contains("v1.8.1"));
    assert!(de_body.contains("v1.6.0 – v1.6.37"));
    assert!(de_body.contains("Native-Runtime-Cutover- und Bereinigungslinie"));
    assert!(de_body.contains("Alle auf GitHub"));

    let (ch_status, _headers, ch_body) = request("/ch/download").await;
    assert_eq!(ch_status, StatusCode::OK);
    assert!(ch_body.contains(r#"<html lang="de-CH""#));
    assert!(ch_body.contains("Herunterladen v1.8.1"));
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
    assert!(sv_body.contains("Ladda ner v1.8.1"));
    assert!(sv_body.contains("🇸🇪"));

    let (nb_status, _headers, nb_body) = request("/no/download").await;
    assert_eq!(nb_status, StatusCode::OK);
    assert!(nb_body.contains(r#"<html lang="nb-NO""#));
    assert!(nb_body.contains("Last ned v1.8.1"));
    assert!(nb_body.contains("🇳🇴"));

    let (nl_status, _headers, nl_body) = request("/nl/download").await;
    assert_eq!(nl_status, StatusCode::OK);
    assert!(nl_body.contains(r#"<html lang="nl-NL""#));
    assert!(nl_body.contains("Download versie 1.8.1"));
    assert!(nl_body.contains("🇳🇱"));

    let (fi_status, _headers, fi_body) = request("/fi/download").await;
    assert_eq!(fi_status, StatusCode::OK);
    assert!(fi_body.contains(r#"<html lang="fi-FI""#));
    assert!(fi_body.contains("Lataa v1.8.1"));
    assert!(fi_body.contains("🇫🇮"));

    let (is_status, _headers, is_body) = request("/is/download").await;
    assert_eq!(is_status, StatusCode::OK);
    assert!(is_body.contains(r#"<html lang="is-IS""#));
    assert!(is_body.contains("Sækja v1.8.1"));
    assert!(is_body.contains("🇮🇸"));

    let (da_status, _headers, da_body) = request("/da/download").await;
    assert_eq!(da_status, StatusCode::OK);
    assert!(da_body.contains(r#"<html lang="da-DK""#));
    assert!(da_body.contains("Hent v1.8.1"));
    assert!(da_body.contains("🇩🇰"));

    let (es_status, _headers, es_body) = request("/es/download").await;
    assert_eq!(es_status, StatusCode::OK);
    assert!(es_body.contains(r#"<html lang="es-ES""#));
    assert!(es_body.contains("Descargar v1.8.1"));
    assert!(es_body.contains("🇪🇸"));

    let (pt_status, _headers, pt_body) = request("/pt/download").await;
    assert_eq!(pt_status, StatusCode::OK);
    assert!(pt_body.contains(r#"<html lang="pt-PT""#));
    assert!(pt_body.contains("Transferir v1.8.1"));
    assert!(pt_body.contains("🇵🇹"));

    let (et_status, _headers, et_body) = request("/et/download").await;
    assert_eq!(et_status, StatusCode::OK);
    assert!(et_body.contains(r#"<html lang="et-EE""#));
    assert!(et_body.contains("Laadi alla v1.8.1"));
    assert!(et_body.contains("🇪🇪"));

    let (lv_status, _headers, lv_body) = request("/lv/download").await;
    assert_eq!(lv_status, StatusCode::OK);
    assert!(lv_body.contains(r#"<html lang="lv-LV""#));
    assert!(lv_body.contains("Lejupielādēt v1.8.1"));
    assert!(lv_body.contains("🇱🇻"));

    let (el_status, _headers, el_body) = request("/el/download").await;
    assert_eq!(el_status, StatusCode::OK);
    assert!(el_body.contains(r#"<html lang="el-GR""#));
    assert!(el_body.contains("Λήψη v1.8.1"));
    assert!(el_body.contains("🇬🇷"));

    let (it_status, _headers, it_body) = request("/it/download").await;
    assert_eq!(it_status, StatusCode::OK);
    assert!(it_body.contains(r#"<html lang="it-IT""#));
    assert!(it_body.contains("Scarica v1.8.1"));
    assert!(it_body.contains("🇮🇹"));

    let (lt_status, _headers, lt_body) = request("/lt/download").await;
    assert_eq!(lt_status, StatusCode::OK);
    assert!(lt_body.contains(r#"<html lang="lt-LT""#));
    assert!(lt_body.contains("Atsisiųsti v1.8.1"));
    assert!(lt_body.contains("🇱🇹"));

    let (hr_status, _headers, hr_body) = request("/hr/download").await;
    assert_eq!(hr_status, StatusCode::OK);
    assert!(hr_body.contains(r#"<html lang="hr-HR""#));
    assert!(hr_body.contains("Preuzmi v1.8.1"));
    assert!(hr_body.contains("🇭🇷"));

    let (cs_status, _headers, cs_body) = request("/cs/download").await;
    assert_eq!(cs_status, StatusCode::OK);
    assert!(cs_body.contains(r#"<html lang="cs-CZ""#));
    assert!(cs_body.contains("Stáhnout v1.8.1"));
    assert!(cs_body.contains("🇨🇿"));

    let (bs_status, _headers, bs_body) = request("/bs/download").await;
    assert_eq!(bs_status, StatusCode::OK);
    assert!(bs_body.contains(r#"<html lang="bs-BA""#));
    assert!(bs_body.contains("Preuzmi v1.8.1"));
    assert!(bs_body.contains("🇧🇦"));

    let (bg_status, _headers, bg_body) = request("/bg/download").await;
    assert_eq!(bg_status, StatusCode::OK);
    assert!(bg_body.contains(r#"<html lang="bg-BG""#));
    assert!(bg_body.contains("Изтегли v1.8.1"));
    assert!(bg_body.contains("🇧🇬"));

    let (ro_status, _headers, ro_body) = request("/ro/download").await;
    assert_eq!(ro_status, StatusCode::OK);
    assert!(ro_body.contains(r#"<html lang="ro-RO""#));
    assert!(ro_body.contains("Descarcă v1.8.1"));
    assert!(ro_body.contains("🇷🇴"));

    let (pl_status, _headers, pl_body) = request("/pl/download").await;
    assert_eq!(pl_status, StatusCode::OK);
    assert!(pl_body.contains(r#"<html lang="pl-PL""#));
    assert!(pl_body.contains("Pobierz v1.8.1"));
    assert!(pl_body.contains("🇵🇱"));

    let (ru_status, _headers, ru_body) = request("/ru/download").await;
    assert_eq!(ru_status, StatusCode::OK);
    assert!(ru_body.contains(r#"<html lang="ru-RU""#));
    assert!(ru_body.contains("Скачать v1.8.1"));
    assert!(ru_body.contains("🇷🇺"));

    let (ja_status, _headers, ja_body) = request("/ja/download").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains(r#"<html lang="ja-JP""#));
    assert!(ja_body.contains("ダウンロード v1.8.1"));
    assert!(ja_body.contains("🇯🇵"));

    let (ko_status, _headers, ko_body) = request("/ko/download").await;
    assert_eq!(ko_status, StatusCode::OK);
    assert!(ko_body.contains(r#"<html lang="ko-KR""#));
    assert!(ko_body.contains("v1.8.1 다운로드"));
    assert!(ko_body.contains("🇰🇷"));

    let (hu_status, _headers, hu_body) = request("/hu/download").await;
    assert_eq!(hu_status, StatusCode::OK);
    assert!(hu_body.contains(r#"<html lang="hu-HU""#));
    assert!(hu_body.contains("v1.8.1 letöltése"));
    assert!(hu_body.contains("🇭🇺"));
}

#[tokio::test]
async fn english_variant_prefixes_preserve_english_content() {
    let (gb_status, _headers, gb_body) = request("/en-gb/download").await;
    assert_eq!(gb_status, StatusCode::OK);
    assert!(gb_body.contains(r#"<html lang="en-GB""#));
    assert!(gb_body.contains("Download v1.8.1"));
    assert!(gb_body.contains("Linux x86_64 · Linux aarch64 · macOS Apple Silicon"));
    assert!(gb_body.contains("2026-08-28"));
    assert!(gb_body.contains("stronger diagnostic and filesystem protections"));
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

    let (us_download_status, _headers, us_download_body) = request("/en-us/download").await;
    assert_eq!(us_download_status, StatusCode::OK);
    assert!(us_download_body.contains("stronger diagnostic and file system protections"));
    assert!(!us_download_body.contains("stronger diagnostic and filesystem protections"));
}

#[tokio::test]
async fn locale_prefixes_apply_runtime_translations() {
    let (de_status, _headers, de_body) = request("/de/").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains(r#"<html lang="de-DE""#));
    assert!(de_body.contains("Speichersicher"));
    assert!(de_body.contains("Herunterladen v1.8.1"));

    let (fr_status, _headers, fr_body) = request("/fr/").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains(r#"<html lang="fr-FR""#));
    assert!(fr_body.contains("Sûr pour la mémoire"));
    assert!(fr_body.contains("Télécharger v1.8.1"));

    let (sv_status, _headers, sv_body) = request("/sv/").await;
    assert_eq!(sv_status, StatusCode::OK);
    assert!(sv_body.contains(r#"<html lang="sv-SE""#));
    assert!(sv_body.contains("Minnessäker"));
    assert!(sv_body.contains("Ladda ner v1.8.1"));

    let (nb_status, _headers, nb_body) = request("/no/").await;
    assert_eq!(nb_status, StatusCode::OK);
    assert!(nb_body.contains(r#"<html lang="nb-NO""#));
    assert!(nb_body.contains("Minnesikker"));
    assert!(nb_body.contains("Last ned v1.8.1"));

    let (nl_status, _headers, nl_body) = request("/nl/").await;
    assert_eq!(nl_status, StatusCode::OK);
    assert!(nl_body.contains(r#"<html lang="nl-NL""#));
    assert!(nl_body.contains("Memory-safe"));
    assert!(nl_body.contains("Download versie 1.8.1"));

    let (fi_status, _headers, fi_body) = request("/fi/").await;
    assert_eq!(fi_status, StatusCode::OK);
    assert!(fi_body.contains(r#"<html lang="fi-FI""#));
    assert!(fi_body.contains("Muistiturvallinen"));
    assert!(fi_body.contains("Lataa v1.8.1"));

    let (is_status, _headers, is_body) = request("/is/").await;
    assert_eq!(is_status, StatusCode::OK);
    assert!(is_body.contains(r#"<html lang="is-IS""#));
    assert!(is_body.contains("Minnisöruggur"));
    assert!(is_body.contains("Sækja v1.8.1"));

    let (da_status, _headers, da_body) = request("/da/").await;
    assert_eq!(da_status, StatusCode::OK);
    assert!(da_body.contains(r#"<html lang="da-DK""#));
    assert!(da_body.contains("Hukommelsessikker"));
    assert!(da_body.contains("Hent v1.8.1"));

    let (es_status, _headers, es_body) = request("/es/").await;
    assert_eq!(es_status, StatusCode::OK);
    assert!(es_body.contains(r#"<html lang="es-ES""#));
    assert!(es_body.contains("Seguro para memoria"));
    assert!(es_body.contains("Descargar v1.8.1"));

    let (pt_status, _headers, pt_body) = request("/pt/").await;
    assert_eq!(pt_status, StatusCode::OK);
    assert!(pt_body.contains(r#"<html lang="pt-PT""#));
    assert!(pt_body.contains("Seguro para memória"));
    assert!(pt_body.contains("Transferir v1.8.1"));

    let (et_status, _headers, et_body) = request("/et/").await;
    assert_eq!(et_status, StatusCode::OK);
    assert!(et_body.contains(r#"<html lang="et-EE""#));
    assert!(et_body.contains("Mäluturvaline"));
    assert!(et_body.contains("Laadi alla v1.8.1"));

    let (lv_status, _headers, lv_body) = request("/lv/").await;
    assert_eq!(lv_status, StatusCode::OK);
    assert!(lv_body.contains(r#"<html lang="lv-LV""#));
    assert!(lv_body.contains("Atmiņdrošs"));
    assert!(lv_body.contains("Lejupielādēt v1.8.1"));

    let (el_status, _headers, el_body) = request("/el/").await;
    assert_eq!(el_status, StatusCode::OK);
    assert!(el_body.contains(r#"<html lang="el-GR""#));
    assert!(el_body.contains("Memory-safe"));
    assert!(el_body.contains("Λήψη v1.8.1"));

    let (it_status, _headers, it_body) = request("/it/").await;
    assert_eq!(it_status, StatusCode::OK);
    assert!(it_body.contains(r#"<html lang="it-IT""#));
    assert!(it_body.contains("Memory-safe"));
    assert!(it_body.contains("Scarica v1.8.1"));

    let (lt_status, _headers, lt_body) = request("/lt/").await;
    assert_eq!(lt_status, StatusCode::OK);
    assert!(lt_body.contains(r#"<html lang="lt-LT""#));
    assert!(lt_body.contains("Atminčiai saugus"));
    assert!(lt_body.contains("Atsisiųsti v1.8.1"));

    let (hr_status, _headers, hr_body) = request("/hr/").await;
    assert_eq!(hr_status, StatusCode::OK);
    assert!(hr_body.contains(r#"<html lang="hr-HR""#));
    assert!(hr_body.contains("Memorijski siguran"));
    assert!(hr_body.contains("Preuzmi v1.8.1"));

    let (cs_status, _headers, cs_body) = request("/cs/").await;
    assert_eq!(cs_status, StatusCode::OK);
    assert!(cs_body.contains(r#"<html lang="cs-CZ""#));
    assert!(cs_body.contains("Paměťově bezpečný"));
    assert!(cs_body.contains("Stáhnout v1.8.1"));

    let (bs_status, _headers, bs_body) = request("/bs/").await;
    assert_eq!(bs_status, StatusCode::OK);
    assert!(bs_body.contains(r#"<html lang="bs-BA""#));
    assert!(bs_body.contains("memorijski siguran") || bs_body.contains("Memorijski siguran"));
    assert!(bs_body.contains("Preuzmi v1.8.1"));

    let (ch_status, _headers, ch_body) = request("/ch/").await;
    assert_eq!(ch_status, StatusCode::OK);
    assert!(ch_body.contains(r#"<html lang="de-CH""#));
    assert!(ch_body.contains("Speichersicher"));
    assert!(ch_body.contains("Herunterladen v1.8.1"));

    let (bg_status, _headers, bg_body) = request("/bg/").await;
    assert_eq!(bg_status, StatusCode::OK);
    assert!(bg_body.contains(r#"<html lang="bg-BG""#));
    assert!(bg_body.contains("Memory-safe"));
    assert!(bg_body.contains("Изтегли v1.8.1"));

    let (ro_status, _headers, ro_body) = request("/ro/").await;
    assert_eq!(ro_status, StatusCode::OK);
    assert!(ro_body.contains(r#"<html lang="ro-RO""#));
    assert!(ro_body.contains("Memory-safe"));
    assert!(ro_body.contains("Descarcă v1.8.1"));

    let (pl_status, _headers, pl_body) = request("/pl/").await;
    assert_eq!(pl_status, StatusCode::OK);
    assert!(pl_body.contains(r#"<html lang="pl-PL""#));
    assert!(pl_body.contains("Memory-safe"));
    assert!(pl_body.contains("Pobierz v1.8.1"));

    let (ru_status, _headers, ru_body) = request("/ru/").await;
    assert_eq!(ru_status, StatusCode::OK);
    assert!(ru_body.contains(r#"<html lang="ru-RU""#));
    assert!(ru_body.contains("Memory-safe"));
    assert!(ru_body.contains("Скачать v1.8.1"));

    let (ja_status, _headers, ja_body) = request("/ja/").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains(r#"<html lang="ja-JP""#));
    assert!(ja_body.contains("メモリ安全"));
    assert!(ja_body.contains("ダウンロード v1.8.1"));

    let (ko_status, _headers, ko_body) = request("/ko/").await;
    assert_eq!(ko_status, StatusCode::OK);
    assert!(ko_body.contains(r#"<html lang="ko-KR""#));
    assert!(ko_body.contains("메모리 안전"));
    assert!(ko_body.contains("v1.8.1 다운로드"));

    let (hu_status, _headers, hu_body) = request("/hu/").await;
    assert_eq!(hu_status, StatusCode::OK);
    assert!(hu_body.contains(r#"<html lang="hu-HU""#));
    assert!(hu_body.contains("Memóriabiztos"));
    assert!(hu_body.contains("v1.8.1 letöltése"));
}
