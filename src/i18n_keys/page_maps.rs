use std::collections::BTreeMap;

use super::{KeyFile, text_replace::HtmlTextReplace};

pub(super) fn apply_page_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
    let is_download_page = html.contains("Download — Fluxheim");
    let is_changelog_page = html.contains("Changelog — Fluxheim");
    let is_build_and_podman_page =
        html.contains("Build And Rootless Podman — Fluxheim Source Docs");

    let html = replace_page_map(html, is_download_page, &source.download, &keys.download);
    let html = replace_page_map(html, is_changelog_page, &source.changelog, &keys.changelog);
    let html = replace_marker_map(
        html,
        "Source Reference — Fluxheim Docs",
        &source.reference,
        &keys.reference,
    );
    let html = replace_page_map(
        html,
        is_download_page || is_changelog_page,
        &source.release_updates,
        &keys.release_updates,
    );
    let html = replace_source_doc_key_map(
        html,
        "runtime_parity_fixtures",
        &source.runtime_parity_fixtures,
        &keys.runtime_parity_fixtures,
    );
    let html = replace_marker_map(
        html,
        "GeoIP / Geo-Context — Fluxheim Source Docs",
        &source.geoip,
        &keys.geoip,
    );
    let html = replace_source_doc_key_map(
        html,
        "load_balancer_ha_design_notes",
        &source.load_balancer_ha,
        &keys.load_balancer_ha,
    );
    let html = replace_marker_map(
        html,
        "Installation & Quick Start — Fluxheim Docs",
        &source.getting_started,
        &keys.getting_started,
    );
    let html = replace_marker_key_map(
        html,
        "config_reference_fluxheim_docs",
        &source.configuration_page,
        &keys.configuration_page,
    );
    let html = replace_marker_map(
        html,
        "Advanced — Fluxheim Docs",
        &source.advanced_page,
        &keys.advanced_page,
    );
    let html = replace_docs_key_map(
        html,
        "features_001_feature_matrix",
        &source.features_page,
        &keys.features_page,
    );
    let html = replace_marker_map(
        html,
        "Cache System — Fluxheim Docs",
        &source.cache,
        &keys.cache,
    );
    let html = replace_marker_key_map(
        html,
        "extraction_dependency_graph_fluxheim_source_docs",
        &source.extraction_dependency_graph,
        &keys.extraction_dependency_graph,
    );
    let html = replace_marker_map(
        html,
        "Runtime Baseline — Fluxheim Source Docs",
        &source.runtime_baseline,
        &keys.runtime_baseline,
    );
    let html = replace_marker_key_map(
        html,
        "modularity_policy_fluxheim_source_docs",
        &source.modularity_policy,
        &keys.modularity_policy,
    );
    let html = replace_marker_map(
        html,
        "Observability — Fluxheim Docs",
        &source.observability,
        &keys.observability,
    );
    let html = replace_marker_key_map(
        html,
        "release_notes_template_fluxheim_source_docs",
        &source.release_notes_template,
        &keys.release_notes_template,
    );
    let html = replace_marker_key_map(
        html,
        "tls_acme_fluxheim_docs",
        &source.tls_acme,
        &keys.tls_acme,
    );
    let html = replace_marker_key_map(
        html,
        "owasp_top_10_2025_baseline_fluxheim_source_docs",
        &source.owasp_baseline,
        &keys.owasp_baseline,
    );
    let html = replace_marker_key_map(
        html,
        "macos_development_support_fluxheim_source_docs",
        &source.macos_development,
        &keys.macos_development,
    );
    let html = replace_marker_key_map(
        html,
        "gateway_recipes_fluxheim_source_docs",
        &source.gateway_recipes,
        &keys.gateway_recipes,
    );
    let html = replace_marker_key_map(
        html,
        "systemd_containers_fluxheim_docs",
        &source.deployment,
        &keys.deployment,
    );
    let html = replace_marker_key_map(
        html,
        "secure_links_fluxheim_source_docs",
        &source.secure_links,
        &keys.secure_links,
    );
    let html = replace_marker_key_map(
        html,
        "vhost_config_guide_fluxheim_source_docs",
        &source.vhost_config,
        &keys.vhost_config,
    );
    let html = replace_marker_key_map(
        html,
        "fluxheim_ecosystem_idea_fluxheim_source_docs",
        &source.fluxheim_ecosystem_idea,
        &keys.fluxheim_ecosystem_idea,
    );
    let html = replace_source_doc_key_map(
        html,
        "github_repository_setup",
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
    let html = replace_source_doc_key_map(
        html,
        "cache_encryption",
        &source.cache_encryption,
        &keys.cache_encryption,
    );
    let html = replace_source_doc_key_map(
        html,
        "perl_cgi_support",
        &source.perl_cgi_support,
        &keys.perl_cgi_support,
    );
    let html = replace_source_doc_key_map(
        html,
        "systemd_deployment",
        &source.systemd_deployment,
        &keys.systemd_deployment,
    );
    let html = replace_source_doc_key_map(
        html,
        "config_snapshots_and_rollback",
        &source.config_snapshots,
        &keys.config_snapshots,
    );
    let html = replace_source_doc_key_map(
        html,
        "pingora_core_patch",
        &source.pingora_core_patch,
        &keys.pingora_core_patch,
    );
    let html = replace_source_doc_key_map(
        html,
        "supply_chain_security",
        &source.supply_chain_security,
        &keys.supply_chain_security,
    );
    let html =
        replace_source_doc_key_map(html, "compression", &source.compression, &keys.compression);
    let html = replace_source_doc_key_map(
        html,
        "load_balancer_migration_notes",
        &source.load_balancer_migration,
        &keys.load_balancer_migration,
    );
    let html = replace_marker_key_map(
        html,
        "runtime_facts_and_policy_proofs_fluxheim_source_docs",
        &source.runtime_facts_and_policy_proofs,
        &keys.runtime_facts_and_policy_proofs,
    );
    let html = replace_source_doc_key_map(
        html,
        "production_readiness",
        &source.production_readiness,
        &keys.production_readiness,
    );
    let html = replace_marker_key_map(
        html,
        "cache_backends_fluxheim_source_docs",
        &source.cache_backends,
        &keys.cache_backends,
    );
    let html = replace_marker_key_map(
        html,
        "waf_architecture_fluxheim_source_docs",
        &source.waf_architecture,
        &keys.waf_architecture,
    );
    let html = replace_marker_key_map(
        html,
        "image_filter_fluxheim_source_docs",
        &source.image_filter,
        &keys.image_filter,
    );
    let html = replace_marker_key_map(
        html,
        "feature_matrix_fluxheim_source_docs",
        &source.source_features,
        &keys.source_features,
    );
    let html = replace_marker_key_map(
        html,
        "cloudflare_origin_support_fluxheim_source_docs",
        &source.cloudflare_origin_support,
        &keys.cloudflare_origin_support,
    );
    let html = replace_marker_key_map(
        html,
        "certificate_renewal_and_reload_fluxheim_source_docs",
        &source.certificate_renewal,
        &keys.certificate_renewal,
    );
    let html = replace_marker_key_map(
        html,
        "logging_architecture_fluxheim_source_docs",
        &source.logging_architecture,
        &keys.logging_architecture,
    );
    let html = replace_marker_key_map(
        html,
        "legacy_static_http_support_fluxheim_source_docs",
        &source.legacy_static_http,
        &keys.legacy_static_http,
    );
    let html = replace_marker_key_map(
        html,
        "metrics_architecture_fluxheim_source_docs",
        &source.metrics_architecture,
        &keys.metrics_architecture,
    );
    let html = replace_marker_key_map(
        html,
        "php_runtime_support_fluxheim_source_docs",
        &source.php_runtime_support_source,
        &keys.php_runtime_support_source,
    );
    let html = replace_marker_key_map(
        html,
        "external_authorization_request_fluxheim_source_docs",
        &source.auth_request_source,
        &keys.auth_request_source,
    );
    let html = replace_marker_key_map(
        html,
        "programmable_media_edge_fluxheim_source_docs",
        &source.programmable_media_edge,
        &keys.programmable_media_edge,
    );
    let html = replace_source_doc_key_map(
        html,
        "wasm_extensibility",
        &source.wasm_extensibility,
        &keys.wasm_extensibility,
    );
    let html = replace_marker_key_map(
        html,
        "opentelemetry_tracing_fluxheim_source_docs",
        &source.opentelemetry_tracing,
        &keys.opentelemetry_tracing,
    );
    let html = replace_marker_key_map(
        html,
        "php_fpm_application_recipes_fluxheim_source_docs",
        &source.php_fpm_app_recipes,
        &keys.php_fpm_app_recipes,
    );
    let html = replace_marker_key_map(
        html,
        "sentinel_mesh_fluxheim_source_docs",
        &source.sentinel_mesh,
        &keys.sentinel_mesh,
    );
    let html = replace_marker_key_map(
        html,
        "crypto_rpc_edge_fluxheim_source_docs",
        &source.crypto_rpc_edge_source,
        &keys.crypto_rpc_edge_source,
    );
    let html = replace_marker_key_map(
        html,
        "fips_capable_deployment_fluxheim_source_docs",
        &source.fips_source,
        &keys.fips_source,
    );
    let html = replace_marker_key_map(
        html,
        "versioning_plan_fluxheim_source_docs",
        &source.versioning_plan_source,
        &keys.versioning_plan_source,
    );
    let html = replace_marker_key_map(
        html,
        "release_checklist_fluxheim_source_docs",
        &source.release_checklist_source,
        &keys.release_checklist_source,
    );
    let html = replace_marker_key_map(
        html,
        "modularity_exceptions_fluxheim_source_docs",
        &source.modularity_exceptions_source,
        &keys.modularity_exceptions_source,
    );
    let html = replace_marker_key_map(
        html,
        "release_runbook_fluxheim_source_docs",
        &source.release_runbook_source,
        &keys.release_runbook_source,
    );
    let html = replace_marker_key_map(
        html,
        "compliance_evidence_template_fluxheim_source_docs",
        &source.compliance_evidence_template_source,
        &keys.compliance_evidence_template_source,
    );
    let html = replace_marker_key_map(
        html,
        "common_criteria_roadmap_fluxheim_source_docs",
        &source.common_criteria_roadmap_source,
        &keys.common_criteria_roadmap_source,
    );
    let html = replace_marker_key_map(
        html,
        "config_ref_001_config_reference_fluxheim_source_docs",
        &source.config_reference_source,
        &keys.config_reference_source,
    );
    replace_marker_key_map(
        html,
        "zero_retention_privacy_mode_fluxheim_source_docs",
        &source.zero_retention_privacy_mode,
        &keys.zero_retention_privacy_mode,
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

fn replace_marker_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = source
        .get(marker_key)
        .unwrap_or_else(|| panic!("source marker i18n key exists: {marker_key}"));
    replace_marker_map(html, marker, source, keys)
}

fn replace_docs_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(source, marker_key, "Fluxheim Docs");
    replace_marker_map(html, &marker, source, keys)
}

fn replace_source_doc_key_map(
    html: String,
    marker_key: &str,
    source: &BTreeMap<String, String>,
    keys: &BTreeMap<String, String>,
) -> String {
    let marker = title_marker(source, marker_key, "Fluxheim Source Docs");
    replace_marker_map(html, &marker, source, keys)
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

fn title_marker(source: &BTreeMap<String, String>, marker_key: &str, suffix: &str) -> String {
    let title = source
        .get(marker_key)
        .unwrap_or_else(|| panic!("source title i18n key exists: {marker_key}"));
    format!("{title} — {suffix}")
}
