#!/usr/bin/env python3
"""Validate structured release metadata against the rendered legacy site."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION_RE = re.compile(r"^\d+\.\d+\.\d+$")


def main() -> int:
    site = load_toml(ROOT / "config/site.toml")
    releases = load_toml(ROOT / "content/releases.toml")
    errors: list[str] = []

    latest = releases.get("latest")
    site_version = site.get("fluxheim_version")
    if latest != site_version:
        errors.append(
            f"content/releases.toml latest {latest!r} != site fluxheim_version {site_version!r}"
        )
    expected_image = f"fluxheim-website:{site_version}"
    for path in ("container/podman-compose.yml", "scripts/podman_smoke.sh"):
        body = read(ROOT / path)
        if expected_image not in body:
            errors.append(f"{path} does not use image tag {expected_image}")

    rows = releases.get("release", [])
    versions = [row.get("version") for row in rows]
    if len(versions) != len(set(versions)):
        errors.append("content/releases.toml contains duplicate release versions")

    for version in versions:
        if not isinstance(version, str) or not VERSION_RE.match(version):
            errors.append(f"invalid release version {version!r}")

    if versions != sorted(versions, key=version_tuple, reverse=True):
        errors.append("content/releases.toml releases must be newest-first")

    if versions and versions[0] != latest:
        errors.append("first content/releases.toml release must match latest")

    for row in rows:
        validate_release(row, errors)

    rendered = {
        "download.html": read(ROOT / "download.html"),
        "changelog.html": read(ROOT / "changelog.html"),
    }
    for version in versions:
        needle = f"v{version}"
        for name, body in rendered.items():
            if not page_mentions_version(name, body, version, latest):
                errors.append(f"{name} does not mention {needle}")

    if errors:
        for error in errors:
            print(f"release-data error: {error}", file=sys.stderr)
        return 1

    print(f"release-data ok: {len(versions)} releases, latest {latest}")
    return 0


def validate_release(row: dict[str, object], errors: list[str]) -> None:
    version = row.get("version")
    for key in ("date", "title", "summary"):
        if not isinstance(row.get(key), str) or not row[key]:
            errors.append(f"release {version!r} missing {key}")

    if not isinstance(version, str):
        return

    notes = ROOT / "docs/releases" / f"RELEASE_NOTES_{version}.md"
    if not notes.is_file():
        errors.append(f"missing release notes for {version}: {notes.relative_to(ROOT)}")
        return

    title = f"# Fluxheim {version} Release Notes"
    if title not in read(notes):
        errors.append(f"{notes.relative_to(ROOT)} missing title {title!r}")


def page_mentions_version(
    name: str, body: str, version: object, latest: object
) -> bool:
    if not isinstance(version, str):
        return False

    if f"v{version}" in body:
        return True

    if name != "download.html" or not isinstance(latest, str):
        return False

    collapsed_range = "v1.6.0 – v1.6.37"
    if collapsed_range not in body:
        return False

    major, minor, patch = version_tuple(version)
    return (major, minor) == (1, 6) and 0 <= patch <= 37


def version_tuple(version: object) -> tuple[int, int, int]:
    if not isinstance(version, str) or not VERSION_RE.match(version):
        return (0, 0, 0)
    major, minor, patch = version.split(".")
    return (int(major), int(minor), int(patch))


def load_toml(path: Path) -> dict[str, object]:
    return tomllib.loads(read(path))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
