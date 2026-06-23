use serde::Deserialize;
use std::{cmp::Reverse, sync::OnceLock};

use crate::content::Locale;

const DE_TOML_FILES: &[&str] = &[
    include_str!("../config/i18n-de.toml"),
    include_str!("../config/i18n/de/code-comments.toml"),
    include_str!("../config/i18n/de/release-updates.toml"),
    include_str!("../config/i18n/de/download.toml"),
    include_str!("../config/i18n/de/changelog.toml"),
    include_str!("../config/i18n/de/docs-index.toml"),
    include_str!("../config/i18n/de/getting-started.toml"),
    include_str!("../config/i18n/de/configuration.toml"),
    include_str!("../config/i18n/de/features.toml"),
    include_str!("../config/i18n/de/deployment.toml"),
    include_str!("../config/i18n/de/tls-acme.toml"),
    include_str!("../config/i18n/de/cache.toml"),
    include_str!("../config/i18n/de/observability.toml"),
    include_str!("../config/i18n/de/advanced.toml"),
    include_str!("../config/i18n/de/reference.toml"),
    include_str!("../config/i18n/de/load-balancer-migration.toml"),
    include_str!("../config/i18n/de/load-balancer-ha.toml"),
    include_str!("../config/i18n/de/build-and-podman.toml"),
    include_str!("../config/i18n/de/build-and-podman-runtime.toml"),
    include_str!("../config/i18n/de/build-and-podman-builds.toml"),
    include_str!("../config/i18n/de/build-and-podman-final.toml"),
    include_str!("../config/i18n/de/systemd.toml"),
    include_str!("../config/i18n/de/production-readiness.toml"),
    include_str!("../config/i18n/de/compression.toml"),
    include_str!("../config/i18n/de/vhost-config.toml"),
    include_str!("../config/i18n/de/config-snapshots.toml"),
    include_str!("../config/i18n/de/supply-chain-security.toml"),
    include_str!("../config/i18n/de/github-setup.toml"),
    include_str!("../config/i18n/de/geoip.toml"),
    include_str!("../config/i18n/de/macos-development.toml"),
    include_str!("../config/i18n/de/logging-architecture.toml"),
    include_str!("../config/i18n/de/metrics-architecture.toml"),
    include_str!("../config/i18n/de/legacy-static-http.toml"),
    include_str!("../config/i18n/de/auth-request.toml"),
    include_str!("../config/i18n/de/secure-links.toml"),
    include_str!("../config/i18n/de/release-checklist.toml"),
    include_str!("../config/i18n/de/release-notes-template.toml"),
    include_str!("../config/i18n/de/release-runbook.toml"),
    include_str!("../config/i18n/de/compliance-evidence-template.toml"),
    include_str!("../config/i18n/de/runtime-baseline.toml"),
    include_str!("../config/i18n/de/cloudflare-origin-support.toml"),
    include_str!("../config/i18n/de/cache-encryption.toml"),
    include_str!("../config/i18n/de/runtime-parity-fixtures.toml"),
    include_str!("../config/i18n/de/pingora-core-patch.toml"),
    include_str!("../config/i18n/de/owasp-top10-2025-baseline.toml"),
    include_str!("../config/i18n/de/extraction-dependency-graph.toml"),
    include_str!("../config/i18n/de/modularity-policy.toml"),
    include_str!("../config/i18n/de/modularity-exceptions.toml"),
    include_str!("../config/i18n/de/fluxheim-ecosystem-idea.toml"),
    include_str!("../config/i18n/de/runtime-facts-and-policy-proofs.toml"),
    include_str!("../config/i18n/de/perl-cgi-support.toml"),
    include_str!("../config/i18n/de/wasm-extensibility.toml"),
    include_str!("../config/i18n/de/image-filter.toml"),
    include_str!("../config/i18n/de/source-features.toml"),
    include_str!("../config/i18n/de/cache-backends.toml"),
    include_str!("../config/i18n/de/certificate-renewal.toml"),
    include_str!("../config/i18n/de/common-criteria-roadmap.toml"),
    include_str!("../config/i18n/de/config-reference.toml"),
    include_str!("../config/i18n/de/crypto-rpc-edge.toml"),
    include_str!("../config/i18n/de/versioning-plan.toml"),
    include_str!("../config/i18n/de/gateway-recipes.toml"),
    include_str!("../config/i18n/de/opentelemetry-tracing.toml"),
    include_str!("../config/i18n/de/php-fpm-app-recipes.toml"),
    include_str!("../config/i18n/de/php-runtime-support.toml"),
    include_str!("../config/i18n/de/sentinel-mesh.toml"),
    include_str!("../config/i18n/de/programmable-media-edge.toml"),
    include_str!("../config/i18n/de/waf-architecture.toml"),
    include_str!("../config/i18n/de/fips.toml"),
    include_str!("../config/i18n/de/zero-retention-privacy-mode.toml"),
];
const FR_TOML_FILES: &[&str] = &[
    include_str!("../config/i18n-fr.toml"),
    include_str!("../config/i18n/fr/code-comments.toml"),
    include_str!("../config/i18n/fr/release-updates.toml"),
    include_str!("../config/i18n/fr/download.toml"),
    include_str!("../config/i18n/fr/changelog.toml"),
    include_str!("../config/i18n/fr/docs-index.toml"),
    include_str!("../config/i18n/fr/getting-started.toml"),
    include_str!("../config/i18n/fr/configuration.toml"),
    include_str!("../config/i18n/fr/features.toml"),
    include_str!("../config/i18n/fr/deployment.toml"),
    include_str!("../config/i18n/fr/tls-acme.toml"),
    include_str!("../config/i18n/fr/cache.toml"),
    include_str!("../config/i18n/fr/observability.toml"),
    include_str!("../config/i18n/fr/advanced.toml"),
    include_str!("../config/i18n/fr/reference.toml"),
    include_str!("../config/i18n/fr/load-balancer-migration.toml"),
    include_str!("../config/i18n/fr/load-balancer-ha.toml"),
    include_str!("../config/i18n/fr/build-and-podman.toml"),
    include_str!("../config/i18n/fr/build-and-podman-runtime.toml"),
    include_str!("../config/i18n/fr/build-and-podman-builds.toml"),
    include_str!("../config/i18n/fr/build-and-podman-final.toml"),
    include_str!("../config/i18n/fr/systemd.toml"),
    include_str!("../config/i18n/fr/production-readiness.toml"),
    include_str!("../config/i18n/fr/compression.toml"),
    include_str!("../config/i18n/fr/vhost-config.toml"),
    include_str!("../config/i18n/fr/config-snapshots.toml"),
    include_str!("../config/i18n/fr/supply-chain-security.toml"),
    include_str!("../config/i18n/fr/github-setup.toml"),
    include_str!("../config/i18n/fr/geoip.toml"),
    include_str!("../config/i18n/fr/macos-development.toml"),
    include_str!("../config/i18n/fr/logging-architecture.toml"),
    include_str!("../config/i18n/fr/metrics-architecture.toml"),
    include_str!("../config/i18n/fr/legacy-static-http.toml"),
    include_str!("../config/i18n/fr/auth-request.toml"),
    include_str!("../config/i18n/fr/secure-links.toml"),
    include_str!("../config/i18n/fr/release-checklist.toml"),
    include_str!("../config/i18n/fr/release-notes-template.toml"),
    include_str!("../config/i18n/fr/release-runbook.toml"),
    include_str!("../config/i18n/fr/compliance-evidence-template.toml"),
    include_str!("../config/i18n/fr/runtime-baseline.toml"),
    include_str!("../config/i18n/fr/cloudflare-origin-support.toml"),
    include_str!("../config/i18n/fr/cache-encryption.toml"),
    include_str!("../config/i18n/fr/runtime-parity-fixtures.toml"),
    include_str!("../config/i18n/fr/pingora-core-patch.toml"),
    include_str!("../config/i18n/fr/owasp-top10-2025-baseline.toml"),
    include_str!("../config/i18n/fr/extraction-dependency-graph.toml"),
    include_str!("../config/i18n/fr/modularity-policy.toml"),
    include_str!("../config/i18n/fr/modularity-exceptions.toml"),
    include_str!("../config/i18n/fr/fluxheim-ecosystem-idea.toml"),
    include_str!("../config/i18n/fr/runtime-facts-and-policy-proofs.toml"),
    include_str!("../config/i18n/fr/perl-cgi-support.toml"),
    include_str!("../config/i18n/fr/wasm-extensibility.toml"),
    include_str!("../config/i18n/fr/image-filter.toml"),
    include_str!("../config/i18n/fr/source-features.toml"),
    include_str!("../config/i18n/fr/cache-backends.toml"),
    include_str!("../config/i18n/fr/certificate-renewal.toml"),
    include_str!("../config/i18n/fr/common-criteria-roadmap.toml"),
    include_str!("../config/i18n/fr/config-reference.toml"),
    include_str!("../config/i18n/fr/crypto-rpc-edge.toml"),
    include_str!("../config/i18n/fr/versioning-plan.toml"),
    include_str!("../config/i18n/fr/gateway-recipes.toml"),
    include_str!("../config/i18n/fr/opentelemetry-tracing.toml"),
    include_str!("../config/i18n/fr/php-fpm-app-recipes.toml"),
    include_str!("../config/i18n/fr/php-runtime-support.toml"),
    include_str!("../config/i18n/fr/sentinel-mesh.toml"),
    include_str!("../config/i18n/fr/programmable-media-edge.toml"),
    include_str!("../config/i18n/fr/waf-architecture.toml"),
    include_str!("../config/i18n/fr/fips.toml"),
    include_str!("../config/i18n/fr/zero-retention-privacy-mode.toml"),
];

#[derive(Debug, Clone, Deserialize)]
struct TranslationFile {
    locale_id: String,
    html_lang: String,
    phrase: Vec<Phrase>,
}

#[derive(Debug, Clone, Deserialize)]
struct Phrase {
    from: String,
    to: String,
}

pub fn translate_html(locale: &Locale, html: String) -> String {
    if locale.locale_id == "en-EU" {
        return html_lang(locale, html);
    }

    let Some(file) = translations()
        .iter()
        .find(|translation| translation.locale_id == locale.locale_id)
    else {
        return html_lang(locale, html);
    };

    let mut output = html_lang_override(&file.html_lang, html);
    for phrase in &file.phrase {
        output = output.replace(&phrase.from, &phrase.to);
    }
    output
}

fn html_lang(locale: &Locale, html: String) -> String {
    html_lang_override(&locale.html_lang, html)
}

fn html_lang_override(html_lang: &str, html: String) -> String {
    html.replace(
        r#"<html lang="en""#,
        &format!(r#"<html lang="{html_lang}""#),
    )
}

fn translations() -> &'static [TranslationFile] {
    static TRANSLATIONS: OnceLock<Vec<TranslationFile>> = OnceLock::new();
    TRANSLATIONS.get_or_init(|| {
        let mut files = vec![
            merge_translation_files(DE_TOML_FILES, "German"),
            merge_translation_files(FR_TOML_FILES, "French"),
        ];
        for file in &mut files {
            file.phrase.sort_by_key(|phrase| Reverse(phrase.from.len()));
        }
        files
    })
}

fn merge_translation_files(files: &[&str], label: &str) -> TranslationFile {
    let mut merged: Option<TranslationFile> = None;

    for file in files {
        let mut parsed = toml::from_str::<TranslationFile>(file)
            .unwrap_or_else(|error| panic!("valid {label} i18n TOML: {error}"));
        if let Some(merged) = &mut merged {
            assert_eq!(
                merged.locale_id, parsed.locale_id,
                "{label} i18n locale ids must match"
            );
            assert_eq!(
                merged.html_lang, parsed.html_lang,
                "{label} i18n html_lang values must match"
            );
            merged.phrase.append(&mut parsed.phrase);
        } else {
            merged = Some(parsed);
        }
    }

    merged.unwrap_or_else(|| panic!("{label} i18n must include at least one file"))
}

#[cfg(test)]
mod tests {
    use super::translate_html;
    use crate::content::Site;

    #[test]
    fn translates_german_html_lang() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("de-DE").expect("German locale");
        let html = r#"<html lang="en"><body>Fluxheim</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains(r#"<html lang="de-DE""#));
    }

    #[test]
    fn translates_french_html_lang() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("fr-FR").expect("French locale");
        let html = r#"<html lang="en"><body>Fluxheim</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains(r#"<html lang="fr-FR""#));
    }

    #[test]
    fn leaves_unmatched_text_unchanged_when_legacy_bundles_are_empty() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("de-DE").expect("German locale");
        let html = r#"<html lang="en"><body>Unmatched runtime text</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains("Unmatched runtime text"));
    }
}
