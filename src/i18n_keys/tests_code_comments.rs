use super::apply_shared_keys;
use crate::content::Site;

#[test]
fn applies_stable_code_comment_keys_globally() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "# Pull GHCR images (full, load-balancer, cache, proxy, and PHP variants)",
        "\n# Run rootless — internal ports 8080 and 8443",
        "\nNative TLS/listener preview and compatibility-boundary release with explicit ",
        "<code>pingora-compat</code>",
        ", Pingora-free native web TLS proofs, ",
        "<code>fluxheim-tls</code>",
        "and Fluxheim-owned rustls/OpenSSL SNI and certificate material.",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");

    assert!(translated.contains("# GHCR-Images ziehen"));
    assert!(translated.contains("# Rootless ausführen — interne Ports 8080 und 8443"));
    assert!(translated.contains("Release mit nativer TLS-/Listener-Vorschau"));
    assert!(translated.contains("<code>pingora-compat</code>"));
    assert!(translated.contains("mit Pingora-freien nativen Web-TLS-Nachweisen"));
    assert!(translated.contains("<code>fluxheim-tls</code>"));
    assert!(translated.contains("Fluxheim-eigenem rustls/OpenSSL-SNI"));
}
