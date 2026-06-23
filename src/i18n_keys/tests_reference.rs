use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_reference_keys_only_on_reference_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Source Reference — Fluxheim Docs</title>",
        "The complete upstream Markdown documentation from Fluxheim main is vendored here so the website exposes the full operator, architecture, and roadmap reference set.",
        "source files",
        "Operations And Deployment",
        "Build And Rootless Podman",
        "Repository And Release Templates",
        "Runtime Compatibility Patch",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Operations And Deployment<".to_owned(), "1.6.28");

    assert!(translated.contains("vollständige Upstream-Markdown-Dokumentation"));
    assert!(translated.contains("Source-Dateien"));
    assert!(translated.contains("Betrieb und Deployment"));
    assert!(translated.contains("Build und Rootless Podman"));
    assert!(translated.contains("Repository- und Release-Vorlagen"));
    assert!(translated.contains("Runtime-Kompatibilitäts-Patch"));
    assert!(unrelated.contains(">Operations And Deployment<"));
}
