use crate::content::{Locale, Site};
use crate::i18n_keys;

pub(crate) fn inject(site: &Site, active_locale: &Locale, slug: &str, html: String) -> String {
    let selector = render(site, active_locale, slug);
    if let Some(index) = html.rfind("</body>") {
        let mut output = String::with_capacity(html.len() + selector.len());
        output.push_str(&html[..index]);
        output.push_str(&selector);
        output.push_str(&html[index..]);
        output
    } else {
        format!("{html}{selector}")
    }
}

fn render(site: &Site, active_locale: &Locale, slug: &str) -> String {
    let label = i18n_keys::language_selector_label(active_locale);
    let active_display_name = i18n_keys::language_display_name(
        active_locale,
        &active_locale.locale_id,
        &active_locale.display_name,
    );
    let mut html = String::from(
        r#"<style>
.fh-language-switcher{position:fixed;right:1rem;bottom:1rem;z-index:60;font-family:Inter,ui-sans-serif,system-ui,sans-serif}
.fh-language-switcher summary{display:inline-flex;align-items:center;gap:.45rem;max-width:min(15rem,calc(100vw - 2rem));list-style:none;cursor:pointer;border:1px solid rgb(55 65 81);border-radius:.5rem;background:rgba(17,24,39,.94);color:rgb(229 231 235);padding:.45rem .65rem;font-size:.75rem;font-weight:700;box-shadow:0 10px 30px rgba(0,0,0,.28)}
.fh-language-switcher summary::-webkit-details-marker{display:none}
.fh-language-switcher summary span:last-child{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.fh-language-switcher summary span:last-child::after{content:" ▾";font-size:.82em}
.fh-language-menu{position:absolute;right:0;bottom:2.55rem;width:min(18rem,calc(100vw - 2rem));border:1px solid rgb(55 65 81);border-radius:.625rem;background:rgba(17,24,39,.98);padding:.55rem;display:grid;gap:.45rem;box-shadow:0 18px 40px rgba(0,0,0,.35)}
.fh-language-search{width:100%;border:1px solid rgb(55 65 81);border-radius:.5rem;background:rgba(3,7,18,.88);color:rgb(229 231 235);padding:.5rem .6rem;font:inherit;font-size:.8125rem;outline:none}
.fh-language-search:focus{border-color:rgb(34 211 238);box-shadow:0 0 0 3px rgba(34,211,238,.16)}
.fh-language-list{display:grid;gap:.2rem;max-height:min(17rem,42vh);overflow-y:auto;overscroll-behavior:contain;padding-right:.2rem;scrollbar-width:thin}
.fh-language-switcher a{display:flex;align-items:center;gap:.45rem;border-radius:.45rem;padding:.42rem .55rem;color:rgb(209 213 219);font-size:.8125rem;text-decoration:none;white-space:nowrap}
.fh-language-switcher a[hidden]{display:none!important}
.fh-language-switcher a:hover{background:rgb(31 41 55);color:white}
.fh-language-switcher a[aria-current=true]{background:rgb(34 211 238);color:rgb(3 7 18);font-weight:800}
.fh-language-flag{display:inline-flex;width:1.25em;justify-content:center;font-size:1rem;line-height:1}
</style>
<details class="fh-language-switcher">
  <summary aria-label=""#,
    );
    html.push_str(&html_escape(label));
    html.push_str(r#""><span class="fh-language-flag" aria-hidden="true">"#);
    html.push_str(language_flag(&active_locale.locale_id));
    html.push_str(r#"</span><span>"#);
    html.push_str(&html_escape(&active_display_name));
    html.push_str(
        r#"</span></summary>
  <div class="fh-language-menu">
    <input class="fh-language-search" type="search" aria-label=""#,
    );
    html.push_str(&html_escape(label));
    html.push_str(r#"" placeholder=""#);
    html.push_str(&html_escape(label));
    html.push_str(
        r#"" autocomplete="off" spellcheck="false">
    <div class="fh-language-list">
"#,
    );

    for link in site.language_links(&active_locale.locale_id, slug) {
        let active = if link.active {
            r#" aria-current="true""#
        } else {
            ""
        };
        let display_name =
            i18n_keys::language_display_name(active_locale, &link.locale_id, &link.display_name);
        html.push_str(&format!(
            r#"  <a href="{}"{}><span class="fh-language-flag" aria-hidden="true">{}</span><span>{}</span></a>
"#,
            html_escape(&link.href),
            active,
            language_flag(&link.locale_id),
            html_escape(&display_name)
        ));
    }

    html.push_str(
        r#"    </div>
  </div>
</details>
<script>
for (const switcher of document.querySelectorAll(".fh-language-switcher")) {
  const search = switcher.querySelector(".fh-language-search");
  const links = Array.from(switcher.querySelectorAll(".fh-language-list a"));
  if (!search || links.length === 0) {
    continue;
  }
  search.addEventListener("input", () => {
    const query = search.value.trim().toLocaleLowerCase();
    for (const link of links) {
      link.hidden = !link.textContent.toLocaleLowerCase().includes(query);
    }
  });
  switcher.addEventListener("toggle", () => {
    if (switcher.open) {
      search.value = "";
      search.dispatchEvent(new Event("input"));
      window.setTimeout(() => search.focus(), 0);
    }
  });
}
</script>
"#,
    );
    html
}

fn html_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn language_flag(locale_id: &str) -> &'static str {
    match locale_id {
        "en-EU" => "🇪🇺",
        "en-GB" => "🇬🇧",
        "en-US" => "🇺🇸",
        "de-DE" => "🇩🇪",
        "de-CH" => "🇨🇭",
        "fr-FR" => "🇫🇷",
        "sv-SE" => "🇸🇪",
        "nb-NO" => "🇳🇴",
        "nl-NL" => "🇳🇱",
        "fi-FI" => "🇫🇮",
        "is-IS" => "🇮🇸",
        "da-DK" => "🇩🇰",
        "es-ES" => "🇪🇸",
        "pt-PT" => "🇵🇹",
        "et-EE" => "🇪🇪",
        "lv-LV" => "🇱🇻",
        "el-GR" => "🇬🇷",
        "it-IT" => "🇮🇹",
        "lt-LT" => "🇱🇹",
        "hr-HR" => "🇭🇷",
        "cs-CZ" => "🇨🇿",
        "bs-BA" => "🇧🇦",
        "bg-BG" => "🇧🇬",
        "ro-RO" => "🇷🇴",
        "pl-PL" => "🇵🇱",
        "ru-RU" => "🇷🇺",
        "ja-JP" => "🇯🇵",
        "ko-KR" => "🇰🇷",
        "hu-HU" => "🇭🇺",
        "sk-SK" => "🇸🇰",
        "sl-SI" => "🇸🇮",
        "ga-IE" => "🇮🇪",
        "mt-MT" => "🇲🇹",
        "la-VA" => "🇻🇦",
        "uk-UA" => "🇺🇦",
        "sr-RS" => "🇷🇸",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::language_flag;

    #[test]
    fn maps_language_flags() {
        assert_eq!(language_flag("en-EU"), "🇪🇺");
        assert_eq!(language_flag("en-GB"), "🇬🇧");
        assert_eq!(language_flag("en-US"), "🇺🇸");
        assert_eq!(language_flag("de-DE"), "🇩🇪");
        assert_eq!(language_flag("de-CH"), "🇨🇭");
        assert_eq!(language_flag("fr-FR"), "🇫🇷");
        assert_eq!(language_flag("sv-SE"), "🇸🇪");
        assert_eq!(language_flag("nb-NO"), "🇳🇴");
        assert_eq!(language_flag("nl-NL"), "🇳🇱");
        assert_eq!(language_flag("fi-FI"), "🇫🇮");
        assert_eq!(language_flag("is-IS"), "🇮🇸");
        assert_eq!(language_flag("da-DK"), "🇩🇰");
        assert_eq!(language_flag("es-ES"), "🇪🇸");
        assert_eq!(language_flag("pt-PT"), "🇵🇹");
        assert_eq!(language_flag("et-EE"), "🇪🇪");
        assert_eq!(language_flag("lv-LV"), "🇱🇻");
        assert_eq!(language_flag("el-GR"), "🇬🇷");
        assert_eq!(language_flag("it-IT"), "🇮🇹");
        assert_eq!(language_flag("lt-LT"), "🇱🇹");
        assert_eq!(language_flag("hr-HR"), "🇭🇷");
        assert_eq!(language_flag("cs-CZ"), "🇨🇿");
        assert_eq!(language_flag("bs-BA"), "🇧🇦");
        assert_eq!(language_flag("bg-BG"), "🇧🇬");
        assert_eq!(language_flag("ro-RO"), "🇷🇴");
        assert_eq!(language_flag("pl-PL"), "🇵🇱");
        assert_eq!(language_flag("ru-RU"), "🇷🇺");
        assert_eq!(language_flag("ja-JP"), "🇯🇵");
        assert_eq!(language_flag("ko-KR"), "🇰🇷");
        assert_eq!(language_flag("hu-HU"), "🇭🇺");
        assert_eq!(language_flag("sk-SK"), "🇸🇰");
        assert_eq!(language_flag("sl-SI"), "🇸🇮");
        assert_eq!(language_flag("ga-IE"), "🇮🇪");
        assert_eq!(language_flag("mt-MT"), "🇲🇹");
        assert_eq!(language_flag("la-VA"), "🇻🇦");
        assert_eq!(language_flag("uk-UA"), "🇺🇦");
        assert_eq!(language_flag("sr-RS"), "🇷🇸");
        assert_eq!(language_flag("unknown"), "");
    }
}
