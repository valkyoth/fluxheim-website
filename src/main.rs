#![forbid(unsafe_code)]

use std::io;
use std::net::{IpAddr, SocketAddr};
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
    if startup_probe_requested() {
        telemetry_guard.shutdown();
        return Ok(());
    }
    let addr = bind_addr()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!(%addr, "starting fluxheim-website");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    telemetry_guard.shutdown();
    Ok(())
}

fn startup_probe_requested() -> bool {
    std::env::args_os().any(|argument| argument == "--startup-probe")
}

fn validate_embedded_i18n(site: &Site) {
    let locale = site.default_locale();
    let _ = fluxheim_website::i18n_keys::apply_shared_keys(
        locale,
        String::new(),
        &site.config.fluxheim_version,
    );
}

fn bind_addr() -> io::Result<SocketAddr> {
    bind_addr_from(
        std::env::var("FLUXHEIM_WEBSITE_BIND").ok().as_deref(),
        std::env::var("FLUXHEIM_WEBSITE_PORT").ok().as_deref(),
    )
}

fn bind_addr_from(host: Option<&str>, port: Option<&str>) -> io::Result<SocketAddr> {
    let host = host
        .unwrap_or("127.0.0.1")
        .parse::<IpAddr>()
        .map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid bind IP: {error}"),
            )
        })?;
    let port = port.unwrap_or("8080").parse::<u16>().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid bind port: {error}"),
        )
    })?;
    Ok(SocketAddr::new(host, port))
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

#[cfg(test)]
mod tests {
    use super::bind_addr_from;

    #[test]
    fn defaults_native_server_to_loopback() {
        assert_eq!(
            bind_addr_from(None, None)
                .expect("default bind")
                .to_string(),
            "127.0.0.1:8080"
        );
    }

    #[test]
    fn accepts_explicit_container_bind() {
        assert_eq!(
            bind_addr_from(Some("0.0.0.0"), Some("18080"))
                .expect("container bind")
                .to_string(),
            "0.0.0.0:18080"
        );
    }
}
