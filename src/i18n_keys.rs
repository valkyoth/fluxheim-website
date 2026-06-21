use serde::Deserialize;
use std::sync::OnceLock;

use crate::content::Locale;

const KEY_TOML_FILES: &[&str] = &[
    include_str!("../config/i18n/keys/en-EU.toml"),
    include_str!("../config/i18n/keys/en-GB.toml"),
    include_str!("../config/i18n/keys/en-US.toml"),
    include_str!("../config/i18n/keys/de-DE.toml"),
    include_str!("../config/i18n/keys/fr-FR.toml"),
];

#[derive(Debug, Clone, Deserialize)]
struct KeyFile {
    locale_id: String,
    language: LanguageKeys,
    nav: NavKeys,
    release: ReleaseKeys,
}

#[derive(Debug, Clone, Deserialize)]
struct LanguageKeys {
    selector_label: String,
    english_eu: String,
    english_uk: String,
    english_us: String,
    german: String,
    french: String,
}

#[derive(Debug, Clone, Deserialize)]
struct NavKeys {
    docs: String,
    download: String,
    changelog: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ReleaseKeys {
    latest_stable: String,
    download_version: String,
}

pub fn apply_shared_keys(locale: &Locale, html: String, version: &str) -> String {
    let Some(keys) = locale_keys(&locale.locale_id) else {
        return html;
    };

    html.replace(
        ">Download v1.6.28<",
        &format!(">{}<", versioned(&keys.release.download_version, version)),
    )
    .replace(
        "Latest Stable —",
        &format!("{} —", keys.release.latest_stable),
    )
    .replace(
        ">Latest Stable<",
        &format!(">{}<", keys.release.latest_stable),
    )
    .replace(">Changelog<", &format!(">{}<", keys.nav.changelog))
    .replace(">Download<", &format!(">{}<", keys.nav.download))
    .replace(">Docs<", &format!(">{}<", keys.nav.docs))
}

pub fn language_selector_label(locale: &Locale) -> &str {
    locale_keys(&locale.locale_id)
        .map(|keys| keys.language.selector_label.as_str())
        .unwrap_or("Language")
}

pub fn language_display_name(active_locale: &Locale, locale_id: &str, fallback: &str) -> String {
    let Some(keys) = locale_keys(&active_locale.locale_id) else {
        return fallback.to_owned();
    };

    match locale_id {
        "en-EU" => keys.language.english_eu.clone(),
        "en-GB" => keys.language.english_uk.clone(),
        "en-US" => keys.language.english_us.clone(),
        "de-DE" => keys.language.german.clone(),
        "fr-FR" => keys.language.french.clone(),
        _ => fallback.to_owned(),
    }
}

fn locale_keys(locale_id: &str) -> Option<&'static KeyFile> {
    key_files().iter().find(|keys| keys.locale_id == locale_id)
}

fn versioned(template: &str, version: &str) -> String {
    template.replace("{version}", version)
}

fn key_files() -> &'static [KeyFile] {
    static KEY_FILES: OnceLock<Vec<KeyFile>> = OnceLock::new();
    KEY_FILES.get_or_init(|| {
        KEY_TOML_FILES
            .iter()
            .map(|file| {
                toml::from_str::<KeyFile>(file)
                    .unwrap_or_else(|error| panic!("valid i18n key TOML: {error}"))
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
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
}
