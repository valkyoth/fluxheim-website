#!/usr/bin/env python3
"""Reject known English sentence fragments inside non-English i18n values."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
ENGLISH_LOCALES = {"en-EU", "en-GB", "en-US"}
FRAGMENTS = (
    "workspace crate is optional and added",
    "explicit, disabled-by-default",
    "continues the native proxy cutover",
    "cache outcomes for continue",
    "without exposing raw cache",
    "under the bounded fluxheim_policy",
    "bounded native HTTP/1 cache-policy",
    "bounded native HTTP/1 cache-policy Wasm hooks",
    "cache lookup and cache store decisions",
    "can continue, pass",
    "hooks can continue",
    "continue normal cache behavior",
    "normal cache behavior",
    "pass through origin",
    "skip storage or deny",
    "mutate raw cache",
    "process-wide admission ceiling so",
    "strict approved-root plugin loading",
    "disabled-by-default Wasm feature gates",
    "while Fluxheim route/cache policy remains enforced",
    "cache-store aggregation most-restrictive-wins",
    "approved roots, without symlinked files",
    "Plugin files must be absolute regular files",
)


def main() -> int:
    configured = tomllib.loads((ROOT / "config/locales.toml").read_text(encoding="utf-8"))[
        "locales"
    ]
    locale_ids = [
        locale["locale_id"]
        for locale in configured
        if locale["locale_id"] not in ENGLISH_LOCALES
    ]
    errors: list[str] = []

    for locale_id in locale_ids:
        for path in locale_paths(locale_id):
            for key, value in flatten(tomllib.loads(path.read_text(encoding="utf-8"))).items():
                if key == "locale_id" or not isinstance(value, str):
                    continue
                lowered = value.lower()
                for fragment in FRAGMENTS:
                    if fragment.lower() in lowered:
                        errors.append(
                            f"{path.relative_to(ROOT)} {key} contains English fragment "
                            f"{fragment!r}: {value!r}"
                        )

    if errors:
        for error in errors:
            print(f"i18n-english-fragment error: {error}", file=sys.stderr)
        return 1

    print(f"i18n-english-fragments ok: {len(locale_ids)} non-English locales")
    return 0


def locale_paths(locale_id: str) -> list[Path]:
    root_path = KEY_ROOT / f"{locale_id}.toml"
    part_dir = KEY_ROOT / locale_id
    paths = [root_path]
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


if __name__ == "__main__":
    sys.exit(main())
