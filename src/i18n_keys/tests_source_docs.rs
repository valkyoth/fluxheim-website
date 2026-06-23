use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_vhost_config_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Vhost Config Guide — Fluxheim Source Docs</title>",
        "<h1>Vhost Config Guide</h1>",
        "<h2>Common Mistakes</h2>",
        "Fluxheim uses TOML array-of-tables syntax for virtual hosts:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Common Mistakes<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Vhost-Konfigurationsleitfaden</h1>"));
    assert!(translated.contains("<h2>Häufige Fehler</h2>"));
    assert!(unrelated.contains(">Common Mistakes<"));
}

#[test]
fn applies_stable_fluxheim_ecosystem_idea_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Fluxheim Ecosystem Idea — Fluxheim Source Docs</title>",
        "<h1>Fluxheim Ecosystem Idea</h1>",
        "<h2>Proposed Shape</h2>",
        "The useful direction is a set of separate crates and projects that can integrate with Fluxheim while keeping each product boundary reviewable.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Proposed Shape<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Fluxheim-Ecosystem-Idee</h1>"));
    assert!(translated.contains("<h2>Vorgeschlagene Form</h2>"));
    assert!(translated.contains("Fluxheim integrieren koennen"));
    assert!(unrelated.contains(">Proposed Shape<"));
}

#[test]
fn applies_stable_github_setup_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>GitHub Repository Setup — Fluxheim Source Docs</title>",
        "<h1>GitHub Repository Setup</h1>",
        "<h2>Recommended Safe Path</h2>",
        "Use this when the remote has files you do not want to accidentally overwrite.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Recommended Safe Path<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>GitHub-Repository-Einrichtung</h1>"));
    assert!(translated.contains("<h2>Empfohlener sicherer Weg</h2>"));
    assert!(translated.contains("nicht versehentlich ueberschreiben"));
    assert!(unrelated.contains(">Recommended Safe Path<"));
}

#[test]
fn applies_stable_cache_encryption_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Cache Encryption — Fluxheim Source Docs</title>",
        "Cache Encryption",
        "What Gets Encrypted",
        "A minimal OpenBao policy for one cache key is:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Cache Encryption<".to_owned(), "1.6.28");

    assert!(translated.contains("Cache-Verschluesselung"));
    assert!(translated.contains("Was verschluesselt wird"));
    assert!(translated.contains("Eine minimale OpenBao-Policy"));
    assert!(unrelated.contains(">Cache Encryption<"));
}

#[test]
fn applies_stable_perl_cgi_support_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Perl CGI Support — Fluxheim Source Docs</title>",
        "Perl CGI Support",
        "Current Recommendation",
        "Planned feature flags:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Perl CGI Support<".to_owned(), "1.6.28");

    assert!(translated.contains("Perl-CGI-Unterstuetzung"));
    assert!(translated.contains("Aktuelle Empfehlung"));
    assert!(translated.contains("Geplante Feature-Flags:"));
    assert!(unrelated.contains(">Perl CGI Support<"));
}

#[test]
fn applies_stable_systemd_deployment_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>systemd Deployment — Fluxheim Source Docs</title>",
        "systemd Deployment",
        "Manual Binary Install",
        "TLS And Content Paths",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">systemd Deployment<".to_owned(), "1.6.28");

    assert!(translated.contains("systemd-Deployment"));
    assert!(translated.contains("Manuelle Binary-Installation"));
    assert!(translated.contains("TLS- und Content-Pfade"));
    assert!(unrelated.contains(">systemd Deployment<"));
}

#[test]
fn applies_stable_config_snapshots_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Config Snapshots And Rollback — Fluxheim Source Docs</title>",
        "Store Layout",
        "Admin API Shape",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Store Layout<".to_owned(), "1.6.28");

    assert!(translated.contains("Store-Layout"));
    assert!(translated.contains("Form der Admin-API"));
    assert!(unrelated.contains(">Store Layout<"));
}

#[test]
fn applies_stable_pingora_core_patch_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Pingora Core Patch — Fluxheim Source Docs</title>",
        "Pingora Core Patch",
        "Pingora Patches",
        "Removal Criteria",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Pingora Patches<".to_owned(), "1.6.28");

    assert!(translated.contains("Pingora-Core-Patch"));
    assert!(translated.contains("Pingora-Patches"));
    assert!(translated.contains("Entfernungskriterien"));
    assert!(unrelated.contains(">Pingora Patches<"));
}

#[test]
fn applies_stable_supply_chain_security_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Supply Chain Security — Fluxheim Source Docs</title>",
        "<h1 id=\"rust-supply-chain-security\">Rust Supply-Chain Security</h1>",
        "Current Controls",
        "Build Scripts And Procedural Macros",
        "Update <code>SECURITY.md</code>, release notes, <code>deny.toml</code>, or <code>.cargo/audit.toml</code> when an advisory exception or license exception changes.",
        "Accepted Limitations",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Current Controls<".to_owned(), "1.6.28");

    assert!(translated.contains("Rust-Supply-Chain-Sicherheit"));
    assert!(translated.contains("Aktuelle Kontrollen"));
    assert!(translated.contains("Build-Scripts und prozedurale Makros"));
    assert!(translated.contains("Release Notes"));
    assert!(translated.contains("Akzeptierte Einschraenkungen"));
    assert!(unrelated.contains(">Current Controls<"));
}

#[test]
fn applies_stable_compression_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Compression — Fluxheim Source Docs</title>",
        "<h1 id=\"compression\">Compression</h1>",
        "Cargo features:",
        "Goals",
        "Privacy And Security",
        "Cache Integration",
        "Test Plan",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Goals<".to_owned(), "1.6.28");

    assert!(translated.contains("Komprimierung"));
    assert!(translated.contains("Cargo-Features:"));
    assert!(translated.contains("Ziele"));
    assert!(translated.contains("Datenschutz und Sicherheit"));
    assert!(translated.contains("Cache-Integration"));
    assert!(translated.contains("Testplan"));
    assert!(unrelated.contains(">Goals<"));
}

#[test]
fn applies_stable_load_balancer_migration_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Load Balancer Migration Notes — Fluxheim Source Docs</title>",
        "Load Balancer Migration Notes",
        "Runtime Operations",
        "Known Migration Boundaries",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Runtime Operations<".to_owned(), "1.6.28");

    assert!(translated.contains("Load-Balancer-Migrationshinweise"));
    assert!(translated.contains("Runtime-Operationen"));
    assert!(translated.contains("Bekannte Migrationsgrenzen"));
    assert!(unrelated.contains(">Runtime Operations<"));
}

#[test]
fn applies_stable_runtime_facts_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Runtime Facts And Policy Proofs — Fluxheim Source Docs</title>",
        "Runtime Facts And Policy Proofs",
        "Non-Goals",
        "Security Rules",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Security Rules<".to_owned(), "1.6.28");

    assert!(translated.contains("Runtime Facts und Policy Proofs"));
    assert!(translated.contains("Nicht-Ziele"));
    assert!(translated.contains("Sicherheit-Regeln"));
    assert!(unrelated.contains(">Security Rules<"));
}

#[test]
fn applies_stable_production_readiness_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Production Readiness — Fluxheim Source Docs</title>",
        "Operator Checks",
        "Configuration Review",
        "Deployment Notes",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Operator Checks<".to_owned(), "1.6.28");

    assert!(translated.contains("Operator-Pruefungen"));
    assert!(translated.contains("Konfigurationsreview"));
    assert!(translated.contains("Deployment-Hinweise"));
    assert!(unrelated.contains(">Operator Checks<"));
}

#[test]
fn applies_stable_cache_backends_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Cache Backends — Fluxheim Source Docs</title>",
        "Cache Backends",
        "Fluxheim&#x27;s cache configuration is intentionally byte-budgeted even when a backend crate is count-based. Operators should be able to say &quot;use 1 GiB of RAM&quot; or &quot;use this 10 GiB disk directory&quot; globally or per vhost without knowing the internal cache implementation.",
        "Memory Cache Evaluation",
        "Disk eviction maintains an ordered LRU view inside the runtime disk-object index.",
        "Admissions that need space walk only the oldest entries needed to free the target byte count instead of cloning and sorting the full disk inventory on every eviction cycle.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Memory Cache Evaluation<".to_owned(), "1.6.28");

    assert!(translated.contains("Cache-Backends"));
    assert!(translated.contains("Fluxheims Cache-Konfiguration"));
    assert!(translated.contains("Bewertung des Memory-Caches"));
    assert!(translated.contains("Disk-Eviction haelt eine geordnete LRU-Sicht"));
    assert!(translated.contains("die Speicherplatz benoetigen"));
    assert!(unrelated.contains(">Memory Cache Evaluation<"));
}

#[test]
fn applies_stable_waf_architecture_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>WAF Architecture — Fluxheim Source Docs</title>",
        "WAF Architecture",
        "Fluxheim WAF support is future optional security functionality. It must not be compiled into default builds. The normal secure default remains a small edge proxy/static server focused on strict protocol handling, TLS, cache, logging, and metrics.",
        "Feature Flags",
        "WAF audit logs should include:",
        "<p>Unknown or invalid hosts must map to fixed buckets such as <code>unknown</code> or <code>invalid_host</code>.</p>",
        "WAF engine failure follows",
        "<li>WAF engine failure follows <code>fail_closed</code> or <code>fail_open</code> exactly.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">WAF audit logs should include:<".to_owned(), "1.6.28");

    assert!(translated.contains("Architecture WAF"));
    assert!(translated.contains("Le support WAF de Fluxheim"));
    assert!(translated.contains("Feature flags"));
    assert!(translated.contains("Les logs d'audit WAF doivent inclure"));
    assert!(translated.contains("fixes comme <code>unknown</code> ou <code>invalid_host</code>"));
    assert!(translated.contains("L'echec du moteur WAF suit"));
    assert!(
        translated.contains("suit exactement <code>fail_closed</code> ou <code>fail_open</code>")
    );
    assert!(unrelated.contains(">WAF audit logs should include:<"));
}

#[test]
fn applies_stable_image_filter_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Image Filter — Fluxheim Source Docs</title>",
        "Image Filter",
        "This module adds safe, bounded image validation and transformation at the edge. It is intended for small static sites, media-heavy origins, and cache-backed deployments that want predictable image variants without adding a separate image service.",
        "Security Requirements",
        "<li>rotate by <code>90</code>, <code>180</code>, or <code>270</code> degrees;</li>",
        "<li>cache transformed variants when <code>cache</code> is enabled;</li>",
        "<li>never cache transformed output when request or response policy says <code>Cache-Control: no-store</code>.</li>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Security Requirements<".to_owned(), "1.6.28");

    assert!(translated.contains("Bildfilter"));
    assert!(translated.contains("sichere, begrenzte Bildvalidierung"));
    assert!(translated.contains("Sicherheitsanforderungen"));
    assert!(translated.contains("oder <code>270</code> Grad"));
    assert!(translated.contains("wenn <code>cache</code> aktiviert ist"));
    assert!(translated.contains("<code>Cache-Control: no-store</code> sagt"));
    assert!(unrelated.contains(">Security Requirements<"));
}

#[test]
fn applies_stable_cloudflare_origin_support_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Cloudflare Origin Support — Fluxheim Source Docs</title>",
        "Cloudflare support is a feasible future optional module, but it should be split into phases. Fluxheim should first treat Cloudflare as a trusted proxy only after the direct peer is verified. Certificate automation and Authenticated Origin Pulls are valuable, but they involve credentials, TLS reload behavior, and security-sensitive trust decisions.",
        "<p>If none of those conditions succeeds, Fluxheim must treat <code>CF-Connecting-IP</code>, <code>CF-Ray</code>, <code>CF-IPCountry</code>, and related headers as untrusted remote input.</p>",
        "<li>record <code>last_success</code>, <code>last_failure</code>, and active range count in metrics and admin status;</li>",
        "<p>Use <code>ArcSwap</code> or equivalent atomic state so new requests see the fresh range set without interrupting active requests.</p>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Origin CA Automation<".to_owned(), "1.6.28");

    assert!(translated.contains("Prise en charge de Cloudflare Origin"));
    assert!(translated.contains("futur module optionnel realisable"));
    assert!(translated.contains("entrees distantes non fiables"));
    assert!(translated.contains("nombre de plages actives dans les metriques"));
    assert!(translated.contains("Utiliser <code>ArcSwap</code>"));
    assert!(unrelated.contains(">Origin CA Automation<"));
}
