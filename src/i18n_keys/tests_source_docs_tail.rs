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

#[test]
fn applies_stable_common_criteria_roadmap_keys_only_on_source_page() {
    let site = Site::load().expect("site loads");
    let de = site.locale("de-DE").expect("German locale");
    let html = concat!(
        "<title>Common Criteria Roadmap — Fluxheim Source Docs</title>",
        "<span>Common Criteria Roadmap</span>",
        "<h1>Common Criteria Readiness Roadmap</h1>",
        "<p>This document tracks how Fluxheim can use ISO/IEC 15408:2026 concepts as an engineering and evidence framework. It is not a certification claim. Common Criteria evaluation is a product evaluation track, while FIPS 140-3 and ISO/IEC 19790 are cryptographic-module tracks. They can support each other, but they are not interchangeable.</p>",
        "<h2>Security Problem Definition</h2>",
    )
    .to_owned();

    let translated = apply_shared_keys(de, html, "1.6.28");
    let unrelated = apply_shared_keys(de, ">Security Problem Definition<".to_owned(), "1.6.28");

    assert!(translated.contains("Common-Criteria-Roadmap"));
    assert!(translated.contains("Common-Criteria-Readiness-Roadmap"));
    assert!(translated.contains("ISO/IEC-15408:2026-Konzepte"));
    assert!(translated.contains("Definition des Sicherheitsproblems"));
    assert!(unrelated.contains(">Security Problem Definition<"));
}
