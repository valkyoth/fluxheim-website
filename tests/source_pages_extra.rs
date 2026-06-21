use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router;
use tower::ServiceExt;

async fn request(path: &str) -> (StatusCode, String) {
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
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body bytes");

    (status, String::from_utf8(body.to_vec()).expect("utf8 body"))
}

#[tokio::test]
async fn ecosystem_idea_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/fluxheim-ecosystem-idea").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Fluxheim-Ecosystem-Idee"));
    assert!(de_body.contains("Fluxheim integrieren koennen"));

    let (fr_status, fr_body) = request("/fr/docs/source/fluxheim-ecosystem-idea").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Idee d'ecosysteme Fluxheim"));
    assert!(fr_body.contains("Fluxheim tout en gardant"));
}

#[tokio::test]
async fn runtime_facts_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/runtime-facts-and-policy-proofs").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Runtime Facts und Policy Proofs"));
    assert!(de_body.contains("Policy Proof"));

    let (fr_status, fr_body) = request("/fr/docs/source/runtime-facts-and-policy-proofs").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Runtime Facts et Policy Proofs"));
    assert!(fr_body.contains("Policy Proof"));
}

#[tokio::test]
async fn perl_cgi_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/perl-cgi-support").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Perl-CGI-Unterstuetzung"));
    assert!(de_body.contains("Sicherheitsanforderungen"));

    let (fr_status, fr_body) = request("/fr/docs/source/perl-cgi-support").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Prise en charge CGI Perl"));
    assert!(fr_body.contains("Exigences de securite"));
}

#[tokio::test]
async fn wasm_extensibility_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/wasm-extensibility").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("WASM-Erweiterbarkeit"));
    assert!(de_body.contains("Designziele"));

    let (fr_status, fr_body) = request("/fr/docs/source/wasm-extensibility").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Extensibilité WASM"));
    assert!(fr_body.contains("Objectifs de conception"));
}

#[tokio::test]
async fn image_filter_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/image-filter").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Bildfilter"));
    assert!(de_body.contains("Sicherheitsanforderungen"));

    let (fr_status, fr_body) = request("/fr/docs/source/image-filter").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Filtre d'image"));
    assert!(fr_body.contains("Exigences de securite"));
}

#[tokio::test]
async fn source_features_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/features").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Stabile Core-Features"));
    assert!(de_body.contains("Profil-Aliasse"));

    let (fr_status, fr_body) = request("/fr/docs/source/features").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Fonctionnalités core stables"));
    assert!(fr_body.contains("Alias de profil"));
}

#[tokio::test]
async fn cache_backends_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/cache-backends").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Cache-Backends"));
    assert!(de_body.contains("Bewertung des Memory-Caches"));

    let (fr_status, fr_body) = request("/fr/docs/source/cache-backends").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Backends de cache"));
    assert!(fr_body.contains("Evaluation du cache memoire"));
}

#[tokio::test]
async fn certificate_renewal_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/certificate-renewal").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Zertifikatserneuerung und Reload"));
    assert!(de_body.contains("Planung der Erneuerungswarteschlange"));

    let (fr_status, fr_body) = request("/fr/docs/source/certificate-renewal").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Renouvellement et reload des certificats"));
    assert!(fr_body.contains("Planification de la file de renouvellement"));
}

#[tokio::test]
async fn programmable_media_edge_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/programmable-media-edge").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Programmierbarer Media-Edge"));
    assert!(de_body.contains("Geplantes Verhalten:"));

    let (fr_status, fr_body) = request("/fr/docs/source/programmable-media-edge").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Media edge programmable"));
    assert!(fr_body.contains("Comportement prevu :"));
}

#[tokio::test]
async fn waf_architecture_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/waf-architecture").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("WAF-Architektur"));
    assert!(de_body.contains("WAF-Audit-Logs sollten enthalten"));

    let (fr_status, fr_body) = request("/fr/docs/source/waf-architecture").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Architecture WAF"));
    assert!(fr_body.contains("Les logs d'audit WAF doivent inclure"));
}

#[tokio::test]
async fn fips_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/fips").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("FIPS-/ISO-faehige Deployments"));
    assert!(de_body.contains("Compliance-Grenze"));

    let (fr_status, fr_body) = request("/fr/docs/source/fips").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Deploiements compatibles FIPS / ISO"));
    assert!(fr_body.contains("Frontiere de compliance"));
}

#[tokio::test]
async fn zero_retention_privacy_mode_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/zero-retention-privacy-mode").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Zero-Retention-Privacy-Modus"));
    assert!(de_body.contains("Ehrliche Grenze"));

    let (fr_status, fr_body) = request("/fr/docs/source/zero-retention-privacy-mode").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Mode privacy zero-retention"));
    assert!(fr_body.contains("Frontiere honnete"));
}

#[tokio::test]
async fn common_criteria_roadmap_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/common-criteria-roadmap").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Common-Criteria-Roadmap"));
    assert!(de_body.contains("Definition des Sicherheitsproblems"));

    let (fr_status, fr_body) = request("/fr/docs/source/common-criteria-roadmap").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Feuille de route de preparation Common Criteria"));
    assert!(fr_body.contains("Definition du probleme de securite"));
}

#[tokio::test]
async fn gateway_recipes_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/gateway-recipes").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Gateway-Rezepte"));
    assert!(de_body.contains("Gemeinsame Server-Basisline"));

    let (fr_status, fr_body) = request("/fr/docs/source/gateway-recipes").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Recettes gateway"));
    assert!(fr_body.contains("Baseline serveur partagee"));
}

#[tokio::test]
async fn crypto_rpc_edge_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/crypto-rpc-edge").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Crypto-RPC-Edge"));
    assert!(de_body.contains("Warum Ethereum zuerst"));

    let (fr_status, fr_body) = request("/fr/docs/source/crypto-rpc-edge").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Edge RPC crypto"));
    assert!(fr_body.contains("Pourquoi Ethereum d'abord"));
}

#[tokio::test]
async fn versioning_plan_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/versioning-plan").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Versionierungsplan"));
    assert!(de_body.contains("Release-Leiter"));

    let (fr_status, fr_body) = request("/fr/docs/source/versioning-plan").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Plan de versioning"));
    assert!(fr_body.contains("Echelle de release"));
}

#[tokio::test]
async fn config_reference_source_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/config-reference").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Konfigurationsreferenz"));
    assert!(de_body.contains("TCP-Stream-Proxy"));

    let (fr_status, fr_body) = request("/fr/docs/source/config-reference").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Reference de configuration"));
    assert!(fr_body.contains("Proxy de flux TCP"));
}

#[tokio::test]
async fn opentelemetry_tracing_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/opentelemetry-tracing").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("OpenTelemetry-Tracing"));
    assert!(de_body.contains("Designziele"));

    let (fr_status, fr_body) = request("/fr/docs/source/opentelemetry-tracing").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Tracing OpenTelemetry"));
    assert!(fr_body.contains("Objectifs de conception"));
}

#[tokio::test]
async fn php_runtime_support_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/php-runtime-support").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("PHP-Runtime-Unterstützung"));
    assert!(de_body.contains("Implementierte Feature-Flags"));

    let (fr_status, fr_body) = request("/fr/docs/source/php-runtime-support").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Support runtime PHP"));
    assert!(fr_body.contains("Fonctionnalité flags implementes"));
}

#[tokio::test]
async fn php_fpm_app_recipes_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/php-fpm-app-recipes").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("PHP-FPM-Anwendungsrezepte"));
    assert!(de_body.contains("Unterstuetzte PHP-FPM-Funktionalitaet"));

    let (fr_status, fr_body) = request("/fr/docs/source/php-fpm-app-recipes").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Recettes d'applications PHP-FPM"));
    assert!(fr_body.contains("Fonctionnalite PHP-FPM prise en charge"));
}

#[tokio::test]
async fn sentinel_mesh_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/sentinel-mesh").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Smartes WireGuard-Load-Balancing"));
    assert!(de_body.contains("WireGuard-Transportoptionen"));

    let (fr_status, fr_body) = request("/fr/docs/source/sentinel-mesh").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("load balancing WireGuard intelligent"));
    assert!(fr_body.contains("Options de transport WireGuard"));
}
