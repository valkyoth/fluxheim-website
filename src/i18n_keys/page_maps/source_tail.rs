use crate::i18n_keys::KeyFile;

use super::replace::{marker_key_map, source_doc_key_map};

pub(super) fn apply_source_tail_maps(html: String, keys: &KeyFile, source: &KeyFile) -> String {
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
