use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router;
use fluxheim_website::legacy::{LOCALE_PREFIXES, legacy_html_paths, slug_for_path};
use tower::ServiceExt;

const SOURCE_FLUXHEIM_VERSION: &str = "1.6.28";

#[tokio::test]
async fn all_legacy_html_pages_are_served_for_each_locale_prefix() {
    let site = Arc::new(Site::load().expect("site content loads"));
    let app = build_router(site.clone());

    for path in legacy_html_paths() {
        let slug = slug_for_path(&path).expect("legacy slug");
        let source = std::fs::read_to_string(&path).expect("read legacy html");
        let title = extract_title(&source).expect("legacy html title");

        for prefix in LOCALE_PREFIXES {
            let uri = localized_uri(prefix, &slug);
            let body = get_body(app.clone(), &uri).await;
            if prefix == "en-eu" {
                assert!(
                    body.contains(&title),
                    "{uri} did not preserve title {title:?}"
                );
                assert_preserves_source_html(&source, &body, &uri, &site.config.fluxheim_version);
            } else {
                assert!(
                    body.contains("Fluxheim"),
                    "{uri} lost the Fluxheim page structure"
                );
            }
            assert!(
                body.contains("fh-language-switcher"),
                "{uri} missing language selector"
            );
        }
    }
}

async fn get_body(app: axum::Router, uri: &str) -> String {
    let response = app
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request builds"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK, "{uri}");
    let body = to_bytes(response.into_body(), 2 * 1024 * 1024)
        .await
        .expect("body bytes");
    String::from_utf8(body.to_vec()).expect("utf8 body")
}

fn localized_uri(prefix: &str, slug: &str) -> String {
    if prefix == "en-eu" {
        if slug.is_empty() {
            "/".to_owned()
        } else {
            format!("/{slug}")
        }
    } else if slug.is_empty() {
        format!("/{prefix}/")
    } else {
        format!("/{prefix}/{slug}")
    }
}

fn extract_title(html: &str) -> Option<String> {
    let start = html.find("<title>")? + "<title>".len();
    let end = html[start..].find("</title>")? + start;
    Some(html[start..end].to_owned())
}

fn assert_preserves_source_html(source: &str, served: &str, uri: &str, version: &str) {
    let source = source.replace(
        &format!("v{SOURCE_FLUXHEIM_VERSION}"),
        &format!("v{version}"),
    );
    let body_index = source
        .rfind("</body>")
        .unwrap_or_else(|| panic!("{uri} source missing </body>"));
    let prefix = &source[..body_index];
    let suffix = &source[body_index..];

    assert!(
        served.starts_with(prefix),
        "{uri} changed legacy HTML before injected selector"
    );
    assert!(
        served.ends_with(suffix),
        "{uri} changed legacy HTML after injected selector"
    );
}
