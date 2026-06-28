use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_reference_keys_only_on_reference_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Full Reference - Fluxheim Docs</title>",
        "<h3>Guides</h3>",
        "<a>Full Reference</a>",
        "<h2>Where the deep docs live</h2>",
        "<a>Full docs on GitHub</a>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.32");
    let unrelated = apply_shared_keys(de, ">Full Reference<".to_owned(), "1.6.32");

    assert!(translated.contains("<h3>Anleitungen</h3>"));
    assert!(translated.contains(">Vollständige Referenz<"));
    assert!(translated.contains("<h2>Wo die Detaildokumentation liegt</h2>"));
    assert_eq!(unrelated, ">Full Reference<");
}
