#!/usr/bin/env python3
"""Render vendored Fluxheim Markdown docs into static site pages.

This intentionally uses only the Python standard library. The site has no
Node/package-manager build step, so this generator covers the Markdown subset
used by the upstream Fluxheim docs: headings, paragraphs, fenced code blocks,
tables, bullet/numbered lists, inline code, and links.
"""

from __future__ import annotations

import html
import posixpath
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIR = ROOT / "docs" / "source"

DOCS = {
    "build-and-podman.md": "Build And Rootless Podman",
    "systemd.md": "systemd Deployment",
    "macos-development.md": "macOS Development Support",
    "production-readiness.md": "Production Readiness",
    "fips.md": "FIPS-Capable Deployment",
    "supply-chain-security.md": "Supply Chain Security",
    "owasp-top10-2025-baseline.md": "OWASP Top 10 2025 Baseline",
    "compliance-evidence-template.md": "Compliance Evidence Template",
    "common-criteria-roadmap.md": "Common Criteria Roadmap",
    "release-runbook.md": "Release Runbook",
    "release-checklist.md": "Release Checklist",
    "config-reference.md": "Config Reference",
    "vhost-config.md": "Vhost Config Guide",
    "gateway-recipes.md": "Gateway Recipes",
    "load-balancer-ha.md": "Load Balancer HA Design Notes",
    "config-snapshots.md": "Config Snapshots And Rollback",
    "certificate-renewal.md": "Certificate Renewal And Reload",
    "geoip.md": "GeoIP / Geo-Context",
    "cache-backends.md": "Cache Backends",
    "cache-encryption.md": "Cache Encryption",
    "metrics-architecture.md": "Metrics Architecture",
    "logging-architecture.md": "Logging Architecture",
    "opentelemetry-tracing.md": "OpenTelemetry Tracing",
    "php-runtime-support.md": "PHP Runtime Support",
    "php-fpm-app-recipes.md": "PHP-FPM Application Recipes",
    "perl-cgi-support.md": "Perl CGI Support",
    "features.md": "Feature Matrix",
    "versioning-plan.md": "Versioning Plan",
    "waf-architecture.md": "WAF Architecture",
    "wasm-extensibility.md": "WASM Extensibility",
    "crypto-rpc-edge.md": "Crypto RPC Edge",
    "compression.md": "Compression",
    "image-filter.md": "Image Filter",
    "programmable-media-edge.md": "Programmable Media Edge",
    "auth-request.md": "External Authorization Request",
    "secure-links.md": "Secure Links",
    "cloudflare-origin-support.md": "Cloudflare Origin Support",
    "zero-retention-privacy-mode.md": "Zero-Retention Privacy Mode",
    "legacy-static-http.md": "Legacy Static HTTP Support",
    "sentinel-mesh.md": "Sentinel Mesh",
    "pingora-core-patch.md": "Pingora Core Patch",
    "github-setup.md": "GitHub Repository Setup",
    "release-notes-template.md": "Release Notes Template",
}


def slugify(value: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")
    return slug or "section"


def render_inline(text: str) -> str:
    escaped = html.escape(text)
    escaped = re.sub(r"`([^`]+)`", r"<code>\1</code>", escaped)

    def link(match: re.Match[str]) -> str:
        label = match.group(1)
        href = html.unescape(match.group(2))
        if href.startswith("../"):
            repo_path = posixpath.normpath(posixpath.join("docs", href))
            href = f"https://github.com/valkyoth/fluxheim/blob/main/{repo_path}"
        elif href.endswith(".md"):
            href = href[:-3] + ".html"
        attrs = ' class="text-cyan-400 hover:text-cyan-300"'
        if href.startswith(("http://", "https://")):
            attrs += ' target="_blank" rel="noopener"'
        return f'<a href="{html.escape(href, quote=True)}"{attrs}>{label}</a>'

    return re.sub(r"\[([^\]]+)\]\(([^)]+)\)", link, escaped)


def is_table(lines: list[str], index: int) -> bool:
    return (
        index + 1 < len(lines)
        and lines[index].lstrip().startswith("|")
        and re.match(r"^\s*\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?\s*$", lines[index + 1])
        is not None
    )


def split_table_row(line: str) -> list[str]:
    stripped = line.strip().strip("|")
    return [cell.strip() for cell in stripped.split("|")]


def render_markdown(markdown: str) -> tuple[str, str]:
    lines = markdown.splitlines()
    title = "Source Document"
    html_lines: list[str] = []
    paragraph: list[str] = []
    list_type: str | None = None
    current_list_item: list[str] = []
    code_lang: str | None = None
    code_lines: list[str] = []

    def flush_paragraph() -> None:
        nonlocal paragraph
        if paragraph:
            html_lines.append(f"<p>{render_inline(' '.join(paragraph))}</p>")
            paragraph = []

    def close_list() -> None:
        nonlocal list_type, current_list_item
        if list_type:
            if current_list_item:
                html_lines.append(f"<li>{render_inline(' '.join(current_list_item))}</li>")
                current_list_item = []
            html_lines.append(f"</{list_type}>")
            list_type = None

    i = 0
    while i < len(lines):
        line = lines[i]

        if code_lang is not None:
            if line.startswith("```"):
                html_lines.append(
                    f'<pre class="language-{html.escape(code_lang)}"><code class="language-{html.escape(code_lang)}">'
                    + html.escape("\n".join(code_lines))
                    + "</code></pre>"
                )
                code_lang = None
                code_lines = []
            else:
                code_lines.append(line)
            i += 1
            continue

        if line.startswith("```"):
            flush_paragraph()
            close_list()
            code_lang = line.strip("`").strip() or "text"
            code_lines = []
            i += 1
            continue

        if not line.strip():
            flush_paragraph()
            close_list()
            i += 1
            continue

        heading = re.match(r"^(#{1,6})\s+(.+?)\s*$", line)
        if heading:
            flush_paragraph()
            close_list()
            level = min(len(heading.group(1)), 4)
            text = heading.group(2)
            if len(heading.group(1)) == 1:
                title = text
                level = 1
            html_lines.append(f'<h{level} id="{slugify(text)}">{render_inline(text)}</h{level}>')
            i += 1
            continue

        if is_table(lines, i):
            flush_paragraph()
            close_list()
            headers = split_table_row(lines[i])
            i += 2
            rows: list[list[str]] = []
            while i < len(lines) and lines[i].lstrip().startswith("|"):
                rows.append(split_table_row(lines[i]))
                i += 1
            html_lines.append('<div class="source-table"><table>')
            html_lines.append("<thead><tr>" + "".join(f"<th>{render_inline(h)}</th>" for h in headers) + "</tr></thead>")
            html_lines.append("<tbody>")
            for row in rows:
                html_lines.append("<tr>" + "".join(f"<td>{render_inline(c)}</td>" for c in row) + "</tr>")
            html_lines.append("</tbody></table></div>")
            continue

        bullet = re.match(r"^\s*[-*]\s+(.+)$", line)
        numbered = re.match(r"^\s*\d+\.\s+(.+)$", line)
        if bullet or numbered:
            flush_paragraph()
            wanted = "ul" if bullet else "ol"
            if list_type != wanted:
                close_list()
                list_type = wanted
                html_lines.append(f"<{wanted}>")
            elif current_list_item:
                html_lines.append(f"<li>{render_inline(' '.join(current_list_item))}</li>")
            current_list_item = [(bullet or numbered).group(1)]
            i += 1
            continue

        if list_type and re.match(r"^\s+\S", line):
            current_list_item.append(line.strip())
            i += 1
            continue

        close_list()
        paragraph.append(line.strip())
        i += 1

    flush_paragraph()
    close_list()
    return title, "\n".join(html_lines)


def page_template(title: str, filename: str, body: str) -> str:
    return f"""<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link rel="icon" href="../../assets/img/fluxheim-logo.webp" type="image/webp">
  <title>{html.escape(title)} — Fluxheim Source Docs</title>
  <meta property="og:type" content="website">
  <meta property="og:site_name" content="Fluxheim">
  <meta property="og:title" content="Fluxheim">
  <meta property="og:description" content="Fluxheim is a high-performance, modular web server, reverse proxy and caching server built in Rust.">
  <meta property="og:url" content="https://fluxheim.eu/">
  <meta property="og:image" content="https://fluxheim.eu/assets/img/fluxheim-social.png">
  <meta property="og:image:secure_url" content="https://fluxheim.eu/assets/img/fluxheim-social.png">
  <meta property="og:image:type" content="image/png">
  <meta property="og:image:width" content="1200">
  <meta property="og:image:height" content="630">
  <meta property="og:image:alt" content="Fluxheim logo">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="Fluxheim">
  <meta name="twitter:description" content="A memory-safe edge server and reverse proxy built in Rust.">
  <meta name="twitter:image" content="https://fluxheim.eu/assets/img/fluxheim-social.png">
  <link rel="stylesheet" href="../../assets/css/prism-dark.min.css">
  <link rel="stylesheet" href="../../assets/css/theme.css?v=20260519">
  <script src="../../assets/js/theme.js?v=20260519"></script>
  <script src="../../assets/js/tailwind.js"></script>
</head>
<body class="bg-gray-950 text-gray-100 antialiased">
  <nav class="fixed top-0 inset-x-0 z-50 bg-gray-950/80 backdrop-blur-md border-b border-gray-800/70">
    <div class="max-w-[1400px] mx-auto px-4 sm:px-6">
      <div class="flex items-center justify-between h-16">
        <a href="../../index.html" class="flex items-center gap-3">
          <img src="../../assets/img/fluxheim-logo.webp" alt="Fluxheim" class="h-8 w-auto">
          <span class="font-bold text-white text-lg">Fluxheim</span>
        </a>
        <div class="hidden md:flex items-center gap-6">
          <a href="../index.html" class="text-sm font-medium text-cyan-400">Docs</a>
          <a href="../../download.html" class="text-sm font-medium text-gray-400 hover:text-white">Download</a>
          <a href="../../changelog.html" class="text-sm font-medium text-gray-400 hover:text-white">Changelog</a>
          <a href="https://github.com/valkyoth/fluxheim" target="_blank" rel="noopener" class="text-sm font-medium text-gray-400 hover:text-white">GitHub</a>
          <a href="../../download.html" class="px-4 py-2 bg-cyan-500 hover:bg-cyan-400 text-gray-950 text-sm font-bold rounded-lg transition-colors">Download v1.5.14</a>
        </div>
        <button type="button" class="theme-toggle ml-auto md:ml-0 p-2 rounded-lg text-gray-400 hover:text-white hover:bg-gray-800 transition-colors" data-theme-toggle aria-label="Switch color theme">
          <svg class="theme-toggle-sun w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364-6.364-.707.707M6.343 17.657l-.707.707m12.728 0-.707-.707M6.343 6.343l-.707-.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/></svg>
          <svg class="theme-toggle-moon w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/></svg>
        </button>
      </div>
    </div>
  </nav>

  <div class="flex min-h-screen pt-16">
    <aside class="hidden md:block fixed top-16 left-0 bottom-0 w-64 lg:w-72 border-r border-gray-800/60 bg-gray-950/80 overflow-y-auto z-20">
      <nav class="p-6 space-y-7">
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Getting Started</h3><ul class="space-y-0.5"><li><a href="../index.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Introduction</a></li><li><a href="../getting-started.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Installation & Quick Start</a></li></ul></div>
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Configuration</h3><ul class="space-y-0.5"><li><a href="../configuration.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Config Reference</a></li></ul></div>
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Features & Modules</h3><ul class="space-y-0.5"><li><a href="../features.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Feature Matrix</a></li><li><a href="load-balancer-migration.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Load Balancer Migration</a></li><li><a href="load-balancer-ha.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Load Balancer HA</a></li><li><a href="../tls-acme.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">TLS & ACME</a></li><li><a href="../cache.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Cache System</a></li><li><a href="../observability.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Observability</a></li></ul></div>
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Deployment</h3><ul class="space-y-0.5"><li><a href="../deployment.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Systemd & Containers</a></li></ul></div>
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Advanced</h3><ul class="space-y-0.5"><li><a href="../advanced.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">PHP-FPM, WAF & More</a></li><li><a href="../reference.html" class="block px-3 py-2 rounded-lg text-sm bg-cyan-500/8 text-cyan-400 border-l-2 border-cyan-400 pl-[10px]">Source Reference</a></li></ul></div>
        <div class="border-t border-gray-800/60 pt-5"><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Links</h3><ul class="space-y-0.5"><li><a href="https://github.com/valkyoth/fluxheim" target="_blank" rel="noopener" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">GitHub Repository</a></li><li><a href="../../changelog.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Changelog</a></li><li><a href="../../download.html" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Download</a></li></ul></div>
      </nav>
    </aside>

    <main class="flex-1 md:ml-64 lg:ml-72 min-w-0">
      <div class="max-w-5xl mx-auto px-6 lg:px-10 py-12">
        <div class="flex items-center gap-2 text-sm text-gray-500 mb-8">
          <a href="../../index.html" class="hover:text-gray-300">Fluxheim</a>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          <a href="../index.html" class="hover:text-gray-300">Docs</a>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          <a href="../reference.html" class="hover:text-gray-300">Source Reference</a>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          <span class="text-gray-400">{html.escape(title)}</span>
        </div>

        <article class="source-doc">
{body}
        </article>
      </div>
    </main>
  </div>
  <script src="../../assets/js/alpine.min.js" defer></script>
  <script src="../../assets/js/prism.min.js"></script>
  <script src="../../assets/js/prism-bash.min.js"></script>
  <script src="../../assets/js/prism-toml.min.js"></script>
</body>
</html>
"""


def main() -> None:
    for path in sorted(SOURCE_DIR.glob("*.md")):
        title, body = render_markdown(path.read_text())
        title = DOCS.get(path.name, title)
        path.with_suffix(".html").write_text(page_template(title, path.name, body))


if __name__ == "__main__":
    main()
