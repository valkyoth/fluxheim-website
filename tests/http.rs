use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
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

#[tokio::test]
async fn renders_default_english_home() {
    let (status, _headers, body) = request("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Memory-Safe"));
    assert!(body.contains("Edge Server"));
    assert!(body.contains("Download v1.6.28"));
    assert!(body.contains("English (EU)"));
    assert!(body.contains("English (UK)"));
    assert!(body.contains("English (US)"));
    assert!(body.contains("Rootless Containers"));
}

#[tokio::test]
async fn locale_prefixes_preserve_legacy_pages() {
    let (de_status, _headers, de_body) = request("/de/download").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Systemd-Dienst"));
    assert!(de_body.contains("Cache-Edge-Build"));
    assert!(de_body.contains("Herunterladen v1.6.28"));
    assert!(de_body.contains("Native HTTP/1.1-Upstream-Pooling-Version"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/deployment").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(
        fr_body.contains("Systemd &amp; conteneurs") || fr_body.contains("Systemd & conteneurs")
    );
    assert!(fr_body.contains("Podman Quadlet"));
}

#[tokio::test]
async fn english_variant_prefixes_preserve_english_content() {
    let (gb_status, _headers, gb_body) = request("/en-gb/download").await;
    assert_eq!(gb_status, StatusCode::OK);
    assert!(gb_body.contains(r#"<html lang="en-GB""#));
    assert!(gb_body.contains("Download v1.6.28"));
    assert!(gb_body.contains("Pre-built Linux binaries"));
    assert!(gb_body.contains(r#"<a href="/en-gb/download" aria-current="true">English (UK)</a>"#));

    let (us_status, _headers, us_body) = request("/en-us/docs").await;
    assert_eq!(us_status, StatusCode::OK);
    assert!(us_body.contains(r#"<html lang="en-US""#));
    assert!(us_body.contains("Documentation"));
    assert!(us_body.contains(r#"<a href="/en-us/docs" aria-current="true">English (US)</a>"#));
}

#[tokio::test]
async fn locale_prefixes_apply_runtime_translations() {
    let (de_status, _headers, de_body) = request("/de/").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains(r#"<html lang="de-DE""#));
    assert!(de_body.contains("Speichersicher"));
    assert!(de_body.contains("Herunterladen v1.6.28"));

    let (fr_status, _headers, fr_body) = request("/fr/").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains(r#"<html lang="fr-FR""#));
    assert!(fr_body.contains("Sûr pour la mémoire"));
    assert!(fr_body.contains("Télécharger v1.6.28"));
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
    assert!(de_body.contains("Statisches Website-Hosting mit MIME-Erkennung"));

    let (fr_status, _headers, fr_body) = request("/fr/docs").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Hébergement de site statique avec détection MIME"));
}

#[tokio::test]
async fn getting_started_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/getting-started").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Voraussetzungen"));
    assert!(de_body.contains("Deine erste Konfiguration"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/getting-started").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Prérequis"));
    assert!(fr_body.contains("Votre première configuration"));
}

#[tokio::test]
async fn configuration_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/configuration").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Fluxheim wird über eine einzelne TOML-Datei konfiguriert"));
    assert!(de_body.contains("Schlüssel"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/configuration").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Fluxheim se configure via un seul fichier TOML"));
    assert!(fr_body.contains("Clé"));
}

#[tokio::test]
async fn features_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/features").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Alle Cargo-Features"));
    assert!(de_body.contains("Build-Profil-Aliasse"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/features").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Toutes les fonctionnalités Cargo"));
    assert!(fr_body.contains("Alias de profil de build"));
}

#[tokio::test]
async fn deployment_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/deployment").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("rootless Podman-Container"));
    assert!(de_body.contains("Checkliste für Produktionsreife"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/deployment").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Conteneurs Podman rootless"));
    assert!(fr_body.contains("Checklist de préparation production"));
}

#[tokio::test]
async fn tls_acme_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/tls-acme").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("TLS-Backends"));
    assert!(de_body.contains("ACME-Challenge-Methoden"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/tls-acme").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Backends TLS"));
    assert!(fr_body.contains("Méthodes de challenge ACME"));
}

#[tokio::test]
async fn cache_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/cache").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Cache-Backends"));
    assert!(de_body.contains("Cache-Operationen"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/cache").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Backends de cache"));
    assert!(fr_body.contains("Opérations de cache"));
}

#[tokio::test]
async fn observability_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/observability").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Prometheus-Metriken"));
    assert!(de_body.contains("Was getraced wird"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/observability").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Métriques Prometheus"));
    assert!(fr_body.contains("Ce qui est tracé"));
}

#[tokio::test]
async fn advanced_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/advanced").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Fortgeschrittene Funktionen"));
    assert!(de_body.contains("Zero-Retention-Privacy-Modus"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/advanced").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Fonctionnalités avancées"));
    assert!(fr_body.contains("Mode privacy sans rétention"));
}

#[tokio::test]
async fn reference_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/reference").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Betrieb und Deployment"));
    assert!(de_body.contains("Repository- und Release-Vorlagen"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/reference").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Opérations et déploiement"));
    assert!(fr_body.contains("Modèles repository et release"));
}

#[tokio::test]
async fn clean_directory_routes_use_legacy_index_pages() {
    let (status, _headers, body) = request("/docs").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Documentation"));
    assert!(body.contains("docs/index.html") || body.contains("Getting Started"));

    let (de_status, _headers, de_body) = request("/de/docs").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Dokumentation"));
    assert!(de_body.contains(r#"<a href="/docs">English (EU)</a>"#));
    assert!(de_body.contains(r#"<a href="/en-gb/docs">English (UK)</a>"#));
    assert!(de_body.contains(r#"<a href="/en-us/docs">English (US)</a>"#));
    assert!(de_body.contains(r#"<a href="/de/docs" aria-current="true">Deutsch</a>"#));
}

#[tokio::test]
async fn html_suffix_routes_still_work_with_locale_prefixes() {
    let (status, _headers, body) = request("/de/download.html").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Systemd-Dienst"));
    assert!(body.contains(r#"<a href="/download.html">English (EU)</a>"#));
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
    assert!(body.contains(r#"<a href="/fr/download""#));
    assert!(body.contains("<summary>Sprache</summary>"));
}

#[tokio::test]
async fn sets_security_headers() {
    let (_status, headers, _body) = request("/").await;
    assert_eq!(headers["x-content-type-options"], "nosniff");
    assert_eq!(headers["x-frame-options"], "DENY");
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
