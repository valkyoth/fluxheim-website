use std::fs;
use std::path::{Path, PathBuf};

const MAX_LINES: usize = 500;
const CHECKED_EXTENSIONS: &[&str] = &["rs", "html", "toml", "sh", "yml", "md", "py"];
const CHECKED_DIRS: &[&str] = &[
    "src",
    "templates",
    "content",
    "config",
    "conf",
    "scripts",
    "container",
    ".github",
    "docs/releases",
    "security",
];

#[test]
fn authored_project_files_stay_under_500_lines() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut oversized = Vec::new();

    for dir in CHECKED_DIRS {
        collect_oversized(&root.join(dir), &mut oversized);
    }

    assert!(
        oversized.is_empty(),
        "files exceed {MAX_LINES} lines: {oversized:?}"
    );
}

fn collect_oversized(path: &Path, oversized: &mut Vec<PathBuf>) {
    if !path.exists() {
        return;
    }

    if path.is_file() {
        if is_checked(path) && line_count(path) > MAX_LINES {
            oversized.push(path.to_path_buf());
        }
        return;
    }

    for entry in fs::read_dir(path).expect("read directory") {
        let entry = entry.expect("directory entry");
        collect_oversized(&entry.path(), oversized);
    }
}

fn is_checked(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| CHECKED_EXTENSIONS.contains(&extension))
}

fn line_count(path: &Path) -> usize {
    fs::read_to_string(path).expect("read file").lines().count()
}
