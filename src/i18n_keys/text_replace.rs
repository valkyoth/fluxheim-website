use std::collections::BTreeMap;

pub(super) trait HtmlTextReplace {
    fn replace_home_marker(self, from: &str, to: &str) -> String;
    fn replace_attr_value(self, from: &str, to: &str) -> String;
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
    fn replace_home_marker(self, from: &str, to: &str) -> String {
        self.replace(&format!(">{from}<"), &format!(">{to}<"))
    }

    fn replace_attr_value(self, from: &str, to: &str) -> String {
        self.replace(&format!("=\"{from}\""), &format!("=\"{to}\""))
    }

    fn replace_map(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let mut output = self;
        let mut entries: Vec<_> = source.iter().collect();
        entries.sort_by_key(|(_, source)| std::cmp::Reverse(source.len()));

        for (key, source) in entries {
            let replacement = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            output = output.replace_home_marker(source, replacement);
            output = output.replace_attr_value(source, replacement);
            if source.len() >= 40 {
                output = output.replace(source, replacement);
                if source != replacement {
                    output = output.replace(&html_escaped(source), replacement);
                }
            }
        }

        output
    }

    fn replace_map_text_nodes(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let mut output = self;
        let mut entries: Vec<_> = source.iter().collect();
        entries.sort_by_key(|(_, source)| std::cmp::Reverse(source.len()));

        for (key, source) in entries {
            let replacement = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            output = output.replace_attr_value(source, replacement);
            if source.contains('<') {
                output = output.replace(source, replacement);
            } else if source.len() < 24 {
                output = replace_exact_text_nodes(&output, source, replacement);
            } else {
                output = replace_text_nodes(&output, source, replacement);
            }
            if source != replacement {
                let escaped = html_escaped(source);
                if source.len() < 24 {
                    output = replace_exact_text_nodes(&output, &escaped, replacement);
                } else {
                    output = replace_text_nodes(&output, &escaped, replacement);
                }
            }
        }

        output
    }

    fn replace_map_everywhere(
        self,
        source: &BTreeMap<String, String>,
        target: &BTreeMap<String, String>,
    ) -> String {
        let mut output = self;
        let mut entries: Vec<_> = source.iter().collect();
        entries.sort_by_key(|(_, source)| std::cmp::Reverse(source.len()));

        for (key, source) in entries {
            let replacement = target
                .get(key)
                .unwrap_or_else(|| panic!("target i18n key exists: {key}"));
            output = output.replace(source, replacement);
            if source != replacement {
                output = output.replace(&html_escaped(source), replacement);
            }
        }

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

fn replace_text_nodes(html: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return html.to_owned();
    }

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
            output.push_str(&text.replace(from, to));
            text.clear();
            output.push(ch);
            in_tag = true;
        } else {
            text.push(ch);
        }
    }

    output.push_str(&text.replace(from, to));
    output
}

fn replace_exact_text_nodes(html: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return html.to_owned();
    }

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
            if text == from {
                output.push_str(to);
            } else {
                output.push_str(&text);
            }
            text.clear();
            output.push(ch);
            in_tag = true;
        } else {
            text.push(ch);
        }
    }

    if text == from {
        output.push_str(to);
    } else {
        output.push_str(&text);
    }
    output
}
