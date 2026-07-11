#!/usr/bin/env python3
"""Reject distinct source phrases collapsed into one long locale translation."""

from __future__ import annotations

import sys
import tomllib
from collections import defaultdict
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
SOURCE_LOCALE = "en-EU"
ENGLISH_LOCALES = {"en-EU", "en-GB", "en-US"}
SECTIONS = ("release_updates", "docs_expanded")
MIN_VALUE_LENGTH = 55


def main() -> int:
    configured = tomllib.loads((ROOT / "config/locales.toml").read_text(encoding="utf-8"))[
        "locales"
    ]
    source = load_bundle(SOURCE_LOCALE)
    errors: list[str] = []
    checked = 0

    for locale in configured:
        locale_id = locale["locale_id"]
        if locale_id in ENGLISH_LOCALES:
            continue
        checked += 1
        target = load_bundle(locale_id)
        for section in SECTIONS:
            find_collapsed_values(locale_id, section, source, target, errors)

    if errors:
        for error in errors:
            print(f"i18n-collapsed-value error: {error}", file=sys.stderr)
        return 1

    print(f"i18n-collapsed-values ok: {checked} non-English locales")
    return 0


def find_collapsed_values(
    locale_id: str,
    section: str,
    source: dict[str, Any],
    target: dict[str, Any],
    errors: list[str],
) -> None:
    source_values = source.get(section, {})
    target_values = target.get(section, {})
    by_translation: dict[str, list[str]] = defaultdict(list)

    for key, value in target_values.items():
        if isinstance(value, str) and len(value) >= MIN_VALUE_LENGTH:
            by_translation[value].append(key)

    for value, keys in by_translation.items():
        if len(keys) < 2:
            continue
        distinct_sources = {source_values.get(key) for key in keys}
        if len(distinct_sources) < 2:
            continue
        key_list = ", ".join(sorted(keys))
        errors.append(
            f"{locale_id} {section} collapses distinct keys [{key_list}] "
            f"into {value!r}"
        )


def load_bundle(locale_id: str) -> dict[str, Any]:
    root_path = KEY_ROOT / f"{locale_id}.toml"
    parts = [root_path.read_text(encoding="utf-8")]
    part_dir = KEY_ROOT / locale_id
    if part_dir.is_dir():
        parts.extend(path.read_text(encoding="utf-8") for path in sorted(part_dir.glob("*.toml")))
    return tomllib.loads("\n".join(parts))


if __name__ == "__main__":
    sys.exit(main())
