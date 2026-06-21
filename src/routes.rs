use std::sync::Arc;

use axum::body::Body;
use axum::extract::{OriginalUri, State};
use axum::http::{Response, StatusCode, header};
use axum::response::{Html, IntoResponse};

use crate::content::Site;
use crate::legacy;

pub async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub async fn render_page(
    State(site): State<Arc<Site>>,
    OriginalUri(uri): OriginalUri,
) -> Response<Body> {
    let path = uri.path();
    if let Some(page) = legacy::render(&site, path) {
        return html_response(StatusCode::OK, Ok(page.html));
    }

    if let Some(artifact) = legacy::render_static_artifact(&site, path) {
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, artifact.content_type)],
            Body::from(artifact.body),
        )
            .into_response();
    }

    html_response(
        StatusCode::NOT_FOUND,
        Ok("<!doctype html><title>Not Found</title><h1>Page not found</h1>".to_owned()),
    )
}

fn html_response(status: StatusCode, rendered: Result<String, String>) -> Response<Body> {
    match rendered {
        Ok(body) => (
            status,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            Html(body),
        )
            .into_response(),
        Err(error) => {
            tracing::error!(%error, "template render failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                "template render failed",
            )
                .into_response()
        }
    }
}
