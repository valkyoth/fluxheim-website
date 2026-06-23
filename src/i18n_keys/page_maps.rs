mod replace;

use super::KeyFile;
use replace::{
    docs_key_map, docs_marker_from_key_map, marker_key_map, page_map, source_doc_key_map,
    source_doc_marker_from_key_map, title_marker,
};

pub(super) fn apply_page_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
    let download_marker = format!("{} — Fluxheim", source.nav.download);
    let build_and_podman_marker = title_marker(
        &source.reference,
        "build_and_rootless_podman",
        "Fluxheim Source Docs",
    );
    let is_download_page = html.contains(&download_marker);
    let is_changelog_page = html.contains(
        source
            .changelog
            .get("changelog_fluxheim")
            .expect("changelog page title i18n key exists"),
    );
    let is_build_and_podman_page = html.contains(&build_and_podman_marker);

    let html = page_map(html, is_download_page, &source.download, &keys.download);
    let html = page_map(html, is_changelog_page, &source.changelog, &keys.changelog);
    let html = docs_marker_from_key_map(
        html,
        "source_reference",
        &source.docs_index,
        &source.reference,
        &keys.reference,
    );
    let html = page_map(
        html,
        is_download_page || is_changelog_page,
        &source.release_updates,
        &keys.release_updates,
    );
    let html = source_doc_key_map(
        html,
        "runtime_parity_fixtures",
        &source.runtime_parity_fixtures,
        &keys.runtime_parity_fixtures,
    );
    let html = source_doc_marker_from_key_map(
        html,
        "geoip_geo_context",
        &source.reference,
        &source.geoip,
        &keys.geoip,
    );
    let html = source_doc_key_map(
        html,
        "load_balancer_ha_design_notes",
        &source.load_balancer_ha,
        &keys.load_balancer_ha,
    );
    let html = docs_marker_from_key_map(
        html,
        "installation_quick_start",
        &source.docs_index,
        &source.getting_started,
        &keys.getting_started,
    );
    let html = marker_key_map(
        html,
        "config_reference_fluxheim_docs",
        &source.configuration_page,
        &keys.configuration_page,
    );
    let html = docs_marker_from_key_map(
        html,
        "advanced",
        &source.docs_index,
        &source.advanced_page,
        &keys.advanced_page,
    );
    let html = docs_key_map(
        html,
        "features_001_feature_matrix",
        &source.features_page,
        &keys.features_page,
    );
    let html = docs_marker_from_key_map(
        html,
        "cache_system",
        &source.docs_index,
        &source.cache,
        &keys.cache,
    );
    let html = marker_key_map(
        html,
        "extraction_dependency_graph_fluxheim_source_docs",
        &source.extraction_dependency_graph,
        &keys.extraction_dependency_graph,
    );
    let html = marker_key_map(
        html,
        "runtime_baseline_fluxheim_source_docs",
        &source.runtime_baseline,
        &keys.runtime_baseline,
    );
    let html = marker_key_map(
        html,
        "modularity_policy_fluxheim_source_docs",
        &source.modularity_policy,
        &keys.modularity_policy,
    );
    let html = docs_marker_from_key_map(
        html,
        "observability",
        &source.docs_index,
        &source.observability,
        &keys.observability,
    );
    let html = marker_key_map(
        html,
        "release_notes_template_fluxheim_source_docs",
        &source.release_notes_template,
        &keys.release_notes_template,
    );
    let html = marker_key_map(
        html,
        "tls_acme_fluxheim_docs",
        &source.tls_acme,
        &keys.tls_acme,
    );
    let html = marker_key_map(
        html,
        "owasp_top_10_2025_baseline_fluxheim_source_docs",
        &source.owasp_baseline,
        &keys.owasp_baseline,
    );
    let html = marker_key_map(
        html,
        "macos_development_support_fluxheim_source_docs",
        &source.macos_development,
        &keys.macos_development,
    );
    let html = marker_key_map(
        html,
        "gateway_recipes_fluxheim_source_docs",
        &source.gateway_recipes,
        &keys.gateway_recipes,
    );
    let html = marker_key_map(
        html,
        "systemd_containers_fluxheim_docs",
        &source.deployment,
        &keys.deployment,
    );
    let html = marker_key_map(
        html,
        "secure_links_fluxheim_source_docs",
        &source.secure_links,
        &keys.secure_links,
    );
    let html = marker_key_map(
        html,
        "vhost_config_guide_fluxheim_source_docs",
        &source.vhost_config,
        &keys.vhost_config,
    );
    let html = marker_key_map(
        html,
        "fluxheim_ecosystem_idea_fluxheim_source_docs",
        &source.fluxheim_ecosystem_idea,
        &keys.fluxheim_ecosystem_idea,
    );
    let html = source_doc_key_map(
        html,
        "github_repository_setup",
        &source.github_setup,
        &keys.github_setup,
    );
    let html = page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_runtime,
        &keys.build_and_podman_runtime,
    );
    let html = page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_final,
        &keys.build_and_podman_final,
    );
    let html = page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman_builds,
        &keys.build_and_podman_builds,
    );
    let html = page_map(
        html,
        is_build_and_podman_page,
        &source.build_and_podman,
        &keys.build_and_podman,
    );
    let html = source_doc_key_map(
        html,
        "cache_encryption",
        &source.cache_encryption,
        &keys.cache_encryption,
    );
    let html = source_doc_key_map(
        html,
        "perl_cgi_support",
        &source.perl_cgi_support,
        &keys.perl_cgi_support,
    );
    let html = source_doc_key_map(
        html,
        "systemd_deployment",
        &source.systemd_deployment,
        &keys.systemd_deployment,
    );
    let html = source_doc_key_map(
        html,
        "config_snapshots_and_rollback",
        &source.config_snapshots,
        &keys.config_snapshots,
    );
    let html = source_doc_key_map(
        html,
        "pingora_core_patch",
        &source.pingora_core_patch,
        &keys.pingora_core_patch,
    );
    let html = source_doc_key_map(
        html,
        "supply_chain_security",
        &source.supply_chain_security,
        &keys.supply_chain_security,
    );
    let html = source_doc_key_map(html, "compression", &source.compression, &keys.compression);
    let html = source_doc_key_map(
        html,
        "load_balancer_migration_notes",
        &source.load_balancer_migration,
        &keys.load_balancer_migration,
    );
    let html = marker_key_map(
        html,
        "runtime_facts_and_policy_proofs_fluxheim_source_docs",
        &source.runtime_facts_and_policy_proofs,
        &keys.runtime_facts_and_policy_proofs,
    );
    let html = source_doc_key_map(
        html,
        "production_readiness",
        &source.production_readiness,
        &keys.production_readiness,
    );
    let html = marker_key_map(
        html,
        "cache_backends_fluxheim_source_docs",
        &source.cache_backends,
        &keys.cache_backends,
    );
    let html = marker_key_map(
        html,
        "waf_architecture_fluxheim_source_docs",
        &source.waf_architecture,
        &keys.waf_architecture,
    );
    let html = marker_key_map(
        html,
        "image_filter_fluxheim_source_docs",
        &source.image_filter,
        &keys.image_filter,
    );
    let html = marker_key_map(
        html,
        "feature_matrix_fluxheim_source_docs",
        &source.source_features,
        &keys.source_features,
    );
    let html = marker_key_map(
        html,
        "cloudflare_origin_support_fluxheim_source_docs",
        &source.cloudflare_origin_support,
        &keys.cloudflare_origin_support,
    );
    let html = marker_key_map(
        html,
        "certificate_renewal_and_reload_fluxheim_source_docs",
        &source.certificate_renewal,
        &keys.certificate_renewal,
    );
    let html = marker_key_map(
        html,
        "logging_architecture_fluxheim_source_docs",
        &source.logging_architecture,
        &keys.logging_architecture,
    );
    let html = marker_key_map(
        html,
        "legacy_static_http_support_fluxheim_source_docs",
        &source.legacy_static_http,
        &keys.legacy_static_http,
    );
    let html = marker_key_map(
        html,
        "metrics_architecture_fluxheim_source_docs",
        &source.metrics_architecture,
        &keys.metrics_architecture,
    );
    let html = marker_key_map(
        html,
        "php_runtime_support_fluxheim_source_docs",
        &source.php_runtime_support_source,
        &keys.php_runtime_support_source,
    );
    let html = marker_key_map(
        html,
        "external_authorization_request_fluxheim_source_docs",
        &source.auth_request_source,
        &keys.auth_request_source,
    );
    let html = marker_key_map(
        html,
        "programmable_media_edge_fluxheim_source_docs",
        &source.programmable_media_edge,
        &keys.programmable_media_edge,
    );
    let html = source_doc_key_map(
        html,
        "wasm_extensibility",
        &source.wasm_extensibility,
        &keys.wasm_extensibility,
    );
    let html = marker_key_map(
        html,
        "opentelemetry_tracing_fluxheim_source_docs",
        &source.opentelemetry_tracing,
        &keys.opentelemetry_tracing,
    );
    let html = marker_key_map(
        html,
        "php_fpm_application_recipes_fluxheim_source_docs",
        &source.php_fpm_app_recipes,
        &keys.php_fpm_app_recipes,
    );
    let html = marker_key_map(
        html,
        "sentinel_mesh_fluxheim_source_docs",
        &source.sentinel_mesh,
        &keys.sentinel_mesh,
    );
    let html = marker_key_map(
        html,
        "crypto_rpc_edge_fluxheim_source_docs",
        &source.crypto_rpc_edge_source,
        &keys.crypto_rpc_edge_source,
    );
    let html = marker_key_map(
        html,
        "fips_capable_deployment_fluxheim_source_docs",
        &source.fips_source,
        &keys.fips_source,
    );
    let html = marker_key_map(
        html,
        "versioning_plan_fluxheim_source_docs",
        &source.versioning_plan_source,
        &keys.versioning_plan_source,
    );
    let html = marker_key_map(
        html,
        "release_checklist_fluxheim_source_docs",
        &source.release_checklist_source,
        &keys.release_checklist_source,
    );
    let html = marker_key_map(
        html,
        "modularity_exceptions_fluxheim_source_docs",
        &source.modularity_exceptions_source,
        &keys.modularity_exceptions_source,
    );
    let html = marker_key_map(
        html,
        "release_runbook_fluxheim_source_docs",
        &source.release_runbook_source,
        &keys.release_runbook_source,
    );
    let html = marker_key_map(
        html,
        "compliance_evidence_template_fluxheim_source_docs",
        &source.compliance_evidence_template_source,
        &keys.compliance_evidence_template_source,
    );
    let html = marker_key_map(
        html,
        "common_criteria_roadmap_fluxheim_source_docs",
        &source.common_criteria_roadmap_source,
        &keys.common_criteria_roadmap_source,
    );
    let html = marker_key_map(
        html,
        "config_ref_001_config_reference_fluxheim_source_docs",
        &source.config_reference_source,
        &keys.config_reference_source,
    );
    marker_key_map(
        html,
        "zero_retention_privacy_mode_fluxheim_source_docs",
        &source.zero_retention_privacy_mode,
        &keys.zero_retention_privacy_mode,
    )
}
