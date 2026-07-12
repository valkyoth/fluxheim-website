use crate::{content::Site, i18n_keys::apply_shared_keys};

#[test]
fn applies_stable_docs_guide_keys_to_generated_pages() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let sv = site.locale("sv-SE").expect("Swedish locale");
    let html = concat!(
        "<title>Builds & Features - Fluxheim Docs</title>",
        r#"<a class="sidebar-link">Start</a>"#,
        "<h3>Guides</h3>",
        "<a>Builds & Features</a>",
        "<h2>Common builds</h2>",
        "<h2>Things that cannot go together</h2>",
    )
    .to_owned();

    let de_html = apply_shared_keys(de, html.clone(), "1.6.36");
    let sv_html = apply_shared_keys(sv, html, "1.6.36");

    assert!(de_html.contains("<h3>Anleitungen</h3>"));
    assert!(de_html.contains("<h2>Häufige Builds</h2>"));
    assert!(sv_html.contains("<h3>Guider</h3>"));
    assert!(sv_html.contains("<h2>Vanliga byggen</h2>"));
}

#[test]
fn applies_expanded_docs_keys_only_to_docs_pages() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let paragraph = "Fluxheim 1.7.9 adds runnable migration examples for iRules-style access policy, OpenResty-style header policy, HAProxy/SPOE-style routing, and VCL-style cache policy. These map common policy jobs to Fluxheim's typed ABI; they do not run source code from those products.";
    let strict_host = "For multi-tenant deployments, enable <code>[server.host_routing].strict = true</code> so missing or invalid host identity returns <code>400</code> and unknown hosts return <code>421</code> instead of reaching the default site.";
    let docs =
        format!(r#"<a class="sidebar-link">Start</a><p>{paragraph}</p><p>{strict_host}</p>"#);

    let docs_html = apply_shared_keys(de, docs, "1.7.9");
    assert!(docs_html.contains("ausführbare Migrationsbeispiele"));
    assert!(docs_html.contains("mandantenfähigen Bereitstellungen"));
    assert!(docs_html.contains("<code>[server.host_routing].strict = true</code>"));

    let non_docs = apply_shared_keys(de, format!("<p>{paragraph}</p>"), "1.7.9");
    assert!(non_docs.contains(paragraph));
}
