mod replace;
use super::KeyFile;
use replace::page_map;

pub(super) fn apply_page_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
    let is_download_page =
        html.contains("<!-- Platform Downloads -->") || html.contains("<!-- All Releases -->");
    let is_changelog_page = html.contains(
        source
            .changelog
            .get("release_history_for_fluxheim_full_release_notes_are")
            .expect("changelog intro i18n key exists"),
    ) || html.contains("<!-- Timeline -->");
    let is_docs_page = html.contains("sidebar-link") || html.contains("Full docs on GitHub");
    let is_home_page = html.contains("FEATURES GRID");

    let html = page_map(
        html,
        is_home_page || is_download_page || is_changelog_page || is_docs_page,
        &source.release_updates,
        &keys.release_updates,
    );
    let html = page_map(
        html,
        is_download_page || is_changelog_page,
        &source.download,
        &keys.download,
    );
    let html = page_map(html, is_changelog_page, &source.changelog, &keys.changelog);
    let html = page_map(
        html,
        is_home_page || is_download_page || is_changelog_page || is_docs_page,
        &source.docs_guides,
        &keys.docs_guides,
    );
    page_map(
        html,
        is_docs_page,
        &source.docs_expanded,
        &keys.docs_expanded,
    )
}
