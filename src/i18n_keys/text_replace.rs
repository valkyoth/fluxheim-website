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
            replacements.push((format!(">{source}<"), format!(">{target}<")));
            replacements.push((format!("=\"{source}\""), format!("=\"{target}\"")));
            if source.len() >= 40 {
                replacements.push((source.to_owned(), target.to_owned()));
                if source != target {
                    replacements.push((html_escaped(source), target.to_owned()));
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
            .map(|(source, target)| (format!("=\"{source}\""), format!("=\"{target}\"")))
            .collect();
        let output = replace_many_cached(&self, 1, attributes);

        let markup = entries
            .iter()
            .filter(|(source, _)| source.contains('<'))
            .map(|(source, target)| ((*source).to_owned(), (*target).to_owned()))
            .collect();
        let output = replace_many_cached(&output, 2, markup);

        let mut exact = HashMap::new();
        let mut text = Vec::new();
        for (source, target) in entries {
            if source.contains('<') {
                continue;
            }
            if source.len() < 24 {
                exact.entry(source.to_owned()).or_insert(target.to_owned());
            } else {
                text.push((source.to_owned(), target.to_owned()));
            }
            if source != target {
                let escaped = html_escaped(source);
                if source.len() < 24 {
                    exact.entry(escaped).or_insert(target.to_owned());
                } else {
                    text.push((escaped, target.to_owned()));
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
            replacements.push((source.to_owned(), target.to_owned()));
            if source != target {
                replacements.push((html_escaped(source), target.to_owned()));
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

fn html_escaped(source: &str) -> String {
    source
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
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
