mod replace;
use super::KeyFile;
use replace::page_map;

pub(super) fn apply_page_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
    let download_marker = format!("{} — Fluxheim", source.nav.download);
    let is_download_page = html.contains(&download_marker);
    let is_changelog_page = html.contains(
        source
            .changelog
            .get("changelog_fluxheim")
            .expect("changelog page title i18n key exists"),
    );
    let is_docs_page = html.contains("Fluxheim Docs") || html.contains("Documentation — Fluxheim");

    let html = page_map(
        html,
        is_download_page || is_changelog_page,
        &source.release_updates,
        &keys.release_updates,
    );
    let html = page_map(html, is_download_page, &source.download, &keys.download);
    let html = page_map(html, is_changelog_page, &source.changelog, &keys.changelog);
    page_map(html, is_docs_page, &source.docs_guides, &keys.docs_guides)
}
