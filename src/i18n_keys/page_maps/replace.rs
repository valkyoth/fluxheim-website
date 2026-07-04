use std::collections::BTreeMap;

use crate::i18n_keys::text_replace::HtmlTextReplace;

pub(super) fn page_map(
    html: String,
    is_page: bool,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    if is_page {
        html.replace_map_text_nodes(source, keys)
    } else {
        html
    }
}
