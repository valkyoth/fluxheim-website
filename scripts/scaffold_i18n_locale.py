#!/usr/bin/env python3
"""Scaffold a new stable i18n locale from an existing locale key bundle."""

from __future__ import annotations

import argparse
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
LOCALES_TOML = ROOT / "config/locales.toml"
DEFAULT_SOURCE_LOCALE = "en-EU"
LOCALE_RE = re.compile(r"^[a-z]{2,3}(?:-[A-Z]{2})?$")
PREFIX_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Create stable i18n key files for a new locale."
    )
    parser.add_argument("--locale-id", required=True, help="Locale id, e.g. it-IT")
    parser.add_argument("--html-lang", required=True, help="HTML lang value")
    parser.add_argument("--url-prefix", required=True, help="Route prefix, e.g. it")
    parser.add_argument("--display-name", required=True, help="Selector label")
    parser.add_argument("--from-locale", default=DEFAULT_SOURCE_LOCALE)
    parser.add_argument("--root", default=str(ROOT), help=argparse.SUPPRESS)
    parser.add_argument("--force", action="store_true")
    args = parser.parse_args()
    configure_root(Path(args.root).resolve())

    validate_args(args)
    source_root = KEY_ROOT / f"{args.from_locale}.toml"
    source_dir = KEY_ROOT / args.from_locale
    target_root = KEY_ROOT / f"{args.locale_id}.toml"
    target_dir = KEY_ROOT / args.locale_id

    if not source_root.is_file() or not source_dir.is_dir():
        fail(f"missing source locale key bundle: {args.from_locale}")
    locale_exists = locale_config_contains(args.locale_id)
    prefix_exists = prefix_config_contains(args.url_prefix)

    if (target_root.exists() or target_dir.exists()) and not args.force:
        fail(f"target locale already exists: {args.locale_id}")
    if locale_exists and not args.force:
        fail(f"config/locales.toml already contains {args.locale_id}")
    if prefix_exists and not args.force:
        fail(f"config/locales.toml already contains prefix {args.url_prefix}")

    if target_root.exists():
        target_root.unlink()
    if target_dir.exists():
        shutil.rmtree(target_dir)

    target_root.write_text(
        rewrite_locale_id(source_root.read_text(encoding="utf-8"), args.locale_id),
        encoding="utf-8",
    )
    shutil.copytree(source_dir, target_dir)
    update_language_names(args.locale_id, args.display_name)
    if not locale_exists:
        append_locale(args.locale_id, args.html_lang, args.url_prefix, args.display_name)

    print(f"created stable i18n locale {args.locale_id}")
    print(f"- {target_root.relative_to(ROOT)}")
    print(f"- {target_dir.relative_to(ROOT)}/")
    return 0


def configure_root(root: Path) -> None:
    global ROOT, KEY_ROOT, LOCALES_TOML
    ROOT = root
    KEY_ROOT = ROOT / "config/i18n/keys"
    LOCALES_TOML = ROOT / "config/locales.toml"


def validate_args(args: argparse.Namespace) -> None:
    if not LOCALE_RE.fullmatch(args.locale_id):
        fail("--locale-id must look like en-GB, fr-FR, or sv")
    if not args.html_lang or has_control(args.html_lang):
        fail("--html-lang must be non-empty text without control characters")
    if not PREFIX_RE.fullmatch(args.url_prefix):
        fail("--url-prefix must be lowercase letters, digits, and hyphens")
    if not args.display_name.strip() or has_control(args.display_name):
        fail("--display-name must be non-empty text without control characters")
    if not LOCALE_RE.fullmatch(args.from_locale):
        fail("--from-locale must look like en-EU, de-DE, or fr-FR")


def locale_config_contains(locale_id: str) -> bool:
    return f'locale_id = "{locale_id}"' in LOCALES_TOML.read_text(encoding="utf-8")


def prefix_config_contains(url_prefix: str) -> bool:
    return f'url_prefix = "{url_prefix}"' in LOCALES_TOML.read_text(encoding="utf-8")


def rewrite_locale_id(contents: str, locale_id: str) -> str:
    return re.sub(r'^locale_id = "[^"]+"$', f'locale_id = "{locale_id}"', contents, count=1, flags=re.M)


def update_language_names(locale_id: str, display_name: str) -> None:
    for root_file in sorted(KEY_ROOT.glob("*.toml")):
        contents = root_file.read_text(encoding="utf-8")
        if f"{locale_id} =" in contents:
            continue
        root_file.write_text(
            insert_language_name(contents, locale_id, display_name),
            encoding="utf-8",
        )


def insert_language_name(contents: str, locale_id: str, display_name: str) -> str:
    marker = "\n[nav]\n"
    entry = f'{locale_id} = "{toml_string(display_name)}"\n'
    if marker not in contents:
        fail("root key file missing [nav] marker")
    return contents.replace(marker, f"{entry}{marker}", 1)


def append_locale(locale_id: str, html_lang: str, url_prefix: str, display_name: str) -> None:
    entry = (
        "\n[[locales]]\n"
        f'locale_id = "{locale_id}"\n'
        f'html_lang = "{toml_string(html_lang)}"\n'
        f'url_prefix = "{url_prefix}"\n'
        f'display_name = "{toml_string(display_name)}"\n'
    )
    with LOCALES_TOML.open("a", encoding="utf-8") as file:
        file.write(entry)


def toml_string(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def has_control(value: str) -> bool:
    return any(ord(char) < 0x20 for char in value)


def fail(message: str) -> None:
    print(f"i18n scaffold error: {message}", file=sys.stderr)
    raise SystemExit(1)


if __name__ == "__main__":
    raise SystemExit(main())
