use super::apply_shared_keys;
use crate::content::Site;

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
