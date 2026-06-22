mod apply;
mod types;

use std::sync::OnceLock;

use crate::content::Locale;

use types::KeyFile;

const KEY_TOML_FILES: &[&str] = &[
    include_str!("../../config/i18n/keys/en-EU.toml"),
    include_str!("../../config/i18n/keys/en-GB.toml"),
    include_str!("../../config/i18n/keys/en-US.toml"),
    include_str!("../../config/i18n/keys/de-DE.toml"),
    include_str!("../../config/i18n/keys/fr-FR.toml"),
];

pub fn apply_shared_keys(locale: &Locale, html: String, version: &str) -> String {
    let Some(keys) = locale_keys(&locale.locale_id) else {
        return html;
    };

    apply::apply_keys(keys, html, version)
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

fn home<'a>(keys: &'a KeyFile, name: &str) -> &'a str {
    keys.home
        .get(name)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("home i18n key exists: {name}"))
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
mod tests;
