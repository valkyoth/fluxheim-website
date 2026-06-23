use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{get, post};
use http::HeaderValue;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::Instrument;

use crate::app_state::AppState;
use crate::content::Site;
use crate::observability::Observability;
use crate::routes::{
    download_outbound, github_outbound, healthz, page_visible, render_page, telemetry_click,
};

const CONTENT_SECURITY_POLICY: &str = concat!(
    "default-src 'self'; ",
    "img-src 'self' data:; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "style-src 'self' 'unsafe-inline'; ",
    "object-src 'none'; ",
    "base-uri 'self'; ",
    "frame-ancestors 'none'"
);

pub fn build_router(site: Arc<Site>) -> Router {
    build_router_with_observability(site, Observability::disabled())
}

pub fn build_router_with_observability(site: Arc<Site>, observability: Observability) -> Router {
    let state = Arc::new(AppState::new((*site).clone(), observability));
    Router::new()
        .route("/healthz", get(healthz))
        .route("/out/github/{target}", get(github_outbound))
        .route("/out/download/{artifact}", get(download_outbound))
        .route("/telemetry/page-visible", post(page_visible))
        .route("/telemetry/click", post(telemetry_click))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/de/assets", ServeDir::new("assets"))
        .nest_service("/fr/assets", ServeDir::new("assets"))
        .nest_service("/en-eu/assets", ServeDir::new("assets"))
        .fallback(render_page)
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state, observe_request))
        .layer(TraceLayer::new_for_http())
        .layer(static_header("x-content-type-options", "nosniff"))
        .layer(static_header("x-frame-options", "DENY"))
        .layer(static_header(
            "referrer-policy",
            "strict-origin-when-cross-origin",
        ))
        .layer(static_header(
            "permissions-policy",
            "camera=(), microphone=(), geolocation=(), payment=(), usb=()",
        ))
        .layer(static_header(
            "content-security-policy",
            CONTENT_SECURITY_POLICY,
        ))
}

async fn observe_request(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let labels = Observability::classify_request(&state.site, &path);
    let span = tracing::info_span!(
        "http.request",
        http.route = %labels.route,
        fluxheim.locale = %labels.locale,
        fluxheim.section = %labels.section
    );
    let started = Instant::now();
    let response = next.run(request).instrument(span).await;
    let status = response.status();

    state
        .observability
        .record_request(&labels, status, started.elapsed());
    if method == http::Method::GET && status.is_success() {
        state.observability.record_page_view(&labels);
    }

    response
}

fn static_header(name: &'static str, value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        http::HeaderName::from_static(name),
        HeaderValue::from_static(value),
    )
}
