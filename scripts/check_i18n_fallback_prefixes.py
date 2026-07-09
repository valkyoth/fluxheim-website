#!/usr/bin/env python3
"""Reject locale-name-prefixed English fallback translations."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"


def main() -> int:
    locales = tomllib.loads((ROOT / "config/locales.toml").read_text(encoding="utf-8"))[
        "locales"
    ]
    display_names = {
        locale["locale_id"]: locale["display_name"]
        for locale in locales
        if not locale["locale_id"].startswith("en-")
    }
    errors: list[str] = []

    for locale_id, display_name in display_names.items():
        for path in locale_paths(locale_id):
            data = tomllib.loads(path.read_text(encoding="utf-8"))
            for key, value in flatten(data).items():
                if key == "locale_id" or not isinstance(value, str):
                    continue
                if has_bad_prefix(value, locale_id, display_name):
                    errors.append(
                        f"{path.relative_to(ROOT)} {key} uses fallback prefix: {value!r}"
                    )

    if errors:
        for error in errors:
            print(f"i18n-fallback-prefix error: {error}", file=sys.stderr)
        return 1

    print(f"i18n-fallback-prefixes ok: {len(display_names)} non-English locales")
    return 0


def locale_paths(locale_id: str) -> list[Path]:
    paths = [KEY_ROOT / f"{locale_id}.toml"]
    part_dir = KEY_ROOT / locale_id
    if part_dir.is_dir():
        paths.extend(sorted(part_dir.glob("*.toml")))
    return [path for path in paths if path.is_file()]


def flatten(data: dict[str, Any], prefix: str = "") -> dict[str, Any]:
    output: dict[str, Any] = {}
    for key, value in data.items():
        full_key = f"{prefix}.{key}" if prefix else key
        if isinstance(value, dict):
            output.update(flatten(value, full_key))
        else:
            output[full_key] = value
    return output


def has_bad_prefix(value: str, locale_id: str, display_name: str) -> bool:
    prefixes = {locale_id, display_name, display_name.split(" (", maxsplit=1)[0]}
    for prefix in (prefix for prefix in prefixes if prefix):
        if value.startswith(f"{prefix}: "):
            return True
        if value.startswith(f"{prefix} WebAssembly line:"):
            return True
        if value.startswith(f"{prefix} cache-policy update:"):
            return True
    return value.startswith("Nederlandse cache-policy update:")


if __name__ == "__main__":
    sys.exit(main())
