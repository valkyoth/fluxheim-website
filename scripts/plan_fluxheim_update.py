#!/usr/bin/env python3
"""Plan a website update from a local Fluxheim checkout without editing files."""

from __future__ import annotations

import argparse
import filecmp
import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NOTE_RE = re.compile(r"RELEASE_NOTES_(\d+\.\d+\.\d+)\.md$")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fluxheim", default=str(ROOT.parent / "fluxheim"))
    parser.add_argument("--since", default="1.6.19")
    args = parser.parse_args()

    fluxheim = Path(args.fluxheim).resolve()
    if not fluxheim.is_dir():
        print(f"missing Fluxheim checkout: {fluxheim}")
        return 1

    site_version = load_toml(ROOT / "config/site.toml")["fluxheim_version"]
    source_version = load_toml(fluxheim / "Cargo.toml")["package"]["version"]
    catalog = load_toml(ROOT / "content/releases.toml")
    latest = catalog["latest"]

    print("Fluxheim website update plan")
    print(f"- website version: {site_version}")
    print(f"- release catalog:  {latest}")
    print(f"- source version:   {source_version}")
    print()

    if source_version == site_version == latest:
        print("Status: versions already match.")
    else:
        print("Status: website metadata needs a version update.")
        print(f"- update config/site.toml to {source_version}")
        print(f"- update content/releases.toml latest to {source_version}")
        print(f"- update container image tags to fluxheim-website:{source_version}")

    print_section("Release notes to mirror")
    for version in source_release_versions(fluxheim / "release-notes", args.since, source_version):
        source = fluxheim / "release-notes" / note_name(version)
        target = ROOT / "docs/releases" / note_name(version)
        status = "present" if target.is_file() else "missing"
        print(f"- {version}: {status} ({source.relative_to(fluxheim)})")

    print_section("Source docs that differ")
    changed_docs = differing_docs(fluxheim / "docs", ROOT / "docs/source")
    if changed_docs:
        for rel in changed_docs:
            print(f"- docs/{rel} -> docs/source/{rel}")
    else:
        print("- none detected")

    print_section("Public pages to inspect")
    print("- download.html release table and hero")
    print("- changelog.html latest card and backfilled release cards")
    print("- README.md version/update instructions")

    print_section("Required verification")
    print("- scripts/check_fluxheim_source.py --fluxheim ../fluxheim")
    print("- scripts/checks.sh")
    print("- scripts/smoke_local.sh")
    print("- scripts/podman_smoke.sh")
    return 0


def differing_docs(source_dir: Path, target_dir: Path) -> list[str]:
    results: list[str] = []
    for source in sorted(source_dir.iterdir()):
        if source.suffix not in {".md", ".tsv"}:
            continue
        target = target_dir / source.name
        if not target.is_file() or not filecmp.cmp(source, target, shallow=False):
            results.append(source.name)
    return results


def source_release_versions(path: Path, since: str, until: str) -> list[str]:
    versions = []
    for note in path.glob("RELEASE_NOTES_*.md"):
        match = NOTE_RE.match(note.name)
        if match and version_tuple(since) <= version_tuple(match.group(1)) <= version_tuple(until):
            versions.append(match.group(1))
    return sorted(versions, key=version_tuple)


def print_section(title: str) -> None:
    print()
    print(title)


def note_name(version: str) -> str:
    return f"RELEASE_NOTES_{version}.md"


def version_tuple(version: str) -> tuple[int, int, int]:
    major, minor, patch = version.split(".")
    return (int(major), int(minor), int(patch))


def load_toml(path: Path) -> dict[str, object]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


if __name__ == "__main__":
    raise SystemExit(main())
