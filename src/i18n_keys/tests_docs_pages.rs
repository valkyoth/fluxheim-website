use crate::{content::Site, i18n_keys::apply_shared_keys};

#[test]
fn applies_stable_docs_guide_keys_to_generated_pages() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let sv = site.locale("sv-SE").expect("Swedish locale");
    let html = concat!(
        "<title>Builds & Features - Fluxheim Docs</title>",
        "<h3>Guides</h3>",
        "<a>Builds & Features</a>",
        "<h2>Common builds</h2>",
        "<h2>Things that cannot go together</h2>",
    )
    .to_owned();

    let de_html = apply_shared_keys(de, html.clone(), "1.6.34");
    let sv_html = apply_shared_keys(sv, html, "1.6.34");

    assert!(de_html.contains("<h3>Anleitungen</h3>"));
    assert!(de_html.contains("<h2>Häufige Builds</h2>"));
    assert!(sv_html.contains("<h3>Guider</h3>"));
    assert!(sv_html.contains("<h2>Vanliga byggen</h2>"));
}
