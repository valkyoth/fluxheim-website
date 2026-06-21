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
async fn load_balancer_migration_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/load-balancer-migration").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Load-Balancer-Migrationshinweise"));
    assert!(de_body.contains("Bekannte Migrationsgrenzen"));

    let (fr_status, fr_body) = request("/fr/docs/source/load-balancer-migration").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Notes de migration du load balancer"));
    assert!(fr_body.contains("Limites de migration connues"));
}

#[tokio::test]
async fn load_balancer_ha_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/load-balancer-ha").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Load-Balancer-HA-Designhinweise"));
    assert!(de_body.contains("Nichtziele fuer 1.5.3"));

    let (fr_status, fr_body) = request("/fr/docs/source/load-balancer-ha").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Notes de conception HA du load balancer"));
    assert!(fr_body.contains("Non-objectifs pour 1.5.3"));
}

#[tokio::test]
async fn build_and_podman_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/build-and-podman").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Container-Varianten"));
    assert!(de_body.contains("Volume-Mapping"));
    assert!(de_body.contains("Codex und Rootless Podman"));
    assert!(de_body.contains("Example-Host-Layout") || de_body.contains("Beispiel-Host-Layout"));
    assert!(de_body.contains("HTTPS in der Hauptkonfiguration"));
    assert!(de_body.contains("FIPS-/ISO-faehige OpenSSL-Tests"));
    assert!(de_body.contains("Standard-Image enthaelt Proxying"));
    assert!(de_body.contains("HTTP-01 ACME muss die CA"));
    assert!(de_body.contains("direkte Proxy-Upstream-Namen"));

    let (fr_status, fr_body) = request("/fr/docs/source/build-and-podman").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Variantes de conteneurs"));
    assert!(fr_body.contains("Mapping des volumes"));
    assert!(fr_body.contains("Codex et Podman rootless"));
    assert!(fr_body.contains("Exemple de layout hote"));
    assert!(fr_body.contains("HTTPS dans la config principale"));
    assert!(fr_body.contains("tests OpenSSL compatibles FIPS/ISO"));
    assert!(fr_body.contains("image par defaut inclut proxying"));
    assert!(fr_body.contains("ACME HTTP-01, la CA"));
    assert!(fr_body.contains("noms upstream de proxy direct"));
}

#[tokio::test]
async fn source_systemd_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/systemd").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Manuelle Binary-Installation"));
    assert!(de_body.contains("Sandbox-Overrides"));
    assert!(de_body.contains("TLS- und Content-Pfade"));

    let (fr_status, fr_body) = request("/fr/docs/source/systemd").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Installation binaire manuelle"));
    assert!(fr_body.contains("Overrides de sandbox"));
    assert!(fr_body.contains("Chemins TLS et contenu"));
}

#[tokio::test]
async fn production_readiness_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/production-readiness").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Produktionsreife"));
    assert!(de_body.contains("Operator-Pruefungen"));
    assert!(de_body.contains("Konfigurationsreview"));

    let (fr_status, fr_body) = request("/fr/docs/source/production-readiness").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Préparation production"));
    assert!(fr_body.contains("Controles operateur"));
    assert!(fr_body.contains("Revue de configuration"));
}

#[tokio::test]
async fn compression_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/compression").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Komprimierung"));
    assert!(de_body.contains("Datenschutz und Sicherheit"));

    let (fr_status, fr_body) = request("/fr/docs/source/compression").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Compression"));
    assert!(fr_body.contains("Confidentialité et sécurité"));
}

#[tokio::test]
async fn vhost_config_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/vhost-config").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Vhost-Konfigurationsleitfaden"));
    assert!(de_body.contains("Häufige Fehler"));

    let (fr_status, fr_body) = request("/fr/docs/source/vhost-config").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Guide de configuration vhost"));
    assert!(fr_body.contains("Erreurs fréquentes"));
}

#[tokio::test]
async fn config_snapshots_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/config-snapshots").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Konfigurations-Snapshots"));
    assert!(de_body.contains("Store-Layout"));

    let (fr_status, fr_body) = request("/fr/docs/source/config-snapshots").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Snapshots de configuration"));
    assert!(fr_body.contains("Layout du store"));
}

#[tokio::test]
async fn supply_chain_security_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/supply-chain-security").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Supply-Chain-Sicherheit"));
    assert!(de_body.contains("Aktuelle Kontrollen"));

    let (fr_status, fr_body) = request("/fr/docs/source/supply-chain-security").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Sécurité supply chain"));
    assert!(fr_body.contains("Controles actuels"));
}

#[tokio::test]
async fn github_setup_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/github-setup").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("GitHub-Repository-Einrichtung"));
    assert!(de_body.contains("Empfohlener sicherer Weg"));

    let (fr_status, fr_body) = request("/fr/docs/source/github-setup").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Configuration du repository GitHub"));
    assert!(fr_body.contains("Chemin sur recommande"));
}

#[tokio::test]
async fn geoip_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/geoip").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Lokale Datenbanken"));
    assert!(de_body.contains("Zugriffspolitik"));

    let (fr_status, fr_body) = request("/fr/docs/source/geoip").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Bases locales"));
    assert!(fr_body.contains("Policy d'acces"));
}

#[tokio::test]
async fn macos_development_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/macos-development").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("macOS-Entwicklungsunterstützung"));
    assert!(de_body.contains("Lokale Runtime-Pfade"));

    let (fr_status, fr_body) = request("/fr/docs/source/macos-development").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Support de développement macOS"));
    assert!(fr_body.contains("Chemins runtime locaux"));
}

#[tokio::test]
async fn logging_architecture_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/logging-architecture").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Log-Klassen"));
    assert!(de_body.contains("Ereignisfelder"));

    let (fr_status, fr_body) = request("/fr/docs/source/logging-architecture").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Classes de logs"));
    assert!(fr_body.contains("Champs d'événement"));
}

#[tokio::test]
async fn metrics_architecture_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/metrics-architecture").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Metriken-Architektur"));
    assert!(de_body.contains("Kardinalitätsregeln"));

    let (fr_status, fr_body) = request("/fr/docs/source/metrics-architecture").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Règles de cardinalité"));
    assert!(fr_body.contains("Labels autorisés"));
}

#[tokio::test]
async fn legacy_static_http_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/legacy-static-http").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Legacy-Static-HTTP-Support"));
    assert!(de_body.contains("HTTP/1.0-Static-Modus"));

    let (fr_status, fr_body) = request("/fr/docs/source/legacy-static-http").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Support HTTP statique legacy"));
    assert!(fr_body.contains("Mode statique HTTP/1.0"));
}

#[tokio::test]
async fn auth_request_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/auth-request").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Externe Autorisierungsanfrage"));
    assert!(de_body.contains("Entscheidungsvertrag"));

    let (fr_status, fr_body) = request("/fr/docs/source/auth-request").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Requete d'autorisation externe"));
    assert!(fr_body.contains("Contrat de decision"));
}

#[tokio::test]
async fn secure_links_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/secure-links").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Kryptografie"));
    assert!(de_body.contains("Secure-Link-Claims sollten"));

    let (fr_status, fr_body) = request("/fr/docs/source/secure-links").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Cryptographie"));
    assert!(fr_body.contains("Les claims secure-link doivent"));
}

#[tokio::test]
async fn release_checklist_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/release-checklist").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Release-Checkliste"));
    assert!(de_body.contains("Version und Toolchain"));

    let (fr_status, fr_body) = request("/fr/docs/source/release-checklist").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Checklist de publication"));
    assert!(fr_body.contains("Version et chaine d'outils"));
}

#[tokio::test]
async fn release_notes_template_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/release-notes-template").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Release-Hinweise-Vorlage"));
    assert!(de_body.contains("Sicherheits- und Stabilitäts-Gate"));

    let (fr_status, fr_body) = request("/fr/docs/source/release-notes-template").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Modele de notes de publication"));
    assert!(fr_body.contains("Gate de securite et de stabilite"));
}

#[tokio::test]
async fn release_runbook_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/release-runbook").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Release-Runbook"));
    assert!(de_body.contains("GitHub-Release entwerfen"));

    let (fr_status, fr_body) = request("/fr/docs/source/release-runbook").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Runbook de publication"));
    assert!(fr_body.contains("Rediger la publication GitHub"));
}

#[tokio::test]
async fn compliance_evidence_template_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/compliance-evidence-template").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Compliance-Nachweispaket-Vorlage"));
    assert!(de_body.contains("Kandidaten-TOE-Grenze"));

    let (fr_status, fr_body) = request("/fr/docs/source/compliance-evidence-template").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Modele de paquet de preuves de conformite"));
    assert!(fr_body.contains("Frontiere TOE candidate"));
}

#[tokio::test]
async fn runtime_baseline_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/runtime-baseline").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Release-Nachweise"));
    assert!(de_body.contains("Pingora-Dependency-Ausnahmen"));

    let (fr_status, fr_body) = request("/fr/docs/source/runtime-baseline").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("baseline de sortie Pingora"));
    assert!(fr_body.contains("Preuves de release"));
}

#[tokio::test]
async fn cloudflare_origin_support_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/cloudflare-origin-support").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Cloudflare-Origin-Unterstuetzung"));
    assert!(de_body.contains("Origin-CA-Automatisierung"));

    let (fr_status, fr_body) = request("/fr/docs/source/cloudflare-origin-support").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Prise en charge de Cloudflare Origin"));
    assert!(fr_body.contains("Automatisation Origin CA"));
}

#[tokio::test]
async fn cache_encryption_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/cache-encryption").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Cache-Verschluesselung"));
    assert!(de_body.contains("Eine minimale OpenBao-Policy"));

    let (fr_status, fr_body) = request("/fr/docs/source/cache-encryption").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Chiffrement du cache"));
    assert!(fr_body.contains("Configuration OpenBao Transit"));
}

#[tokio::test]
async fn runtime_parity_fixtures_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/runtime-parity-fixtures").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Runtime-Parity-Fixtures"));
    assert!(de_body.contains("Das maschinenlesbare Inventar"));

    let (fr_status, fr_body) = request("/fr/docs/source/runtime-parity-fixtures").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Relation avec la baseline runtime"));
    assert!(fr_body.contains("inventaire explicite"));
}

#[tokio::test]
async fn pingora_core_patch_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/pingora-core-patch").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Pingora-Patches"));
    assert!(de_body.contains("Rustls-Listener-Zertifikatresolver"));

    let (fr_status, fr_body) = request("/fr/docs/source/pingora-core-patch").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Patches Pingora"));
    assert!(fr_body.contains("Resolver de certificat du listener rustls"));
}

#[tokio::test]
async fn owasp_baseline_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/owasp-top10-2025-baseline").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("A01 Fehlerhafte Zugriffskontrolle"));
    assert!(de_body.contains("Wartungsregel"));

    let (fr_status, fr_body) = request("/fr/docs/source/owasp-top10-2025-baseline").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("A01 Controle d'acces casse"));
    assert!(fr_body.contains("Categorie OWASP 2025"));
}

#[tokio::test]
async fn extraction_dependency_graph_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/extraction-dependency-graph").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Extraktions-Abhaengigkeitsgraph"));
    assert!(de_body.contains("Abhaengigkeitsrichtung"));

    let (fr_status, fr_body) = request("/fr/docs/source/extraction-dependency-graph").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Graphe de dependances d'extraction"));
    assert!(fr_body.contains("Direction des dependances"));
}

#[tokio::test]
async fn modularity_policy_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/modularity-policy").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Modularitaet"));
    assert!(de_body.contains("500 Zeilen"));

    let (fr_status, fr_body) = request("/fr/docs/source/modularity-policy").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("modularite"));
    assert!(fr_body.contains("500 lignes"));
}

#[tokio::test]
async fn modularity_exceptions_uses_page_specific_translations() {
    let (de_status, de_body) = request("/de/docs/source/modularity-exceptions").await;
    assert_eq!(de_status, StatusCode::OK);
    assert!(de_body.contains("Legacy-Ausnahmen"));
    assert!(de_body.contains("Legacy-Pingora"));

    let (fr_status, fr_body) = request("/fr/docs/source/modularity-exceptions").await;
    assert_eq!(fr_status, StatusCode::OK);
    assert!(fr_body.contains("Exceptions de modularite"));
    assert!(fr_body.contains("Exceptions legacy"));
}
