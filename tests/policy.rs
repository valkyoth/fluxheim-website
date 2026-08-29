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
fn prism_is_locked_to_the_dom_clobbering_fixed_release() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let package = fs::read_to_string(root.join("package-lock.json")).expect("package lock");
    let prism = fs::read_to_string(root.join("assets/js/prism.min.js")).expect("Prism asset");
    assert!(package.contains(r#""prismjs": "1.30.0""#));
    assert!(prism.contains(r#"document.currentScript.tagName"#));
    assert!(prism.contains(r#""SCRIPT"==="#));
}

#[test]
fn immutable_pages_and_artifacts_are_embedded() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let legacy = fs::read_to_string(root.join("src/legacy.rs")).expect("legacy module");
    let dockerfile = fs::read_to_string(root.join("container/Dockerfile")).expect("Dockerfile");
    assert!(!legacy.contains("read_to_end"));
    assert!(!legacy.contains("read_to_string"));
    assert!(!legacy.contains("OpenOptions"));
    assert!(legacy.contains("embedded_content.rs"));
    assert!(!dockerfile.contains("COPY --chown=root:root docs/"));
    assert!(!dockerfile.contains("COPY --chown=root:root conf/"));
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
        assert!(compose.contains("pids_limit: 128"), "{relative}");
    }

    let compose =
        fs::read_to_string(root.join("container/podman-compose.yml")).expect("website compose");
    assert!(compose.contains("127.0.0.1:8080:8080"));
    assert!(!compose.contains("- \"8080:8080\""));
}

#[test]
fn startup_memory_gate_uses_the_cargo_emitted_binary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let gate = fs::read_to_string(root.join("scripts/check_startup_memory.sh"))
        .expect("startup memory gate");
    assert!(gate.contains("--message-format=json-render-diagnostics"));
    assert!(gate.contains("scripts/cargo_binary_path.py fluxheim-website"));
    assert!(!gate.contains("target/release/fluxheim-website"));
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
