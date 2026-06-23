mod replace;
mod source_tail;

use super::KeyFile;
use replace::{
    docs_key_map, docs_marker_from_key_map, marker_key_map, page_map, source_doc_key_map,
    source_doc_marker_from_key_map, title_marker,
};
use source_tail::apply_source_tail_maps;

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
    apply_source_tail_maps(html, keys, source)
}
