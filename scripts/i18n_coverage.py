#!/usr/bin/env python3
"""Report visible HTML phrases missing from stable i18n key coverage."""

from __future__ import annotations

import argparse
import html
import re
import sys
import tomllib
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_TAGS = {"script", "style", "svg", "path"}
IGNORED_EXACT = {
    "Fluxheim",
    "GitHub",
    "Rust",
    "Linux",
    "Apple Silicon",
    "ARM64",
    "x86_64",
    "EUPL-1.2",
    "PHP-FPM",
    "ACME",
    "TLS",
    "HTTP/2",
    "OpenTelemetry",
    "Prometheus",
    "Podman",
    "Wolfi",
    "Alpine",
    "Debian",
    "SUSE Micro",
    "WordPress",
}
IGNORED_PATTERNS = [
    re.compile(r"^v?\d+(?:\.\d+)+"),
    re.compile(r"^[A-Z0-9_./:-]+$"),
    re.compile(r"^\[[A-Za-z0-9_.-]+\]$"),
    re.compile(r'^"[A-Za-z0-9_./:-]+"$'),
    re.compile(r"^[a-z][a-z0-9_]+$"),
    re.compile(r"^[a-z][a-z0-9]+(?:-[a-z0-9]+)+$"),
    re.compile(r"^~?\d+(?:\.\d+)?\s*(?:MB|GB)$"),
    re.compile(r"^\d{4}$"),
]


class VisibleTextParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.stack: list[str] = []
        self.values: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        self.stack.append(tag.lower())
        for name, value in attrs:
            if name in {"aria-label", "alt", "title"} and value:
                self.values.append(value)

    def handle_endtag(self, tag: str) -> None:
        tag = tag.lower()
        for index in range(len(self.stack) - 1, -1, -1):
            if self.stack[index] == tag:
                del self.stack[index:]
                return

    def handle_data(self, data: str) -> None:
        if any(tag in SKIP_TAGS for tag in self.stack):
            return
        if any(tag in {"pre", "code"} for tag in self.stack):
            self.values.extend(code_comment_phrases(data))
            return
        self.values.append(data)


def load_phrases(locale: str) -> list[str]:
    phrases: list[str] = stable_key_sources()
    phrases.sort(key=len, reverse=True)
    return phrases


def stable_key_sources() -> list[str]:
    source = ROOT / "config/i18n/keys/en-EU.toml"
    data = tomllib.loads("\n".join(stable_key_parts(source)))
    phrases: list[str] = []
    for value in flatten_strings(data):
        if "{version}" in value:
            continue
        phrases.append(value)
        phrases.extend(visible_parts(value))
    return phrases


def stable_key_parts(path: Path) -> list[str]:
    parts = [path.read_text(encoding="utf-8")]
    part_dir = path.with_suffix("")
    if part_dir.is_dir():
        parts.extend(part.read_text(encoding="utf-8") for part in sorted(part_dir.glob("*.toml")))
    return parts


def flatten_strings(value: object) -> list[str]:
    if isinstance(value, str):
        return [value]
    if isinstance(value, dict):
        strings: list[str] = []
        for nested in value.values():
            strings.extend(flatten_strings(nested))
        return strings
    return []


def visible_parts(source: str) -> list[str]:
    parser = VisibleTextParser()
    parser.feed(source)
    return [text for value in parser.values if (text := normalize(value))]


def code_comment_phrases(source: str) -> list[str]:
    phrases: list[str] = []
    for line in source.splitlines():
        stripped = line.strip()
        if stripped.startswith("#") and re.search(r"[A-Za-z]", stripped):
            phrases.append(stripped)
            continue
        match = re.search(r"\s(#\s+[A-Za-z].*)$", line)
        if match:
            phrases.append(match.group(1).strip())
    return phrases


def page_phrases(path: Path) -> list[str]:
    parser = VisibleTextParser()
    parser.feed(path.read_text(encoding="utf-8"))
    phrases: list[str] = []
    seen: set[str] = set()
    for value in parser.values:
        text = normalize(value)
        if not text or should_ignore(text) or text in seen:
            continue
        seen.add(text)
        phrases.append(text)
    return phrases


def normalize(value: str) -> str:
    value = html.unescape(value)
    value = re.sub(r"\s+", " ", value).strip()
    return value


def should_ignore(text: str) -> bool:
    if text in IGNORED_EXACT:
        return True
    if len(text) < 3:
        return True
    if not re.search(r"[A-Za-z]", text):
        return True
    return any(pattern.search(text) for pattern in IGNORED_PATTERNS)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--locale", choices=["de", "fr"], required=True)
    parser.add_argument("--fail-under", type=float, default=0.0)
    parser.add_argument("--summary-only", action="store_true")
    parser.add_argument("pages", nargs="*", default=default_pages())
    args = parser.parse_args()

    translated = load_phrases(args.locale)
    total = 0
    missing: list[tuple[str, str]] = []

    for page in args.pages:
        path = ROOT / page
        for phrase in page_phrases(path):
            total += 1
            if not is_covered(phrase, translated):
                missing.append((page, phrase))

    covered = total - len(missing)
    coverage = 100.0 if total == 0 else covered * 100.0 / total
    print(f"{args.locale}: {covered}/{total} visible phrases covered ({coverage:.1f}%)")

    if not args.summary_only:
        for page, phrase in missing[:80]:
            print(f"{page}: {phrase}")
        if len(missing) > 80:
            print(f"... {len(missing) - 80} more missing phrases")

    if coverage < args.fail_under:
        return 1
    return 0


def is_covered(phrase: str, translated: list[str]) -> bool:
    changed = phrase
    for source in translated:
        changed = changed.replace(source, "")
    return changed != phrase


def default_pages() -> list[str]:
    html_paths = [
        *ROOT.glob("*.html"),
        *(ROOT / "docs").glob("*.html"),
        *(ROOT / "docs/source").glob("*.html"),
    ]
    return [str(path.relative_to(ROOT)) for path in sorted(html_paths)]


if __name__ == "__main__":
    sys.exit(main())
