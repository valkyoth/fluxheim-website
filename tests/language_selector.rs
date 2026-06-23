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
        "/en-gb/download",
        "/en-us/docs/cache",
    ];

    for page in pages {
        let (status, body) = get(page).await;
        assert_eq!(status, StatusCode::OK, "{page} should render");

        let links = language_selector_links(&body);
        assert_eq!(links.len(), 5, "{page} should render all language links");

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
