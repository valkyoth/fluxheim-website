use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use http::HeaderValue;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

use crate::content::Site;
use crate::routes::{healthz, render_page};

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
    Router::new()
        .route("/healthz", get(healthz))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/de/assets", ServeDir::new("assets"))
        .nest_service("/fr/assets", ServeDir::new("assets"))
        .nest_service("/en-eu/assets", ServeDir::new("assets"))
        .fallback(render_page)
        .with_state(site)
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

fn static_header(name: &'static str, value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        http::HeaderName::from_static(name),
        HeaderValue::from_static(value),
    )
}
