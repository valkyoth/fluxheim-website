use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_download_keys_for_release_page_copy() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Download — Fluxheim</title>",
        ">Cache Edge Build<",
        "<span class=\"text-xs font-bold uppercase tracking-widest text-amber-400\">Cache</span>",
        "Latest series",
        "Native-runtime cutover line: Pingora-exit foundations, Fluxheim-owned HTTP/1 ",
        "and HTTP/2 paths, native TLS/listener previews, route proxy/static-web parity, ",
        "compression and error pages, forwarded-header policy, auth-request, traffic mirroring, ",
        "rate limits, gRPC validation, pooled upstream HTTP/2, and hardened runtime evidence.",
        "proxy.error_pages</code> fallback pages backed by <code>fluxheim-web",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.32");

    assert!(translated.contains(">Cache-Edge-Build<"));
    assert!(translated.contains(">Cache-Profil</span>"));
    assert!(translated.contains("Neueste Reihe"));
    assert!(translated.contains("Native-Runtime-Cutover-Reihe"));
    assert!(translated.contains("Fluxheim-eigene HTTP/1- und HTTP/2-Pfade"));
    assert!(translated.contains("proxy.error_pages</code> Fallback-Seiten, gestützt durch"));
}

#[test]
fn applies_stable_changelog_keys_only_on_changelog_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Changelog — Fluxheim</title>",
        "Released June 23, 2026",
        "Moves plaintext upstream HTTP/2 forwarding into the native HTTP/1 proxy path ",
        "for h2c/prior-knowledge origins",
        "Adds pooled native upstream H2 connections with bounded stream capacity ",
        "and safe-method retry after pre-response pooled-handle failure",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.32");
    let unrelated = apply_shared_keys(de, "Released June 23, 2026".to_owned(), "1.6.32");

    assert!(translated.contains("<title>Änderungen — Fluxheim</title>"));
    assert!(translated.contains("Veröffentlicht am 23. Juni 2026"));
    assert!(translated.contains("Verschiebt Plaintext-Upstream-HTTP/2-Forwarding"));
    assert_eq!(unrelated, "Released June 23, 2026");
}

#[test]
fn applies_public_docs_guide_keys_on_docs_pages() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>PHP-FPM - Fluxheim Docs</title>",
        "<h3>Guides</h3>",
        "<a>Static Sites</a>",
        "<a>Future Modules</a>",
        "<h2>External PHP-FPM pool</h2>",
        "<h2>Common checks</h2>",
    )
    .to_owned();

    let de_html = apply_shared_keys(de, html.clone(), "1.6.32");
    let fr_html = apply_shared_keys(fr, html, "1.6.32");
    let unrelated = apply_shared_keys(de, ">External PHP-FPM pool<".to_owned(), "1.6.32");

    assert!(de_html.contains("<h3>Anleitungen</h3>"));
    assert!(de_html.contains(">Statische Sites<"));
    assert!(de_html.contains(">Zukünftige Module<"));
    assert!(de_html.contains("<h2>Externer PHP-FPM-Pool</h2>"));
    assert!(de_html.contains("<h2>Häufige Prüfungen</h2>"));
    assert!(fr_html.contains("<h3>Guides</h3>"));
    assert!(fr_html.contains(">Sites statiques<"));
    assert!(fr_html.contains(">Modules futurs<"));
    assert!(fr_html.contains("<h2>Pool PHP-FPM externe</h2>"));
    assert!(fr_html.contains("<h2>Vérifications courantes</h2>"));
    assert_eq!(unrelated, ">External PHP-FPM pool<");
}
