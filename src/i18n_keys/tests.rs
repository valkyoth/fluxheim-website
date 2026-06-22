use super::{apply_shared_keys, language_display_name, language_selector_label};
use crate::content::Site;

#[test]
fn reads_stable_language_keys() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let fr = site.locale("fr-FR").expect("French locale");
    let us = site.locale("en-US").expect("US English locale");

    assert_eq!(language_selector_label(de), "Sprache");
    assert_eq!(language_selector_label(fr), "Langue");
    assert_eq!(language_selector_label(us), "Language");
    assert_eq!(
        language_display_name(de, "en-US", "fallback"),
        "English (US)"
    );
    assert_eq!(language_display_name(fr, "de-DE", "fallback"), "Deutsch");
}

#[test]
fn applies_stable_shared_keys_before_phrase_maps() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let fr = site.locale("fr-FR").expect("French locale");

    let html = ">Docs<>Changelog<>Latest Stable<>Download v1.6.28<".to_owned();
    let de_html = apply_shared_keys(de, html.clone(), "1.6.28");
    let fr_html = apply_shared_keys(fr, html, "1.6.28");

    assert!(de_html.contains("Dokumentation"));
    assert!(de_html.contains("Aktuelle stabile Version"));
    assert!(de_html.contains("Herunterladen v1.6.28"));
    assert!(fr_html.contains("Journal des changements"));
    assert!(fr_html.contains("Dernière version stable"));
    assert!(fr_html.contains("Télécharger v1.6.28"));
}

#[test]
fn applies_stable_shell_and_footer_keys() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "Fluxheim — Memory-Safe Edge Server Built in Rust",
        ">View on GitHub<>Quick Start<>GitHub Repository<",
        ">Project<>Community<>EUPL-1.2 License<",
        "Memory-safe edge server built in Rust. Licensed under EUPL-1.2.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");

    assert!(translated.contains("Speichersicherer Edge-Server"));
    assert!(translated.contains(">Auf GitHub ansehen<"));
    assert!(translated.contains(">Projekt<"));
    assert!(translated.contains(">EUPL-1.2-Lizenz<"));
}

#[test]
fn applies_stable_home_keys_before_phrase_maps() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        ">Memory-Safe by Design<>Memory-Safe<",
        "Fluxheim ships as focused, modular builds — use only what your deployment needs.",
        "A Rust-native edge runtime with connection pooling, upstream retries, active health checks, HTTP/2, WebSocket upgrades, and gRPC pass-through.",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");

    assert!(translated.contains(">Sûr pour la mémoire par conception<"));
    assert!(translated.contains(">Sûr pour la mémoire<"));
    assert!(translated.contains("builds ciblés et modulaires"));
    assert!(translated.contains("runtime edge natif Rust"));
}

#[test]
fn applies_stable_common_keys_for_text_and_attributes() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        ">Full Production Build<>Recommended<",
        r#"<img alt="Fluxheim architecture overview">"#,
        "Run without root. Internal ports 8080/8443 by default. ",
        "Explicit runtime images for different operational policies.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");

    assert!(translated.contains(">Vollständiger Produktions-Build<"));
    assert!(translated.contains(">Empfohlen<"));
    assert!(translated.contains(r#"alt="Fluxheim-Architekturübersicht""#));
    assert!(translated.contains("Ohne root ausführen."));
}

#[test]
fn applies_stable_download_keys_for_release_page_copy() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Download — Fluxheim</title>",
        ">Cache Edge Build<",
        "<span class=\"text-xs font-bold uppercase tracking-widest text-amber-400\">Cache</span>",
        "Native HTTP/1.1 upstream pooling release with bounded keepalive reuse, ",
        "pool-size config, upstream idle timeout handling, conservative no-reuse guards, ",
        "and real socket reuse/expiry tests.",
        "proxy.error_pages</code> fallback pages backed by <code>fluxheim-web",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");

    assert!(translated.contains(">Cache-Edge-Build<"));
    assert!(translated.contains(">Cache-Profil</span>"));
    assert!(translated.contains("Native HTTP/1.1-Upstream-Pooling-Version"));
    assert!(translated.contains("proxy.error_pages</code> Fallback-Seiten, gestützt durch"));
}

#[test]
fn applies_stable_changelog_keys_only_on_changelog_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Changelog — Fluxheim</title>",
        "Released June 19, 2026",
        "Moves route-level native response compression",
        "Adds explicit <code>pingora-compat</code> ",
        "feature gating for the remaining compatibility runtime boundary",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, "Released June 19, 2026".to_owned(), "1.6.28");

    assert!(translated.contains("<title>Änderungen — Fluxheim</title>"));
    assert!(translated.contains("Veröffentlicht am 19. Juni 2026"));
    assert!(translated.contains("Verschiebt route-level native response compression"));
    assert!(translated.contains("Fügt explizites <code>pingora-compat</code> Feature-Gating"));
    assert_eq!(unrelated, "Released June 19, 2026");
}

#[test]
fn applies_stable_runtime_parity_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Runtime Parity Fixtures — Fluxheim Source Docs</title>",
        "<h1>Runtime Parity Fixtures</h1>",
        "The machine-readable inventory is:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Runtime Parity Fixtures<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Runtime-Parity-Fixtures</h1>"));
    assert!(translated.contains("Das maschinenlesbare Inventar ist:"));
    assert_eq!(unrelated, ">Runtime Parity Fixtures<");
}

#[test]
fn applies_stable_geoip_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>GeoIP / Geo-Context — Fluxheim Source Docs</title>",
        "<h2>Local Databases</h2>",
        "Fluxheim <code>1.4.5</code> adds a bounded optional <code>geoip</code> ",
        "feature. It is a local Geo-Context foundation for access policy, ",
        "not a dynamic downloader or programmable geo engine.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Local Databases<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Lokale Datenbanken</h2>"));
    assert!(
        translated.contains("fuegt ein begrenztes optionales <code>geoip</code> Feature hinzu")
    );
    assert_eq!(unrelated, ">Local Databases<");
}

#[test]
fn applies_stable_load_balancer_ha_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Load Balancer HA Design Notes — Fluxheim Source Docs</title>",
        "<h1>Load Balancer HA Design Notes</h1>",
        "Current 1.5.3 Behavior",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Load Balancer HA Design Notes<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Load-Balancer-HA-Designhinweise</h1>"));
    assert!(translated.contains("Aktuelles 1.5.3-Verhalten"));
    assert_eq!(unrelated, ">Load Balancer HA Design Notes<");
}

#[test]
fn applies_stable_getting_started_keys_only_on_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Installation & Quick Start — Fluxheim Docs</title>",
        "<h2>Prerequisites</h2>",
        "<th>Profile</th>",
        "Get Fluxheim running in under five minutes — from tarball, container, or source.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Prerequisites<>Profile<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Voraussetzungen</h2>"));
    assert!(translated.contains("<th>Profil</th>"));
    assert!(translated.contains("Starte Fluxheim in unter fünf Minuten"));
    assert_eq!(unrelated, ">Prerequisites<>Profile<");
}

#[test]
fn applies_stable_cache_keys_only_on_cache_docs_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Cache System — Fluxheim Docs</title>",
        "<h2>Enabling Cache</h2>",
        "<h3>Memory Cache</h3>",
        "Fluxheim's cache system supports memory, disk, tiered, and encrypted backends ",
        "with route-scoped policies, cache locks, stale serving, distributed peer fill, ",
        "and range caching.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Enabling Cache<>Memory Cache<".to_owned(), "1.6.28");

    assert!(translated.contains("<h2>Cache aktivieren</h2>"));
    assert!(translated.contains("<h3>Memory-Cache</h3>"));
    assert!(translated.contains("Fluxheims Cache-System unterstützt"));
    assert_eq!(unrelated, ">Enabling Cache<>Memory Cache<");
}

#[test]
fn applies_stable_extraction_dependency_graph_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Extraction Dependency Graph — Fluxheim Source Docs</title>",
        "<h1>Extraction Dependency Graph</h1>",
        "<h2>Dependency Direction</h2>",
        "Target dependency direction:",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Extraction Dependency Graph<".to_owned(), "1.6.28");

    assert!(translated.contains("<h1>Extraktions-Abhaengigkeitsgraph</h1>"));
    assert!(translated.contains("<h2>Abhaengigkeitsrichtung</h2>"));
    assert!(translated.contains("Zielrichtung der Abhaengigkeiten:"));
    assert_eq!(unrelated, ">Extraction Dependency Graph<");
}
