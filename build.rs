#![forbid(unsafe_code)]

use std::{env, fs, path::Path};

const MAX_ARTIFACT_BYTES: u64 = 2 * 1024 * 1024;

fn main() {
    println!("cargo:rerun-if-changed=config/locales.toml");
    println!("cargo:rerun-if-changed=config/i18n/keys");
    for path in [
        "index.html",
        "download.html",
        "changelog.html",
        "cookies.html",
        "privacy.html",
        "gdpr.html",
        "docs",
        "localized",
        "conf",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("manifest dir");
    let root = Path::new(&manifest_dir);
    let locale_ids = locale_ids(root);
    let part_files = source_part_files(root);
    let generated = generate_key_registry(root, &locale_ids, &part_files);
    let embedded_content = generate_embedded_content(root);

    let out_dir = env::var("OUT_DIR").expect("out dir");
    fs::write(Path::new(&out_dir).join("i18n_key_files.rs"), generated)
        .expect("write generated i18n key registry");
    fs::write(
        Path::new(&out_dir).join("embedded_content.rs"),
        embedded_content,
    )
    .expect("write embedded content registry");
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

fn generate_embedded_content(root: &Path) -> String {
    let mut html_paths = root_html_paths(root);
    collect_files(root, Path::new("docs"), is_html, &mut html_paths);
    collect_files(root, Path::new("localized"), is_html, &mut html_paths);
    html_paths.sort();
    html_paths.dedup();

    let mut artifact_paths = Vec::new();
    for directory in ["docs/source", "docs/releases", "conf"] {
        collect_files(
            root,
            Path::new(directory),
            is_allowed_artifact,
            &mut artifact_paths,
        );
    }
    artifact_paths.sort();

    let mut output = String::from("const EMBEDDED_HTML: &[(&str, &str)] = &[\n");
    for relative in html_paths {
        let absolute = root.join(&relative);
        output.push_str(&format!(
            "    (r#\"{}\"#, include_str!(r#\"{}\"#)),\n",
            relative.display(),
            absolute.display()
        ));
    }
    output.push_str("];\n\nconst EMBEDDED_ARTIFACTS: &[(&str, &[u8], &str)] = &[\n");
    for relative in artifact_paths {
        let absolute = root.join(&relative);
        let metadata = fs::symlink_metadata(&absolute).expect("artifact metadata");
        assert!(
            metadata.len() <= MAX_ARTIFACT_BYTES,
            "embedded artifact exceeds {MAX_ARTIFACT_BYTES} bytes: {}",
            relative.display()
        );
        output.push_str(&format!(
            "    (r#\"{}\"#, include_bytes!(r#\"{}\"#), r#\"{}\"#),\n",
            relative.display(),
            absolute.display(),
            artifact_content_type(&relative).expect("allowed artifact content type")
        ));
    }
    output.push_str("];\n");
    output
}

fn root_html_paths(root: &Path) -> Vec<std::path::PathBuf> {
    [
        "index.html",
        "download.html",
        "changelog.html",
        "cookies.html",
        "privacy.html",
        "gdpr.html",
    ]
    .into_iter()
    .map(|path| {
        let relative = std::path::PathBuf::from(path);
        assert_regular_file(root, &relative);
        relative
    })
    .collect()
}

fn collect_files(
    root: &Path,
    relative: &Path,
    allowed: fn(&Path) -> bool,
    output: &mut Vec<std::path::PathBuf>,
) {
    let absolute = root.join(relative);
    let metadata = fs::symlink_metadata(&absolute)
        .unwrap_or_else(|error| panic!("inspect {}: {error}", relative.display()));
    assert!(
        !metadata.file_type().is_symlink(),
        "embedded content path must not be a symlink: {}",
        relative.display()
    );
    if metadata.is_file() {
        if allowed(relative) {
            output.push(relative.to_path_buf());
        }
        return;
    }
    assert!(metadata.is_dir(), "embedded content path is not regular");

    let mut children = fs::read_dir(&absolute)
        .unwrap_or_else(|error| panic!("read {}: {error}", relative.display()))
        .map(|entry| entry.expect("embedded content directory entry").file_name())
        .collect::<Vec<_>>();
    children.sort();
    for child in children {
        collect_files(root, &relative.join(child), allowed, output);
    }
}

fn assert_regular_file(root: &Path, relative: &Path) {
    let metadata = fs::symlink_metadata(root.join(relative))
        .unwrap_or_else(|error| panic!("inspect {}: {error}", relative.display()));
    assert!(
        metadata.file_type().is_file(),
        "embedded HTML must be a regular file: {}",
        relative.display()
    );
}

fn is_html(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("html")
        && !path.starts_with("docs/source")
        && !path.starts_with("docs/releases")
}

fn is_allowed_artifact(path: &Path) -> bool {
    let extension = path.extension().and_then(|extension| extension.to_str());
    (path.starts_with("docs/source") && matches!(extension, Some("md" | "toml" | "tsv" | "txt")))
        || (path.starts_with("docs/releases") && extension == Some("md"))
        || (path.starts_with("conf") && extension == Some("toml"))
}

fn artifact_content_type(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("md") => Some("text/markdown; charset=utf-8"),
        Some("toml") => Some("application/toml; charset=utf-8"),
        Some("tsv") => Some("text/tab-separated-values; charset=utf-8"),
        Some("txt") => Some("text/plain; charset=utf-8"),
        _ => None,
    }
}
