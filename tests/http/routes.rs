use axum::http::{StatusCode, header};

use super::locales::{request, request_with_body};

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
async fn wasm_guide_and_release_profile_are_public() {
    let (status, _headers, body) = request("/docs/wasm").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("WASM extensions"));
    assert!(body.contains("profile-wasm"));
    assert!(body.contains("v1.8.0-wasm"));
    assert!(body.contains("expected SHA-256") || body.contains("sha256"));

    let (de_status, _headers, de_body) = request("/de/docs/wasm").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains(r#"href="/de/docs/wasm" aria-current="true""#));
    assert!(!de_body.contains("Dedicated Wasm build based on the full production profile"));

    let (download_status, _headers, download_body) = request("/download").await;
    assert_eq!(download_status, StatusCode::OK);
    assert!(download_body.contains("fluxheim-1.8.0-wasm-x86_64-linux.tar.gz"));
    assert!(download_body.contains("ghcr.io/valkyoth/fluxheim:v1.8.0-wasm"));
    assert!(download_body.contains("quay.io/valkyoth/fluxheim:v1.8.0-wasm"));
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
    let (status, headers, body) = request("/fr/docs/releases/RELEASE_NOTES_1.8.0.md").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-type"], "text/markdown; charset=utf-8");
    assert!(body.contains("# Fluxheim 1.8.0 Release Notes"));
    assert!(body.contains("Wasm Distribution Profile"));
    assert!(body.contains("Portable Archives"));
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
    let artifact = "fluxheim-1.8.0-full-x86_64-linux.tar.gz";
    let (status, headers, _body) = request(&format!("/out/download/{artifact}?locale=en-EU")).await;
    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        headers[header::LOCATION],
        format!("https://github.com/valkyoth/fluxheim/releases/download/v1.8.0/{artifact}")
    );

    let wasm_artifact = "fluxheim-1.8.0-wasm-x86_64-linux.tar.gz";
    let (wasm_status, wasm_headers, _body) =
        request(&format!("/out/download/{wasm_artifact}?locale=en-EU")).await;
    assert_eq!(wasm_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        wasm_headers[header::LOCATION],
        format!("https://github.com/valkyoth/fluxheim/releases/download/v1.8.0/{wasm_artifact}")
    );

    let wasm_macos_artifact = "fluxheim-1.8.0-wasm-aarch64-macos.tar.gz";
    let (wasm_macos_status, wasm_macos_headers, _body) =
        request(&format!("/out/download/{wasm_macos_artifact}?locale=en-EU")).await;
    assert_eq!(wasm_macos_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        wasm_macos_headers[header::LOCATION],
        format!(
            "https://github.com/valkyoth/fluxheim/releases/download/v1.8.0/{wasm_macos_artifact}"
        )
    );

    let (unknown_status, _headers, body) =
        request("/out/download/fluxheim-1.8.0-private-token.tar.gz").await;
    assert_eq!(unknown_status, StatusCode::NOT_FOUND);
    assert!(body.contains("Unknown download artifact"));

    let unlisted = "fluxheim-999999.1.1-full-x86_64-linux.tar.gz";
    let (unlisted_status, _headers, body) =
        request(&format!("/out/download/{unlisted}?locale=en-EU")).await;
    assert_eq!(unlisted_status, StatusCode::NOT_FOUND);
    assert!(body.contains("Unknown download artifact"));

    let removed_historical = "fluxheim-1.6.37-cache-x86_64-linux.tar.gz";
    let (historical_status, _headers, historical_body) =
        request(&format!("/out/download/{removed_historical}?locale=de-DE")).await;
    assert_eq!(historical_status, StatusCode::NOT_FOUND);
    assert!(historical_body.contains("Unknown download artifact"));
}

#[tokio::test]
async fn changelog_leaves_artifact_links_to_the_download_page() {
    for route in ["/changelog", "/de/changelog"] {
        let (status, _headers, body) = request(route).await;
        assert_eq!(status, StatusCode::OK, "{route}");
        assert!(body.contains("v1.8.0"), "{route}");
        assert!(body.contains("releases/tag/v1.8.0"), "{route}");
        assert!(!body.contains("/out/download/"), "{route}");
        assert!(!body.contains("/releases/download/"), "{route}");
    }
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
async fn direct_client_click_telemetry_is_not_exposed() {
    let payload = r#"{"kind":"github","locale":"en-EU","target":"repo"}"#;
    let (status, _headers, body) =
        request_with_body(http::Method::POST, "/telemetry/click", payload).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Page not found"));
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
async fn rendered_pages_use_validated_click_redirects() {
    let (status, _headers, body) = request("/download").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#"href="/out/github/repo?locale=en-EU""#));
    assert!(
        body.contains(
            r#"href="/out/download/fluxheim-1.8.0-full-x86_64-linux.tar.gz?locale=en-EU""#
        )
    );
    assert!(body.contains("navigator.sendBeacon"));
    assert!(!body.contains("/telemetry/click"));
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
    assert!(csp.contains("connect-src 'self'"));
    assert!(csp.contains("object-src 'none'"));
    assert!(csp.contains("base-uri 'none'"));
    assert!(csp.contains("form-action 'self'"));
    assert!(csp.contains("frame-ancestors 'none'"));
}

#[tokio::test]
async fn returns_404_for_unknown_page() {
    let (status, _headers, body) = request("/de/no-such-page").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Page not found"));
}
