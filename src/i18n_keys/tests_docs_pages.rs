use crate::{content::Site, i18n_keys::apply_shared_keys};

#[test]
fn applies_stable_configuration_keys_only_on_configuration_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Config Reference — Fluxheim Docs</title>",
        "<h1>Config Reference</h1>",
        "<p>Fluxheim is configured via a single TOML file. Use <code>--check-config</code> to validate before restarting.</p>",
        "<thead><tr><th>Key</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>",
        "<td>Cleartext listener addresses. Each entry is <code>host:port</code>.</td>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(
        de,
        ">Cleartext listener addresses. Each entry is<".to_owned(),
        "1.6.28",
    );

    assert!(translated.contains("Konfigurationsreferenz"));
    assert!(translated.contains("Fluxheim wird über eine einzelne TOML-Datei konfiguriert"));
    assert!(translated.contains("<th>Schlüssel</th>"));
    assert!(translated.contains("<th>Beschreibung</th>"));
    assert!(translated.contains("Klartext-Listener-Adressen"));
    assert!(unrelated.contains(">Cleartext listener addresses. Each entry is<"));
}

#[test]
fn applies_stable_advanced_keys_only_on_advanced_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Advanced — Fluxheim Docs</title>",
        "<h1>Advanced Features</h1>",
        "PHP-FPM, proxy operations, admin API, zero-retention privacy mode, WAF, WASM extensibility, and planned future modules.",
        "<h2>PHP-FPM Bridge</h2>",
        "Available since v1.3.1 via the <code>php-fpm</code> Cargo feature.",
        "<h2>Security — Dependency & Vulnerability Policy</h2>",
    )
    .to_owned();

    let de_html = apply_shared_keys(de, html.clone(), "1.6.28");
    let fr_html = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Advanced Features<".to_owned(), "1.6.28");

    assert!(de_html.contains("<h1>Fortgeschrittene Funktionen</h1>"));
    assert!(de_html.contains("PHP-FPM, Proxy-Operationen, Admin-API"));
    assert!(de_html.contains("<h2>PHP-FPM-Bridge</h2>"));
    assert!(de_html.contains("Verfügbar seit v1.3.1"));
    assert!(fr_html.contains("<h1>Fonctionnalités avancées</h1>"));
    assert!(fr_html.contains("opérations proxy, API admin"));
    assert!(fr_html.contains("<h2>Pont PHP-FPM</h2>"));
    assert!(fr_html.contains("Disponible depuis v1.3.1"));
    assert_eq!(unrelated, ">Advanced Features<");
}
