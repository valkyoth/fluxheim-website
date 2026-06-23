#!/usr/bin/env python3
"""Validate stable i18n key files for every configured locale."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
SOURCE_LOCALE = "en-EU"
LEGACY_PHRASE_PATHS = (
    ROOT / "config/i18n-de.toml",
    ROOT / "config/i18n-fr.toml",
)
LEGACY_PHRASE_DIRS = (
    ROOT / "config/i18n/de",
    ROOT / "config/i18n/fr",
)


def main() -> int:
    locales = load_toml(ROOT / "config/locales.toml")["locales"]
    locale_ids = [locale["locale_id"] for locale in locales]
    errors: list[str] = []

    source_path = KEY_ROOT / f"{SOURCE_LOCALE}.toml"
    source = load_key_file(source_path, SOURCE_LOCALE, errors)
    source_keys = set(flatten(source).keys())
    source_parts = key_part_names(SOURCE_LOCALE)

    for locale_id in locale_ids:
        path = KEY_ROOT / f"{locale_id}.toml"
        data = load_key_file(path, locale_id, errors)
        keys = set(flatten(data).keys())
        parts = key_part_names(locale_id)
        for missing in sorted(source_parts - parts):
            errors.append(f"{path.relative_to(ROOT)} missing part file {missing}")
        for extra in sorted(parts - source_parts):
            errors.append(f"{path.relative_to(ROOT)} has extra part file {extra}")
        for missing in sorted(source_keys - keys):
            errors.append(f"{path.relative_to(ROOT)} missing key {missing}")
        for extra in sorted(keys - source_keys):
            errors.append(f"{path.relative_to(ROOT)} has extra key {extra}")
        names = data.get("language", {}).get("names", {})
        for configured_locale_id in locale_ids:
            if not names.get(configured_locale_id):
                errors.append(
                    f"{path.relative_to(ROOT)} missing language.names.{configured_locale_id}"
                )

    for path in sorted(KEY_ROOT.glob("*.toml")):
        locale_id = path.stem
        if locale_id not in locale_ids:
            errors.append(f"{path.relative_to(ROOT)} is not configured in config/locales.toml")

    check_legacy_phrase_bundles_are_absent(errors)

    if errors:
        for error in errors:
            print(f"i18n-key error: {error}", file=sys.stderr)
        return 1

    print(f"i18n-keys ok: {len(locale_ids)} locales, {len(source_keys)} keys")
    return 0


def load_key_file(path: Path, locale_id: str, errors: list[str]) -> dict[str, Any]:
    if not path.is_file():
        errors.append(f"missing key file {path.relative_to(ROOT)}")
        return {}
    data = load_key_bundle(path)
    if data.get("locale_id") != locale_id:
        errors.append(f"{path.relative_to(ROOT)} locale_id must be {locale_id}")
    for key, value in flatten(data).items():
        if key != "locale_id" and (not isinstance(value, str) or not value.strip()):
            errors.append(f"{path.relative_to(ROOT)} key {key} must be non-empty text")
    return data


def load_key_bundle(path: Path) -> dict[str, Any]:
    parts = [path.read_text(encoding="utf-8")]
    part_dir = path.with_suffix("")
    if part_dir.is_dir():
        parts.extend(part.read_text(encoding="utf-8") for part in sorted(part_dir.glob("*.toml")))
    return tomllib.loads("\n".join(parts))


def key_part_names(locale_id: str) -> set[str]:
    part_dir = KEY_ROOT / locale_id
    if not part_dir.is_dir():
        return set()
    return {path.name for path in part_dir.glob("*.toml")}


def flatten(data: dict[str, Any], prefix: str = "") -> dict[str, Any]:
    output: dict[str, Any] = {}
    for key, value in data.items():
        full_key = f"{prefix}.{key}" if prefix else key
        if isinstance(value, dict):
            output.update(flatten(value, full_key))
        else:
            output[full_key] = value
    return output


def load_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def check_legacy_phrase_bundles_are_absent(errors: list[str]) -> None:
    for path in LEGACY_PHRASE_PATHS:
        if path.exists():
            errors.append(f"{path.relative_to(ROOT)} is a removed legacy phrase bundle")
    for directory in LEGACY_PHRASE_DIRS:
        for path in sorted(directory.glob("*.toml")):
            errors.append(f"{path.relative_to(ROOT)} is a removed legacy phrase bundle")


if __name__ == "__main__":
    raise SystemExit(main())
