use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

const SITE_TOML: &str = include_str!("../config/site.toml");
const LOCALES_TOML: &str = include_str!("../config/locales.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfig {
    pub project_name: String,
    pub fluxheim_version: String,
    pub github_url: String,
    pub releases_url: String,
    pub license: String,
    pub default_locale: String,
}

#[derive(Debug, Clone)]
pub struct Site {
    pub config: SiteConfig,
    locales: Vec<Locale>,
    prefixes: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Locale {
    pub locale_id: String,
    pub html_lang: String,
    pub url_prefix: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageLink {
    pub locale_id: String,
    pub display_name: String,
    pub href: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
struct LocaleConfig {
    locales: Vec<Locale>,
}

impl Site {
    pub fn load() -> Result<Self, String> {
        let config: SiteConfig = toml::from_str(SITE_TOML).map_err(|e| e.to_string())?;
        let locale_config: LocaleConfig =
            toml::from_str(LOCALES_TOML).map_err(|e| e.to_string())?;
        let mut prefixes = BTreeMap::new();

        for (index, locale) in locale_config.locales.iter().enumerate() {
            if prefixes
                .insert(locale.url_prefix.to_lowercase(), index)
                .is_some()
            {
                return Err(format!("duplicate locale prefix {}", locale.url_prefix));
            }
        }

        let site = Self {
            config,
            locales: locale_config.locales,
            prefixes,
        };
        site.validate()?;
        Ok(site)
    }

    pub fn default_locale(&self) -> &Locale {
        self.locale(&self.config.default_locale)
            .expect("validated default locale")
    }

    pub fn locale(&self, locale_id: &str) -> Option<&Locale> {
        self.locales
            .iter()
            .find(|locale| locale.locale_id == locale_id)
    }

    pub fn locales(&self) -> impl Iterator<Item = &Locale> {
        self.locales.iter()
    }

    pub fn split_path<'a>(&'a self, clean_path: &'a str) -> (&'a Locale, &'a str) {
        let mut parts = clean_path.splitn(2, '/');
        let first = parts.next().unwrap_or_default().to_lowercase();
        let rest = parts.next().unwrap_or_default();

        if let Some(index) = self.prefixes.get(&first)
            && let Some(locale) = self.locales.get(*index)
        {
            return (locale, rest);
        }

        (self.default_locale(), clean_path)
    }

    pub fn language_links(&self, active_locale_id: &str, slug: &str) -> Vec<LanguageLink> {
        self.locales()
            .map(|locale| LanguageLink {
                locale_id: locale.locale_id.clone(),
                display_name: locale.display_name.clone(),
                href: self.path_for(locale, slug),
                active: locale.locale_id == active_locale_id,
            })
            .collect()
    }

    pub fn path_for(&self, locale: &Locale, slug: &str) -> String {
        let slug = slug.trim_matches('/');
        if locale.locale_id == self.config.default_locale {
            return if slug.is_empty() {
                "/".to_owned()
            } else {
                format!("/{slug}")
            };
        }

        if slug.is_empty() {
            format!("/{}/", locale.url_prefix)
        } else {
            format!("/{}/{slug}", locale.url_prefix)
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.locales.is_empty() {
            return Err("at least one locale is required".to_owned());
        }

        let mut locale_ids = BTreeSet::new();
        for locale in &self.locales {
            if locale.locale_id.trim().is_empty()
                || locale.html_lang.trim().is_empty()
                || locale.url_prefix.trim().is_empty()
                || locale.display_name.trim().is_empty()
            {
                return Err("locale fields must not be empty".to_owned());
            }

            if !locale_ids.insert(&locale.locale_id) {
                return Err(format!("duplicate locale id {}", locale.locale_id));
            }

            if locale.url_prefix != locale.url_prefix.to_lowercase()
                || locale.url_prefix.contains('/')
                || locale.url_prefix.contains(char::is_whitespace)
            {
                return Err(format!("invalid locale prefix {}", locale.url_prefix));
            }
        }

        if self.locale(&self.config.default_locale).is_none() {
            return Err("default locale is missing from locale config".to_owned());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Site;

    #[test]
    fn loads_configured_locales() {
        let site = Site::load().expect("site content loads");
        assert!(site.locale("en-EU").is_some());
        assert!(site.locale("en-GB").is_some());
        assert!(site.locale("en-US").is_some());
        assert!(site.locale("de-DE").is_some());
        assert!(site.locale("fr-FR").is_some());
        assert!(site.locale("sv-SE").is_some());
        assert!(site.locale("nb-NO").is_some());
        assert!(site.locale("nl-NL").is_some());
        assert!(site.locale("fi-FI").is_some());
        assert!(site.locale("is-IS").is_some());
        assert!(site.locale("da-DK").is_some());
    }

    #[test]
    fn splits_default_and_localized_routes() {
        let site = Site::load().expect("site content loads");
        assert_eq!(site.split_path("download").0.locale_id, "en-EU");
        assert_eq!(site.split_path("en-gb/download").0.locale_id, "en-GB");
        assert_eq!(site.split_path("en-us/download").0.locale_id, "en-US");
        assert_eq!(site.split_path("de/download").0.locale_id, "de-DE");
        assert_eq!(site.split_path("fr/docs/cache").0.locale_id, "fr-FR");
        assert_eq!(site.split_path("sv/docs/cache").0.locale_id, "sv-SE");
        assert_eq!(site.split_path("no/docs/cache").0.locale_id, "nb-NO");
        assert_eq!(site.split_path("nl/docs/cache").0.locale_id, "nl-NL");
        assert_eq!(site.split_path("fi/docs/cache").0.locale_id, "fi-FI");
        assert_eq!(site.split_path("is/docs/cache").0.locale_id, "is-IS");
        assert_eq!(site.split_path("da/docs/cache").0.locale_id, "da-DK");
    }

    #[test]
    fn builds_language_links_for_same_page() {
        let site = Site::load().expect("site content loads");
        let links = site.language_links("de-DE", "download");
        assert_eq!(links.len(), site.locales().count());
        assert!(links.iter().any(|link| link.href == "/download"));
        assert!(links.iter().any(|link| link.href == "/en-gb/download"));
        assert!(links.iter().any(|link| link.href == "/en-us/download"));
        assert!(links.iter().any(|link| link.href == "/de/download"));
        assert!(links.iter().any(|link| link.href == "/fr/download"));
        assert!(links.iter().any(|link| link.href == "/sv/download"));
        assert!(links.iter().any(|link| link.href == "/no/download"));
        assert!(links.iter().any(|link| link.href == "/nl/download"));
        assert!(links.iter().any(|link| link.href == "/fi/download"));
        assert!(links.iter().any(|link| link.href == "/is/download"));
        assert!(links.iter().any(|link| link.href == "/da/download"));
    }
}
