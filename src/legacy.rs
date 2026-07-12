use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

use crate::content::{Locale, Site};
use crate::i18n;
use crate::i18n_keys;
use crate::language_selector;
use crate::page_enhancements;

const SOURCE_FLUXHEIM_VERSION: &str = "1.7.9";
const MAX_STATIC_ARTIFACT_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPage {
    pub html: String,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticArtifact {
    pub body: Vec<u8>,
    pub content_type: &'static str,
}

pub fn render(site: &Site, request_path: &str) -> Option<LegacyPage> {
    let clean = request_path.trim_matches('/');
    let (locale, slug) = site.split_path(clean);
    let file_path = html_path_for_locale(site, locale, slug)?;
    let mut html = std::fs::read_to_string(&file_path).ok()?;
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

pub fn render_static_artifact(site: &Site, request_path: &str) -> Option<StaticArtifact> {
    let clean = request_path.trim_matches('/');
    let (_locale, slug) = site.split_path(clean);
    let normalized = normalize_slug(slug)?;
    let path = PathBuf::from(normalized);

    if !is_allowed_artifact(&path) {
        return None;
    }

    let content_type = artifact_content_type(&path)?;
    let body = read_static_artifact(&path).ok()?;
    Some(StaticArtifact { body, content_type })
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
    let mut paths = vec![
        PathBuf::from("index.html"),
        PathBuf::from("download.html"),
        PathBuf::from("changelog.html"),
        PathBuf::from("cookies.html"),
        PathBuf::from("privacy.html"),
        PathBuf::from("gdpr.html"),
    ];
    collect_html_paths(Path::new("docs"), &mut paths);
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

fn collect_html_paths(root: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_html_paths(&path, paths);
        } else if is_allowed_html(&path) {
            paths.push(path);
        }
    }
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
    if has_unsafe_components(path) {
        return false;
    }

    safe_regular_file(path) && (is_allowed_html(path) || is_allowed_localized_html(path))
}

fn is_allowed_artifact(path: &Path) -> bool {
    let extension = path.extension().and_then(|ext| ext.to_str());
    let is_source_artifact =
        path.starts_with("docs/source") && matches!(extension, Some("md" | "toml" | "tsv" | "txt"));
    let is_release_artifact = path.starts_with("docs/releases") && extension == Some("md");
    let is_config_artifact = path.starts_with("conf") && extension == Some("toml");

    is_source_artifact || is_release_artifact || is_config_artifact
}

fn read_static_artifact(path: &Path) -> io::Result<Vec<u8>> {
    if has_unsafe_components(path) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "unsafe path",
        ));
    }

    let file = open_artifact(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > MAX_STATIC_ARTIFACT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact is not a bounded regular file",
        ));
    }

    let mut body = Vec::with_capacity(metadata.len() as usize);
    file.take(MAX_STATIC_ARTIFACT_BYTES + 1)
        .read_to_end(&mut body)?;
    if body.len() as u64 > MAX_STATIC_ARTIFACT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact exceeds size limit",
        ));
    }
    Ok(body)
}

#[cfg(unix)]
fn open_artifact(path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)
}

#[cfg(not(unix))]
fn open_artifact(path: &Path) -> io::Result<File> {
    if fs::symlink_metadata(path)?.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact symlinks are not allowed",
        ));
    }
    OpenOptions::new().read(true).open(path)
}

fn has_unsafe_components(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        )
    })
}

fn safe_regular_file(path: &Path) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return false;
    };
    metadata.file_type().is_file()
}

fn artifact_content_type(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("md") => Some("text/markdown; charset=utf-8"),
        Some("toml") => Some("application/toml; charset=utf-8"),
        Some("tsv") => Some("text/tab-separated-values; charset=utf-8"),
        Some("txt") => Some("text/plain; charset=utf-8"),
        _ => None,
    }
}

fn is_allowed_localized_html(path: &Path) -> bool {
    path.starts_with("localized") && path.extension().and_then(|ext| ext.to_str()) == Some("html")
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
        MAX_STATIC_ARTIFACT_BYTES, html_path, legacy_html_paths, render, render_static_artifact,
        slug_for_path,
    };
    use crate::content::Site;
    use std::fs;
    use std::path::Path;

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
    fn localized_file_overrides_english_fallback() {
        let site = Site::load().expect("site loads");
        let dir = Path::new("localized/de/docs");
        let path = dir.join("__fixture.html");

        fs::create_dir_all(dir).expect("create localized fixture dir");
        fs::write(
            &path,
            "<!doctype html><html><head><title>Fixture</title></head><body>Deutsch fixture</body></html>",
        )
        .expect("write localized fixture");

        let page = render(&site, "/de/docs/__fixture").expect("localized page");
        assert!(page.html.contains("Deutsch fixture"));
        assert!(page.html.contains("fh-language-switcher"));

        fs::remove_file(&path).expect("remove localized fixture");
    }

    #[test]
    fn serves_source_markdown_artifacts() {
        let site = Site::load().expect("site loads");
        let artifact =
            render_static_artifact(&site, "/de/docs/source/systemd.md").expect("markdown artifact");
        assert_eq!(artifact.content_type, "text/markdown; charset=utf-8");
        assert!(
            String::from_utf8(artifact.body)
                .expect("utf8 markdown")
                .contains("# systemd Deployment")
        );
    }

    #[test]
    fn serves_legacy_fluxheim_config_artifacts() {
        let site = Site::load().expect("site loads");
        let artifact =
            render_static_artifact(&site, "/fr/conf/fluxheim.toml").expect("config artifact");
        assert_eq!(artifact.content_type, "application/toml; charset=utf-8");
        assert!(
            String::from_utf8(artifact.body)
                .expect("utf8 toml")
                .contains("hosts = [\"fluxheim.eu\"]")
        );
    }

    #[test]
    fn rejects_unlisted_source_artifact_extensions() {
        let site = Site::load().expect("site loads");
        let path = Path::new("docs/source/__private.json");
        fs::write(path, r#"{"secret":true}"#).expect("write private fixture");

        assert!(render_static_artifact(&site, "/docs/source/__private.json").is_none());

        fs::remove_file(path).expect("remove private fixture");
    }

    #[test]
    fn rejects_oversized_source_artifacts_before_reading() {
        let site = Site::load().expect("site loads");
        let path = Path::new("docs/source/__oversized.md");
        let file = fs::File::create(path).expect("create oversized fixture");
        file.set_len(MAX_STATIC_ARTIFACT_BYTES + 1)
            .expect("size oversized fixture");

        assert!(render_static_artifact(&site, "/docs/source/__oversized.md").is_none());

        fs::remove_file(path).expect("remove oversized fixture");
    }

    #[cfg(unix)]
    #[test]
    fn rejects_source_artifact_symlinks() {
        let site = Site::load().expect("site loads");
        let path = Path::new("docs/source/__symlink.md");
        std::os::unix::fs::symlink("../../README.md", path).expect("create symlink fixture");

        assert!(render_static_artifact(&site, "/docs/source/__symlink.md").is_none());

        fs::remove_file(path).expect("remove symlink fixture");
    }
}
