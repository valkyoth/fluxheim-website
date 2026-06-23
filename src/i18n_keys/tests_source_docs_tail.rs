use crate::{content::Site, i18n_keys::apply_shared_keys};

#[test]
fn applies_stable_compliance_evidence_template_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let fr = site.locale("fr-FR").expect("French locale");
    let html = concat!(
        "<title>Compliance Evidence Template — Fluxheim Source Docs</title>",
        "<span>Compliance Evidence Template</span>",
        "<h1>Compliance Evidence Package Template</h1>",
        "<h2>Release Metadata</h2>",
        "<h2>Candidate TOE Boundary</h2>",
        "<p>Pick one candidate Target of Evaluation (TOE) boundary for this evidence package. Do not mix boundaries in the same evidence record.</p>",
    )
    .to_owned();

    let translated = apply_shared_keys(fr, html, "1.6.28");
    let unrelated = apply_shared_keys(fr, ">Candidate TOE Boundary<".to_owned(), "1.6.28");

    assert!(translated.contains("Modele de preuves de conformite"));
    assert!(translated.contains("Modele de paquet de preuves de conformite"));
    assert!(translated.contains("Metadonnees de release"));
    assert!(translated.contains("Frontiere TOE candidate"));
    assert!(translated.contains("paquet de preuves"));
    assert!(unrelated.contains(">Candidate TOE Boundary<"));
}
