use super::{KeyFile, page_maps::apply_page_maps, text_replace::HtmlTextReplace, versioned};

pub(super) fn apply_keys(keys: &KeyFile, source: &KeyFile, html: String, version: &str) -> String {
    let html = html
        .replace(&source.shell.home_title, &keys.shell.home_title)
        .replace(
            &source.release.latest_stable_release,
            &keys.release.latest_stable_release,
        )
        .replace(
            &source.shell.switch_color_theme,
            &keys.shell.switch_color_theme,
        )
        .replace(
            &format!(">{}<", source.shell.view_on_github),
            &format!(">{}<", keys.shell.view_on_github),
        )
        .replace(
            &format!(">{}<", source.shell.quick_start),
            &format!(">{}<", keys.shell.quick_start),
        )
        .replace(
            &format!(">{}<", source.shell.links),
            &format!(">{}<", keys.shell.links),
        )
        .replace(
            &format!(">{}<", source.shell.github_repository),
            &format!(">{}<", keys.shell.github_repository),
        )
        .replace(
            &format!(">{}<", source.shell.issues),
            &format!(">{}<", keys.shell.issues),
        )
        .replace(
            &format!(">{}<", source.shell.menu),
            &format!(">{}<", keys.shell.menu),
        )
        .replace(&source.footer.tagline, &keys.footer.tagline)
        .replace(
            &format!(">{}<", source.footer.project),
            &format!(">{}<", keys.footer.project),
        )
        .replace(
            &format!(">{}<", source.footer.releases),
            &format!(">{}<", keys.footer.releases),
        )
        .replace(
            &format!(">{}<", source.footer.roadmap),
            &format!(">{}<", keys.footer.roadmap),
        )
        .replace(
            &format!(">{}<", source.footer.community),
            &format!(">{}<", keys.footer.community),
        )
        .replace(
            &format!(">{}<", source.footer.discussions),
            &format!(">{}<", keys.footer.discussions),
        )
        .replace(
            &format!(">{}<", source.footer.eupl_license),
            &format!(">{}<", keys.footer.eupl_license),
        )
        .replace(
            &format!(">{}<", source.footer.valkyoth_org),
            &format!(">{}<", keys.footer.valkyoth_org),
        )
        .replace(
            &source.footer.copyright_prefix,
            &keys.footer.copyright_prefix,
        )
        .replace(&source.footer.built_with, &keys.footer.built_with)
        .replace_map(&source.home, &keys.home)
        .replace_map(&source.docs_index, &keys.docs_index)
        .replace_map(&source.common, &keys.common)
        .replace_map_everywhere(&source.code_comments, &keys.code_comments);

    let html = apply_page_maps(html, keys, source);

    html.replace(
        &format!(">{}<", versioned(&source.release.download_version, version)),
        &format!(">{}<", versioned(&keys.release.download_version, version)),
    )
    .replace(
        &format!("{} —", source.release.latest_stable),
        &format!("{} —", keys.release.latest_stable),
    )
    .replace(
        &format!(">{}<", source.release.latest_stable),
        &format!(">{}<", keys.release.latest_stable),
    )
    .replace(
        &format!(">{}<", source.nav.changelog),
        &format!(">{}<", keys.nav.changelog),
    )
    .replace(
        &format!(">{}<", source.nav.download),
        &format!(">{}<", keys.nav.download),
    )
    .replace(
        &format!(">{}<", source.nav.docs),
        &format!(">{}<", keys.nav.docs),
    )
}
