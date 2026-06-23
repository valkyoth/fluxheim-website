use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_auth_request_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>External Authorization Request — Fluxheim Source Docs</title>",
        "<h1 id=\"external-authorization-request\">External Authorization Request</h1>",
        "<p>Status: future optional module.</p>",
        "<p>This module lets Fluxheim ask a configured authorization service whether a client request should continue. It is designed for deployments that already have a policy service, session service, identity gateway, or internal access decision API and want Fluxheim to enforce the decision before proxying or serving static content.</p>",
        "<h2 id=\"decision-contract\">Decision Contract</h2>",
        "<li>Auth backend TLS verification must be enabled by default.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">External Authorization Request<".to_owned(), "1.6.28");

    assert!(translated.contains("Externe Autorisierungsanfrage"));
    assert!(translated.contains("kuenftiges optionales Modul"));
    assert!(translated.contains("konfigurierten Autorisierungsdienst"));
    assert!(translated.contains("Entscheidungsvertrag"));
    assert!(translated.contains("TLS-Verifikation fuer das Auth-Backend"));
    assert!(unrelated.contains(">External Authorization Request<"));
}

#[test]
fn applies_stable_programmable_media_edge_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Programmable Media Edge — Fluxheim Source Docs</title>",
        "<h1 id=\"programmable-media-edge\">Programmable Media Edge</h1>",
        "<p>Status: far-future optional module family.</p>",
        "<li>Integrate with <code>auth-request</code>, identity-aware routing, cache, metrics, and privacy profiles only through explicit policy.</li>",
        "<li>isolate cache keys by vhost, route, asset ID, representation, byte range, media sequence, encryption key ID, and policy version;</li>",
        "<h2 id=\"security-requirements\">Security Requirements</h2>",
        "<li>Do not enable in <code>privacy-mode</code> by default.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Programmable Media Edge<".to_owned(), "1.6.28");

    assert!(translated.contains("Programmierbarer Media-Edge"));
    assert!(translated.contains("optionale Modulfamilie"));
    assert!(translated.contains("Nur ueber explizite Policy"));
    assert!(translated.contains("Cache-Schluessel nach Vhost"));
    assert!(translated.contains("Sicherheitsanforderungen"));
    assert!(translated.contains("standardmaessig nicht aktivieren"));
    assert!(unrelated.contains(">Programmable Media Edge<"));
}

#[test]
fn applies_stable_zero_retention_privacy_mode_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Zero-Retention Privacy Mode — Fluxheim Source Docs</title>",
        "<h1 id=\"zero-retention-privacy-mode\">Zero-Retention Privacy Mode</h1>",
        "<p>Zero-retention privacy mode is a future compile-time build profile for users who want Fluxheim to serve static files and reverse-proxy requests without persisting request logs, client IPs, request metadata, or per-client telemetry.</p>",
        "<h2 id=\"honest-boundary\">Honest Boundary</h2>",
        "<li>no Fluxheim request metrics;</li>",
        "<li>Invalid feature combinations fail at compile time.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Zero-Retention Privacy Mode<".to_owned(), "1.6.28");

    assert!(translated.contains("Zero-Retention-Privacy-Modus"));
    assert!(translated.contains("kuenftiges Compile-Time-Build-Profil"));
    assert!(translated.contains("Ehrliche Grenze"));
    assert!(translated.contains("keine Fluxheim-Request-Metriken"));
    assert!(translated.contains("Feature-Kombinationen"));
    assert!(unrelated.contains(">Zero-Retention Privacy Mode<"));
}

#[test]
fn applies_stable_wasm_extensibility_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>WASM Extensibility — Fluxheim Source Docs</title>",
        "<h1 id=\"wasm-extensibility\">WASM Extensibility</h1>",
        "<p>WASM extensibility gives Fluxheim a sandboxed way to run operator-provided logic without compiling that logic into the Fluxheim binary. It should be treated as a major extension boundary, not as a small scripting feature.</p>",
        "<h2 id=\"design-goals\">Design Goals</h2>",
        "<li>Keep WASM runtime code out of default builds.</li>",
        "<li>Make all host calls explicit, small, and auditable.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">WASM Extensibility<".to_owned(), "1.6.28");

    assert!(translated.contains("WASM-Erweiterbarkeit"));
    assert!(translated.contains("sandboxed Weg"));
    assert!(translated.contains("Designziele"));
    assert!(translated.contains("WASM-Runtime-Code"));
    assert!(translated.contains("Host-Calls"));
    assert!(unrelated.contains(">WASM Extensibility<"));
}

#[test]
fn applies_stable_opentelemetry_tracing_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>OpenTelemetry Tracing — Fluxheim Source Docs</title>",
        "<h1 id=\"opentelemetry-tracing\">OpenTelemetry Tracing</h1>",
        "<p>Tracing is different from metrics and logs. Metrics show aggregate behavior, logs record events, and traces explain the path of a specific request through Fluxheim and its upstream services.</p>",
        "<h2 id=\"design-goals\">Design Goals</h2>",
        "<li>Keep OpenTelemetry code out of default builds.</li>",
        "<li>Use W3C Trace Context propagation.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">OpenTelemetry Tracing<".to_owned(), "1.6.28");

    assert!(translated.contains("OpenTelemetry-Tracing"));
    assert!(translated.contains("Tracing unterscheidet sich"));
    assert!(translated.contains("Designziele"));
    assert!(translated.contains("OpenTelemetry-Code"));
    assert!(translated.contains("W3C-Trace-Context-Propagation"));
    assert!(unrelated.contains(">OpenTelemetry Tracing<"));
}

#[test]
fn applies_stable_php_fpm_app_recipes_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>PHP-FPM Application Recipes — Fluxheim Source Docs</title>",
        "<h1 id=\"php-fpm-application-recipes\">PHP-FPM Application Recipes</h1>",
        "<h2 id=\"supported-php-fpm-functionality\">Supported PHP-FPM Functionality</h2>",
        "<p>FastCGI and backend connectivity:</p>",
        "<p>CGI/FastCGI request construction:</p>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">PHP-FPM Application Recipes<".to_owned(), "1.6.28");

    assert!(translated.contains("PHP-FPM-Anwendungsrezepte"));
    assert!(translated.contains("Unterstuetzte PHP-FPM-Funktionalitaet"));
    assert!(translated.contains("FastCGI- und Backend-Konnektivitaet"));
    assert!(translated.contains("CGI-/FastCGI-Anfragekonstruktion"));
    assert!(unrelated.contains(">PHP-FPM Application Recipes<"));
}

#[test]
fn applies_stable_sentinel_mesh_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Sentinel Mesh — Fluxheim Source Docs</title>",
        "<h1 id=\"sentinel-mesh-smart-wireguard-load-balancing\">Sentinel Mesh: Smart WireGuard Load Balancing</h1>",
        "<p>Sentinel Mesh is a future Fluxheim architecture for a self-healing smart load balancer. It combines Pingora&#x27;s proxy and load-balancing hooks with a private WireGuard transport and real-time backend telemetry. The goal is to route by observed health and load instead of only round-robin or static weights.</p>",
        "<h2 id=\"high-level-architecture\">High-Level Architecture</h2>",
        "<h2 id=\"wireguard-transport-options\">WireGuard Transport Options</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(
        de,
        ">Sentinel Mesh: Smart WireGuard Load Balancing<".to_owned(),
        "1.6.28",
    );

    assert!(translated.contains("Smartes WireGuard-Load-Balancing"));
    assert!(translated.contains("selbstheilenden smarten Load Balancer"));
    assert!(translated.contains("High-Level-Architektur"));
    assert!(translated.contains("WireGuard-Transportoptionen"));
    assert!(unrelated.contains(">Sentinel Mesh: Smart WireGuard Load Balancing<"));
}

#[test]
fn applies_stable_source_features_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Feature Matrix — Fluxheim Source Docs</title>",
        "<h1>Feature Matrix</h1>",
        "<p>Fluxheim uses Cargo features for compile-time module selection. The default binary is intentionally useful but conservative:</p>",
        "<h2>Stable Core Features</h2>",
        "<h2>Profile Aliases</h2>",
        "<p>Those profile aliases are deliberately narrow proof builds. FIPS/ISO-capable TLS is not limited to those aliases: custom builds can combine <code>tls-openssl-fips</code> or <code>tls-rustls-fips</code> with cache, static web serving, reverse proxying, or PHP-FPM. Do not add a FIPS-capable TLS backend to an existing profile alias that already enables <code>tls-rustls</code>, because Cargo features are additive and Fluxheim supports only one Pingora TLS backend per binary. Select the raw modules instead:</p>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Stable Core Features<".to_owned(), "1.6.28");

    assert!(translated.contains("Funktionsmatrix"));
    assert!(translated.contains("Fluxheim nutzt Cargo-Features"));
    assert!(translated.contains("Stabile Core-Features"));
    assert!(translated.contains("Profil-Aliasse"));
    assert!(translated.contains("FIPS-/ISO-faehiges TLS ist nicht auf diese Aliasse beschraenkt"));
    assert!(!translated.contains("FIPS/ISO-capable TLS is not limited"));
    assert!(unrelated.contains(">Stable Core Features<"));
}

#[test]
fn applies_stable_crypto_rpc_edge_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Crypto RPC Edge — Fluxheim Source Docs</title>",
        "<h1>Crypto RPC Edge</h1>",
        "<p>Status: future optional module family.</p>",
        "<p>The crypto RPC edge track is a future Fluxheim module family for running blockchain-aware RPC gateways in front of local nodes or hosted-compatible upstreams. It should be designed as focused compile-time modules, not as part of the default web/proxy build.</p>",
        "<h2>Why Ethereum First</h2>",
        "<h2>Compile-Time Shape</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Why Ethereum First<".to_owned(), "1.6.28");

    assert!(translated.contains("Edge RPC crypto"));
    assert!(translated.contains("Statut: future famille de modules optionnelle."));
    assert!(translated.contains("passerelles RPC conscientes de la blockchain"));
    assert!(translated.contains("Pourquoi Ethereum d'abord"));
    assert!(translated.contains("Forme compile-time"));
    assert!(unrelated.contains(">Why Ethereum First<"));
}

#[test]
fn applies_stable_fips_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>FIPS-Capable Deployment — Fluxheim Source Docs</title>",
        "<h1>FIPS / ISO-Capable Deployments</h1>",
        "<p>This document defines Fluxheim's FIPS 140-3 and ISO/IEC 19790 direction. It is intentionally strict about language: Fluxheim can provide FIPS/ISO-capable builds and fail-closed configuration enforcement, but Fluxheim itself is not a validated cryptographic module.</p>",
        "<p>Current release line:</p>",
        "<h2>Official References</h2>",
        "<h2>Compliance Boundary</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Compliance Boundary<".to_owned(), "1.6.28");

    assert!(translated.contains("FIPS-/ISO-faehige Deployments"));
    assert!(translated.contains("Fluxheims FIPS-140-3- und ISO/IEC-19790-Richtung"));
    assert!(translated.contains("Aktuelle Release-Linie:"));
    assert!(translated.contains("Offizielle Referenzen"));
    assert!(translated.contains("Compliance-Grenze"));
    assert!(unrelated.contains(">Compliance Boundary<"));
}

#[test]
fn applies_stable_versioning_plan_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Versioning Plan — Fluxheim Source Docs</title>",
        "<h1>Versioning Plan</h1>",
        "<p>Fluxheim should use SemVer, but with a conservative interpretation: a feature is not considered stable just because it compiles. A feature becomes stable only after it has docs, config validation, tests, release checks, and a clear security boundary.</p>",
        "<h2>Versioning Rules</h2>",
        "<h2>Release Ladder</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Release Ladder<".to_owned(), "1.6.28");

    assert!(translated.contains("Plan de versioning"));
    assert!(translated.contains("Fluxheim doit utiliser SemVer"));
    assert!(translated.contains("Regles de versioning"));
    assert!(translated.contains("Echelle de release"));
    assert!(unrelated.contains(">Release Ladder<"));
}

#[test]
fn applies_stable_php_runtime_support_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>PHP Runtime Support — Fluxheim Source Docs</title>",
        "<h1>PHP Runtime Support</h1>",
        "<p>Implemented feature flags:</p>",
        "<p>Release order:</p>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Implemented feature flags:<".to_owned(), "1.6.28");

    assert!(translated.contains("PHP-Runtime-Unterstützung"));
    assert!(translated.contains("PHP-Runtime-Unterstützung - Fluxheim-Source-Dokumentation"));
    assert!(translated.contains("Implementierte Feature-Flags"));
    assert!(translated.contains("Release-Reihenfolge"));
    assert!(unrelated.contains(">Implemented feature flags:<"));
}

#[test]
fn applies_stable_release_checklist_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Release Checklist — Fluxheim Source Docs</title>",
        "<h1>Release Checklist</h1>",
        "<p>Use this checklist before publishing a Fluxheim release, changing dependency versions, changing TLS/cache/proxy behavior, or building an image for other people to run.</p>",
        "<h2>Version And Toolchain</h2>",
        "<h2>Dependency, License, And Advisory Gates</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Version And Toolchain<".to_owned(), "1.6.28");

    assert!(translated.contains("Release-Checkliste"));
    assert!(translated.contains("Fluxheim-Release veröffentlichst"));
    assert!(translated.contains("Version und Toolchain"));
    assert!(translated.contains("Dependency-, Lizenz- und Advisory-Gates"));
    assert!(unrelated.contains(">Version And Toolchain<"));
}

#[test]
fn applies_stable_modularity_exceptions_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Modularity Exceptions — Fluxheim Source Docs</title>",
        "<h1>Fluxheim Modularity Exceptions</h1>",
        "<p>Status: baseline inventory for the 1.6 line</p>",
        "<p>This file records legacy non-generated Rust files above the 500-line target in</p>",
        "<h2>Legacy Exceptions</h2>",
        "<th>Split target</th>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Legacy Exceptions<".to_owned(), "1.6.28");

    assert!(translated.contains("Exceptions de modularite"));
    assert!(translated.contains("inventaire de baseline"));
    assert!(translated.contains("500 lignes"));
    assert!(translated.contains("Exceptions legacy"));
    assert!(translated.contains("Cible de decoupage"));
    assert!(unrelated.contains(">Legacy Exceptions<"));
}

#[test]
fn applies_stable_release_runbook_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Release Runbook — Fluxheim Source Docs</title>",
        "<h1>Release Runbook</h1>",
        "<p>This is the maintainer procedure for publishing a Fluxheim release. It is the step-by-step operational companion to the broader release checklist.</p>",
        "<h2>1. Preflight</h2>",
        "<h2>5. Draft The GitHub Release</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">5. Draft The GitHub Release<".to_owned(), "1.6.28");

    assert!(translated.contains("Release-Runbook"));
    assert!(translated.contains("Maintainer-Verfahren"));
    assert!(translated.contains("1. Preflight"));
    assert!(translated.contains("5. GitHub-Release entwerfen"));
    assert!(unrelated.contains(">5. Draft The GitHub Release<"));
}
