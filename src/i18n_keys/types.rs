use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct KeyFile {
    pub(super) locale_id: String,
    pub(super) language: LanguageKeys,
    pub(super) nav: NavKeys,
    pub(super) release: ReleaseKeys,
    pub(super) shell: ShellKeys,
    pub(super) footer: FooterKeys,
    pub(super) home: BTreeMap<String, String>,
    pub(super) docs_index: BTreeMap<String, String>,
    pub(super) common: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct LanguageKeys {
    pub(super) selector_label: String,
    pub(super) english_eu: String,
    pub(super) english_uk: String,
    pub(super) english_us: String,
    pub(super) german: String,
    pub(super) french: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct NavKeys {
    pub(super) docs: String,
    pub(super) download: String,
    pub(super) changelog: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ReleaseKeys {
    pub(super) latest_stable: String,
    pub(super) latest_stable_release: String,
    pub(super) download_version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ShellKeys {
    pub(super) home_title: String,
    pub(super) view_on_github: String,
    pub(super) quick_start: String,
    pub(super) switch_color_theme: String,
    pub(super) links: String,
    pub(super) github_repository: String,
    pub(super) issues: String,
    pub(super) menu: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FooterKeys {
    pub(super) tagline: String,
    pub(super) project: String,
    pub(super) releases: String,
    pub(super) roadmap: String,
    pub(super) community: String,
    pub(super) discussions: String,
    pub(super) eupl_license: String,
    pub(super) valkyoth_org: String,
    pub(super) copyright_prefix: String,
    pub(super) built_with: String,
}
