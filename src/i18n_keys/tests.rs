use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_download_keys_for_release_page_copy() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Download — Fluxheim</title>",
        ">Cache Edge Build<",
        "<span class=\"text-xs font-bold uppercase tracking-widest text-amber-400\">Cache</span>",
        "Released June 23, 2026",
        "Native upstream HTTP/2 release with plaintext h2c/prior-knowledge origins, ",
        "TLS ALPN HTTP/2 origins, pooled native upstream H2, bounded H2 policy timeouts, ",
        "and explicit h2c Upgrade fallback.",
        "proxy.error_pages</code> fallback pages backed by <code>fluxheim-web",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.30");

    assert!(translated.contains(">Cache-Edge-Build<"));
    assert!(translated.contains(">Cache-Profil</span>"));
    assert!(translated.contains("Veröffentlicht am 23. Juni 2026"));
    assert!(translated.contains("Native Upstream-HTTP/2-Version"));
    assert!(translated.contains("gepooltem nativem Upstream-H2"));
    assert!(translated.contains("proxy.error_pages</code> Fallback-Seiten, gestützt durch"));
}

#[test]
fn applies_stable_changelog_keys_only_on_changelog_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Changelog — Fluxheim</title>",
        "Released June 23, 2026",
        "Moves plaintext upstream HTTP/2 forwarding into the native HTTP/1 proxy path ",
        "for h2c/prior-knowledge origins",
        "Adds pooled native upstream H2 connections with bounded stream capacity ",
        "and safe-method retry after pre-response pooled-handle failure",
        "Adds explicit, disabled-by-default h2c Upgrade fallback for plaintext ",
        "<code class=\"text-cyan-400 text-xs\">http1-and-http2</code> origins",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.30");
    let unrelated = apply_shared_keys(de, "Released June 23, 2026".to_owned(), "1.6.30");

    assert!(translated.contains("<title>Änderungen — Fluxheim</title>"));
    assert!(translated.contains("Veröffentlicht am 23. Juni 2026"));
    assert!(translated.contains("Verschiebt Plaintext-Upstream-HTTP/2-Forwarding"));
    assert!(translated.contains("gepoolte native Upstream-H2-Verbindungen"));
    assert!(translated.contains("standardmäßig deaktivierten h2c-Upgrade-Fallback"));
    assert_eq!(unrelated, "Released June 23, 2026");
}

#[test]
fn applies_stable_runtime_parity_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Runtime Parity Fixtures — Fluxheim Source Docs</title>",
        "<h1>Runtime Parity Fixtures</h1>",
        "The machine-readable inventory is:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Runtime Parity Fixtures<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Runtime-Parity-Fixtures</h1>"));
    assert!(translated.contains("Das maschinenlesbare Inventar ist:"));
    assert_eq!(unrelated, ">Runtime Parity Fixtures<");
}

#[test]
fn applies_stable_geoip_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>GeoIP / Geo-Context — Fluxheim Source Docs</title>",
        "<h2>Local Databases</h2>",
        "Fluxheim <code>1.4.5</code> adds a bounded optional <code>geoip</code> ",
        "feature. It is a local Geo-Context foundation for access policy, ",
        "not a dynamic downloader or programmable geo engine.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Local Databases<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Lokale Datenbanken</h2>"));
    assert!(
        translated.contains("fuegt ein begrenztes optionales <code>geoip</code> Feature hinzu")
    );
    assert_eq!(unrelated, ">Local Databases<");
}

#[test]
fn applies_stable_load_balancer_ha_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Load Balancer HA Design Notes — Fluxheim Source Docs</title>",
        "<h1>Load Balancer HA Design Notes</h1>",
        "Current 1.5.3 Behavior",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Load Balancer HA Design Notes<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Load-Balancer-HA-Designhinweise</h1>"));
    assert!(translated.contains("Aktuelles 1.5.3-Verhalten"));
    assert_eq!(unrelated, ">Load Balancer HA Design Notes<");
}

#[test]
fn applies_stable_getting_started_keys_only_on_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Installation & Quick Start — Fluxheim Docs</title>",
        "<h2>Prerequisites</h2>",
        "<th>Profile</th>",
        "Get Fluxheim running in under five minutes — from tarball, container, or source.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Prerequisites<>Profile<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Voraussetzungen</h2>"));
    assert!(translated.contains("<th>Profil</th>"));
    assert!(translated.contains("Starte Fluxheim in unter fünf Minuten"));
    assert_eq!(unrelated, ">Prerequisites<>Profile<");
}

#[test]
fn applies_stable_cache_keys_only_on_cache_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Cache System — Fluxheim Docs</title>",
        "<h2>Enabling Cache</h2>",
        "<h3>Memory Cache</h3>",
        "Fluxheim's cache system supports memory, disk, tiered, and encrypted backends ",
        "with route-scoped policies, cache locks, stale serving, distributed peer fill, ",
        "and range caching.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Enabling Cache<>Memory Cache<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Cache aktivieren</h2>"));
    assert!(translated.contains("<h3>Memory-Cache</h3>"));
    assert!(translated.contains("Fluxheims Cache-System unterstützt"));
    assert_eq!(unrelated, ">Enabling Cache<>Memory Cache<");
}

#[test]
fn applies_stable_extraction_dependency_graph_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Extraction Dependency Graph — Fluxheim Source Docs</title>",
        "<h1>Extraction Dependency Graph</h1>",
        "<h2>Dependency Direction</h2>",
        "Target dependency direction:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Extraction Dependency Graph<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Extraktions-Abhaengigkeitsgraph</h1>"));
    assert!(translated.contains("<h2>Abhaengigkeitsrichtung</h2>"));
    assert!(translated.contains("Zielrichtung der Abhaengigkeiten:"));
    assert_eq!(unrelated, ">Extraction Dependency Graph<");
}

#[test]
fn applies_stable_runtime_baseline_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Runtime Baseline — Fluxheim Source Docs</title>",
        "<h2>Release Evidence</h2>",
        "<h2>Pingora Dependency Exceptions</h2>",
        "Release gates write baseline output to:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Release Evidence<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Release-Nachweise</h2>"));
    assert!(translated.contains("<h2>Pingora-Dependency-Ausnahmen</h2>"));
    assert!(translated.contains("Release-Gates schreiben Baseline-Ausgaben nach:"));
    assert_eq!(unrelated, ">Release Evidence<");
}

#[test]
fn applies_stable_modularity_policy_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Modularity Policy — Fluxheim Source Docs</title>",
        "<h1>Fluxheim Modularity Policy</h1>",
        "<h2>Core Rule</h2>",
        "New or newly split Rust implementation files should follow:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Modularity Policy<>Core Rule<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Fluxheim-Modularitaets-Policy</h1>"));
    assert!(translated.contains("<h2>Kernregel</h2>"));
    assert!(translated.contains("Neue oder neu aufgeteilte Rust-Implementierungsdateien"));
    assert_eq!(unrelated, ">Modularity Policy<>Core Rule<");
}

#[test]
fn applies_stable_observability_keys_only_on_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Observability — Fluxheim Docs</title>",
        "<h2>Prometheus Metrics</h2>",
        "<h2>What gets traced</h2>",
        "Fluxheim supports Prometheus metrics, OpenTelemetry metrics and traces, ",
        "and structured logging. All observability features are opt-in at compile time.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Prometheus Metrics<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Prometheus-Metriken</h2>"));
    assert!(translated.contains("<h2>Was getraced wird</h2>"));
    assert!(translated.contains("Fluxheim unterstützt Prometheus-Metriken"));
    assert_eq!(unrelated, ">Prometheus Metrics<");
}

#[test]
fn applies_stable_release_notes_template_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Release Notes Template — Fluxheim Source Docs</title>",
        "<h1>Fluxheim Release Notes Template</h1>",
        "<h2>Security And Stability Gate</h2>",
        "Gate command:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Release Notes Template<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Fluxheim-Release-Hinweise-Vorlage</h1>"));
    assert!(translated.contains("<h2>Sicherheits- und Stabilitäts-Gate</h2>"));
    assert!(translated.contains("Gate-Befehl:"));
    assert_eq!(unrelated, ">Release Notes Template<");
}

#[test]
fn applies_stable_tls_acme_keys_only_on_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>TLS & ACME — Fluxheim Docs</title>",
        "<h2>TLS Backends</h2>",
        "<h3>ACME Challenge Methods</h3>",
        "Fluxheim ships with rustls as the default TLS backend and full managed ACME support for automatic certificate issuance and renewal.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">ACME Challenge Methods<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>TLS-Backends</h2>"));
    assert!(translated.contains("<h3>ACME-Challenge-Methoden</h3>"));
    assert!(translated.contains("vollständiger verwalteter ACME-Unterstützung"));
    assert_eq!(unrelated, ">ACME Challenge Methods<");
}

#[test]
fn applies_stable_owasp_baseline_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>OWASP Top 10 2025 Baseline — Fluxheim Source Docs</title>",
        "<h1>OWASP Top 10 2025 Baseline</h1>",
        "<td>A01 Broken Access Control</td>",
        "<h2>Maintenance Rule</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">A01 Broken Access Control<".to_owned(), "1.6.28");

    assert!(translated.contains("<td>A01 Fehlerhafte Zugriffskontrolle</td>"));
    assert!(translated.contains("<h2>Wartungsregel</h2>"));
    assert_eq!(unrelated, ">A01 Broken Access Control<");
}

#[test]
fn applies_stable_macos_development_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>macOS Development Support — Fluxheim Source Docs</title>",
        "<h1>macOS Development Support</h1>",
        "<h2>Local Runtime Paths</h2>",
        "<th>Recommended macOS dev path</th>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Local Runtime Paths<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>macOS-Entwicklungsunterstützung</h1>"));
    assert!(translated.contains("<h2>Lokale Runtime-Pfade</h2>"));
    assert!(translated.contains("<th>Empfohlener macOS-Dev-Pfad</th>"));
    assert_eq!(unrelated, ">Local Runtime Paths<");
}

#[test]
fn applies_stable_gateway_recipes_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Gateway Recipes — Fluxheim Source Docs</title>",
        "<h1>Gateway Recipes</h1>",
        "<h2>Shared Server Baseline</h2>",
        "<h2>Browser Login Probe For WordPress</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Gateway Recipes<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Gateway-Rezepte</h1>"));
    assert!(translated.contains("<h2>Gemeinsame Server-Basisline</h2>"));
    assert!(translated.contains("<h2>Browser-Login-Probe fuer WordPress</h2>"));
    assert_eq!(unrelated, ">Gateway Recipes<");
}

#[test]
fn applies_stable_deployment_keys_only_on_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Systemd & Containers — Fluxheim Docs</title>",
        "<h1>Systemd & Containers</h1>",
        "<h2>Rootless Podman Containers</h2>",
        "<h2>Production Readiness Checklist</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Rootless Podman Containers<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Systemd & Container</h1>"));
    assert!(translated.contains("<h2>Rootless Podman-Container</h2>"));
    assert!(translated.contains("<h2>Checkliste für Produktionsreife</h2>"));
    assert_eq!(unrelated, ">Rootless Podman Containers<");
}

#[test]
fn applies_stable_secure_links_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Secure Links — Fluxheim Source Docs</title>",
        "<h1>Secure Links</h1>",
        "<h2>Cryptography</h2>",
        "Secure-link claims should be typed and bounded:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Cryptography<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Kryptografie</h2>"));
    assert!(translated.contains("Secure-Link-Claims sollten typisiert"));
    assert_eq!(unrelated, ">Cryptography<");
}
