use std::collections::BTreeMap;

pub(super) trait HtmlTextReplace {
    fn replace_home_marker(self, from: &str, to: &str) -> String;
    fn replace_attr_value(self, from: &str, to: &str) -> String;
    fn replace_map(
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
        }

        output
    }
}
