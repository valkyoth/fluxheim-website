use axum::http::StatusCode;

use super::locales::request;

#[tokio::test]
async fn changelog_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/changelog").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Veröffentlicht am 10. Juli 2026"));
    assert!(de_body.contains("optionale Proxy-Wasm-ABI-Vorschau"));
    assert!(de_body.contains("Veröffentlicht am 19. Juni 2026"));
    assert!(de_body.contains("Auf GitHub ansehen"));

    let (fr_status, _headers, fr_body) = request("/fr/changelog").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Publié le 10 juillet 2026"));
    assert!(fr_body.contains("aperçu facultatif de l’ABI Proxy-Wasm"));
    assert!(fr_body.contains("Publié le 19 juin 2026"));
    assert!(fr_body.contains("Voir sur GitHub"));

    let (ja_status, _headers, ja_body) = request("/ja/changelog").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains("2026年7月10日リリース"));
    assert!(ja_body.contains("Proxy-Wasm ABI preview"));
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
    assert!(de_body.contains("mandantenfähigen Bereitstellungen"));
    assert!(de_body.contains("unbekannter Host <code>421</code>"));
    assert!(de_body.contains("HTTP-Antwortsicherheit"));
    assert!(de_body.contains("<code>baseline</code>"));
    assert!(de_body.contains("Standardbasierte Antwortmetadaten"));
    assert!(de_body.contains("Live-Schnappschüsse und Rollback"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/configuration").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Configuration"));
    assert!(fr_body.contains("Bonnes habitudes sûres"));
    assert!(fr_body.contains("Sécurité des réponses HTTP"));
}

#[tokio::test]
async fn reverse_proxy_uses_localized_cors_guidance() {
    let (de_status, _headers, de_body) = request("/de/docs/reverse-proxy").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("CORS für Browser-Apps"));
    assert!(de_body.contains("<code>403</code>"));
    assert!(!de_body.contains("Fluxheim answers valid preflight requests locally"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/reverse-proxy").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("CORS pour les applications de navigateur"));
    assert!(fr_body.contains("<code>403</code>"));
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
    assert!(de_body.contains("Upgrades ohne Ausfallzeit"));
    assert!(de_body.contains("Vollständige Upgrade-Vorgaben lesen"));
    assert!(!de_body.contains("Read the complete upgrade contract"));
    assert!(
        de_body.contains("jedem Storage-Bin-Replikat einen eigenen lokalen oder RWO-Datenträger")
    );

    let (fr_status, _headers, fr_body) = request("/fr/docs/deployment").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Podman rootless"));
    assert!(fr_body.contains("Checklist de production"));
    assert!(fr_body.contains("Mises à niveau sans interruption"));
}

#[tokio::test]
async fn tls_acme_uses_page_specific_translations() {
    let (de_status, _headers, de_body) = request("/de/docs/tls-acme").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Verwaltete Zertifikate"));
    assert!(de_body.contains("Detailreferenz"));
    assert!(de_body.contains("Widerruf des Client-Zertifikats"));

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
    assert!(de_body.contains("beratende Dateisystemsperre"));
    assert!(de_body.contains("Hinweis zum Upgrade des verschlüsselten Caches"));

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
    assert!(de_body.contains("jede Migrationsfamilie unabhängig testbar"));
    assert!(de_body.contains("Migrationsbeispiele"));
    assert!(de_body.contains("Wasm-Host-Rückrufe"));
    assert!(de_body.contains("Release-Tests"));
    assert!(de_body.contains("Kontrollen für das Laden von Modulen"));

    let (fr_status, _headers, fr_body) = request("/fr/docs/advanced").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Modules futurs"));
    assert!(fr_body.contains("Extensions WASM"));
    assert!(fr_body.contains("chaque famille de migration testable indépendamment"));
    assert!(fr_body.contains("Rappels de l'hôte Wasm"));

    let (ja_status, _headers, ja_body) = request("/ja/docs/advanced").await;
    assert_eq!(ja_status, StatusCode::OK);
    assert!(ja_body.contains("移行例の各系統を個別にテスト"));
    assert!(ja_body.contains("Wasmホストコールバック"));
    assert!(ja_body.contains("暗号学的ダイジェストに固定した Wasm モジュール"));
    assert!(ja_body.contains("読み込み時の受け入れ制御"));
    assert!(ja_body.contains("ファイルの同一性"));
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
