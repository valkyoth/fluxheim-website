use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=config/locales.toml");
    println!("cargo:rerun-if-changed=config/i18n/keys");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("manifest dir");
    let root = Path::new(&manifest_dir);
    let locale_ids = locale_ids(root);
    let part_files = source_part_files(root);
    let generated = generate_key_registry(root, &locale_ids, &part_files);

    let out_dir = env::var("OUT_DIR").expect("out dir");
    fs::write(Path::new(&out_dir).join("i18n_key_files.rs"), generated)
        .expect("write generated i18n key registry");
}

fn locale_ids(root: &Path) -> Vec<String> {
    let path = root.join("config/locales.toml");
    let contents = fs::read_to_string(&path).expect("read config/locales.toml");
    let value = toml::from_str::<toml::Value>(&contents).expect("parse config/locales.toml");
    let locales = value
        .get("locales")
        .and_then(toml::Value::as_array)
        .expect("config/locales.toml contains locales array");

    locales
        .iter()
        .map(|locale| {
            locale
                .get("locale_id")
                .and_then(toml::Value::as_str)
                .unwrap_or_else(|| panic!("locale entry has locale_id"))
                .to_owned()
        })
        .collect()
}

fn source_part_files(root: &Path) -> Vec<String> {
    let source_dir = root.join("config/i18n/keys/en-EU");
    let mut parts = fs::read_dir(&source_dir)
        .expect("read en-EU key part directory")
        .map(|entry| {
            let entry = entry.expect("read key part entry");
            let file_name = entry
                .file_name()
                .into_string()
                .expect("key part file name is utf-8");
            assert!(
                file_name.ends_with(".toml"),
                "key part files must be TOML: {file_name}"
            );
            file_name
        })
        .collect::<Vec<_>>();
    parts.sort();
    parts
}

fn generate_key_registry(root: &Path, locale_ids: &[String], part_files: &[String]) -> String {
    let mut output = String::from("const KEY_TOML_FILES: &[KeyTomlFile] = &[\n");
    for locale_id in locale_ids {
        output.push_str("    KeyTomlFile {\n");
        output.push_str(&format!(
            "        root: include_str!(r#\"{}\"#),\n",
            root.join(format!("config/i18n/keys/{locale_id}.toml"))
                .display()
        ));
        output.push_str("        parts: &[\n");
        for part in part_files {
            output.push_str(&format!(
                "            include_str!(r#\"{}\"#),\n",
                root.join(format!("config/i18n/keys/{locale_id}/{part}"))
                    .display()
            ));
        }
        output.push_str("        ],\n");
        output.push_str("    },\n");
    }
    output.push_str("];\n");
    output
}
