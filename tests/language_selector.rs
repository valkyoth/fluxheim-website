use std::sync::Arc;

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router;
use tower::ServiceExt;

async fn get(path: &str) -> (StatusCode, String) {
    let site = Arc::new(Site::load().expect("site content loads"));
    let app = build_router(site);
    let response = app
        .oneshot(
            Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())
                .expect("request builds"),
        )
        .await
        .expect("response");
    let status = response.status();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body bytes");

    (status, String::from_utf8(body.to_vec()).expect("utf8 body"))
}

#[tokio::test]
async fn every_rendered_language_selector_link_resolves() {
    let pages = [
        "/",
        "/download",
        "/docs",
        "/docs/cache",
        "/de/download",
        "/ch/docs",
        "/fr/docs",
        "/sv/docs",
        "/no/docs",
        "/nl/docs",
        "/fi/docs",
        "/is/docs",
        "/da/docs",
        "/es/docs",
        "/pt/docs",
        "/et/docs",
        "/lv/docs",
        "/el/docs",
        "/it/docs",
        "/lt/docs",
        "/hr/docs",
        "/cs/docs",
        "/bs/docs",
        "/bg/docs",
        "/ro/docs",
        "/pl/docs",
        "/ru/docs",
        "/ja/docs",
        "/ko/docs",
        "/hu/docs",
        "/ga/docs",
        "/mt/docs",
        "/la/docs",
        "/en-gb/download",
        "/en-us/docs/cache",
    ];

    for page in pages {
        let (status, body) = get(page).await;
        assert_eq!(status, StatusCode::OK, "{page} should render");

        let links = language_selector_links(&body);
        assert_eq!(links.len(), 34, "{page} should render all language links");
        assert!(
            body.contains(r#"<input class="fh-language-search" type="search""#),
            "{page} should render the language search input"
        );
        assert!(
            body.contains(r#"<div class="fh-language-list">"#),
            "{page} should render the scrollable language list"
        );
        assert!(
            body.contains(r#".fh-language-switcher a[hidden]{display:none!important}"#),
            "{page} should hide filtered language links"
        );
        assert!(
            body.contains(r#"document.querySelectorAll(".fh-language-switcher")"#),
            "{page} should render the language filter script"
        );

        for href in links {
            let (link_status, link_body) = get(&href).await;
            assert_eq!(
                link_status,
                StatusCode::OK,
                "{page} selector link {href} should resolve"
            );
            assert!(
                link_body.contains("fh-language-switcher"),
                "{page} selector link {href} should render a page"
            );
        }
    }
}

#[tokio::test]
async fn english_variant_selector_links_mark_the_selected_locale() {
    for (path, locale_name, lang) in [
        ("/en-gb/download", "English (UK)", "en-GB"),
        ("/en-us/download", "English (US)", "en-US"),
    ] {
        let (status, body) = get(path).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains(&format!(r#"<html lang="{lang}""#)));
        assert!(body.contains(&format!("<span>{locale_name}</span></summary>")));
        assert!(body.contains(r#"aria-current="true""#));
    }
}

#[tokio::test]
async fn locale_prefixed_asset_paths_resolve_for_english_variants() {
    for path in [
        "/assets/css/theme.css?v=20260519",
        "/en-eu/assets/css/theme.css?v=20260519",
        "/en-gb/assets/css/theme.css?v=20260519",
        "/en-us/assets/css/theme.css?v=20260519",
        "/de/assets/css/theme.css?v=20260519",
        "/ch/assets/css/theme.css?v=20260519",
        "/fr/assets/css/theme.css?v=20260519",
        "/sv/assets/css/theme.css?v=20260519",
        "/no/assets/css/theme.css?v=20260519",
        "/nl/assets/css/theme.css?v=20260519",
        "/fi/assets/css/theme.css?v=20260519",
        "/is/assets/css/theme.css?v=20260519",
        "/da/assets/css/theme.css?v=20260519",
        "/es/assets/css/theme.css?v=20260519",
        "/pt/assets/css/theme.css?v=20260519",
        "/et/assets/css/theme.css?v=20260519",
        "/lv/assets/css/theme.css?v=20260519",
        "/el/assets/css/theme.css?v=20260519",
        "/it/assets/css/theme.css?v=20260519",
        "/lt/assets/css/theme.css?v=20260519",
        "/hr/assets/css/theme.css?v=20260519",
        "/cs/assets/css/theme.css?v=20260519",
        "/bs/assets/css/theme.css?v=20260519",
        "/bg/assets/css/theme.css?v=20260519",
        "/ro/assets/css/theme.css?v=20260519",
        "/pl/assets/css/theme.css?v=20260519",
        "/ru/assets/css/theme.css?v=20260519",
        "/ja/assets/css/theme.css?v=20260519",
        "/ko/assets/css/theme.css?v=20260519",
        "/hu/assets/css/theme.css?v=20260519",
        "/ga/assets/css/theme.css?v=20260519",
        "/mt/assets/css/theme.css?v=20260519",
        "/la/assets/css/theme.css?v=20260519",
    ] {
        let (status, body) = get(path).await;
        assert_eq!(status, StatusCode::OK, "{path} should resolve");
        assert!(body.contains("--fh-bg"), "{path} should serve theme CSS");
    }
}

fn language_selector_links(body: &str) -> Vec<String> {
    let Some((_before, selector)) = body.split_once(r#"<details class="fh-language-switcher">"#)
    else {
        return Vec::new();
    };
    let Some((selector, _after)) = selector.split_once("</details>") else {
        return Vec::new();
    };

    selector
        .match_indices(r#"<a href=""#)
        .filter_map(|(index, marker)| {
            let start = index + marker.len();
            let rest = &selector[start..];
            rest.split_once('"').map(|(href, _)| href.to_owned())
        })
        .collect()
}
