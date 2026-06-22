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
    shell: ShellKeys,
    footer: FooterKeys,
    home: HomeKeys,
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
    latest_stable_release: String,
    download_version: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ShellKeys {
    home_title: String,
    view_on_github: String,
    quick_start: String,
    switch_color_theme: String,
    links: String,
    github_repository: String,
    issues: String,
    menu: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FooterKeys {
    tagline: String,
    project: String,
    releases: String,
    roadmap: String,
    community: String,
    discussions: String,
    eupl_license: String,
    valkyoth_org: String,
    copyright_prefix: String,
    built_with: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HomeKeys {
    meta_description: String,
    meta_description_secure: String,
    meta_description_short: String,
    hero_memory_safe: String,
    hero_edge_server: String,
    hero_built_in_rust: String,
    hero_line_one: String,
    hero_line_two: String,
    hero_line_three: String,
    tag_fluxheim_core: String,
    tag_macos_dev: String,
    tag_rootless_containers: String,
    features_heading: String,
    features_intro: String,
    memory_safe_by_design: String,
    memory_safe_by_design_text: String,
    fluxheim_http_core: String,
    fluxheim_http_core_text: String,
    load_balancer_control_plane: String,
    load_balancer_control_plane_text: String,
    modular_build_profiles: String,
    modular_build_profiles_text: String,
    tls_managed_acme: String,
    tls_managed_acme_text: String,
    advanced_cache_system: String,
    advanced_cache_system_text: String,
}

pub fn apply_shared_keys(locale: &Locale, html: String, version: &str) -> String {
    let Some(keys) = locale_keys(&locale.locale_id) else {
        return html;
    };

    html.replace(
        "Fluxheim — Memory-Safe Edge Server Built in Rust",
        &keys.shell.home_title,
    )
    .replace("Latest Stable Release", &keys.release.latest_stable_release)
    .replace("Switch color theme", &keys.shell.switch_color_theme)
    .replace(
        ">View on GitHub<",
        &format!(">{}<", keys.shell.view_on_github),
    )
    .replace(">Quick Start<", &format!(">{}<", keys.shell.quick_start))
    .replace(">Links<", &format!(">{}<", keys.shell.links))
    .replace(
        ">GitHub Repository<",
        &format!(">{}<", keys.shell.github_repository),
    )
    .replace(">Issues<", &format!(">{}<", keys.shell.issues))
    .replace(">Menu<", &format!(">{}<", keys.shell.menu))
    .replace(
        "Memory-safe edge server built in Rust. Licensed under EUPL-1.2.",
        &keys.footer.tagline,
    )
    .replace(">Project<", &format!(">{}<", keys.footer.project))
    .replace(">Releases<", &format!(">{}<", keys.footer.releases))
    .replace(">Roadmap<", &format!(">{}<", keys.footer.roadmap))
    .replace(">Community<", &format!(">{}<", keys.footer.community))
    .replace(">Discussions<", &format!(">{}<", keys.footer.discussions))
    .replace(
        ">EUPL-1.2 License<",
        &format!(">{}<", keys.footer.eupl_license),
    )
    .replace(">Valkyoth Org<", &format!(">{}<", keys.footer.valkyoth_org))
    .replace(
        "© 2026 Valkyoth. Distributed under the",
        &keys.footer.copyright_prefix,
    )
    .replace(
        "Built with Rust · Powered by Fluxheim",
        &keys.footer.built_with,
    )
    .replace(
        "Fluxheim is a high-performance, modular web server, reverse proxy and caching server built in Rust.",
        &keys.home.meta_description,
    )
    .replace(
        "Fluxheim is a high-performance, modular web server, reverse proxy and caching server. Written in Rust. Secure by default.",
        &keys.home.meta_description_secure,
    )
    .replace(
        "A memory-safe edge server and reverse proxy built in Rust.",
        &keys.home.meta_description_short,
    )
    .replace(
        "Written in Rust with a pinned stable toolchain. No buffer overflows, no use-after-free, no data races by construction.",
        &keys.home.memory_safe_by_design_text,
    )
    .replace(
        "A Rust-native edge runtime with connection pooling, upstream retries, active health checks, HTTP/2, WebSocket upgrades, and gRPC pass-through.",
        &keys.home.fluxheim_http_core_text,
    )
    .replace(
        "Focused 1.5 load-balancer binary and image with advanced selection, local persistence, health/ejection policy, bounded queueing, and runtime member controls.",
        &keys.home.load_balancer_control_plane_text,
    )
    .replace(
        "Compile only what you need. Focused profiles for static site, cache edge, reverse proxy, load balancing, TCP stream proxying, PHP-FPM, GeoIP, traffic mirroring, and compression-enabled production bundles.",
        &keys.home.modular_build_profiles_text,
    )
    .replace(
        "rustls-first with supported OpenSSL and FIPS/ISO proof build paths, client certificate auth, upstream mTLS, automatic ACME issuance, and multi-cert SNI.",
        &keys.home.tls_managed_acme_text,
    )
    .replace(
        "Memory, disk, tiered, and encrypted cache backends with cache-safe gzip, Zstandard, and Brotli compression plus range caching for large objects.",
        &keys.home.advanced_cache_system_text,
    )
    .replace(
        "Fluxheim ships as focused, modular builds — use only what your deployment needs.",
        &keys.home.features_intro,
    )
    .replace(
        "Modular reverse proxy, cache, load balancer, and static host written",
        &keys.home.hero_line_one,
    )
    .replace(
        "in Rust. Secure by default with TLS, ACME, compression, edge policy,",
        &keys.home.hero_line_two,
    )
    .replace(
        "dynamic upstream discovery, and safe traffic mirroring built in.",
        &keys.home.hero_line_three,
    )
    .replace(
        ">Everything You Need at the Edge<",
        &format!(">{}<", keys.home.features_heading),
    )
    .replace(
        ">Memory-Safe by Design<",
        &format!(">{}<", keys.home.memory_safe_by_design),
    )
    .replace(
        ">Fluxheim HTTP Core<",
        &format!(">{}<", keys.home.fluxheim_http_core),
    )
    .replace(
        ">Load Balancer Control Plane<",
        &format!(">{}<", keys.home.load_balancer_control_plane),
    )
    .replace(
        ">Modular Build Profiles<",
        &format!(">{}<", keys.home.modular_build_profiles),
    )
    .replace(
        ">TLS & Managed ACME<",
        &format!(">{}<", keys.home.tls_managed_acme),
    )
    .replace(
        ">Advanced Cache System<",
        &format!(">{}<", keys.home.advanced_cache_system),
    )
    .replace(">Memory-Safe<", &format!(">{}<", keys.home.hero_memory_safe))
    .replace(">Edge Server<", &format!(">{}<", keys.home.hero_edge_server))
    .replace(
        ">Built in Rust<",
        &format!(">{}<", keys.home.hero_built_in_rust),
    )
    .replace(
        ">Fluxheim Core<",
        &format!(">{}<", keys.home.tag_fluxheim_core),
    )
    .replace(">macOS Dev<", &format!(">{}<", keys.home.tag_macos_dev))
    .replace(
        ">Rootless Containers<",
        &format!(">{}<", keys.home.tag_rootless_containers),
    )
    .replace(
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
}
