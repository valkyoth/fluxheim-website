use std::path::{Component, Path, PathBuf};

use crate::content::{Locale, Site};
use crate::i18n;
use crate::i18n_keys;
use crate::page_enhancements;

const SOURCE_FLUXHEIM_VERSION: &str = "1.6.30";

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
    html = inject_language_selector(site, locale, slug, html);

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

    if !safe_existing_artifact(&path) {
        return None;
    }

    let content_type = artifact_content_type(&path)?;
    let body = std::fs::read(&path).ok()?;
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
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        )
    }) {
        return false;
    }

    path.exists() && (is_allowed_html(path) || is_allowed_localized_html(path))
}

fn safe_existing_artifact(path: &Path) -> bool {
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        )
    }) {
        return false;
    }

    path.exists() && path.is_file()
}

fn is_allowed_artifact(path: &Path) -> bool {
    let extension = path.extension().and_then(|ext| ext.to_str());
    let is_source_artifact = path.starts_with("docs/source") && extension != Some("html");
    let is_release_artifact = path.starts_with("docs/releases") && extension == Some("md");
    let is_config_artifact = path.starts_with("conf") && extension == Some("toml");

    is_source_artifact || is_release_artifact || is_config_artifact
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

fn inject_language_selector(
    site: &Site,
    active_locale: &Locale,
    slug: &str,
    html: String,
) -> String {
    let selector = language_selector(site, active_locale, slug);
    if let Some(index) = html.rfind("</body>") {
        let mut output = String::with_capacity(html.len() + selector.len());
        output.push_str(&html[..index]);
        output.push_str(&selector);
        output.push_str(&html[index..]);
        output
    } else {
        format!("{html}{selector}")
    }
}

fn language_selector(site: &Site, active_locale: &Locale, slug: &str) -> String {
    let label = i18n_keys::language_selector_label(active_locale);
    let active_display_name = i18n_keys::language_display_name(
        active_locale,
        &active_locale.locale_id,
        &active_locale.display_name,
    );
    let mut html = String::from(
        r#"<style>
.fh-language-switcher{position:fixed;right:1rem;bottom:1rem;z-index:60;font-family:Inter,ui-sans-serif,system-ui,sans-serif}
.fh-language-switcher summary{display:inline-flex;align-items:center;gap:.45rem;list-style:none;cursor:pointer;border:1px solid rgb(55 65 81);border-radius:.5rem;background:rgba(17,24,39,.94);color:rgb(229 231 235);padding:.55rem .75rem;font-size:.8125rem;font-weight:700;box-shadow:0 10px 30px rgba(0,0,0,.28)}
.fh-language-switcher summary::-webkit-details-marker{display:none}
.fh-language-switcher div{position:absolute;right:0;bottom:2.7rem;min-width:11.5rem;border:1px solid rgb(55 65 81);border-radius:.5rem;background:rgba(17,24,39,.98);padding:.35rem;box-shadow:0 10px 30px rgba(0,0,0,.35)}
.fh-language-switcher a{display:flex;align-items:center;gap:.45rem;border-radius:.375rem;padding:.5rem .65rem;color:rgb(209 213 219);font-size:.8125rem;text-decoration:none;white-space:nowrap}
.fh-language-switcher a:hover{background:rgb(31 41 55);color:white}
.fh-language-switcher a[aria-current=true]{background:rgb(34 211 238);color:rgb(3 7 18);font-weight:800}
.fh-language-flag{display:inline-flex;width:1.25em;justify-content:center;font-size:1rem;line-height:1}
</style>
<details class="fh-language-switcher">
  <summary aria-label=""#,
    );
    html.push_str(&html_escape(label));
    html.push_str(r#""><span class="fh-language-flag" aria-hidden="true">"#);
    html.push_str(language_flag(&active_locale.locale_id));
    html.push_str(r#"</span><span>"#);
    html.push_str(&html_escape(&active_display_name));
    html.push_str(
        r#"</span></summary>
  <div>
"#,
    );

    for link in site.language_links(&active_locale.locale_id, slug) {
        let active = if link.active {
            r#" aria-current="true""#
        } else {
            ""
        };
        let display_name =
            i18n_keys::language_display_name(active_locale, &link.locale_id, &link.display_name);
        html.push_str(&format!(
            r#"  <a href="{}"{}><span class="fh-language-flag" aria-hidden="true">{}</span><span>{}</span></a>
"#,
            html_escape(&link.href),
            active,
            language_flag(&link.locale_id),
            html_escape(&display_name)
        ));
    }

    html.push_str("  </div>\n</details>\n");
    html
}

fn html_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn language_flag(locale_id: &str) -> &'static str {
    match locale_id {
        "en-EU" => "🇪🇺",
        "en-GB" => "🇬🇧",
        "en-US" => "🇺🇸",
        "de-DE" => "🇩🇪",
        "de-CH" => "🇨🇭",
        "fr-FR" => "🇫🇷",
        "sv-SE" => "🇸🇪",
        "nb-NO" => "🇳🇴",
        "nl-NL" => "🇳🇱",
        "fi-FI" => "🇫🇮",
        "is-IS" => "🇮🇸",
        "da-DK" => "🇩🇰",
        "es-ES" => "🇪🇸",
        "pt-PT" => "🇵🇹",
        "et-EE" => "🇪🇪",
        "lv-LV" => "🇱🇻",
        "el-GR" => "🇬🇷",
        "it-IT" => "🇮🇹",
        "lt-LT" => "🇱🇹",
        "hr-HR" => "🇭🇷",
        "cs-CZ" => "🇨🇿",
        "bs-BA" => "🇧🇦",
        "bg-BG" => "🇧🇬",
        "ro-RO" => "🇷🇴",
        "pl-PL" => "🇵🇱",
        "ru-RU" => "🇷🇺",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        html_path, language_flag, legacy_html_paths, render, render_static_artifact, slug_for_path,
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
    fn maps_language_flags() {
        assert_eq!(language_flag("en-EU"), "🇪🇺");
        assert_eq!(language_flag("en-GB"), "🇬🇧");
        assert_eq!(language_flag("en-US"), "🇺🇸");
        assert_eq!(language_flag("de-DE"), "🇩🇪");
        assert_eq!(language_flag("de-CH"), "🇨🇭");
        assert_eq!(language_flag("fr-FR"), "🇫🇷");
        assert_eq!(language_flag("sv-SE"), "🇸🇪");
        assert_eq!(language_flag("nb-NO"), "🇳🇴");
        assert_eq!(language_flag("nl-NL"), "🇳🇱");
        assert_eq!(language_flag("fi-FI"), "🇫🇮");
        assert_eq!(language_flag("is-IS"), "🇮🇸");
        assert_eq!(language_flag("da-DK"), "🇩🇰");
        assert_eq!(language_flag("es-ES"), "🇪🇸");
        assert_eq!(language_flag("pt-PT"), "🇵🇹");
        assert_eq!(language_flag("et-EE"), "🇪🇪");
        assert_eq!(language_flag("lv-LV"), "🇱🇻");
        assert_eq!(language_flag("el-GR"), "🇬🇷");
        assert_eq!(language_flag("it-IT"), "🇮🇹");
        assert_eq!(language_flag("lt-LT"), "🇱🇹");
        assert_eq!(language_flag("hr-HR"), "🇭🇷");
        assert_eq!(language_flag("cs-CZ"), "🇨🇿");
        assert_eq!(language_flag("bs-BA"), "🇧🇦");
        assert_eq!(language_flag("bg-BG"), "🇧🇬");
        assert_eq!(language_flag("ro-RO"), "🇷🇴");
        assert_eq!(language_flag("pl-PL"), "🇵🇱");
        assert_eq!(language_flag("ru-RU"), "🇷🇺");
        assert_eq!(language_flag("unknown"), "");
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
}
