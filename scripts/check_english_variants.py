#!/usr/bin/env python3
"""Check that configured English variants keep intentional spelling differences."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"

GB_MARKERS = (
    "behaviour",
    "normalised",
    "normalise",
    "licence",
    "Licence",
    "finalised",
)
US_MARKERS = (
    "behavior",
    "normalized",
    "normalize",
    "license",
    "License",
    "finalized",
)


def main() -> int:
    gb_text = locale_text("en-GB")
    us_text = locale_text("en-US")
    errors = []
    for marker in GB_MARKERS:
        if marker not in gb_text:
            errors.append(f"en-GB missing expected British English marker: {marker}")
    for marker in US_MARKERS:
        if marker not in us_text:
            errors.append(f"en-US missing expected American English marker: {marker}")
    if gb_text == us_text:
        errors.append("en-GB and en-US key bundles are identical")
    if errors:
        for error in errors:
            print(f"english-variant error: {error}", file=sys.stderr)
        return 1
    print("english-variants ok: en-GB and en-US keep intentional spelling differences")
    return 0


def locale_text(locale_id: str) -> str:
    paths = [KEY_ROOT / f"{locale_id}.toml"]
    paths.extend(sorted((KEY_ROOT / locale_id).glob("*.toml")))
    return "\n".join(path.read_text(encoding="utf-8") for path in paths)


if __name__ == "__main__":
    raise SystemExit(main())
