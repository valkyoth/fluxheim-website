mod apply;
mod page_maps;
mod text_replace;
mod types;

use std::sync::OnceLock;

use crate::content::Locale;

use types::KeyFile;

struct KeyTomlFile {
    root: &'static str,
    parts: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/i18n_key_files.rs"));

pub fn apply_shared_keys(locale: &Locale, html: String, version: &str) -> String {
    let Some(keys) = locale_keys(&locale.locale_id) else {
        return html;
    };

    apply::apply_keys(keys, source_keys(), html, version)
}

pub(crate) fn clear_replacer_cache() {
    text_replace::clear_replacer_cache();
}

#[cfg(test)]
pub(crate) fn replacer_cache_len() -> usize {
    text_replace::replacer_cache_len()
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

    keys.language
        .names
        .get(locale_id)
        .cloned()
        .unwrap_or_else(|| fallback.to_owned())
}

pub fn common_text(locale: &Locale, key: &str, fallback: &str) -> String {
    locale_keys(&locale.locale_id)
        .and_then(|keys| keys.common.get(key))
        .cloned()
        .unwrap_or_else(|| fallback.to_owned())
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
                let mut toml = file.root.to_owned();
                for part in file.parts {
                    toml.push('\n');
                    toml.push_str(part);
                }
                toml::from_str::<KeyFile>(&toml)
                    .unwrap_or_else(|error| panic!("valid i18n key TOML: {error}"))
            })
            .collect()
    })
}

fn source_keys() -> &'static KeyFile {
    key_files()
        .iter()
        .find(|keys| keys.locale_id == "en-EU")
        .expect("en-EU i18n keys exist")
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_shared;

#[cfg(test)]
mod tests_docs_pages;

#[cfg(test)]
mod tests_code_comments;

#[cfg(test)]
mod tests_reference;
