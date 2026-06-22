use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct KeyFile {
    pub(super) locale_id: String,
    pub(super) language: LanguageKeys,
    pub(super) nav: NavKeys,
    pub(super) release: ReleaseKeys,
    pub(super) shell: ShellKeys,
    pub(super) footer: FooterKeys,
    pub(super) home: BTreeMap<String, String>,
    pub(super) docs_index: BTreeMap<String, String>,
    pub(super) common: BTreeMap<String, String>,
    pub(super) download: BTreeMap<String, String>,
    pub(super) changelog: BTreeMap<String, String>,
    pub(super) release_updates: BTreeMap<String, String>,
    pub(super) runtime_parity_fixtures: BTreeMap<String, String>,
    pub(super) geoip: BTreeMap<String, String>,
    pub(super) load_balancer_ha: BTreeMap<String, String>,
    pub(super) getting_started: BTreeMap<String, String>,
    pub(super) cache: BTreeMap<String, String>,
    pub(super) extraction_dependency_graph: BTreeMap<String, String>,
    pub(super) runtime_baseline: BTreeMap<String, String>,
    pub(super) modularity_policy: BTreeMap<String, String>,
    pub(super) observability: BTreeMap<String, String>,
    pub(super) release_notes_template: BTreeMap<String, String>,
    pub(super) tls_acme: BTreeMap<String, String>,
    pub(super) owasp_baseline: BTreeMap<String, String>,
    pub(super) macos_development: BTreeMap<String, String>,
    pub(super) gateway_recipes: BTreeMap<String, String>,
    pub(super) deployment: BTreeMap<String, String>,
    pub(super) secure_links: BTreeMap<String, String>,
    pub(super) vhost_config: BTreeMap<String, String>,
    pub(super) fluxheim_ecosystem_idea: BTreeMap<String, String>,
    pub(super) github_setup: BTreeMap<String, String>,
    pub(super) build_and_podman: BTreeMap<String, String>,
    pub(super) build_and_podman_runtime: BTreeMap<String, String>,
    pub(super) build_and_podman_final: BTreeMap<String, String>,
    pub(super) build_and_podman_builds: BTreeMap<String, String>,
    pub(super) cache_encryption: BTreeMap<String, String>,
    pub(super) perl_cgi_support: BTreeMap<String, String>,
    pub(super) systemd_deployment: BTreeMap<String, String>,
    pub(super) config_snapshots: BTreeMap<String, String>,
    pub(super) pingora_core_patch: BTreeMap<String, String>,
    pub(super) supply_chain_security: BTreeMap<String, String>,
    pub(super) compression: BTreeMap<String, String>,
    pub(super) load_balancer_migration: BTreeMap<String, String>,
    pub(super) runtime_facts_and_policy_proofs: BTreeMap<String, String>,
    pub(super) production_readiness: BTreeMap<String, String>,
    pub(super) cache_backends: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct LanguageKeys {
    pub(super) selector_label: String,
    pub(super) english_eu: String,
    pub(super) english_uk: String,
    pub(super) english_us: String,
    pub(super) german: String,
    pub(super) french: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct NavKeys {
    pub(super) docs: String,
    pub(super) download: String,
    pub(super) changelog: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ReleaseKeys {
    pub(super) latest_stable: String,
    pub(super) latest_stable_release: String,
    pub(super) download_version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ShellKeys {
    pub(super) home_title: String,
    pub(super) view_on_github: String,
    pub(super) quick_start: String,
    pub(super) switch_color_theme: String,
    pub(super) links: String,
    pub(super) github_repository: String,
    pub(super) issues: String,
    pub(super) menu: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FooterKeys {
    pub(super) tagline: String,
    pub(super) project: String,
    pub(super) releases: String,
    pub(super) roadmap: String,
    pub(super) community: String,
    pub(super) discussions: String,
    pub(super) eupl_license: String,
    pub(super) valkyoth_org: String,
    pub(super) copyright_prefix: String,
    pub(super) built_with: String,
}
