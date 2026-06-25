use std::env;
use std::time::Duration;

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;

use crate::content::Site;

const SERVICE_NAME: &str = "fluxheim-website";
const UNKNOWN: &str = "unknown";
const COUNTRY_UNKNOWN: &str = "unknown";
const MAX_VISIBLE_SECONDS: u64 = 86_400;

#[derive(Clone)]
pub struct Observability {
    enabled: bool,
    requests: Counter<u64>,
    page_views: Counter<u64>,
    download_clicks: Counter<u64>,
    outbound_clicks: Counter<u64>,
    request_duration: Histogram<f64>,
    page_visible: Histogram<f64>,
}

#[derive(Debug, Default)]
pub struct TelemetryGuard {
    meter_provider: Option<SdkMeterProvider>,
    tracer_provider: Option<SdkTracerProvider>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestLabels {
    pub locale: String,
    pub route: String,
    pub section: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisiblePageEvent {
    pub locale: String,
    pub route: String,
    pub section: String,
    pub seconds: u64,
}

impl Observability {
    pub fn disabled() -> Self {
        Self::from_meter(false)
    }

    pub fn from_env(service_version: &str) -> (Self, TelemetryGuard) {
        if !otlp_enabled_from_env() {
            return (Self::disabled(), TelemetryGuard::default());
        }

        match init_providers(service_version) {
            Ok(guard) => (Self::from_meter(true), guard),
            Err(error) => {
                tracing::warn!(%error, "OpenTelemetry disabled after initialization failure");
                (Self::disabled(), TelemetryGuard::default())
            }
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn classify_request(site: &Site, path: &str) -> RequestLabels {
        let clean = path
            .split_once('?')
            .map_or(path, |(without_query, _query)| without_query)
            .trim_matches('/');
        let (locale, slug) = site.split_path(clean);
        RequestLabels {
            locale: locale.locale_id.clone(),
            route: normalize_route(slug),
            section: section_for_slug(slug).to_owned(),
            country: COUNTRY_UNKNOWN.to_owned(),
        }
    }

    pub fn validate_visible_event(
        site: &Site,
        event: VisiblePageEvent,
    ) -> Option<VisiblePageEvent> {
        site.locale(&event.locale)?;
        if !is_allowed_route(&event.route) || !is_allowed_section(&event.section) {
            return None;
        }
        Some(VisiblePageEvent {
            seconds: event.seconds.min(MAX_VISIBLE_SECONDS),
            ..event
        })
    }

    pub fn record_request(
        &self,
        labels: &RequestLabels,
        status: http::StatusCode,
        elapsed: Duration,
    ) {
        if !self.enabled {
            return;
        }
        let status_class = status_class(status);
        let attributes = request_attributes(labels, status_class);
        self.requests.add(1, &attributes);
        self.request_duration
            .record(elapsed.as_secs_f64(), &attributes);
    }

    pub fn record_page_view(&self, labels: &RequestLabels) {
        if !self.enabled || !is_page_view_section(&labels.section) {
            return;
        }
        self.page_views.add(
            1,
            &[
                KeyValue::new("locale", labels.locale.clone()),
                KeyValue::new("route", labels.route.clone()),
                KeyValue::new("section", labels.section.clone()),
                KeyValue::new("country", labels.country.clone()),
            ],
        );
    }

    pub fn record_outbound_click(&self, locale: &str, target: &str) {
        if !self.enabled {
            return;
        }
        self.outbound_clicks.add(
            1,
            &[
                KeyValue::new("locale", locale.to_owned()),
                KeyValue::new("target", target.to_owned()),
                KeyValue::new("country", COUNTRY_UNKNOWN),
            ],
        );
    }

    pub fn record_download_click(&self, locale: &str, artifact: &str) {
        if !self.enabled {
            return;
        }
        self.download_clicks.add(
            1,
            &[
                KeyValue::new("locale", locale.to_owned()),
                KeyValue::new("artifact", artifact.to_owned()),
                KeyValue::new("country", COUNTRY_UNKNOWN),
            ],
        );
    }

    pub fn record_visible_page(&self, event: &VisiblePageEvent) {
        if !self.enabled {
            return;
        }
        self.page_visible.record(
            event.seconds as f64,
            &[
                KeyValue::new("locale", event.locale.clone()),
                KeyValue::new("route", event.route.clone()),
                KeyValue::new("section", event.section.clone()),
            ],
        );
    }

    fn from_meter(enabled: bool) -> Self {
        let meter = global::meter(SERVICE_NAME);
        Self {
            enabled,
            requests: meter.u64_counter("fluxheim_website_requests_total").build(),
            page_views: meter
                .u64_counter("fluxheim_website_page_views_total")
                .build(),
            download_clicks: meter
                .u64_counter("fluxheim_website_download_clicks_total")
                .build(),
            outbound_clicks: meter
                .u64_counter("fluxheim_website_outbound_clicks_total")
                .build(),
            request_duration: meter
                .f64_histogram("fluxheim_website_request_duration_seconds")
                .build(),
            page_visible: meter
                .f64_histogram("fluxheim_website_page_visible_seconds")
                .build(),
        }
    }
}

impl TelemetryGuard {
    pub fn tracer_provider(&self) -> Option<&SdkTracerProvider> {
        self.tracer_provider.as_ref()
    }

    pub fn shutdown(self) {
        if let Some(provider) = self.meter_provider
            && let Err(error) = provider.shutdown()
        {
            tracing::warn!(?error, "failed to shutdown meter provider");
        }
        if let Some(provider) = self.tracer_provider
            && let Err(error) = provider.shutdown()
        {
            tracing::warn!(?error, "failed to shutdown tracer provider");
        }
    }
}

fn init_providers(
    service_version: &str,
) -> Result<TelemetryGuard, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let resource = telemetry_resource(service_version);
    let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()?;
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(metric_exporter)
        .build();
    global::set_meter_provider(meter_provider.clone());

    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()?;
    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(span_exporter)
        .build();
    global::set_tracer_provider(tracer_provider.clone());

    Ok(TelemetryGuard {
        meter_provider: Some(meter_provider),
        tracer_provider: Some(tracer_provider),
    })
}

fn telemetry_resource(service_version: &str) -> Resource {
    Resource::builder()
        .with_service_name(SERVICE_NAME)
        .with_attribute(KeyValue::new("service.version", service_version.to_owned()))
        .build()
}

pub fn normalize_route(slug: &str) -> String {
    let clean = slug.trim_matches('/');
    match clean {
        "" => "/".to_owned(),
        "download" | "download.html" => "/download".to_owned(),
        "changelog" | "changelog.html" => "/changelog".to_owned(),
        "cookies" | "cookies.html" => "/cookies".to_owned(),
        "privacy" | "privacy.html" => "/privacy".to_owned(),
        "gdpr" | "gdpr.html" => "/gdpr".to_owned(),
        "docs" | "docs/index" | "docs/index.html" => "/docs".to_owned(),
        value if value.starts_with("docs/source/") => "/docs/source/:page".to_owned(),
        value if value.starts_with("docs/") => {
            let first = value.trim_start_matches("docs/").trim_end_matches(".html");
            if first.contains('/') {
                "/docs/:page".to_owned()
            } else {
                format!("/docs/{first}")
            }
        }
        value if value.starts_with("assets/") => "/assets/:file".to_owned(),
        value if value.starts_with("out/") => "/out/:target".to_owned(),
        value if value.starts_with("telemetry/") => "/telemetry/:event".to_owned(),
        _ => UNKNOWN.to_owned(),
    }
}

pub fn section_for_slug(slug: &str) -> &'static str {
    let clean = slug.trim_matches('/');
    match clean {
        "" => "home",
        "download" | "download.html" => "download",
        "changelog" | "changelog.html" => "changelog",
        "cookies" | "cookies.html" | "privacy" | "privacy.html" | "gdpr" | "gdpr.html" => "legal",
        value if value.starts_with("docs/source/") => "source_docs",
        value if value.starts_with("docs") => "docs",
        value if value.starts_with("assets/") => "static",
        value if value.starts_with("out/") => "outbound",
        value if value.starts_with("telemetry/") => "telemetry",
        _ => "not_found",
    }
}

fn request_attributes(labels: &RequestLabels, status_class: &'static str) -> Vec<KeyValue> {
    vec![
        KeyValue::new("locale", labels.locale.clone()),
        KeyValue::new("route", labels.route.clone()),
        KeyValue::new("section", labels.section.clone()),
        KeyValue::new("status_class", status_class),
        KeyValue::new("country", labels.country.clone()),
    ]
}

fn otlp_enabled_from_env() -> bool {
    otlp_enabled_from_vars(|name| env::var(name).ok())
}

fn otlp_enabled_from_vars(mut value_for: impl FnMut(&str) -> Option<String>) -> bool {
    for name in [
        "FLUXHEIM_OTLP",
        "FLUXHEIM_OTLP_ENABLED",
        "FLUXHEIM_OTEL_ENABLED",
    ] {
        if let Some(value) = value_for(name).and_then(|value| parse_env_switch(&value)) {
            return value;
        }
    }
    false
}

fn parse_env_switch(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "enabled" | "enable" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" | "disable" => Some(false),
        _ => None,
    }
}

fn status_class(status: http::StatusCode) -> &'static str {
    match status.as_u16() {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => UNKNOWN,
    }
}

fn is_page_view_section(section: &str) -> bool {
    matches!(
        section,
        "home" | "docs" | "download" | "changelog" | "source_docs" | "legal"
    )
}

fn is_allowed_route(route: &str) -> bool {
    matches!(
        route,
        "/" | "/download"
            | "/changelog"
            | "/cookies"
            | "/privacy"
            | "/gdpr"
            | "/docs"
            | "/docs/getting-started"
            | "/docs/deployment"
            | "/docs/configuration"
            | "/docs/features"
            | "/docs/cache"
            | "/docs/tls-acme"
            | "/docs/observability"
            | "/docs/advanced"
            | "/docs/reference"
            | "/docs/source/:page"
            | "/docs/:page"
    )
}

fn is_allowed_section(section: &str) -> bool {
    matches!(
        section,
        "home" | "docs" | "download" | "changelog" | "source_docs" | "legal"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        Observability, VisiblePageEvent, normalize_route, otlp_enabled_from_vars, section_for_slug,
    };
    use crate::content::Site;
    use std::collections::BTreeMap;

    #[test]
    fn normalizes_routes_without_query_or_raw_unknown_paths() {
        assert_eq!(normalize_route(""), "/");
        assert_eq!(normalize_route("docs/cache"), "/docs/cache");
        assert_eq!(normalize_route("docs/source/systemd"), "/docs/source/:page");
        assert_eq!(normalize_route("private/random-token"), "unknown");
    }

    #[test]
    fn classifies_sections_from_bounded_slugs() {
        assert_eq!(section_for_slug(""), "home");
        assert_eq!(section_for_slug("download"), "download");
        assert_eq!(section_for_slug("docs/cache"), "docs");
        assert_eq!(section_for_slug("docs/source/systemd"), "source_docs");
        assert_eq!(section_for_slug("unknown/path"), "not_found");
    }

    #[test]
    fn request_labels_ignore_query_strings() {
        let site = Site::load().expect("site loads");
        let labels = Observability::classify_request(&site, "/de/docs/cache?secret=value");
        assert_eq!(labels.locale, "de-DE");
        assert_eq!(labels.route, "/docs/cache");
        assert_eq!(labels.section, "docs");
    }

    #[test]
    fn visible_page_events_are_allow_listed_and_clamped() {
        let site = Site::load().expect("site loads");
        let event = VisiblePageEvent {
            locale: "fr-FR".to_owned(),
            route: "/docs/cache".to_owned(),
            section: "docs".to_owned(),
            seconds: 999_999,
        };
        let validated = Observability::validate_visible_event(&site, event).expect("valid event");
        assert_eq!(validated.seconds, 86_400);

        let invalid = VisiblePageEvent {
            locale: "fr-FR".to_owned(),
            route: "/private/raw".to_owned(),
            section: "docs".to_owned(),
            seconds: 1,
        };
        assert!(Observability::validate_visible_event(&site, invalid).is_none());
    }

    #[test]
    fn otlp_runtime_switch_accepts_enabled_disabled_values() {
        assert!(otlp_enabled(&[("FLUXHEIM_OTLP", "enabled")]));
        assert!(otlp_enabled(&[("FLUXHEIM_OTLP_ENABLED", "true")]));
        assert!(otlp_enabled(&[("FLUXHEIM_OTEL_ENABLED", "1")]));
        assert!(!otlp_enabled(&[("FLUXHEIM_OTLP", "disabled")]));
        assert!(!otlp_enabled(&[("FLUXHEIM_OTLP_ENABLED", "off")]));
        assert!(!otlp_enabled(&[("FLUXHEIM_OTLP", "maybe")]));
        assert!(!otlp_enabled(&[]));
    }

    #[test]
    fn otlp_runtime_switch_prefers_new_name_over_legacy_alias() {
        assert!(!otlp_enabled(&[
            ("FLUXHEIM_OTLP", "disabled"),
            ("FLUXHEIM_OTEL_ENABLED", "true"),
        ]));
        assert!(otlp_enabled(&[
            ("FLUXHEIM_OTLP", "enabled"),
            ("FLUXHEIM_OTEL_ENABLED", "false"),
        ]));
    }

    fn otlp_enabled(vars: &[(&str, &str)]) -> bool {
        let vars = vars
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect::<BTreeMap<_, _>>();
        otlp_enabled_from_vars(|name| vars.get(name).cloned())
    }
}
