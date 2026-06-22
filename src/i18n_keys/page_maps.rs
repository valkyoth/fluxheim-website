use std::collections::BTreeMap;

use super::{KeyFile, text_replace::HtmlTextReplace};

pub(super) fn apply_page_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
    let is_download_page = html.contains("Download — Fluxheim");
    let is_changelog_page = html.contains("Changelog — Fluxheim");
    let is_build_and_podman_page =
        html.contains("Build And Rootless Podman — Fluxheim Source Docs");

    let html = replace_page_map(html, is_download_page, &source.download, &keys.download);
    let html = replace_page_map(html, is_changelog_page, &source.changelog, &keys.changelog);
    let html = replace_page_map(
        html,
        is_download_page || is_changelog_page,
        &source.release_updates,
        &keys.release_updates,
    );
    let html = replace_marker_map(
        html,
        "Runtime Parity Fixtures — Fluxheim Source Docs",
        &source.runtime_parity_fixtures,
        &keys.runtime_parity_fixtures,
    );
    let html = replace_marker_map(
        html,
        "GeoIP / Geo-Context — Fluxheim Source Docs",
        &source.geoip,
        &keys.geoip,
    );
    let html = replace_marker_map(
        html,
        "Load Balancer HA Design Notes — Fluxheim Source Docs",
        &source.load_balancer_ha,
        &keys.load_balancer_ha,
    );
    let html = replace_marker_map(
        html,
        "Installation & Quick Start — Fluxheim Docs",
        &source.getting_started,
        &keys.getting_started,
    );
    let html = replace_marker_map(
        html,
        "Cache System — Fluxheim Docs",
        &source.cache,
        &keys.cache,
    );
    let html = replace_marker_map(
        html,
        "Extraction Dependency Graph — Fluxheim Source Docs",
        &source.extraction_dependency_graph,
        &keys.extraction_dependency_graph,
    );
    let html = replace_marker_map(
        html,
        "Runtime Baseline — Fluxheim Source Docs",
        &source.runtime_baseline,
        &keys.runtime_baseline,
    );
    let html = replace_marker_map(
        html,
        "Modularity Policy — Fluxheim Source Docs",
        &source.modularity_policy,
        &keys.modularity_policy,
    );
    let html = replace_marker_map(
        html,
        "Observability — Fluxheim Docs",
        &source.observability,
        &keys.observability,
    );
    let html = replace_marker_map(
        html,
        "Release Notes Template — Fluxheim Source Docs",
        &source.release_notes_template,
        &keys.release_notes_template,
    );
    let html = replace_marker_map(
        html,
        "TLS & ACME — Fluxheim Docs",
        &source.tls_acme,
        &keys.tls_acme,
    );
    let html = replace_marker_map(
        html,
        "OWASP Top 10 2025 Baseline — Fluxheim Source Docs",
        &source.owasp_baseline,
        &keys.owasp_baseline,
    );
    let html = replace_marker_map(
        html,
        "macOS Development Support — Fluxheim Source Docs",
        &source.macos_development,
        &keys.macos_development,
    );
    let html = replace_marker_map(
        html,
        "Gateway Recipes — Fluxheim Source Docs",
        &source.gateway_recipes,
        &keys.gateway_recipes,
    );
    let html = replace_marker_map(
        html,
        "Systemd & Containers — Fluxheim Docs",
        &source.deployment,
        &keys.deployment,
    );
    let html = replace_marker_map(
        html,
        "Secure Links — Fluxheim Source Docs",
        &source.secure_links,
        &keys.secure_links,
    );
    let html = replace_marker_map(
        html,
        "Vhost Config Guide — Fluxheim Source Docs",
        &source.vhost_config,
        &keys.vhost_config,
    );
    let html = replace_marker_map(
        html,
        "Fluxheim Ecosystem Idea — Fluxheim Source Docs",
        &source.fluxheim_ecosystem_idea,
        &keys.fluxheim_ecosystem_idea,
    );
    let html = replace_marker_map(
        html,
        "GitHub Repository Setup — Fluxheim Source Docs",
        &source.github_setup,
        &keys.github_setup,
    );
    let html = replace_page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_runtime,
        &keys.build_and_podman_runtime,
    );
    let html = replace_page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_final,
        &keys.build_and_podman_final,
    );
    let html = replace_page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_builds,
        &keys.build_and_podman_builds,
    );
    let html = replace_page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman,
        &keys.build_and_podman,
    );
    let html = replace_marker_map(
        html,
        "Cache Encryption — Fluxheim Source Docs",
        &source.cache_encryption,
        &keys.cache_encryption,
    );
    let html = replace_marker_map(
        html,
        "Perl CGI Support — Fluxheim Source Docs",
        &source.perl_cgi_support,
        &keys.perl_cgi_support,
    );
    let html = replace_marker_map(
        html,
        "systemd Deployment — Fluxheim Source Docs",
        &source.systemd_deployment,
        &keys.systemd_deployment,
    );
    let html = replace_marker_map(
        html,
        "Config Snapshots And Rollback — Fluxheim Source Docs",
        &source.config_snapshots,
        &keys.config_snapshots,
    );
    let html = replace_marker_map(
        html,
        "Pingora Core Patch — Fluxheim Source Docs",
        &source.pingora_core_patch,
        &keys.pingora_core_patch,
    );
    let html = replace_marker_map(
        html,
        "Supply Chain Security — Fluxheim Source Docs",
        &source.supply_chain_security,
        &keys.supply_chain_security,
    );
    let html = replace_marker_map(
        html,
        "Compression — Fluxheim Source Docs",
        &source.compression,
        &keys.compression,
    );
    let html = replace_marker_map(
        html,
        "Load Balancer Migration Notes — Fluxheim Source Docs",
        &source.load_balancer_migration,
        &keys.load_balancer_migration,
    );
    let html = replace_marker_map(
        html,
        "Runtime Facts And Policy Proofs — Fluxheim Source Docs",
        &source.runtime_facts_and_policy_proofs,
        &keys.runtime_facts_and_policy_proofs,
    );
    replace_marker_map(
        html,
        "Production Readiness — Fluxheim Source Docs",
        &source.production_readiness,
        &keys.production_readiness,
    )
}

fn replace_marker_map(
    html: String,
    marker: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let is_page = html.contains(marker);
    replace_page_map(html, is_page, source, keys)
}

fn replace_page_map(
    html: String,
    is_page: bool,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    if is_page {
        html.replace_map_everywhere(source, keys)
    } else {
        html
    }
}
