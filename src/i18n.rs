use crate::content::Locale;

pub fn translate_html(locale: &Locale, html: String) -> String {
    html_lang(locale, html)
}

fn html_lang(locale: &Locale, html: String) -> String {
    html.replace(
        r#"<html lang="en""#,
        &format!(r#"<html lang="{}""#, escape_html(&locale.html_lang)),
    )
}

pub(crate) fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::translate_html;
    use crate::content::Site;

    #[test]
    fn escapes_language_attribute_even_without_startup_validation() {
        let site = Site::load().unwrap();
        let mut locale = site.default_locale().clone();
        locale.html_lang = "en\" onload=\"<script>&'".to_owned();
        let translated = translate_html(&locale, r#"<html lang="en">"#.to_owned());
        assert_eq!(
            translated,
            "<html lang=\"en&quot; onload=&quot;&lt;script&gt;&amp;&#x27;\">"
        );
    }

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
