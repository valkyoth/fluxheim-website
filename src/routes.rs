use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::{Response, StatusCode, header};
use axum::response::{Html, IntoResponse, Redirect};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::legacy;
use crate::observability::{Observability, VisiblePageEvent};
use crate::page_cache;

pub async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub async fn render_page(
    State(state): State<Arc<AppState>>,
    OriginalUri(uri): OriginalUri,
) -> Response<Body> {
    let path = uri.path();
    if let Some(page) = page_cache::cached_page(&state.site, &state.pages, path) {
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            Body::from(page.body.clone()),
        )
            .into_response();
    }

    if let Some(artifact) = legacy::render_static_artifact(&state.site, path) {
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

#[derive(Debug, Deserialize)]
pub struct ClickQuery {
    locale: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VisiblePayload {
    locale: String,
    route: String,
    section: String,
    seconds: u64,
}

pub async fn github_outbound(
    State(state): State<Arc<AppState>>,
    Path(target): Path<String>,
    Query(query): Query<ClickQuery>,
) -> Response<Body> {
    let Some(url) = github_target_url(&state, &target) else {
        return html_response(
            StatusCode::NOT_FOUND,
            Ok(
                "<!doctype html><title>Not Found</title><h1>Unknown outbound target</h1>"
                    .to_owned(),
            ),
        );
    };
    let locale = click_locale(&state, query.locale.as_deref());
    state.observability.record_outbound_click(locale, &target);
    Redirect::temporary(url).into_response()
}

pub async fn download_outbound(
    State(state): State<Arc<AppState>>,
    Path(artifact): Path<String>,
    Query(query): Query<ClickQuery>,
) -> Response<Body> {
    let Some(url) = download_target_url(&state, &artifact) else {
        return html_response(
            StatusCode::NOT_FOUND,
            Ok(
                "<!doctype html><title>Not Found</title><h1>Unknown download artifact</h1>"
                    .to_owned(),
            ),
        );
    };
    let locale = click_locale(&state, query.locale.as_deref());
    state.observability.record_download_click(locale, &artifact);
    Redirect::temporary(&url).into_response()
}

pub async fn page_visible(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<VisiblePayload>,
) -> Response<Body> {
    let event = VisiblePageEvent {
        locale: payload.locale,
        route: payload.route,
        section: payload.section,
        seconds: payload.seconds,
    };
    let Some(event) = Observability::validate_visible_event(&state.site, event) else {
        return (StatusCode::BAD_REQUEST, "invalid page-visible event").into_response();
    };
    state.observability.record_visible_page(&event);
    (StatusCode::ACCEPTED, "ok").into_response()
}

fn download_target_url(state: &AppState, artifact: &str) -> Option<String> {
    if !state.download_artifacts.contains(artifact) {
        return None;
    }
    let version = download_artifact_version(artifact)?;
    Some(format!(
        "{}/download/v{}/{}",
        state.site.config.releases_url, version, artifact
    ))
}

fn download_artifact_version(artifact: &str) -> Option<&str> {
    let allowed_suffixes = [
        "full-x86_64-linux.tar.gz",
        "full-aarch64-linux.tar.gz",
        "full-aarch64-macos.tar.gz",
        "wasm-x86_64-linux.tar.gz",
        "wasm-aarch64-linux.tar.gz",
        "wasm-aarch64-macos.tar.gz",
        "php-x86_64-linux.tar.gz",
        "php-aarch64-linux.tar.gz",
        "php-aarch64-macos.tar.gz",
        "load-balancer-x86_64-linux.tar.gz",
        "load-balancer-aarch64-linux.tar.gz",
        "load-balancer-aarch64-macos.tar.gz",
        "cache-x86_64-linux.tar.gz",
        "cache-aarch64-linux.tar.gz",
        "cache-aarch64-macos.tar.gz",
        "proxy-x86_64-linux.tar.gz",
        "proxy-aarch64-linux.tar.gz",
        "proxy-aarch64-macos.tar.gz",
        "config-tester-x86_64-linux.tar.gz",
        "config-tester-aarch64-linux.tar.gz",
        "config-tester-aarch64-macos.tar.gz",
    ];
    let version = allowed_suffixes.iter().find_map(|suffix| {
        artifact
            .strip_prefix("fluxheim-")?
            .strip_suffix(&format!("-{suffix}"))
    })?;
    if version.is_empty()
        || version
            .split('.')
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    Some(version)
}

fn github_target_url<'a>(state: &'a AppState, target: &str) -> Option<&'a str> {
    match target {
        "repo" => Some(&state.site.config.github_url),
        "releases" => Some(&state.site.config.releases_url),
        _ => None,
    }
}

fn click_locale<'a>(state: &'a AppState, locale: Option<&'a str>) -> &'a str {
    locale
        .filter(|locale_id| state.site.locale(locale_id).is_some())
        .unwrap_or(&state.site.config.default_locale)
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
