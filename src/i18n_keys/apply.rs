use super::{
    KeyFile,
    page_maps::apply_page_maps,
    text_replace::{HtmlTextReplace, html_escaped},
    versioned,
};

pub(super) fn apply_keys(keys: &KeyFile, source: &KeyFile, html: String, version: &str) -> String {
    let html = html
        .replace(
            &source.shell.home_title,
            &html_escaped(&keys.shell.home_title),
        )
        .replace(
            &source.release.latest_stable_release,
            &html_escaped(&keys.release.latest_stable_release),
        )
        .replace(
            &source.shell.switch_color_theme,
            &html_escaped(&keys.shell.switch_color_theme),
        )
        .replace(
            &format!(">{}<", source.shell.view_on_github),
            &format!(">{}<", html_escaped(&keys.shell.view_on_github)),
        )
        .replace(
            &format!(">{}<", source.shell.quick_start),
            &format!(">{}<", html_escaped(&keys.shell.quick_start)),
        )
        .replace(
            &format!(">{}<", source.shell.links),
            &format!(">{}<", html_escaped(&keys.shell.links)),
        )
        .replace(
            &format!(">{}<", source.shell.github_repository),
            &format!(">{}<", html_escaped(&keys.shell.github_repository)),
        )
        .replace(
            &format!(">{}<", source.shell.issues),
            &format!(">{}<", html_escaped(&keys.shell.issues)),
        )
        .replace(
            &format!(">{}<", source.shell.menu),
            &format!(">{}<", html_escaped(&keys.shell.menu)),
        )
        .replace(&source.footer.tagline, &html_escaped(&keys.footer.tagline))
        .replace(
            &format!(">{}<", source.footer.project),
            &format!(">{}<", html_escaped(&keys.footer.project)),
        )
        .replace(
            &format!(">{}<", source.footer.releases),
            &format!(">{}<", html_escaped(&keys.footer.releases)),
        )
        .replace(
            &format!(">{}<", source.footer.roadmap),
            &format!(">{}<", html_escaped(&keys.footer.roadmap)),
        )
        .replace(
            &format!(">{}<", source.footer.community),
            &format!(">{}<", html_escaped(&keys.footer.community)),
        )
        .replace(
            &format!(">{}<", source.footer.discussions),
            &format!(">{}<", html_escaped(&keys.footer.discussions)),
        )
        .replace(
            &format!(">{}<", source.footer.eupl_license),
            &format!(">{}<", html_escaped(&keys.footer.eupl_license)),
        )
        .replace(
            &format!(">{}<", source.footer.valkyoth_org),
            &format!(">{}<", html_escaped(&keys.footer.valkyoth_org)),
        )
        .replace(
            &source.footer.copyright_prefix,
            &html_escaped(&keys.footer.copyright_prefix),
        )
        .replace(
            &source.footer.built_with,
            &html_escaped(&keys.footer.built_with),
        )
        .replace_map(&source.home, &keys.home)
        .replace_map(&source.docs_index, &keys.docs_index)
        .replace_map(&source.common, &keys.common)
        .replace_map_everywhere(&source.code_comments, &keys.code_comments);

    let html = apply_page_maps(html, keys, source);

    html.replace(
        &format!(">{}<", versioned(&source.release.download_version, version)),
        &format!(
            ">{}<",
            html_escaped(&versioned(&keys.release.download_version, version))
        ),
    )
    .replace(
        &format!("{} —", source.release.latest_stable),
        &format!("{} —", html_escaped(&keys.release.latest_stable)),
    )
    .replace(
        &format!(">{}<", source.release.latest_stable),
        &format!(">{}<", html_escaped(&keys.release.latest_stable)),
    )
    .replace(
        &format!(">{}<", source.nav.changelog),
        &format!(">{}<", html_escaped(&keys.nav.changelog)),
    )
    .replace(
        &format!(">{}<", source.nav.download),
        &format!(">{}<", html_escaped(&keys.nav.download)),
    )
    .replace(
        &format!(">{}<", source.nav.docs),
        &format!(">{}<", html_escaped(&keys.nav.docs)),
    )
}
