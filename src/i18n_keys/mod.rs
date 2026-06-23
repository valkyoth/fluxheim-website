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

const KEY_TOML_FILES: &[KeyTomlFile] = &[
    KeyTomlFile {
        root: include_str!("../../config/i18n/keys/en-EU.toml"),
        parts: &[
            include_str!("../../config/i18n/keys/en-EU/10-home-docs-common.toml"),
            include_str!("../../config/i18n/keys/en-EU/20-download-changelog.toml"),
            include_str!("../../config/i18n/keys/en-EU/30-page-groups.toml"),
            include_str!("../../config/i18n/keys/en-EU/40-security-docs.toml"),
            include_str!("../../config/i18n/keys/en-EU/50-platform-recipes.toml"),
            include_str!("../../config/i18n/keys/en-EU/60-source-docs.toml"),
            include_str!("../../config/i18n/keys/en-EU/70-source-docs-extra.toml"),
            include_str!("../../config/i18n/keys/en-EU/80-source-docs-next.toml"),
            include_str!("../../config/i18n/keys/en-EU/90-source-docs-more.toml"),
            include_str!("../../config/i18n/keys/en-EU/100-source-docs-late.toml"),
            include_str!("../../config/i18n/keys/en-EU/110-source-docs-final.toml"),
        ],
    },
    KeyTomlFile {
        root: include_str!("../../config/i18n/keys/en-GB.toml"),
        parts: &[
            include_str!("../../config/i18n/keys/en-GB/10-home-docs-common.toml"),
            include_str!("../../config/i18n/keys/en-GB/20-download-changelog.toml"),
            include_str!("../../config/i18n/keys/en-GB/30-page-groups.toml"),
            include_str!("../../config/i18n/keys/en-GB/40-security-docs.toml"),
            include_str!("../../config/i18n/keys/en-GB/50-platform-recipes.toml"),
            include_str!("../../config/i18n/keys/en-GB/60-source-docs.toml"),
            include_str!("../../config/i18n/keys/en-GB/70-source-docs-extra.toml"),
            include_str!("../../config/i18n/keys/en-GB/80-source-docs-next.toml"),
            include_str!("../../config/i18n/keys/en-GB/90-source-docs-more.toml"),
            include_str!("../../config/i18n/keys/en-GB/100-source-docs-late.toml"),
            include_str!("../../config/i18n/keys/en-GB/110-source-docs-final.toml"),
        ],
    },
    KeyTomlFile {
        root: include_str!("../../config/i18n/keys/en-US.toml"),
        parts: &[
            include_str!("../../config/i18n/keys/en-US/10-home-docs-common.toml"),
            include_str!("../../config/i18n/keys/en-US/20-download-changelog.toml"),
            include_str!("../../config/i18n/keys/en-US/30-page-groups.toml"),
            include_str!("../../config/i18n/keys/en-US/40-security-docs.toml"),
            include_str!("../../config/i18n/keys/en-US/50-platform-recipes.toml"),
            include_str!("../../config/i18n/keys/en-US/60-source-docs.toml"),
            include_str!("../../config/i18n/keys/en-US/70-source-docs-extra.toml"),
            include_str!("../../config/i18n/keys/en-US/80-source-docs-next.toml"),
            include_str!("../../config/i18n/keys/en-US/90-source-docs-more.toml"),
            include_str!("../../config/i18n/keys/en-US/100-source-docs-late.toml"),
            include_str!("../../config/i18n/keys/en-US/110-source-docs-final.toml"),
        ],
    },
    KeyTomlFile {
        root: include_str!("../../config/i18n/keys/de-DE.toml"),
        parts: &[
            include_str!("../../config/i18n/keys/de-DE/10-home-docs-common.toml"),
            include_str!("../../config/i18n/keys/de-DE/20-download-changelog.toml"),
            include_str!("../../config/i18n/keys/de-DE/30-page-groups.toml"),
            include_str!("../../config/i18n/keys/de-DE/40-security-docs.toml"),
            include_str!("../../config/i18n/keys/de-DE/50-platform-recipes.toml"),
            include_str!("../../config/i18n/keys/de-DE/60-source-docs.toml"),
            include_str!("../../config/i18n/keys/de-DE/70-source-docs-extra.toml"),
            include_str!("../../config/i18n/keys/de-DE/80-source-docs-next.toml"),
            include_str!("../../config/i18n/keys/de-DE/90-source-docs-more.toml"),
            include_str!("../../config/i18n/keys/de-DE/100-source-docs-late.toml"),
            include_str!("../../config/i18n/keys/de-DE/110-source-docs-final.toml"),
        ],
    },
    KeyTomlFile {
        root: include_str!("../../config/i18n/keys/fr-FR.toml"),
        parts: &[
            include_str!("../../config/i18n/keys/fr-FR/10-home-docs-common.toml"),
            include_str!("../../config/i18n/keys/fr-FR/20-download-changelog.toml"),
            include_str!("../../config/i18n/keys/fr-FR/30-page-groups.toml"),
            include_str!("../../config/i18n/keys/fr-FR/40-security-docs.toml"),
            include_str!("../../config/i18n/keys/fr-FR/50-platform-recipes.toml"),
            include_str!("../../config/i18n/keys/fr-FR/60-source-docs.toml"),
            include_str!("../../config/i18n/keys/fr-FR/70-source-docs-extra.toml"),
            include_str!("../../config/i18n/keys/fr-FR/80-source-docs-next.toml"),
            include_str!("../../config/i18n/keys/fr-FR/90-source-docs-more.toml"),
            include_str!("../../config/i18n/keys/fr-FR/100-source-docs-late.toml"),
            include_str!("../../config/i18n/keys/fr-FR/110-source-docs-final.toml"),
        ],
    },
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
