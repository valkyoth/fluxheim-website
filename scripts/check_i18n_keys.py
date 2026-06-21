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


def main() -> int:
    locales = load_toml(ROOT / "config/locales.toml")["locales"]
    locale_ids = [locale["locale_id"] for locale in locales]
    errors: list[str] = []

    source_path = KEY_ROOT / f"{SOURCE_LOCALE}.toml"
    source = load_key_file(source_path, SOURCE_LOCALE, errors)
    source_keys = set(flatten(source).keys())

    for locale_id in locale_ids:
        path = KEY_ROOT / f"{locale_id}.toml"
        data = load_key_file(path, locale_id, errors)
        keys = set(flatten(data).keys())
        for missing in sorted(source_keys - keys):
            errors.append(f"{path.relative_to(ROOT)} missing key {missing}")
        for extra in sorted(keys - source_keys):
            errors.append(f"{path.relative_to(ROOT)} has extra key {extra}")

    for path in sorted(KEY_ROOT.glob("*.toml")):
        locale_id = path.stem
        if locale_id not in locale_ids:
            errors.append(f"{path.relative_to(ROOT)} is not configured in config/locales.toml")

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
    data = load_toml(path)
    if data.get("locale_id") != locale_id:
        errors.append(f"{path.relative_to(ROOT)} locale_id must be {locale_id}")
    for key, value in flatten(data).items():
        if key != "locale_id" and (not isinstance(value, str) or not value.strip()):
            errors.append(f"{path.relative_to(ROOT)} key {key} must be non-empty text")
    return data


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


if __name__ == "__main__":
    raise SystemExit(main())
