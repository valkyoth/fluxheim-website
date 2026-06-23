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
