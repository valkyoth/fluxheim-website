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
