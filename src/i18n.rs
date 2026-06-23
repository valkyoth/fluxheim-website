use crate::content::Locale;

pub fn translate_html(locale: &Locale, html: String) -> String {
    html_lang(locale, html)
}

fn html_lang(locale: &Locale, html: String) -> String {
    html.replace(
        r#"<html lang="en""#,
        &format!(r#"<html lang="{}""#, locale.html_lang),
    )
}

#[cfg(test)]
mod tests {
    use super::translate_html;
    use crate::content::Site;

    #[test]
    fn translates_german_html_lang() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("de-DE").expect("German locale");
        let html = r#"<html lang="en"><body>Fluxheim</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains(r#"<html lang="de-DE""#));
    }

    #[test]
    fn translates_french_html_lang() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("fr-FR").expect("French locale");
        let html = r#"<html lang="en"><body>Fluxheim</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains(r#"<html lang="fr-FR""#));
    }

    #[test]
    fn leaves_body_text_unchanged_after_stable_key_pass() {
        let site = Site::load().expect("site loads");
        let locale = site.locale("de-DE").expect("German locale");
        let html = r#"<html lang="en"><body>Stable-key-rendered text</body></html>"#;
        let translated = translate_html(locale, html.to_owned());

        assert!(translated.contains("Stable-key-rendered text"));
    }
}
