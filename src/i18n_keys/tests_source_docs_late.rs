use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_certificate_renewal_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Certificate Renewal And Reload — Fluxheim Source Docs</title>",
        "Certificate Renewal And Reload",
        "<p>Production packages and container images ship the <code>fluxheim-acme</code> companion as the preferred external renewal command. It uses the same ACME engine, storage layout, issuer credentials, and vhost target planner as the integrated gateway, but keeps renewal scheduling outside the traffic-serving process:</p>",
        "<p>By default <code>renew</code> observes the managed certificate files and attempts only missing or due certificates. If nothing needs renewal, it exits successfully with <code>acme attempted: 0</code> and a status message saying no certificates are due. First issuance normally does not need <code>--force-renew</code>: missing certificate files are due targets. The command prints every target with <code>status=due</code>, <code>status=skipped</code>, or <code>status=forced</code>, then reports per-target <code>renewed:</code> and <code>failed:</code> lines. For HTTP-01 failures after challenge files are published, the failure includes <code>published_http_01=</code> URLs so operators can test the exact public challenge paths that the issuer should have reached. It exits non-zero if any target failed, while still reporting successful renewals from the same run.</p>",
        "<li><code>tls.acme.renewal.renew_after</code>, when set</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Certificate Renewal And Reload<".to_owned(), "1.6.28");

    assert!(translated.contains("Zertifikatserneuerung und Reload"));
    assert!(translated.contains("<code>fluxheim-acme</code>-Companion"));
    assert!(translated.contains("haelt die Erneuerungsplanung"));
    assert!(translated.contains("Standardmaessig beobachtet <code>renew</code>"));
    assert!(translated.contains("<code>published_http_01=</code>-URLs"));
    assert!(translated.contains("<code>tls.acme.renewal.renew_after</code>, wenn gesetzt"));
    assert!(unrelated.contains(">Certificate Renewal And Reload<"));
}

#[test]
fn applies_stable_logging_architecture_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Logging Architecture — Fluxheim Source Docs</title>",
        "<h1 id=\"logging-architecture\">Logging Architecture</h1>",
        "<p>Fluxheim logging should be structured, asynchronous, and explicit about the tradeoff between request latency and durability. Logging must never hide security-relevant events, and remote logging failure must not break normal traffic.</p>",
        "<li>Add access/security/audit event schema and request ids.</li>",
        "<li>Add bounded dispatcher queue with <code>drop_new</code> and <code>block</code> policies.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Logging Architecture<".to_owned(), "1.6.28");

    assert!(translated.contains("Logging-Architektur"));
    assert!(translated.contains("Fluxheim-Logging sollte strukturiert"));
    assert!(translated.contains("Zugriffs-/Security-/Audit-Event-Schema"));
    assert!(translated.contains("<code>drop_new</code>- und <code>block</code>-Policies"));
    assert!(unrelated.contains(">Logging Architecture<"));
}

#[test]
fn applies_stable_legacy_static_http_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Legacy Static HTTP Support — Fluxheim Source Docs</title>",
        "<h1 id=\"legacy-static-http-support\">Legacy Static HTTP Support</h1>",
        "<p>Legacy HTTP support is a future experimental compatibility feature for isolated devices that cannot speak modern HTTP. It must never be part of Fluxheim&#x27;s default binary and must never run on normal proxy, cache, admin, PHP, CGI, or TLS listener paths.</p>",
        "<h2 id=\"http-1-0-static-mode\">HTTP/1.0 Static Mode</h2>",
        "<li>HTTP/1.0 requests must never reach proxy/cache/admin/PHP/CGI paths.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Legacy Static HTTP Support<".to_owned(), "1.6.28");

    assert!(translated.contains("Support HTTP statique legacy"));
    assert!(translated.contains("fonctionnalite experimentale de compatibilite"));
    assert!(translated.contains("Mode statique HTTP/1.0"));
    assert!(translated.contains("ne doivent jamais atteindre"));
    assert!(unrelated.contains(">Legacy Static HTTP Support<"));
}

#[test]
fn applies_stable_metrics_architecture_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Metrics Architecture — Fluxheim Source Docs</title>",
        "<h1 id=\"metrics-architecture\">Metrics Architecture</h1>",
        "<p>Fluxheim metrics capture aggregate health and performance. Logs explain what happened for an individual event; metrics answer questions such as request rate, error rate, p95/p99 latency, cache efficiency, and upstream health.</p>",
        "<h2 id=\"cardinality-rules\">Cardinality Rules</h2>",
        "<li>implemented now for OTLP metrics exporter attempts: <code>fluxheim_metrics_otlp_exports_total{outcome}</code> with bounded <code>success</code>, <code>failure</code>, or <code>other</code> outcomes.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Metrics Architecture<".to_owned(), "1.6.28");

    assert!(translated.contains("Metriken-Architektur"));
    assert!(translated.contains("aggregierte Health- und Performance-Daten"));
    assert!(translated.contains("Kardinalitätsregeln"));
    assert!(translated.contains("OTLP-Metrics-Exporter-Versuche"));
    assert!(unrelated.contains(">Metrics Architecture<"));
}

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
