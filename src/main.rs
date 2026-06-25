use std::net::SocketAddr;
use std::sync::Arc;

use fluxheim_website::content::Site;
use fluxheim_website::http_app::build_router_with_observability;
use fluxheim_website::observability::TelemetryGuard;
use opentelemetry::trace::TracerProvider as _;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let site = Site::load()?;
    validate_embedded_i18n(&site);
    let (observability, telemetry_guard) =
        fluxheim_website::observability::Observability::from_env(&site.config.fluxheim_version);
    init_tracing(&telemetry_guard);

    let app = build_router_with_observability(Arc::new(site), observability);
    let addr = bind_addr()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!(%addr, "starting fluxheim-website");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    telemetry_guard.shutdown();
    Ok(())
}

fn validate_embedded_i18n(site: &Site) {
    let locale = site.default_locale();
    let _ = fluxheim_website::i18n_keys::apply_shared_keys(
        locale,
        String::new(),
        &site.config.fluxheim_version,
    );
}

fn bind_addr() -> Result<SocketAddr, std::num::ParseIntError> {
    let port = std::env::var("FLUXHEIM_WEBSITE_PORT")
        .ok()
        .map(|value| value.parse::<u16>())
        .transpose()?
        .unwrap_or(8080);
    Ok(SocketAddr::from(([0, 0, 0, 0], port)))
}

fn init_tracing(telemetry_guard: &TelemetryGuard) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("fluxheim_website=info,tower_http=info"));

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    if let Some(provider) = telemetry_guard.tracer_provider() {
        let tracer = provider.tracer("fluxheim-website");
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::warn!(%error, "failed to listen for ctrl-c");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => tracing::warn!(%error, "failed to listen for sigterm"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
