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
fn applies_stable_build_and_podman_runtime_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Build And Rootless Podman — Fluxheim Source Docs</title>",
        "Run every runtime variant smoke:",
        "Example host layout:",
        "Runtime images and RPMs do include <code>fluxheim-acme</code>, which is the ACME companion entry point for service-manager or container-scheduled renewal workflows:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Example host layout:<".to_owned(), "1.6.28");

    assert!(translated.contains("Jeden Runtime-Varianten-Smoke ausfuehren:"));
    assert!(translated.contains("Beispiel-Host-Layout:"));
    assert!(translated.contains("Runtime-Images und RPMs enthalten"));
    assert!(unrelated.contains(">Example host layout:<"));
}

#[test]
fn applies_stable_build_and_podman_final_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Build And Rootless Podman — Fluxheim Source Docs</title>",
        "For HTTP-01 ACME, the CA must be able to reach Fluxheim on public port",
        "Published images default to the rootless-friendly",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(
        de,
        "For HTTP-01 ACME, the CA must be able to reach Fluxheim on public port".to_owned(),
        "1.6.28",
    );

    assert!(translated.contains("Fuer HTTP-01 ACME muss die CA"));
    assert!(translated.contains("Veroeffentlichte Images verwenden"));
    assert!(unrelated.contains("For HTTP-01 ACME"));
}

#[test]
fn applies_stable_build_and_podman_builds_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Build And Rootless Podman — Fluxheim Source Docs</title>",
        "For FIPS/ISO-capable OpenSSL testing, build with",
        "By default, the bundled Containerfiles compile the full production image profile:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(
        de,
        "For FIPS/ISO-capable OpenSSL testing, build with".to_owned(),
        "1.6.28",
    );

    assert!(translated.contains("Fuer FIPS-/ISO-faehige OpenSSL-Tests"));
    assert!(translated.contains("vollstaendige Produktions-Image-Profil"));
    assert!(unrelated.contains("For FIPS/ISO-capable OpenSSL"));
}

#[test]
fn applies_stable_build_and_podman_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Build And Rootless Podman — Fluxheim Source Docs</title>",
        "Container Variants",
        "Codex And Rootless Podman",
        "Fluxheim ships multiple runtime Containerfiles so operators can choose the base OS that fits their security and operations model.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Container Variants<".to_owned(), "1.6.28");

    assert!(translated.contains("Container-Varianten"));
    assert!(translated.contains("Codex und Rootless Podman"));
    assert!(translated.contains("Fluxheim liefert mehrere Runtime-Containerfiles"));
    assert!(unrelated.contains(">Container Variants<"));
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
        "Current Controls",
        "Build Scripts And Procedural Macros",
        "Accepted Limitations",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Current Controls<".to_owned(), "1.6.28");

    assert!(translated.contains("Aktuelle Kontrollen"));
    assert!(translated.contains("Build-Scripts und prozedurale Makros"));
    assert!(translated.contains("Akzeptierte Einschraenkungen"));
    assert!(unrelated.contains(">Current Controls<"));
}

#[test]
fn applies_stable_compression_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Compression — Fluxheim Source Docs</title>",
        "Goals",
        "Privacy And Security",
        "Cache Integration",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Goals<".to_owned(), "1.6.28");

    assert!(translated.contains("Ziele"));
    assert!(translated.contains("Datenschutz und Sicherheit"));
    assert!(translated.contains("Cache-Integration"));
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
