use crate::content::{Locale, Site};
use crate::i18n_keys;
use crate::observability::{normalize_route, section_for_slug};

pub fn enhance(site: &Site, active_locale: &Locale, slug: &str, html: String) -> String {
    let html = inject_legal_links(site, active_locale, html);
    inject_visible_time_beacon(active_locale, slug, html)
}

fn inject_legal_links(site: &Site, active_locale: &Locale, html: String) -> String {
    let cookies = i18n_keys::common_text(active_locale, "cookies", "Cookies");
    let privacy = i18n_keys::common_text(active_locale, "privacy_policy", "Privacy Policy");
    let gdpr = i18n_keys::common_text(active_locale, "gdpr", "GDPR");
    let links = format!(
        r#"<style>
.fh-legal-links{{display:flex;justify-content:center;align-items:center;gap:.55rem;padding:0 1rem 1rem;color:rgb(107 114 128);font-family:Inter,ui-sans-serif,system-ui,sans-serif;font-size:.8125rem}}
.fh-legal-links a{{color:rgb(156 163 175);text-decoration:none}}
.fh-legal-links a:hover{{color:rgb(229 231 235);text-decoration:underline}}
</style>
<nav class="fh-legal-links" aria-label="Legal">
  <a href="{}">{}</a>
  <span aria-hidden="true">·</span>
  <a href="{}">{}</a>
  <span aria-hidden="true">·</span>
  <a href="{}">{}</a>
</nav>
"#,
        html_escape(&site.path_for(active_locale, "cookies")),
        html_escape(&cookies),
        html_escape(&site.path_for(active_locale, "privacy")),
        html_escape(&privacy),
        html_escape(&site.path_for(active_locale, "gdpr")),
        html_escape(&gdpr)
    );
    inject_before_body_end(html, &links)
}

fn inject_visible_time_beacon(active_locale: &Locale, slug: &str, html: String) -> String {
    let route = normalize_route(slug);
    let section = section_for_slug(slug);
    if !matches!(
        section,
        "home" | "docs" | "download" | "changelog" | "source_docs" | "legal"
    ) {
        return html;
    }
    let script = format!(
        r#"<script>
(() => {{
  let started = Date.now();
  let sent = false;
  const releasePrefix = "https://github.com/valkyoth/fluxheim/releases/download/v";
  const githubPrefix = "https://github.com/valkyoth/fluxheim";
  const payload = () => JSON.stringify({{
    locale: "{}",
    route: "{}",
    section: "{}",
    seconds: Math.max(0, Math.min(86400, Math.round((Date.now() - started) / 1000)))
  }});
  const send = () => {{
    if (sent) return;
    sent = true;
    navigator.sendBeacon("/telemetry/page-visible", new Blob([payload()], {{type: "application/json"}}));
  }};
  document.addEventListener("visibilitychange", () => {{
    if (document.visibilityState === "hidden") send();
  }});
  document.addEventListener("click", event => {{
    const anchor = event.target.closest && event.target.closest("a[href]");
    if (!anchor) return;
    const href = anchor.href;
    if (href.startsWith(releasePrefix)) {{
      const artifact = href.slice(href.lastIndexOf("/") + 1);
      navigator.sendBeacon("/telemetry/click", new Blob([JSON.stringify({{
        kind: "download", locale: "{}", artifact
      }})], {{type: "application/json"}}));
    }} else if (href === githubPrefix || href.startsWith(githubPrefix + "/releases")) {{
      navigator.sendBeacon("/telemetry/click", new Blob([JSON.stringify({{
        kind: "github", locale: "{}", target: href.startsWith(githubPrefix + "/releases") ? "releases" : "repo"
      }})], {{type: "application/json"}}));
    }}
  }}, true);
  window.addEventListener("pagehide", send);
}})();
</script>
"#,
        js_escape(&active_locale.locale_id),
        js_escape(&route),
        js_escape(section),
        js_escape(&active_locale.locale_id),
        js_escape(&active_locale.locale_id)
    );
    inject_before_body_end(html, &script)
}

fn inject_before_body_end(html: String, addition: &str) -> String {
    if let Some(index) = html.rfind("</body>") {
        let mut output = String::with_capacity(html.len() + addition.len());
        output.push_str(&html[..index]);
        output.push_str(addition);
        output.push_str(&html[index..]);
        output
    } else {
        format!("{html}{addition}")
    }
}

fn js_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
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
