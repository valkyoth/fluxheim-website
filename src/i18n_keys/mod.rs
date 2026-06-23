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

macro_rules! key_part {
    ($locale:literal, $file:literal) => {
        include_str!(concat!("../../config/i18n/keys/", $locale, "/", $file))
    };
}

macro_rules! key_toml_parts {
    ($locale:literal) => {
        &[
            key_part!($locale, "10-home-docs-common.toml"),
            key_part!($locale, "20-download-changelog.toml"),
            key_part!($locale, "30-page-groups.toml"),
            key_part!($locale, "40-security-docs.toml"),
            key_part!($locale, "50-platform-recipes.toml"),
            key_part!($locale, "60-source-docs.toml"),
            key_part!($locale, "70-source-docs-extra.toml"),
            key_part!($locale, "80-source-docs-next.toml"),
            key_part!($locale, "90-source-docs-more.toml"),
            key_part!($locale, "100-source-docs-late.toml"),
            key_part!($locale, "110-source-docs-final.toml"),
            key_part!($locale, "120-reference.toml"),
            key_part!($locale, "130-source-docs-tail.toml"),
            key_part!($locale, "140-source-docs-tail.toml"),
            key_part!($locale, "150-source-docs-tail.toml"),
            key_part!($locale, "160-source-docs-tail.toml"),
            key_part!($locale, "170-source-docs-tail.toml"),
            key_part!($locale, "180-docs-pages.toml"),
            key_part!($locale, "190-code-comments.toml"),
            key_part!($locale, "200-source-config-reference.toml"),
        ]
    };
}

macro_rules! key_toml_file {
    ($locale:literal) => {
        KeyTomlFile {
            root: include_str!(concat!("../../config/i18n/keys/", $locale, ".toml")),
            parts: key_toml_parts!($locale),
        }
    };
}

const KEY_TOML_FILES: &[KeyTomlFile] = &[
    key_toml_file!("en-EU"),
    key_toml_file!("en-GB"),
    key_toml_file!("en-US"),
    key_toml_file!("de-DE"),
    key_toml_file!("fr-FR"),
];

pub fn apply_shared_keys(locale: &Locale, html: String, version: &str) -> String {
    let Some(keys) = locale_keys(&locale.locale_id) else {
        return html;
    };

    apply::apply_keys(keys, source_keys(), html, version)
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
mod tests_source_docs;

#[cfg(test)]
mod tests_source_docs_late;

#[cfg(test)]
mod tests_source_docs_tail;

#[cfg(test)]
mod tests_docs_pages;

#[cfg(test)]
mod tests_code_comments;

#[cfg(test)]
mod tests_reference;
