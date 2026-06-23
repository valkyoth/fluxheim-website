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
        "/fr/docs",
        "/sv/docs",
        "/en-gb/download",
        "/en-us/docs/cache",
    ];

    for page in pages {
        let (status, body) = get(page).await;
        assert_eq!(status, StatusCode::OK, "{page} should render");

        let links = language_selector_links(&body);
        assert_eq!(links.len(), 6, "{page} should render all language links");

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
        "/fr/assets/css/theme.css?v=20260519",
        "/sv/assets/css/theme.css?v=20260519",
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
