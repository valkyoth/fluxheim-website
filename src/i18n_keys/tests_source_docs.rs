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
