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
