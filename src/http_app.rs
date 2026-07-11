use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::extract::{Request, State};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{get, post};
use http::{HeaderValue, StatusCode};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::Instrument;

use crate::app_state::AppState;
use crate::content::Site;
use crate::observability::Observability;
use crate::routes::{download_outbound, github_outbound, healthz, page_visible, render_page};

const CONTENT_SECURITY_POLICY: &str = concat!(
    "default-src 'self'; ",
    "img-src 'self' data:; ",
    "font-src 'self'; ",
    "connect-src 'self'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "style-src 'self' 'unsafe-inline'; ",
    "object-src 'none'; ",
    "base-uri 'none'; ",
    "form-action 'self'; ",
    "frame-ancestors 'none'"
);
const HSTS_HEADER_VALUE: &str = "max-age=31536000; includeSubDomains; preload";
const TELEMETRY_RATE_LIMIT_PER_SECOND: u32 = 120;

pub fn build_router(site: Arc<Site>) -> Router {
    build_router_with_observability(site, Observability::disabled())
}

pub fn build_router_with_observability(site: Arc<Site>, observability: Observability) -> Router {
    let state = Arc::new(
        AppState::new((*site).clone(), observability)
            .expect("all configured website pages must render during startup"),
    );
    let mut router = Router::new()
        .route("/healthz", get(healthz))
        .route("/out/github/{target}", get(github_outbound))
        .route("/out/download/{artifact}", get(download_outbound))
        .route(
            "/telemetry/page-visible",
            post(page_visible)
                .layer(DefaultBodyLimit::max(4096))
                .layer(middleware::from_fn(limit_telemetry_rate)),
        )
        .nest_service("/assets", ServeDir::new("assets"));

    for locale in site.locales() {
        router = router.nest_service(
            format!("/{}/assets", locale.url_prefix).as_str(),
            ServeDir::new("assets"),
        );
    }

    let router = router
        .fallback(render_page)
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state, observe_request))
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
        ));

    if hsts_enabled_from_env() {
        router.layer(static_header(
            "strict-transport-security",
            HSTS_HEADER_VALUE,
        ))
    } else {
        router
    }
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

async fn limit_telemetry_rate(request: Request<Body>, next: Next) -> Response {
    if telemetry_rate_limited(Instant::now()) {
        return Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("telemetry rate limit exceeded"))
            .expect("static telemetry rate limit response");
    }
    next.run(request).await
}

fn telemetry_rate_limited(now: Instant) -> bool {
    static LIMITER: OnceLock<Mutex<TelemetryRateLimiter>> = OnceLock::new();
    let limiter = LIMITER.get_or_init(|| Mutex::new(TelemetryRateLimiter::new(now)));
    let Ok(mut limiter) = limiter.lock() else {
        return true;
    };
    !limiter.allow(now)
}

#[derive(Debug)]
struct TelemetryRateLimiter {
    window_start: Instant,
    count: u32,
}

impl TelemetryRateLimiter {
    fn new(now: Instant) -> Self {
        Self {
            window_start: now,
            count: 0,
        }
    }

    fn allow(&mut self, now: Instant) -> bool {
        if now.duration_since(self.window_start) >= Duration::from_secs(1) {
            self.window_start = now;
            self.count = 0;
        }
        if self.count >= TELEMETRY_RATE_LIMIT_PER_SECOND {
            return false;
        }
        self.count += 1;
        true
    }
}

fn hsts_enabled_from_env() -> bool {
    std::env::var("FLUXHEIM_HSTS")
        .ok()
        .and_then(|value| parse_env_switch(&value))
        .unwrap_or(true)
}

fn parse_env_switch(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "enabled" | "enable" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" | "disable" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{TELEMETRY_RATE_LIMIT_PER_SECOND, TelemetryRateLimiter, parse_env_switch};
    use std::time::{Duration, Instant};

    #[test]
    fn telemetry_rate_limiter_resets_after_window() {
        let now = Instant::now();
        let mut limiter = TelemetryRateLimiter::new(now);
        for _ in 0..TELEMETRY_RATE_LIMIT_PER_SECOND {
            assert!(limiter.allow(now));
        }
        assert!(!limiter.allow(now));
        assert!(limiter.allow(now + Duration::from_secs(1)));
    }

    #[test]
    fn parses_security_header_switches() {
        assert_eq!(parse_env_switch("enabled"), Some(true));
        assert_eq!(parse_env_switch("disabled"), Some(false));
        assert_eq!(parse_env_switch("maybe"), None);
    }
}
