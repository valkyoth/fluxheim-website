#!/usr/bin/env python3
"""Verify that translations preserve non-translatable technical structure."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
import unicodedata
from collections import Counter
from pathlib import Path
from typing import Any, Callable

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
SOURCE_LOCALE = "en-EU"
CRITICAL_TERMS = (
    "Fluxheim",
    "HTTP",
    "Wasm",
    "ACME",
    "CORS",
    "CSRF",
    "CSP",
    "HSTS",
    "GeoIP",
    "IPC",
    "PHP-FPM",
)

HTML_TAG = re.compile(r"</?[A-Za-z][^<>]*>")
CODE_VALUE = re.compile(r"<code>(.*?)</code>", re.DOTALL)
PLACEHOLDER = re.compile(r"\{[A-Za-z_][A-Za-z0-9_]*\}")
URL = re.compile(r"https?://[^\s<>\"]+")
STATUS_CODE = re.compile(r"(?<!\d)[1-5]\d\d(?!\d)")
UNSAFE_FORMAT_CHARACTERS = {"\u200b", "\u2060", "\ufeff"}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()

    locale_ids = [
        item["locale_id"]
        for item in load_toml(ROOT / "config/locales.toml")["locales"]
    ]
    current_version = load_toml(ROOT / "config/site.toml")["fluxheim_version"]
    release_part = next(
        (KEY_ROOT / SOURCE_LOCALE).glob(f"*-release-{current_version}.toml")
    )
    current_release_keys = set(load_toml(release_part))
    source = flatten(load_bundle(SOURCE_LOCALE))
    errors: list[str] = []

    for locale_id in locale_ids:
        if locale_id == SOURCE_LOCALE:
            continue
        target = flatten(load_bundle(locale_id))
        for key, source_value in source.items():
            target_value = target.get(key)
            if not isinstance(source_value, str) or not isinstance(target_value, str):
                continue
            check_value(
                locale_id,
                key,
                source_value,
                target_value,
                current_version,
                key.rsplit(".", 1)[-1] in current_release_keys,
                errors,
            )

    if errors:
        for error in errors:
            print(f"i18n-integrity error: {error}", file=sys.stderr)
        return 1

    print(f"i18n-integrity ok: {len(locale_ids) - 1} locale bundles")
    return 0


def self_test() -> int:
    source = (
        "Fluxheim 1.7.10 enables CORS at <code>baseline</code>, returns "
        "<code>403</code>, and keeps {version} at https://example.invalid/docs."
    )
    valid = (
        "Fluxheim 1.7.10 CORS <code>baseline</code> <code>403</code> "
        "{version} https://example.invalid/docs."
    )
    errors: list[str] = []
    check_value("zz-ZZ", "test.valid", source, valid, "1.7.10", True, errors)
    if errors:
        print(f"i18n-integrity self-test rejected valid text: {errors}", file=sys.stderr)
        return 1

    broken = "Fluxheim\u200b <strong>404</strong>"
    check_value("zz-ZZ", "test.broken", source, broken, "1.7.10", True, errors)
    expected_fragments = (
        "HTML tags",
        "code values",
        "placeholders",
        "URLs",
        "HTTP status codes",
        "omitted current version",
        "omitted technical term 'CORS'",
        "format character",
    )
    missing = [
        fragment
        for fragment in expected_fragments
        if not any(fragment in error for error in errors)
    ]
    if missing:
        print(
            f"i18n-integrity self-test missed failure classes: {missing}",
            file=sys.stderr,
        )
        return 1

    print("i18n-integrity self-test ok")
    return 0


def check_value(
    locale_id: str,
    key: str,
    source: str,
    target: str,
    current_version: str,
    current_release_key: bool,
    errors: list[str],
) -> None:
    checks: tuple[tuple[str, Callable[[str], list[str]]], ...] = (
        ("HTML tags", HTML_TAG.findall),
        ("code values", CODE_VALUE.findall),
        ("placeholders", PLACEHOLDER.findall),
        ("URLs", URL.findall),
        ("HTTP status codes", STATUS_CODE.findall),
    )
    for label, extract in checks:
        expected = Counter(extract(source))
        actual = Counter(extract(target))
        if expected != actual:
            errors.append(
                f"{locale_id} {key} changed {label}: "
                f"expected {dict(expected)!r}, found {dict(actual)!r}"
            )

    if current_version in source and current_version not in target:
        errors.append(
            f"{locale_id} {key} omitted current version {current_version!r}"
        )
    if current_release_key:
        source_folded = source.casefold()
        target_folded = target.casefold()
        for term in CRITICAL_TERMS:
            if term.casefold() in source_folded and term.casefold() not in target_folded:
                errors.append(f"{locale_id} {key} omitted technical term {term!r}")

    if any(character in target for character in ("\x00", "\ufffd")):
        errors.append(f"{locale_id} {key} contains an invalid replacement character")
    for character in target:
        if (
            unicodedata.category(character) == "Cf"
            and character in UNSAFE_FORMAT_CHARACTERS
        ):
            errors.append(
                f"{locale_id} {key} contains format character "
                f"U+{ord(character):04X}"
            )


def load_bundle(locale_id: str) -> dict[str, Any]:
    root_path = KEY_ROOT / f"{locale_id}.toml"
    parts = [root_path.read_text(encoding="utf-8")]
    part_dir = KEY_ROOT / locale_id
    if part_dir.is_dir():
        parts.extend(
            path.read_text(encoding="utf-8") for path in sorted(part_dir.glob("*.toml"))
        )
    return tomllib.loads("\n".join(parts))


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
