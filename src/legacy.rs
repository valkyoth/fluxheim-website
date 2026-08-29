use std::collections::HashMap;
use std::path::{Path, PathBuf};

use axum::body::Bytes;

use crate::content::{Locale, Site};
use crate::i18n;
use crate::i18n_keys;
use crate::language_selector;
use crate::page_enhancements;

const SOURCE_FLUXHEIM_VERSION: &str = "1.8.1";
const MAX_STATIC_ARTIFACT_BYTES: u64 = 2 * 1024 * 1024;

include!(concat!(env!("OUT_DIR"), "/embedded_content.rs"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPage {
    pub html: String,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticArtifact {
    pub body: Bytes,
    pub content_type: &'static str,
}

pub type ArtifactCache = HashMap<String, StaticArtifact>;

pub fn render(site: &Site, request_path: &str) -> Option<LegacyPage> {
    let clean = request_path.trim_matches('/');
    let (locale, slug) = site.split_path(clean);
    let file_path = html_path_for_locale(site, locale, slug)?;
    let mut html = embedded_html(&file_path)?.to_owned();
    html = apply_version(site, html);
    html = i18n_keys::apply_shared_keys(locale, html, &site.config.fluxheim_version);
    html = i18n::translate_html(locale, html);
    html = page_enhancements::enhance(site, locale, slug, html);
    html = language_selector::inject(site, locale, slug, html);

    Some(LegacyPage {
        html,
        slug: slug.to_owned(),
    })
}

pub fn preload_static_artifacts() -> Result<ArtifactCache, String> {
    let mut artifacts = HashMap::with_capacity(EMBEDDED_ARTIFACTS.len());
    for (path, body, content_type) in EMBEDDED_ARTIFACTS {
        if body.len() as u64 > MAX_STATIC_ARTIFACT_BYTES {
            return Err(format!("embedded artifact exceeds size limit: {path}"));
        }
        let artifact = StaticArtifact {
            body: Bytes::from_static(body),
            content_type,
        };
        if artifacts.insert((*path).to_owned(), artifact).is_some() {
            return Err(format!("duplicate embedded artifact: {path}"));
        }
    }
    Ok(artifacts)
}

pub fn cached_static_artifact<'a>(
    site: &Site,
    artifacts: &'a ArtifactCache,
    request_path: &str,
) -> Option<&'a StaticArtifact> {
    let clean = request_path.trim_matches('/');
    let (_locale, slug) = site.split_path(clean);
    let normalized = normalize_slug(slug)?;
    let path = PathBuf::from(normalized);

    if !is_allowed_artifact(&path) {
        return None;
    }
    artifacts.get(normalized)
}

fn html_path_for_locale(site: &Site, locale: &Locale, slug: &str) -> Option<PathBuf> {
    if locale.locale_id != site.config.default_locale
        && let Some(path) = localized_html_path(&locale.url_prefix, slug)
    {
        return Some(path);
    }

    html_path(slug)
}

fn localized_html_path(locale_prefix: &str, slug: &str) -> Option<PathBuf> {
    let normalized = normalize_slug(slug)?;
    candidate_paths(normalized)
        .into_iter()
        .map(|path| PathBuf::from("localized").join(locale_prefix).join(path))
        .find(|path| safe_existing_html(path))
}

pub fn legacy_html_paths() -> Vec<PathBuf> {
    let mut paths = EMBEDDED_HTML
        .iter()
        .map(|(path, _)| PathBuf::from(path))
        .filter(|path| is_allowed_html(path))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

pub fn slug_for_path(path: &Path) -> Option<String> {
    if path == Path::new("index.html") {
        return Some(String::new());
    }

    if !is_allowed_html(path) {
        return None;
    }

    let without_extension = path.with_extension("");
    without_extension
        .to_str()
        .map(|slug| slug.trim_matches('/').to_owned())
}

fn html_path(slug: &str) -> Option<PathBuf> {
    let normalized = normalize_slug(slug)?;
    candidate_paths(normalized)
        .into_iter()
        .find(|path| safe_existing_html(path))
}

fn normalize_slug(slug: &str) -> Option<&str> {
    let slug = slug.trim_matches('/');
    if slug.is_empty() || slug == "index" || slug == "index.html" {
        return Some("");
    }

    if slug.contains('\\') || slug.split('/').any(|part| part == "." || part == "..") {
        return None;
    }

    Some(slug)
}

fn is_allowed_html(path: &Path) -> bool {
    if path.extension().and_then(|ext| ext.to_str()) != Some("html") {
        return false;
    }

    if path.starts_with("docs/source") {
        return false;
    }

    path.starts_with("docs")
        || matches!(
            path.to_str(),
            Some(
                "index.html"
                    | "download.html"
                    | "changelog.html"
                    | "cookies.html"
                    | "privacy.html"
                    | "gdpr.html"
            )
        )
}

fn candidate_paths(normalized: &str) -> Vec<PathBuf> {
    if normalized.is_empty() {
        return vec![PathBuf::from("index.html")];
    }

    if normalized.ends_with(".html") {
        vec![PathBuf::from(normalized)]
    } else {
        vec![
            PathBuf::from(format!("{normalized}.html")),
            PathBuf::from(normalized).join("index.html"),
        ]
    }
}

fn safe_existing_html(path: &Path) -> bool {
    (is_allowed_html(path) || is_allowed_localized_html(path)) && embedded_html(path).is_some()
}

fn is_allowed_artifact(path: &Path) -> bool {
    let extension = path.extension().and_then(|ext| ext.to_str());
    let is_source_artifact =
        path.starts_with("docs/source") && matches!(extension, Some("md" | "toml" | "tsv" | "txt"));
    let is_release_artifact = path.starts_with("docs/releases") && extension == Some("md");
    let is_config_artifact = path.starts_with("conf") && extension == Some("toml");

    is_source_artifact || is_release_artifact || is_config_artifact
}

fn is_allowed_localized_html(path: &Path) -> bool {
    path.starts_with("localized") && path.extension().and_then(|ext| ext.to_str()) == Some("html")
}

fn embedded_html(path: &Path) -> Option<&'static str> {
    let path = path.to_str()?;
    EMBEDDED_HTML
        .iter()
        .find_map(|(candidate, html)| (*candidate == path).then_some(*html))
}

fn apply_version(site: &Site, html: String) -> String {
    html.replace(
        &format!("v{SOURCE_FLUXHEIM_VERSION}"),
        &format!("v{}", site.config.fluxheim_version),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_STATIC_ARTIFACT_BYTES, cached_static_artifact, html_path, legacy_html_paths,
        preload_static_artifacts, render, slug_for_path,
    };
    use crate::content::Site;

    #[test]
    fn maps_locale_prefixed_legacy_paths() {
        let site = Site::load().expect("site loads");
        let page = render(&site, "/de/docs/deployment").expect("legacy page");
        assert!(page.html.contains("Systemd & Container"));
        assert!(page.html.contains("fh-language-switcher"));
        assert!(page.html.contains(r#"<summary aria-label="Sprache">"#));
        assert!(page.html.contains("🇩🇪"));
        assert!(page.html.contains("<span>Deutsch</span>"));
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(html_path("../README").is_none());
        assert!(html_path("docs/../README").is_none());
    }

    #[test]
    fn inventories_legacy_pages() {
        let paths = legacy_html_paths();
        assert!(
            paths
                .iter()
                .any(|path| path == std::path::Path::new("index.html"))
        );
        assert!(
            paths
                .iter()
                .any(|path| path == std::path::Path::new("docs/reference.html"))
        );
        assert!(
            !paths
                .iter()
                .any(|path| path.starts_with(std::path::Path::new("docs/source")))
        );
        assert_eq!(
            slug_for_path(std::path::Path::new("download.html")).as_deref(),
            Some("download")
        );
    }

    #[test]
    fn serves_source_markdown_artifacts() {
        let site = Site::load().expect("site loads");
        let artifacts = preload_static_artifacts().expect("artifact cache");
        let artifact = cached_static_artifact(&site, &artifacts, "/de/docs/source/systemd.md")
            .expect("markdown artifact");
        assert_eq!(artifact.content_type, "text/markdown; charset=utf-8");
        assert!(
            std::str::from_utf8(&artifact.body)
                .expect("UTF-8 markdown")
                .contains("# systemd Deployment")
        );
    }

    #[test]
    fn serves_legacy_fluxheim_config_artifacts() {
        let site = Site::load().expect("site loads");
        let artifacts = preload_static_artifacts().expect("artifact cache");
        let artifact = cached_static_artifact(&site, &artifacts, "/fr/conf/fluxheim.toml")
            .expect("config artifact");
        assert_eq!(artifact.content_type, "application/toml; charset=utf-8");
        assert!(
            std::str::from_utf8(&artifact.body)
                .expect("UTF-8 TOML")
                .contains("hosts = [\"fluxheim.eu\"]")
        );
    }

    #[test]
    fn rejects_unlisted_source_artifact_extensions() {
        let site = Site::load().expect("site loads");
        let artifacts = preload_static_artifacts().expect("artifact cache");
        assert!(cached_static_artifact(&site, &artifacts, "/docs/source/private.json").is_none());
    }

    #[test]
    fn embedded_artifacts_are_unique_and_bounded() {
        let artifacts = preload_static_artifacts().expect("artifact cache");
        assert!(!artifacts.is_empty());
        assert!(
            artifacts
                .values()
                .all(|artifact| artifact.body.len() as u64 <= MAX_STATIC_ARTIFACT_BYTES)
        );
    }
}
