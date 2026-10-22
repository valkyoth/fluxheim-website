#!/usr/bin/env python3
"""Compare website release metadata with a local Fluxheim checkout."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION_RE = re.compile(r"RELEASE_NOTES_(\d+\.\d+\.\d+)\.md$")
REF_RE = re.compile(r"v\d+\.\d+\.\d+")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--fluxheim",
        default=str(ROOT.parent / "fluxheim"),
        help="path to the local Fluxheim checkout",
    )
    parser.add_argument(
        "--since",
        default="1.6.19",
        help="oldest Fluxheim release note version the website must mirror",
    )
    parser.add_argument("--ref", help="immutable Fluxheim release tag, for example v1.8.3")
    args = parser.parse_args()

    fluxheim = Path(args.fluxheim).resolve()
    if not fluxheim.is_dir():
        print(f"missing Fluxheim checkout: {fluxheim}", file=sys.stderr)
        return 1

    if args.ref and not REF_RE.fullmatch(args.ref):
        print("--ref must be an exact vMAJOR.MINOR.PATCH tag", file=sys.stderr)
        return 1

    site_version = load_toml(ROOT / "config/site.toml")["fluxheim_version"]
    source_cargo = (
        git_text(fluxheim, args.ref, "Cargo.toml")
        if args.ref
        else (fluxheim / "Cargo.toml").read_text(encoding="utf-8")
    )
    fluxheim_version = tomllib.loads(source_cargo)["package"]["version"]

    print(f"website Fluxheim version: {site_version}")
    print(f"source Fluxheim version:  {fluxheim_version}")

    if site_version != fluxheim_version:
        print("version mismatch: website needs a release metadata update", file=sys.stderr)
        return 1

    expected = release_note_name(site_version)
    website_notes = ROOT / "docs/releases" / expected
    errors: list[str] = []

    if args.ref:
        source_versions = ref_release_versions(fluxheim, args.ref)
        if site_version not in source_versions:
            errors.append(f"missing source release notes: {args.ref}:release-notes/{expected}")
    else:
        source_notes = fluxheim / "release-notes" / expected
        source_versions = mirrored_versions(fluxheim / "release-notes", args.since, site_version)
        if not source_notes.is_file():
            errors.append(f"missing source release notes: {source_notes}")
    if not website_notes.is_file():
        errors.append(f"missing website release notes: {website_notes}")

    for version in selected_versions(source_versions, args.since, site_version):
        mirrored = ROOT / "docs/releases" / release_note_name(version)
        if not mirrored.is_file():
            errors.append(f"missing mirrored website release notes for {version}: {mirrored}")

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    print(f"release notes present: {expected}")
    return 0


def mirrored_versions(path: Path, since: str, until: str) -> list[str]:
    versions = []
    for note in path.glob("RELEASE_NOTES_*.md"):
        match = VERSION_RE.match(note.name)
        if match and version_tuple(since) <= version_tuple(match.group(1)) <= version_tuple(until):
            versions.append(match.group(1))
    return sorted(versions, key=version_tuple)


def ref_release_versions(fluxheim: Path, ref: str) -> list[str]:
    paths = git_output(fluxheim, "ls-tree", "-r", "--name-only", ref, "release-notes")
    versions = []
    for path in paths.splitlines():
        match = VERSION_RE.search(path)
        if match:
            versions.append(match.group(1))
    return versions


def selected_versions(versions: list[str], since: str, until: str) -> list[str]:
    return sorted(
        (version for version in versions if version_tuple(since) <= version_tuple(version) <= version_tuple(until)),
        key=version_tuple,
    )


def git_text(fluxheim: Path, ref: str, path: str) -> str:
    return git_output(fluxheim, "show", f"{ref}:{path}")


def git_output(fluxheim: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(fluxheim), *args],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout


def release_note_name(version: str) -> str:
    return f"RELEASE_NOTES_{version}.md"


def version_tuple(version: str) -> tuple[int, int, int]:
    major, minor, patch = version.split(".")
    return (int(major), int(minor), int(patch))


def load_toml(path: Path) -> dict[str, object]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


if __name__ == "__main__":
    raise SystemExit(main())
