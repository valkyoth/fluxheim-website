use super::{apply_shared_keys, language_display_name, language_selector_label};
use crate::content::Site;

#[test]
fn reads_stable_language_keys() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let ch = site.locale("de-CH").expect("Swiss German locale");
    let fr = site.locale("fr-FR").expect("French locale");
    let sv = site.locale("sv-SE").expect("Swedish locale");
    let nb = site.locale("nb-NO").expect("Norwegian locale");
    let nl = site.locale("nl-NL").expect("Dutch locale");
    let fi = site.locale("fi-FI").expect("Finnish locale");
    let is = site.locale("is-IS").expect("Icelandic locale");
    let da = site.locale("da-DK").expect("Danish locale");
    let es = site.locale("es-ES").expect("Spanish locale");
    let pt = site.locale("pt-PT").expect("Portuguese locale");
    let et = site.locale("et-EE").expect("Estonian locale");
    let lv = site.locale("lv-LV").expect("Latvian locale");
    let el = site.locale("el-GR").expect("Greek locale");
    let it = site.locale("it-IT").expect("Italian locale");
    let lt = site.locale("lt-LT").expect("Lithuanian locale");
    let hr = site.locale("hr-HR").expect("Croatian locale");
    let cs = site.locale("cs-CZ").expect("Czech locale");
    let bs = site.locale("bs-BA").expect("Bosnian locale");
    let bg = site.locale("bg-BG").expect("Bulgarian locale");
    let ro = site.locale("ro-RO").expect("Romanian locale");
    let pl = site.locale("pl-PL").expect("Polish locale");
    let ru = site.locale("ru-RU").expect("Russian locale");
    let ja = site.locale("ja-JP").expect("Japanese locale");
    let us = site.locale("en-US").expect("US English locale");

    assert_eq!(language_selector_label(de), "Sprache");
    assert_eq!(language_selector_label(ch), "Sprache");
    assert_eq!(language_selector_label(fr), "Langue");
    assert_eq!(language_selector_label(sv), "Språk");
    assert_eq!(language_selector_label(nb), "Språk");
    assert_eq!(language_selector_label(nl), "Taal");
    assert_eq!(language_selector_label(fi), "Kieli");
    assert_eq!(language_selector_label(is), "Tungumál");
    assert_eq!(language_selector_label(da), "Sprog");
    assert_eq!(language_selector_label(es), "Idioma");
    assert_eq!(language_selector_label(pt), "Idioma");
    assert_eq!(language_selector_label(et), "Keel");
    assert_eq!(language_selector_label(lv), "Valoda");
    assert_eq!(language_selector_label(el), "Γλώσσα");
    assert_eq!(language_selector_label(it), "Lingua");
    assert_eq!(language_selector_label(lt), "Kalba");
    assert_eq!(language_selector_label(hr), "Jezik");
    assert_eq!(language_selector_label(cs), "Jazyk");
    assert_eq!(language_selector_label(bs), "Jezik");
    assert_eq!(language_selector_label(bg), "Език");
    assert_eq!(language_selector_label(ro), "Limbă");
    assert_eq!(language_selector_label(pl), "Język");
    assert_eq!(language_selector_label(ru), "Язык");
    assert_eq!(language_selector_label(ja), "言語");
    assert_eq!(language_selector_label(us), "Language");
    assert_eq!(
        language_display_name(de, "en-US", "fallback"),
        "English (US)"
    );
    assert_eq!(language_display_name(fr, "de-DE", "fallback"), "Deutsch");
}

#[test]
fn language_menu_names_are_autonyms_in_every_locale() {
    let site = Site::load().expect("site loads");
    let expected_names = [
        ("en-EU", "English (EU)"),
        ("en-GB", "English (UK)"),
        ("en-US", "English (US)"),
        ("de-DE", "Deutsch"),
        ("de-CH", "Deutsch (Schweiz)"),
        ("fr-FR", "Français"),
        ("sv-SE", "Svenska"),
        ("nb-NO", "Norsk"),
        ("nl-NL", "Nederlands"),
        ("fi-FI", "Suomi"),
        ("is-IS", "Íslenska"),
        ("da-DK", "Dansk"),
        ("es-ES", "Español"),
        ("pt-PT", "Português"),
        ("et-EE", "Eesti"),
        ("lv-LV", "Latviešu"),
        ("el-GR", "Ελληνικά"),
        ("it-IT", "Italiano"),
        ("lt-LT", "Lietuvių"),
        ("hr-HR", "Hrvatski"),
        ("cs-CZ", "Čeština"),
        ("bs-BA", "Bosanski"),
        ("bg-BG", "Български"),
        ("ro-RO", "Română"),
        ("pl-PL", "Polski"),
        ("ru-RU", "Русский"),
        ("ja-JP", "日本語"),
    ];

    for active_locale in site.locales() {
        for (locale_id, expected_name) in expected_names {
            assert_eq!(
                language_display_name(active_locale, locale_id, "fallback"),
                expected_name,
                "{} should show {locale_id} as its autonym",
                active_locale.locale_id
            );
        }
    }
}

#[test]
fn applies_stable_shared_keys_before_phrase_maps() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let fr = site.locale("fr-FR").expect("French locale");

    let html = ">Docs<>Changelog<>Latest Stable<>Download v1.6.30<".to_owned();
    let de_html = apply_shared_keys(de, html.clone(), "1.6.30");
    let fr_html = apply_shared_keys(fr, html, "1.6.30");

    assert!(de_html.contains("Dokumentation"));
    assert!(de_html.contains("Aktuelle stabile Version"));
    assert!(de_html.contains("Herunterladen v1.6.30"));
    assert!(fr_html.contains("Journal des changements"));
    assert!(fr_html.contains("Dernière version stable"));
    assert!(fr_html.contains("Télécharger v1.6.30"));
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
        r#"<meta name="description" content="A memory-safe edge server and reverse proxy built in Rust.">"#,
        ">Memory-Safe by Design<>Memory-Safe<",
        "Fluxheim ships as focused, modular builds — use only what your deployment needs.",
        "A Rust-native edge runtime with connection pooling, upstream retries, active health checks, HTTP/2, WebSocket upgrades, and gRPC pass-through.",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");

    assert!(translated.contains(">Sûr pour la mémoire par conception<"));
    assert!(translated.contains(">Sûr pour la mémoire<"));
    assert!(translated.contains("Un serveur edge et reverse proxy sûr pour la mémoire"));
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
        ">Translation notice<",
        "Website translations are AI-assisted and maintained on a best-effort basis. ",
        "Corrections and improved translations are welcome in the ",
        r#"<a href="https://github.com/valkyoth/fluxheim-website/tree/main/config/i18n/keys" class="text-cyan-300 hover:text-cyan-200 underline underline-offset-4">Fluxheim website i18n keys</a>."#,
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");

    assert!(translated.contains(">Vollständiger Produktions-Build<"));
    assert!(translated.contains(">Empfohlen<"));
    assert!(translated.contains(r#"alt="Fluxheim-Architekturübersicht""#));
    assert!(translated.contains("Ohne root ausführen."));
    assert!(translated.contains(">Hinweis zu Übersetzungen<"));
    assert!(translated.contains("i18n-Keys der Fluxheim-Website"));
}
