use std::collections::BTreeMap;

use crate::i18n_keys::text_replace::HtmlTextReplace;

pub(super) fn marker_map(
    html: String,
    marker: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let is_page = html.contains(marker);
    page_map(html, is_page, source, keys)
}

pub(super) fn marker_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = source
        .get(marker_key)
        .unwrap_or_else(|| panic!("source marker i18n key exists: {marker_key}"));
    marker_map(html, marker, source, keys)
}

pub(super) fn docs_marker_from_key_map(
    html: String,
    marker_key: &str,
    marker_source: &BTreeMap<String, String>,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(marker_source, marker_key, "Fluxheim Docs");
    marker_map(html, &marker, source, keys)
}

pub(super) fn source_doc_marker_from_key_map(
    html: String,
    marker_key: &str,
    marker_source: &BTreeMap<String, String>,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(marker_source, marker_key, "Fluxheim Source Docs");
    marker_map(html, &marker, source, keys)
}

pub(super) fn docs_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(source, marker_key, "Fluxheim Docs");
    marker_map(html, &marker, source, keys)
}

pub(super) fn source_doc_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(source, marker_key, "Fluxheim Source Docs");
    marker_map(html, &marker, source, keys)
}

pub(super) fn page_map(
    html: String,
    is_page: bool,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    if is_page {
        html.replace_map_everywhere(source, keys)
    } else {
        html
    }
}

pub(super) fn title_marker(
    source: &BTreeMap<String, String>,
    marker_key: &str,
    suffix: &str,
) -> String {
    let title = source
        .get(marker_key)
        .unwrap_or_else(|| panic!("source title i18n key exists: {marker_key}"));
    format!("{title} — {suffix}")
}
