use super::{KeyFile, home, versioned};

pub(super) fn apply_keys(keys: &KeyFile, source: &KeyFile, html: String, version: &str) -> String {
    let is_download_page = html.contains("Download — Fluxheim");
    let is_changelog_page = html.contains("Changelog — Fluxheim");
    let is_runtime_parity_fixtures_page =
        html.contains("Runtime Parity Fixtures — Fluxheim Source Docs");
    let is_geoip_page = html.contains("GeoIP / Geo-Context — Fluxheim Source Docs");
    let is_load_balancer_ha_page =
        html.contains("Load Balancer HA Design Notes — Fluxheim Source Docs");
    let is_getting_started_page = html.contains("Installation & Quick Start — Fluxheim Docs");
    let is_cache_page = html.contains("Cache System — Fluxheim Docs");
    let is_extraction_dependency_graph_page =
        html.contains("Extraction Dependency Graph — Fluxheim Source Docs");
    let is_runtime_baseline_page = html.contains("Runtime Baseline — Fluxheim Source Docs");
    let is_modularity_policy_page = html.contains("Modularity Policy — Fluxheim Source Docs");
    let is_observability_page = html.contains("Observability — Fluxheim Docs");
    let is_release_notes_template_page =
        html.contains("Release Notes Template — Fluxheim Source Docs");
    let is_tls_acme_page = html.contains("TLS & ACME — Fluxheim Docs");
    let is_owasp_baseline_page = html.contains("OWASP Top 10 2025 Baseline — Fluxheim Source Docs");
    let is_macos_development_page =
        html.contains("macOS Development Support — Fluxheim Source Docs");
    let is_gateway_recipes_page = html.contains("Gateway Recipes — Fluxheim Source Docs");

    let html = html.replace(
        "Fluxheim — Memory-Safe Edge Server Built in Rust",
        &keys.shell.home_title,
    )
    .replace("Latest Stable Release", &keys.release.latest_stable_release)
    .replace("Switch color theme", &keys.shell.switch_color_theme)
    .replace(
        ">View on GitHub<",
        &format!(">{}<", keys.shell.view_on_github),
    )
    .replace(">Quick Start<", &format!(">{}<", keys.shell.quick_start))
    .replace(">Links<", &format!(">{}<", keys.shell.links))
    .replace(
        ">GitHub Repository<",
        &format!(">{}<", keys.shell.github_repository),
    )
    .replace(">Issues<", &format!(">{}<", keys.shell.issues))
    .replace(">Menu<", &format!(">{}<", keys.shell.menu))
    .replace(
        "Memory-safe edge server built in Rust. Licensed under EUPL-1.2.",
        &keys.footer.tagline,
    )
    .replace(">Project<", &format!(">{}<", keys.footer.project))
    .replace(">Releases<", &format!(">{}<", keys.footer.releases))
    .replace(">Roadmap<", &format!(">{}<", keys.footer.roadmap))
    .replace(">Community<", &format!(">{}<", keys.footer.community))
    .replace(">Discussions<", &format!(">{}<", keys.footer.discussions))
    .replace(
        ">EUPL-1.2 License<",
        &format!(">{}<", keys.footer.eupl_license),
    )
    .replace(">Valkyoth Org<", &format!(">{}<", keys.footer.valkyoth_org))
    .replace(
        "© 2026 Valkyoth. Distributed under the",
        &keys.footer.copyright_prefix,
    )
    .replace(
        "Built with Rust · Powered by Fluxheim",
        &keys.footer.built_with,
    )
    .replace(
        "Fluxheim is a high-performance, modular web server, reverse proxy and caching server built in Rust.",
        home(keys, "meta_description"),
    )
    .replace(
        "Fluxheim is a high-performance, modular web server, reverse proxy and caching server. Written in Rust. Secure by default.",
        home(keys, "meta_description_secure"),
    )
    .replace(
        "A memory-safe edge server and reverse proxy built in Rust.",
        home(keys, "meta_description_short"),
    )
    .replace(
        "Written in Rust with a pinned stable toolchain. No buffer overflows, no use-after-free, no data races by construction.",
        home(keys, "memory_safe_by_design_text"),
    )
    .replace(
        "A Rust-native edge runtime with connection pooling, upstream retries, active health checks, HTTP/2, WebSocket upgrades, and gRPC pass-through.",
        home(keys, "fluxheim_http_core_text"),
    )
    .replace(
        "Focused 1.5 load-balancer binary and image with advanced selection, local persistence, health/ejection policy, bounded queueing, and runtime member controls.",
        home(keys, "load_balancer_control_plane_text"),
    )
    .replace(
        "Compile only what you need. Focused profiles for static site, cache edge, reverse proxy, load balancing, TCP stream proxying, PHP-FPM, GeoIP, traffic mirroring, and compression-enabled production bundles.",
        home(keys, "modular_build_profiles_text"),
    )
    .replace(
        "rustls-first with supported OpenSSL and FIPS/ISO proof build paths, client certificate auth, upstream mTLS, automatic ACME issuance, and multi-cert SNI.",
        home(keys, "tls_managed_acme_text"),
    )
    .replace(
        "Memory, disk, tiered, and encrypted cache backends with cache-safe gzip, Zstandard, and Brotli compression plus range caching for large objects.",
        home(keys, "advanced_cache_system_text"),
    )
    .replace(
        "Rootless Podman images for Wolfi, Alpine, SUSE Micro, and Debian. Systemd/RPM for native deployments. Zero external assets on startup.",
        home(keys, "container_native_text"),
    )
    .replace(
        "Opt-in Prometheus metrics listener, OTLP metrics export, trace context propagation, and OTLP trace export for full observability.",
        home(keys, "prometheus_opentelemetry_text"),
    )
    .replace(
        "Optional local MMDB lookups for country and ASN policy using MaxMind GeoIP2/GeoLite2 or CIRCL Geo Open datasets. No remote lookup or downloader in the request path.",
        home(keys, "geo_context_text"),
    )
    .replace(
        "Raw L4 TCP services with dedicated stream routes, true idle/lifetime/byte caps, upstream TLS/mTLS controls, weighted/drain/backup policy, and route-local PROXY protocol.",
        home(keys, "tcp_stream_proxy_text"),
    )
    .replace(
        "Opt-in PHP-FPM FastCGI bridge for WordPress-style front-controller applications. Strict script resolution and bounded request handling.",
        home(keys, "php_fpm_support_text"),
    )
    .replace(
        "Trusted-proxy-aware ACLs, rate limits, auth subrequests, traffic mirroring, regex rewrites, bounded queues, strict config validation, and hardened request handling.",
        home(keys, "edge_policy_controls_text"),
    )
    .replace(
        "Built for operators who want a modern, auditable stack without hidden legacy behaviour.",
        home(keys, "why_fluxheim_text"),
    )
    .replace(
        "Config validation is strict. Ambiguous or insecure options are rejected, not silently accepted.",
        home(keys, "no_hidden_fallback_text"),
    )
    .replace(
        "Reproducible builds. Every dependency is pinned.",
        home(keys, "checked_cargo_lock_text"),
    )
    .replace(
        "A glance at what Fluxheim looks like in a production deployment.",
        home(keys, "overview_text"),
    )
    .replace(
        "Full TOML config reference with examples.",
        home(keys, "full_toml_reference"),
    )
    .replace(
        "All modules, build profiles, and TLS backends.",
        home(keys, "all_modules"),
    )
    .replace(
        "Fluxheim ships as focused, modular builds — use only what your deployment needs.",
        home(keys, "features_intro"),
    )
    .replace(
        "Modular reverse proxy, cache, load balancer, and static host written",
        home(keys, "hero_line_one"),
    )
    .replace(
        "in Rust. Secure by default with TLS, ACME, compression, edge policy,",
        home(keys, "hero_line_two"),
    )
    .replace(
        "dynamic upstream discovery, and safe traffic mirroring built in.",
        home(keys, "hero_line_three"),
    )
    .replace_home_marker("Everything You Need at the Edge", home(keys, "features_heading"))
    .replace_home_marker("Memory-Safe by Design", home(keys, "memory_safe_by_design"))
    .replace_home_marker("Fluxheim HTTP Core", home(keys, "fluxheim_http_core"))
    .replace_home_marker(
        "Load Balancer Control Plane",
        home(keys, "load_balancer_control_plane"),
    )
    .replace_home_marker("Modular Build Profiles", home(keys, "modular_build_profiles"))
    .replace_home_marker("TLS & Managed ACME", home(keys, "tls_managed_acme"))
    .replace_home_marker("Advanced Cache System", home(keys, "advanced_cache_system"))
    .replace_home_marker("Container Native", home(keys, "container_native"))
    .replace_home_marker(
        "Prometheus & OpenTelemetry",
        home(keys, "prometheus_opentelemetry"),
    )
    .replace_home_marker("TCP Stream Proxy", home(keys, "tcp_stream_proxy"))
    .replace_home_marker("PHP-FPM Support", home(keys, "php_fpm_support"))
    .replace_home_marker("Edge Policy Controls", home(keys, "edge_policy_controls"))
    .replace_home_marker("Get Running in Minutes", home(keys, "quick_start_heading"))
    .replace_home_marker("From Source", home(keys, "from_source"))
    .replace_home_marker("Full installation guide →", home(keys, "full_installation_guide"))
    .replace_home_marker("Why Fluxheim?", home(keys, "why_fluxheim"))
    .replace_home_marker("No hidden legacy protocol fallback", home(keys, "no_hidden_fallback"))
    .replace_home_marker("Checked-in Cargo.lock", home(keys, "checked_cargo_lock"))
    .replace_home_marker("Overview", home(keys, "overview"))
    .replace_home_marker("Get Started", home(keys, "get_started"))
    .replace_home_marker("Read guide →", home(keys, "read_guide"))
    .replace_home_marker("Browse reference →", home(keys, "browse_reference"))
    .replace_home_marker("See all features →", home(keys, "see_all_features"))
    .replace_home_marker("Memory-Safe", home(keys, "hero_memory_safe"))
    .replace_home_marker("Edge Server", home(keys, "hero_edge_server"))
    .replace_home_marker("Built in Rust", home(keys, "hero_built_in_rust"))
    .replace_home_marker("Fluxheim Core", home(keys, "tag_fluxheim_core"))
    .replace_home_marker("macOS Dev", home(keys, "tag_macos_dev"))
    .replace_home_marker("Rootless Containers", home(keys, "tag_rootless_containers"))
    .replace_map(&source.docs_index, &keys.docs_index)
    .replace_map(&source.common, &keys.common);

    let html = if is_download_page {
        html.replace_map_everywhere(&source.download, &keys.download)
    } else {
        html
    };

    let html = if is_changelog_page {
        html.replace_map_everywhere(&source.changelog, &keys.changelog)
    } else {
        html
    };

    let html = if is_download_page || is_changelog_page {
        html.replace_map_everywhere(&source.release_updates, &keys.release_updates)
    } else {
        html
    };

    let html = if is_runtime_parity_fixtures_page {
        html.replace_map_everywhere(
            &source.runtime_parity_fixtures,
            &keys.runtime_parity_fixtures,
        )
    } else {
        html
    };

    let html = if is_geoip_page {
        html.replace_map_everywhere(&source.geoip, &keys.geoip)
    } else {
        html
    };

    let html = if is_load_balancer_ha_page {
        html.replace_map_everywhere(&source.load_balancer_ha, &keys.load_balancer_ha)
    } else {
        html
    };

    let html = if is_getting_started_page {
        html.replace_map_everywhere(&source.getting_started, &keys.getting_started)
    } else {
        html
    };

    let html = if is_cache_page {
        html.replace_map_everywhere(&source.cache, &keys.cache)
    } else {
        html
    };

    let html = if is_extraction_dependency_graph_page {
        html.replace_map_everywhere(
            &source.extraction_dependency_graph,
            &keys.extraction_dependency_graph,
        )
    } else {
        html
    };

    let html = if is_runtime_baseline_page {
        html.replace_map_everywhere(&source.runtime_baseline, &keys.runtime_baseline)
    } else {
        html
    };

    let html = if is_modularity_policy_page {
        html.replace_map_everywhere(&source.modularity_policy, &keys.modularity_policy)
    } else {
        html
    };

    let html = if is_observability_page {
        html.replace_map_everywhere(&source.observability, &keys.observability)
    } else {
        html
    };

    let html = if is_release_notes_template_page {
        html.replace_map_everywhere(&source.release_notes_template, &keys.release_notes_template)
    } else {
        html
    };

    let html = if is_tls_acme_page {
        html.replace_map_everywhere(&source.tls_acme, &keys.tls_acme)
    } else {
        html
    };

    let html = if is_owasp_baseline_page {
        html.replace_map_everywhere(&source.owasp_baseline, &keys.owasp_baseline)
    } else {
        html
    };

    let html = if is_macos_development_page {
        html.replace_map_everywhere(&source.macos_development, &keys.macos_development)
    } else {
        html
    };

    let html = if is_gateway_recipes_page {
        html.replace_map_everywhere(&source.gateway_recipes, &keys.gateway_recipes)
    } else {
        html
    };

    html.replace(
        ">Download v1.6.28<",
        &format!(">{}<", versioned(&keys.release.download_version, version)),
    )
    .replace(
        "Latest Stable —",
        &format!("{} —", keys.release.latest_stable),
    )
    .replace(
        ">Latest Stable<",
        &format!(">{}<", keys.release.latest_stable),
    )
    .replace(">Changelog<", &format!(">{}<", keys.nav.changelog))
    .replace(">Download<", &format!(">{}<", keys.nav.download))
    .replace(">Docs<", &format!(">{}<", keys.nav.docs))
}

trait HtmlTextReplace {
    fn replace_home_marker(self, from: &str, to: &str) -> String;
    fn replace_attr_value(self, from: &str, to: &str) -> String;
    fn replace_map(
        self,
        source: &std::collections::BTreeMap<String, String>,
        target: &std::collections::BTreeMap<String, String>,
    ) -> String;
    fn replace_map_everywhere(
        self,
        source: &std::collections::BTreeMap<String, String>,
        target: &std::collections::BTreeMap<String, String>,
    ) -> String;
}

impl HtmlTextReplace for String {
    fn replace_home_marker(self, from: &str, to: &str) -> String {
        self.replace(&format!(">{from}<"), &format!(">{to}<"))
    }

    fn replace_attr_value(self, from: &str, to: &str) -> String {
        self.replace(&format!("=\"{from}\""), &format!("=\"{to}\""))
    }

    fn replace_map(
        self,
        source: &std::collections::BTreeMap<String, String>,
        target: &std::collections::BTreeMap<String, String>,
    ) -> String {
        let mut output = self;
        let mut entries: Vec<_> = source.iter().collect();
        entries.sort_by_key(|(_, source)| std::cmp::Reverse(source.len()));

        for (key, source) in entries {
            let replacement = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            output = output.replace_home_marker(source, replacement);
            output = output.replace_attr_value(source, replacement);
            if source.len() >= 40 {
                output = output.replace(source, replacement);
            }
        }

        output
    }

    fn replace_map_everywhere(
        self,
        source: &std::collections::BTreeMap<String, String>,
        target: &std::collections::BTreeMap<String, String>,
    ) -> String {
        let mut output = self;
        let mut entries: Vec<_> = source.iter().collect();
        entries.sort_by_key(|(_, source)| std::cmp::Reverse(source.len()));

        for (key, source) in entries {
            let replacement = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            output = output.replace(source, replacement);
        }

        output
    }
}
