#!/usr/bin/env python3
"""Validate stable i18n key files for every configured locale."""

from __future__ import annotations

import sys
import tomllib
import argparse
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
KEY_ROOT = ROOT / "config/i18n/keys"
INTENTIONAL_IDENTICAL_PATH = ROOT / "config/i18n/intentional-identical.toml"
SOURCE_LOCALE = "en-EU"
MAX_UNTRANSLATED_PERCENT = 20.0
LEGACY_PHRASE_PATHS = (
    ROOT / "config/i18n-de.toml",
    ROOT / "config/i18n-fr.toml",
)
LEGACY_PHRASE_DIRS = (
    ROOT / "config/i18n/de",
    ROOT / "config/i18n/fr",
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(ROOT), help=argparse.SUPPRESS)
    parser.add_argument("--allow-untranslated-locales", action="store_true")
    parser.add_argument("--progress", action="store_true")
    parser.add_argument("--list-untranslated", metavar="LOCALE")
    parser.add_argument("--untranslated-format", choices=("text", "tsv"), default="text")
    parser.add_argument("--untranslated-limit", type=int, default=80)
    parser.add_argument("--include-intentional", action="store_true")
    args = parser.parse_args()
    configure_root(Path(args.root).resolve())

    locales = load_toml(ROOT / "config/locales.toml")["locales"]
    locale_ids = [locale["locale_id"] for locale in locales]
    errors: list[str] = []
    progress: list[str] = []
    untranslated_reports: dict[str, list[tuple[Path, str, str]]] = {}
    untranslated_locale_ids = selected_untranslated_locale_ids(args.list_untranslated, locale_ids)

    if args.untranslated_limit < 0:
        errors.append("--untranslated-limit must be zero or greater")
    if args.progress and machine_untranslated_output(args):
        errors.append("--progress cannot be combined with --untranslated-format tsv")
    if args.list_untranslated and not untranslated_locale_ids:
        errors.append(f"{args.list_untranslated} is not configured in config/locales.toml")

    source_path = KEY_ROOT / f"{SOURCE_LOCALE}.toml"
    source = load_key_file(source_path, SOURCE_LOCALE, errors)
    source_keys = set(flatten(source).keys())
    source_parts = key_part_names(SOURCE_LOCALE)
    source_language_names = source.get("language", {}).get("names", {})
    intentional_identical = load_intentional_identical(locale_ids, source_keys, errors)

    for locale_id in locale_ids:
        path = KEY_ROOT / f"{locale_id}.toml"
        data = load_key_file(path, locale_id, errors)
        check_intentional_identical_entries(
            locale_id,
            source,
            data,
            intentional_identical.get(locale_id, set()),
            errors,
        )
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
        report = check_locale_translation_progress(
            locale_id,
            source,
            data,
            path,
            errors,
            enforce=not args.allow_untranslated_locales,
            intentional_identical=intentional_identical.get(locale_id, set()),
        )
        if report:
            progress.append(report)
        if locale_id in untranslated_locale_ids:
            untranslated_reports[locale_id] = untranslated_keys(
                locale_id,
                source,
                data,
                intentional_identical.get(locale_id, set()),
                include_intentional=args.include_intentional,
            )
        names = data.get("language", {}).get("names", {})
        for configured_locale_id in locale_ids:
            if not names.get(configured_locale_id):
                errors.append(
                    f"{path.relative_to(ROOT)} missing language.names.{configured_locale_id}"
                )
                continue
            if names.get(configured_locale_id) != source_language_names.get(
                configured_locale_id
            ):
                errors.append(
                    f"{path.relative_to(ROOT)} language.names.{configured_locale_id} "
                    f"must stay autonym {source_language_names.get(configured_locale_id)!r}"
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

    if not machine_untranslated_output(args):
        print(f"i18n-keys ok: {len(locale_ids)} locales, {len(source_keys)} keys")
    if args.progress:
        for report in progress:
            print(report)
    if args.list_untranslated:
        for locale_id in untranslated_locale_ids:
            print_untranslated_keys(
                locale_id,
                untranslated_reports.get(locale_id, []),
                args.untranslated_limit,
                args.untranslated_format,
            )
    return 0


def configure_root(root: Path) -> None:
    global ROOT, KEY_ROOT, INTENTIONAL_IDENTICAL_PATH, LEGACY_PHRASE_PATHS, LEGACY_PHRASE_DIRS
    ROOT = root
    KEY_ROOT = ROOT / "config/i18n/keys"
    INTENTIONAL_IDENTICAL_PATH = ROOT / "config/i18n/intentional-identical.toml"
    LEGACY_PHRASE_PATHS = (
        ROOT / "config/i18n-de.toml",
        ROOT / "config/i18n-fr.toml",
    )
    LEGACY_PHRASE_DIRS = (
        ROOT / "config/i18n/de",
        ROOT / "config/i18n/fr",
    )


def selected_untranslated_locale_ids(
    selected: str | None,
    locale_ids: list[str],
) -> list[str]:
    if not selected:
        return []
    if selected == "all":
        return [
            locale_id
            for locale_id in locale_ids
            if locale_id != SOURCE_LOCALE and not locale_id.startswith("en-")
        ]
    if selected in locale_ids:
        return [selected]
    return []


def machine_untranslated_output(args: argparse.Namespace) -> bool:
    return bool(args.list_untranslated and args.untranslated_format == "tsv")


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


def key_locations(locale_id: str) -> dict[str, Path]:
    locations: dict[str, Path] = {}
    root_path = KEY_ROOT / f"{locale_id}.toml"
    for key in flatten(load_toml(root_path)):
        locations[key] = root_path
    part_dir = KEY_ROOT / locale_id
    if part_dir.is_dir():
        for part in sorted(part_dir.glob("*.toml")):
            for key in flatten(load_toml(part)):
                locations[key] = part
    return locations


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


def load_intentional_identical(
    locale_ids: list[str],
    source_keys: set[str],
    errors: list[str],
) -> dict[str, set[str]]:
    if not INTENTIONAL_IDENTICAL_PATH.exists():
        return {}
    data = load_toml(INTENTIONAL_IDENTICAL_PATH)
    allowed: dict[str, set[str]] = {}
    configured = set(locale_ids)
    for locale_id, table in data.items():
        if locale_id not in configured:
            errors.append(
                f"{INTENTIONAL_IDENTICAL_PATH.relative_to(ROOT)} has unconfigured locale {locale_id}"
            )
            continue
        keys = table.get("keys") if isinstance(table, dict) else None
        if not isinstance(keys, list):
            errors.append(
                f"{INTENTIONAL_IDENTICAL_PATH.relative_to(ROOT)} {locale_id}.keys must be an array"
            )
            continue
        locale_keys: set[str] = set()
        for key in keys:
            if not isinstance(key, str) or not key:
                errors.append(
                    f"{INTENTIONAL_IDENTICAL_PATH.relative_to(ROOT)} {locale_id}.keys contains a non-text key"
                )
                continue
            if key not in source_keys:
                errors.append(
                    f"{INTENTIONAL_IDENTICAL_PATH.relative_to(ROOT)} {locale_id}.{key} is not an i18n key"
                )
            locale_keys.add(key)
        allowed[locale_id] = locale_keys
    return allowed


def check_intentional_identical_entries(
    locale_id: str,
    source: dict[str, Any],
    data: dict[str, Any],
    intentional_identical: set[str],
    errors: list[str],
) -> None:
    if not intentional_identical:
        return
    source_flat = flatten(source)
    data_flat = flatten(data)
    for key in sorted(intentional_identical):
        if key in data_flat and data_flat[key] != source_flat.get(key):
            errors.append(
                f"{INTENTIONAL_IDENTICAL_PATH.relative_to(ROOT)} {locale_id}.{key} is no longer source-identical"
            )


def check_locale_translation_progress(
    locale_id: str,
    source: dict[str, Any],
    data: dict[str, Any],
    path: Path,
    errors: list[str],
    *,
    enforce: bool,
    intentional_identical: set[str],
) -> str | None:
    if locale_id == SOURCE_LOCALE or locale_id.startswith("en-"):
        return None
    source_flat = comparable_translation_values(flatten(source))
    data_flat = comparable_translation_values(flatten(data))
    comparable_keys = sorted((set(source_flat) & set(data_flat)) - intentional_identical)
    if not comparable_keys:
        return None
    untranslated = [key for key in comparable_keys if data_flat[key] == source_flat[key]]
    untranslated_percent = len(untranslated) * 100.0 / len(comparable_keys)
    translated_percent = 100.0 - untranslated_percent
    if enforce and untranslated_percent > MAX_UNTRANSLATED_PERCENT:
        errors.append(
            f"{path.relative_to(ROOT)} appears under-translated; "
            f"{len(untranslated)}/{len(comparable_keys)} comparable text values "
            f"({untranslated_percent:.1f}%) still match {SOURCE_LOCALE}"
        )
    return (
        f"{locale_id}: {translated_percent:.1f}% translated "
        f"({len(comparable_keys) - len(untranslated)}/{len(comparable_keys)} changed)"
    )


def comparable_translation_values(flat: dict[str, Any]) -> dict[str, str]:
    skipped_prefixes = ("language.names.",)
    return {
        key: value
        for key, value in flat.items()
        if key != "locale_id"
        and not key.startswith(skipped_prefixes)
        and isinstance(value, str)
    }


def untranslated_keys(
    locale_id: str,
    source: dict[str, Any],
    data: dict[str, Any],
    intentional_identical: set[str],
    *,
    include_intentional: bool,
) -> list[tuple[Path, str, str]]:
    if locale_id == SOURCE_LOCALE or locale_id.startswith("en-"):
        return []
    source_flat = comparable_translation_values(flatten(source))
    data_flat = comparable_translation_values(flatten(data))
    locations = key_locations(locale_id)
    keys = set(source_flat) & set(data_flat)
    if not include_intentional:
        keys -= intentional_identical
    return [
        (locations.get(key, KEY_ROOT / f"{locale_id}.toml"), key, source_flat[key])
        for key in sorted(keys)
        if data_flat[key] == source_flat[key]
    ]


def print_untranslated_keys(
    locale_id: str,
    untranslated: list[tuple[Path, str, str]],
    limit: int,
    output_format: str,
) -> None:
    shown = untranslated if limit == 0 else untranslated[:limit]
    if output_format == "tsv":
        for path, key, value in shown:
            print(
                "\t".join(
                    (
                        locale_id,
                        str(path.relative_to(ROOT)),
                        key,
                        tsv_field(value),
                    )
                )
            )
        return
    print(f"{locale_id}: {len(untranslated)} keys still match {SOURCE_LOCALE}")
    for path, key, value in shown:
        print(f"{path.relative_to(ROOT)} {key}: {preview(value)}")
    if limit and len(untranslated) > limit:
        print(f"... {len(untranslated) - limit} more; rerun with --untranslated-limit 0")


def preview(value: str) -> str:
    collapsed = " ".join(value.split())
    if len(collapsed) <= 120:
        return collapsed
    return f"{collapsed[:117]}..."


def tsv_field(value: str) -> str:
    return " ".join(value.split())


def check_legacy_phrase_bundles_are_absent(errors: list[str]) -> None:
    for path in LEGACY_PHRASE_PATHS:
        if path.exists():
            errors.append(f"{path.relative_to(ROOT)} is a removed legacy phrase bundle")
    for directory in LEGACY_PHRASE_DIRS:
        for path in sorted(directory.glob("*.toml")):
            errors.append(f"{path.relative_to(ROOT)} is a removed legacy phrase bundle")


if __name__ == "__main__":
    raise SystemExit(main())
