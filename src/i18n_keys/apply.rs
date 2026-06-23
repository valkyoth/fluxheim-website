use super::{KeyFile, page_maps::apply_page_maps, text_replace::HtmlTextReplace, versioned};

pub(super) fn apply_keys(keys: &KeyFile, source: &KeyFile, html: String, version: &str) -> String {
    let html = html
        .replace(
            "Fluxheim — Memory-Safe Edge Server Built in Rust",
            &keys.shell.home_title,
        )
        .replace("Latest Stable Release", &keys.release.latest_stable_release)
        .replace("Switch color theme", &keys.shell.switch_color_theme)
        .replace(
            ">View on GitHub<",
            &format!(">{}<", keys.shell.view_on_github),
        )
        .replace(">Quick Start<", &format!(">{}<", keys.shell.quick_start))
        .replace(">Links<", &format!(">{}<", keys.shell.links))
        .replace(
            ">GitHub Repository<",
            &format!(">{}<", keys.shell.github_repository),
        )
        .replace(">Issues<", &format!(">{}<", keys.shell.issues))
        .replace(">Menu<", &format!(">{}<", keys.shell.menu))
        .replace(
            "Memory-safe edge server built in Rust. Licensed under EUPL-1.2.",
            &keys.footer.tagline,
        )
        .replace(">Project<", &format!(">{}<", keys.footer.project))
        .replace(">Releases<", &format!(">{}<", keys.footer.releases))
        .replace(">Roadmap<", &format!(">{}<", keys.footer.roadmap))
        .replace(">Community<", &format!(">{}<", keys.footer.community))
        .replace(">Discussions<", &format!(">{}<", keys.footer.discussions))
        .replace(
            ">EUPL-1.2 License<",
            &format!(">{}<", keys.footer.eupl_license),
        )
        .replace(">Valkyoth Org<", &format!(">{}<", keys.footer.valkyoth_org))
        .replace(
            "© 2026 Valkyoth. Distributed under the",
            &keys.footer.copyright_prefix,
        )
        .replace(
            "Built with Rust · Powered by Fluxheim",
            &keys.footer.built_with,
        )
        .replace_map(&source.home, &keys.home)
        .replace_map(&source.docs_index, &keys.docs_index)
        .replace_map(&source.common, &keys.common)
        .replace_map_everywhere(&source.code_comments, &keys.code_comments);

    let html = apply_page_maps(html, keys, source);

    html.replace(
        ">Download v1.6.28<",
        &format!(">{}<", versioned(&keys.release.download_version, version)),
    )
    .replace(
        "Latest Stable —",
        &format!("{} —", keys.release.latest_stable),
    )
    .replace(
        ">Latest Stable<",
        &format!(">{}<", keys.release.latest_stable),
    )
    .replace(">Changelog<", &format!(">{}<", keys.nav.changelog))
    .replace(">Download<", &format!(">{}<", keys.nav.download))
    .replace(">Docs<", &format!(">{}<", keys.nav.docs))
}
