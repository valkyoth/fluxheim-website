use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

type CacheKey = (u8, Vec<(String, String)>);
thread_local! {
    static REPLACER_CACHE: RefCell<HashMap<CacheKey, Arc<MultiReplacer>>> =
        RefCell::new(HashMap::new());
}

pub(super) trait HtmlTextReplace {
    fn replace_map(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String;
    fn replace_map_text_nodes(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String;
    fn replace_map_everywhere(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String;
}

impl HtmlTextReplace for String {
    fn replace_map(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let entries = sorted_entries(source, target);
        let mut replacements = Vec::new();
        for (source, target) in entries {
            let safe_text = safe_html_translation(source, target);
            let safe_attribute = html_escaped(target);
            replacements.push((format!(">{source}<"), format!(">{safe_text}<")));
            replacements.push((format!("=\"{source}\""), format!("=\"{safe_attribute}\"")));
            if source.len() >= 40 {
                replacements.push((source.to_owned(), safe_text.clone()));
                if source != target {
                    replacements.push((html_escaped(source), safe_text));
                }
            }
        }
        replace_many_cached(&self, 0, replacements)
    }

    fn replace_map_text_nodes(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let entries = sorted_entries(source, target);
        let attributes = entries
            .iter()
            .map(|(source, target)| {
                (
                    format!("=\"{source}\""),
                    format!("=\"{}\"", html_escaped(target)),
                )
            })
            .collect();
        let output = replace_many_cached(&self, 1, attributes);

        let markup = entries
            .iter()
            .filter(|(source, _)| source.contains('<'))
            .map(|(source, target)| ((*source).to_owned(), safe_html_translation(source, target)))
            .collect();
        let output = replace_many_cached(&output, 2, markup);

        let mut exact = HashMap::new();
        let mut text = Vec::new();
        for (source, target) in entries {
            if source.contains('<') {
                continue;
            }
            if source.len() < 24 {
                exact
                    .entry(source.to_owned())
                    .or_insert_with(|| html_escaped(target));
            } else {
                text.push((source.to_owned(), html_escaped(target)));
            }
            if source != target {
                let escaped = html_escaped(source);
                if source.len() < 24 {
                    exact.entry(escaped).or_insert_with(|| html_escaped(target));
                } else {
                    text.push((escaped, html_escaped(target)));
                }
            }
        }
        let replacer = cached_replacer(3, text);
        replace_text_nodes(&output, &exact, replacer)
    }

    fn replace_map_everywhere(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let mut replacements = Vec::new();
        for (source, target) in sorted_entries(source, target) {
            let safe_target = html_escaped(target);
            replacements.push((source.to_owned(), safe_target.clone()));
            if source != target {
                replacements.push((html_escaped(source), safe_target));
            }
        }
        replace_many_cached(&self, 4, replacements)
    }
}

fn sorted_entries<'a>(
    source: &'a BTreeMap<String, String>,
    target: &'a BTreeMap<String, String>,
) -> Vec<(&'a str, &'a str)> {
    let mut entries: Vec<_> = source
        .iter()
        .map(|(key, source)| {
            let target = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            (source.as_str(), target.as_str())
        })
        .collect();
    entries.sort_by_key(|(source, _)| std::cmp::Reverse(source.len()));
    entries
}

fn replace_many_cached(input: &str, kind: u8, replacements: Vec<(String, String)>) -> String {
    let Some(replacer) = cached_replacer(kind, replacements) else {
        return input.to_owned();
    };
    replacer.replace(input)
}

fn cached_replacer(kind: u8, replacements: Vec<(String, String)>) -> Option<Arc<MultiReplacer>> {
    if replacements.is_empty() {
        return None;
    }
    let key = (kind, replacements);
    REPLACER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(replacer) = cache.get(&key) {
            return Some(replacer.clone());
        }
        let replacer = Arc::new(MultiReplacer::new(key.1.clone())?);
        cache.insert(key, replacer.clone());
        Some(replacer)
    })
}

pub(super) fn clear_replacer_cache() {
    REPLACER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.clear();
        cache.shrink_to_fit();
    });
}

#[cfg(test)]
pub(super) fn replacer_cache_len() -> usize {
    REPLACER_CACHE.with(|cache| cache.borrow().len())
}

struct MultiReplacer {
    matcher: AhoCorasick,
    replacements: Vec<(String, String)>,
}

impl MultiReplacer {
    fn new(replacements: Vec<(String, String)>) -> Option<Self> {
        let mut seen = HashSet::new();
        let replacements: Vec<_> = replacements
            .into_iter()
            .filter(|(source, _)| !source.is_empty() && seen.insert(source.clone()))
            .collect();
        if replacements.is_empty() {
            return None;
        }

        let matcher = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(replacements.iter().map(|(source, _)| source))
            .expect("valid non-empty i18n patterns");
        Some(Self {
            matcher,
            replacements,
        })
    }

    fn replace(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut copied = 0;
        for matched in self.matcher.find_iter(input) {
            output.push_str(&input[copied..matched.start()]);
            output.push_str(&self.replacements[matched.pattern().as_usize()].1);
            copied = matched.end();
        }
        output.push_str(&input[copied..]);
        output
    }
}

pub(super) fn html_escaped(source: &str) -> String {
    source
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn safe_html_translation(source: &str, target: &str) -> String {
    if !source.contains('<') {
        return html_escaped(target);
    }

    sanitize_rich_translation(source, target).unwrap_or_else(|| html_escaped(target))
}

fn sanitize_rich_translation(source: &str, target: &str) -> Option<String> {
    let source_tags = html_tag_ranges(source)?;
    let target_tags = html_tag_ranges(target)?;
    if source_tags.len() != target_tags.len()
        || source_tags
            .iter()
            .zip(&target_tags)
            .any(|((start, end), (target_start, target_end))| {
                source[*start..*end] != target[*target_start..*target_end]
            })
    {
        return None;
    }

    let mut output = String::with_capacity(target.len());
    let mut copied = 0;
    for (start, end) in target_tags {
        output.push_str(&html_escaped(&target[copied..start]));
        output.push_str(&target[start..end]);
        copied = end;
    }
    output.push_str(&html_escaped(&target[copied..]));
    Some(output)
}

fn html_tag_ranges(value: &str) -> Option<Vec<(usize, usize)>> {
    let mut tags = Vec::new();
    let mut offset = 0;
    while let Some(relative_start) = value[offset..].find('<') {
        let start = offset + relative_start;
        let relative_end = value[start..].find('>')?;
        let end = start + relative_end + 1;
        let name = value[start + 1..end - 1].trim_start_matches('/');
        if !name.starts_with(|character: char| character.is_ascii_alphabetic()) {
            return None;
        }
        tags.push((start, end));
        offset = end;
    }
    Some(tags)
}

fn replace_text_nodes(
    html: &str,
    exact: &HashMap<String, String>,
    replacer: Option<Arc<MultiReplacer>>,
) -> String {
    let mut output = String::with_capacity(html.len());
    let mut text = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        if in_tag {
            output.push(ch);
            if ch == '>' {
                in_tag = false;
            }
        } else if ch == '<' {
            replace_text_segment(&mut output, &text, exact, replacer.as_deref());
            text.clear();
            output.push(ch);
            in_tag = true;
        } else {
            text.push(ch);
        }
    }

    replace_text_segment(&mut output, &text, exact, replacer.as_deref());
    output
}

fn replace_text_segment(
    output: &mut String,
    text: &str,
    exact: &HashMap<String, String>,
    replacer: Option<&MultiReplacer>,
) {
    if let Some(replacement) = exact.get(text) {
        output.push_str(replacement);
    } else if let Some(replacer) = replacer {
        output.push_str(&replacer.replace(text));
    } else {
        output.push_str(text);
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlTextReplace;
    use std::collections::BTreeMap;

    #[test]
    fn escapes_plain_text_and_attribute_translations() {
        let source = BTreeMap::from([("label".to_owned(), "Welcome".to_owned())]);
        let target = BTreeMap::from([(
            "label".to_owned(),
            r#"" onmouseover="alert(1)<script>alert(2)</script>"#.to_owned(),
        )]);
        let html = r#"<p title="Welcome">Welcome</p>"#.to_owned();
        let rendered = html.replace_map(&source, &target);

        assert!(!rendered.contains(r#"title="" onmouseover="#));
        assert!(!rendered.contains("<script>"));
        assert!(rendered.contains("&quot; onmouseover=&quot;alert(1)"));
        assert!(rendered.contains("&lt;script&gt;alert(2)&lt;/script&gt;"));
    }

    #[test]
    fn rich_translations_keep_only_source_approved_markup() {
        let source = BTreeMap::from([(
            "notice".to_owned(),
            r#"Read <code class="safe">this</code>."#.to_owned(),
        )]);
        let safe = BTreeMap::from([(
            "notice".to_owned(),
            r#"Les <code class="safe">dette</code>."#.to_owned(),
        )]);
        let unsafe_target = BTreeMap::from([(
            "notice".to_owned(),
            r#"Les <code class="safe" onclick="alert(1)">dette</code>."#.to_owned(),
        )]);
        let html = r#"<p>Read <code class="safe">this</code>.</p>"#.to_owned();

        let rendered = html.clone().replace_map_text_nodes(&source, &safe);
        assert!(rendered.contains(r#"Les <code class="safe">dette</code>."#));

        let rendered = html.replace_map_text_nodes(&source, &unsafe_target);
        assert!(!rendered.contains(r#"<code class="safe" onclick="#));
        assert!(rendered.contains("&lt;code class=&quot;safe&quot; onclick="));
    }
}
