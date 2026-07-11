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

#[test]
fn external_container_dependencies_are_digest_pinned() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dockerfile = fs::read_to_string(root.join("container/Dockerfile")).expect("Dockerfile");
    for line in dockerfile.lines().filter(|line| line.starts_with("FROM ")) {
        assert!(line.contains("@sha256:"), "unpinned base image: {line}");
    }

    let compose = fs::read_to_string(root.join("container/observability/podman-compose.yml"))
        .expect("observability compose");
    for line in compose
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("image: docker.io/") || line.starts_with("image: cgr.dev/"))
    {
        assert!(line.contains("@sha256:"), "unpinned service image: {line}");
    }
}

#[test]
fn website_compose_keeps_rootless_hardening() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative in [
        "container/podman-compose.yml",
        "container/observability/podman-compose.yml",
    ] {
        let compose = fs::read_to_string(root.join(relative)).expect("compose file");
        assert!(compose.contains("read_only: true"), "{relative}");
        assert!(compose.contains("- ALL"), "{relative}");
        assert!(compose.contains("no-new-privileges:true"), "{relative}");
        assert!(compose.contains("noexec,nosuid,nodev"), "{relative}");
    }
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
