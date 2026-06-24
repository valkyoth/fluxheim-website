#!/usr/bin/env python3
"""Render the public, task-focused Fluxheim documentation pages."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROOT_PAGES = {"index.html", "download.html", "changelog.html", "cookies.html", "privacy.html", "gdpr.html"}

NAV = [
    ("index.html", "Start"),
    ("getting-started.html", "Install"),
    ("static-sites.html", "Static Sites"),
    ("reverse-proxy.html", "Reverse Proxy"),
    ("tls-acme.html", "TLS & ACME"),
    ("cache.html", "Cache"),
    ("load-balancer.html", "Load Balancer"),
    ("php-fpm.html", "PHP-FPM"),
    ("wordpress.html", "WordPress"),
    ("observability.html", "Observability"),
    ("features.html", "Builds & Features"),
    ("configuration.html", "Configuration"),
    ("deployment.html", "Systemd & Containers"),
    ("advanced.html", "Future Modules"),
    ("reference.html", "Full Reference"),
]

PAGES = {
    "index.html": (
        "Fluxheim Docs",
        "Choose what you want to run. Each guide starts with a working example, then links to the full source docs on GitHub when you need every detail.",
        """
<section class="grid md:grid-cols-2 gap-4">
  {cards}
</section>
<section class="guide-section">
  <h2>Good first path</h2>
  <ol class="steps">
    <li>Install Fluxheim from a release tarball or container image.</li>
    <li>Pick one job: static site, reverse proxy, cache, load balancer, or PHP-FPM.</li>
    <li>Validate the TOML config before you restart the service.</li>
  </ol>
</section>
""",
    ),
    "getting-started.html": (
        "Get Fluxheim Running",
        "Use this page when you want a local server quickly. It keeps the config small and checks it before serving traffic.",
        """
<section class="guide-section">
  <h2>Install a release binary</h2>
  <pre><code>curl -L https://github.com/valkyoth/fluxheim/releases/download/v1.6.30/fluxheim-1.6.30-full-x86_64-linux.tar.gz -o fluxheim.tar.gz
tar -xzf fluxheim.tar.gz
sudo install -m 0755 fluxheim /usr/local/bin/fluxheim</code></pre>
</section>
<section class="guide-section">
  <h2>Minimal static site</h2>
  <pre><code>[server]
listen = ["0.0.0.0:8080"]
default_vhost = "site"

[[vhosts]]
name = "site"
hosts = ["example.com"]

[vhosts.web]
root = "/srv/example"
index_files = ["index.html"]</code></pre>
</section>
<section class="guide-section">
  <h2>Check and start</h2>
  <pre><code>fluxheim --config /etc/fluxheim/fluxheim.toml --check-config
fluxheim --config /etc/fluxheim/fluxheim.toml</code></pre>
</section>
""",
    ),
    "static-sites.html": (
        "Static Sites",
        "Serve HTML, CSS, images, and downloads directly from disk. This is the simplest Fluxheim mode.",
        """
<section class="guide-section">
  <h2>When to use it</h2>
  <p>Use static hosting for documentation, product pages, downloads, and sites that do not need application code on each request.</p>
  <ul class="checks"><li>ETag and conditional requests reduce repeat traffic.</li><li>Byte ranges work for larger files.</li><li>Dotfiles should stay denied for public roots.</li></ul>
</section>
<section class="guide-section">
  <h2>Example</h2>
  <pre><code>[[vhosts]]
name = "docs"
hosts = ["docs.example.com"]

[vhosts.web]
root = "/srv/docs"
index_files = ["index.html"]
deny_dotfiles = true</code></pre>
</section>
""",
    ),
    "reverse-proxy.html": (
        "Reverse Proxy",
        "Put Fluxheim in front of an application server. Fluxheim handles TLS, headers, limits, and upstream selection.",
        """
<section class="guide-section">
  <h2>Basic proxy</h2>
  <pre><code>[[vhosts]]
name = "app"
hosts = ["app.example.com"]

[[vhosts.routes]]
path_prefix = "/"
action = "proxy"
upstreams = ["http://127.0.0.1:3000"]</code></pre>
</section>
<section class="guide-section">
  <h2>Production notes</h2>
  <ul class="checks"><li>Keep the application on loopback or a private network.</li><li>Set request body limits before exposing uploads.</li><li>Use trusted proxy settings only for networks you control.</li></ul>
</section>
""",
    ),
    "tls-acme.html": (
        "TLS & ACME",
        "Fluxheim can serve TLS with static certificates or manage ACME certificates for you.",
        """
<section class="guide-section">
  <h2>Managed certificates</h2>
  <pre><code>[[vhosts]]
name = "site"
hosts = ["example.com"]

[vhosts.tls]
acme = true
email = "admin@example.com"
storage = "/var/lib/fluxheim/acme"</code></pre>
</section>
<section class="guide-section">
  <h2>Before enabling ACME</h2>
  <ul class="checks"><li>Make sure public DNS points to this server.</li><li>Allow HTTP-01 or TLS-ALPN-01 challenge traffic.</li><li>Mount ACME state as persistent writable storage.</li></ul>
</section>
""",
    ),
    "cache.html": (
        "Cache",
        "Use cache when Fluxheim sits in front of an origin and repeated responses should be served faster.",
        """
<section class="guide-section">
  <h2>Proxy cache example</h2>
  <pre><code>[vhosts.cache]
enabled = true
backend = "disk"
path = "/var/cache/fluxheim/site"

[[vhosts.routes]]
path_prefix = "/assets/"
action = "proxy"
upstreams = ["http://127.0.0.1:3000"]
cache_ttl_secs = 3600</code></pre>
</section>
<section class="guide-section">
  <h2>Use cache carefully</h2>
  <ul class="checks"><li>Do not cache private user pages unless the route is explicitly safe.</li><li>Keep cache keys bounded and predictable.</li><li>Use purge or short TTLs when content changes often.</li></ul>
</section>
""",
    ),
    "load-balancer.html": (
        "Load Balancer",
        "Run several upstreams behind one public endpoint. Start simple, then add health checks and runtime controls.",
        """
<section class="guide-section">
  <h2>Two upstreams</h2>
  <pre><code>[[vhosts.routes]]
path_prefix = "/"
action = "proxy"
upstreams = [
  "http://10.0.0.11:8080",
  "http://10.0.0.12:8080",
]</code></pre>
</section>
<section class="guide-section">
  <h2>What it gives you</h2>
  <ul class="checks"><li>Focused load-balancer release images are available.</li><li>Health checks can keep broken backends out of rotation.</li><li>Drain and force-down operations help with maintenance windows.</li></ul>
</section>
""",
    ),
    "php-fpm.html": (
        "PHP-FPM",
        "Use PHP-FPM for PHP apps while Fluxheim serves static assets and forwards PHP requests safely.",
        """
<section class="guide-section">
  <h2>External PHP-FPM pool</h2>
  <pre><code>[vhosts.php]
enabled = true
root = "/srv/app/public"
index = "index.php"
socket = "/run/php-fpm/app.sock"</code></pre>
</section>
<section class="guide-section">
  <h2>Managed PHP-FPM</h2>
  <p>Managed mode lets Fluxheim start a private php-fpm master for the vhost. Use it when you want Fluxheim to own the socket and generated pool files.</p>
  <ul class="checks"><li>Keep PHP files inside the configured root.</li><li>Serve static assets directly when possible.</li><li>Never expose PHP source when PHP execution fails.</li></ul>
</section>
""",
    ),
    "wordpress.html": (
        "WordPress",
        "A practical PHP-FPM recipe for a normal WordPress front controller.",
        """
<section class="guide-section">
  <h2>WordPress vhost</h2>
  <pre><code>[[vhosts]]
name = "wordpress"
hosts = ["blog.example.com"]

[vhosts.web]
root = "/srv/wordpress"
index_files = ["index.php", "index.html"]
deny_dotfiles = true

[vhosts.php]
enabled = true
root = "/srv/wordpress"
index = "index.php"
socket = "/run/php-fpm/wordpress.sock"</code></pre>
</section>
<section class="guide-section">
  <h2>Common checks</h2>
  <ul class="checks"><li>Make sure the PHP-FPM user can read WordPress files.</li><li>Keep uploads writable only where WordPress needs writes.</li><li>Put cache in front of public assets, not wp-admin.</li></ul>
</section>
""",
    ),
    "observability.html": (
        "Observability",
        "Use Prometheus and OpenTelemetry to see aggregate traffic, errors, downloads, and page usage without tracking individual visitors.",
        """
<section class="guide-section">
  <h2>Prometheus metrics</h2>
  <pre><code>[metrics]
enabled = true
listen = "127.0.0.1:9100"</code></pre>
</section>
<section class="guide-section">
  <h2>OTLP export</h2>
  <pre><code>[observability.otlp]
enabled = true
endpoint = "http://127.0.0.1:4317"</code></pre>
  <p>Keep labels low-cardinality: route group, language, status class, download artifact, and GitHub target are useful. Raw IP addresses and user identifiers are not.</p>
</section>
""",
    ),
    "features.html": (
        "Builds & Features",
        "Fluxheim is compiled with feature sets. Pick the smallest build that contains what your deployment needs.",
        """
<section class="guide-section">
  <h2>Common builds</h2>
  <table><thead><tr><th>Build</th><th>Use it for</th></tr></thead><tbody>
    <tr><td>full</td><td>General production server with web, proxy, cache, TLS, ACME, PHP-FPM, metrics, and tracing.</td></tr>
    <tr><td>proxy</td><td>Reverse proxy without local static hosting or cache.</td></tr>
    <tr><td>cache</td><td>Cache edge in front of another origin.</td></tr>
    <tr><td>load-balancer</td><td>Focused upstream balancing and health checks.</td></tr>
    <tr><td>php</td><td>Static web plus PHP-FPM applications such as WordPress.</td></tr>
  </tbody></table>
</section>
<section class="guide-section">
  <h2>Things that cannot go together</h2>
  <ul class="checks"><li>Pick exactly one TLS backend.</li><li>Privacy builds do not include cache, metrics, tracing, or OTLP export.</li><li>Focused edge images intentionally omit modules outside their job.</li></ul>
</section>
""",
    ),
    "configuration.html": (
        "Configuration Basics",
        "Fluxheim uses TOML. Unknown fields are rejected, so spelling mistakes fail before restart.",
        """
<section class="guide-section">
  <h2>Main shape</h2>
  <pre><code>[server]
listen = ["0.0.0.0:8080"]
default_vhost = "site"

[[vhosts]]
name = "site"
hosts = ["example.com"]</code></pre>
</section>
<section class="guide-section">
  <h2>Safe habits</h2>
  <ul class="checks"><li>Run <code>fluxheim --check-config</code> before reloads.</li><li>Use one vhost per site or application boundary.</li><li>Keep secrets in files, environment, or container secrets, not in public docs.</li></ul>
</section>
""",
    ),
    "deployment.html": (
        "Systemd & Containers",
        "Use systemd for native hosts or rootless containers for isolated deployments.",
        """
<section class="guide-section">
  <h2>Rootless Podman</h2>
  <pre><code>podman run --name fluxheim --replace \\
  -p 8080:8080 \\
  -v ./fluxheim.toml:/etc/fluxheim/fluxheim.toml:ro \\
  ghcr.io/valkyoth/fluxheim:v1.6.30</code></pre>
</section>
<section class="guide-section">
  <h2>Production checklist</h2>
  <ul class="checks"><li>Pin release versions or image digests.</li><li>Mount config and content read-only.</li><li>Keep ACME and cache state on persistent volumes.</li></ul>
</section>
""",
    ),
    "advanced.html": (
        "Future Modules",
        "Some modules are planned or experimental. Keep them separate from production basics until they are stable.",
        """
<section class="guide-section">
  <h2>WAF</h2>
  <p>The WAF track is for rule-based request filtering, logging, and challenge/block responses. It should become its own clear page when it is production-ready.</p>
</section>
<section class="guide-section">
  <h2>WASM extensions</h2>
  <p>WASM extensions are planned for bounded operator logic. They must stay sandboxed, explicit, and disabled in default builds until the boundary is proven.</p>
</section>
""",
    ),
    "reference.html": (
        "Full Reference",
        "The website keeps short guides here. Full design notes, release runbooks, and source-level documents live in GitHub.",
        """
<section class="guide-section">
  <h2>Where the deep docs live</h2>
  <p>Use the GitHub repository when you need complete generated source documentation, release evidence, architecture tracks, and internal planning notes.</p>
  <p><a class="action" href="https://github.com/valkyoth/fluxheim/tree/main/docs">Open full docs on GitHub</a></p>
</section>
""",
    ),
}


def card(page: str, label: str) -> str:
    title, intro, _body = PAGES[page]
    return (
        f'<a class="doc-card" href="{page}">'
        f'<span>{label}</span><strong>{title}</strong><p>{intro}</p></a>'
    )


def code_language(code: str) -> str:
    first_line = code.lstrip().splitlines()[0] if code.strip() else ""
    shell_prefixes = ("curl ", "tar ", "sudo ", "fluxheim ", "podman ")
    if first_line.startswith(shell_prefixes):
        return "bash"
    return "toml"


def colorize_code_blocks(body: str) -> str:
    def replace(match: re.Match[str]) -> str:
        code = match.group(1)
        language = code_language(code)
        return (
            f'<pre class="language-{language}">'
            f'<code class="language-{language}">{code}</code></pre>'
        )

    return re.sub(r"<pre><code>(.*?)</code></pre>", replace, body, flags=re.DOTALL)


def docs_relative_fragment(fragment: str) -> str:
    def replace_attr(match: re.Match[str]) -> str:
        attr, target = match.groups()
        if target.startswith(("https://", "http://", "#", "mailto:", "tel:")):
            return match.group(0)
        if target.startswith("assets/"):
            target = f"../{target}"
        elif target.startswith("docs/"):
            target = target.removeprefix("docs/")
        elif target in ROOT_PAGES:
            target = f"../{target}"
        return f'{attr}="{target}"'

    return re.sub(r'(href|src)="([^"]+)"', replace_attr, fragment)


def homepage_fragment(start: str, end: str, include_end: bool = False) -> str:
    html = (ROOT / "index.html").read_text(encoding="utf-8")
    start_at = html.index(start)
    end_at = html.index(end, start_at)
    if include_end:
        end_at += len(end)
    return docs_relative_fragment(html[start_at:end_at])


def shared_header() -> str:
    header = homepage_fragment('  <nav x-data', '\n  </nav>', include_end=True)
    header = header.replace(
        'href="index.html" class="text-sm font-medium text-gray-400 hover:text-white transition-colors">Docs',
        'href="index.html" class="text-sm font-medium text-cyan-400 transition-colors">Docs',
    )
    return header.replace(
        'href="index.html" class="block px-3 py-2 text-sm rounded-lg text-gray-300 hover:text-white hover:bg-gray-800">Docs',
        'href="index.html" class="block px-3 py-2 text-sm rounded-lg text-cyan-400 bg-cyan-500/10">Docs',
    )


def shared_footer() -> str:
    return homepage_fragment('  <footer class="border-t border-gray-800/50 py-14 mt-4">', '\n\n  <script src="assets/js/alpine.min.js"')


def render(page: str, title: str, intro: str, body: str) -> str:
    nav = "\n".join(
        f'<li><a class="{nav_class(page, href)}" href="{href}">{label}</a></li>'
        for href, label in NAV
    )
    cards = "\n".join(card(href, label) for href, label in NAV[1:13])
    body = colorize_code_blocks(body.format(cards=cards))
    site_header = shared_header()
    site_footer = shared_footer()
    return f"""<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link rel="icon" href="../assets/img/fluxheim-logo.webp" type="image/webp">
  <title>{title} - Fluxheim Docs</title>
  <meta name="description" content="{intro}">
  <link rel="stylesheet" href="../assets/css/prism-dark.min.css">
  <link rel="stylesheet" href="../assets/css/theme.css?v=20260519">
  <script src="../assets/js/theme.js?v=20260519"></script>
  <script src="../assets/js/tailwind.js"></script>
  <style>
    body {{ background-color: var(--fh-bg); }}
    pre[class*="language-"] {{ background:#0d1117; border:1px solid #21262d; border-radius:10px; padding:1rem; overflow:auto; color:#d1d5db; font-size:.82rem; line-height:1.65; white-space:pre; word-break:normal; overflow-wrap:normal; }}
    code[class*="language-"] {{ display:block; width:max-content; min-width:100%; white-space:pre; word-break:normal; overflow-wrap:normal; }}
    :not(pre) > code {{ color:#67e8f9; background:rgba(6,182,212,.08); border:1px solid rgba(6,182,212,.15); border-radius:6px; padding:.08rem .28rem; }}
    .token.key, .token.keyword {{ color:#79c0ff; }}
    .token.string {{ color:#a5d6ff; }}
    .token.comment {{ color:#8b949e; }}
    table {{ width:100%; border-collapse:collapse; overflow:hidden; border-radius:10px; }}
    th,td {{ border:1px solid #1f2937; padding:.75rem; text-align:left; vertical-align:top; }}
    th {{ color:#9ca3af; background:rgba(17,24,39,.75); }}
    .sidebar-link.active {{ background:rgba(6,182,212,.08); color:#22d3ee; border-left:2px solid #22d3ee; }}
    .guide-section {{ margin-top:2rem; padding-top:2rem; border-top:1px solid #1f2937; }}
    .guide-section:first-of-type {{ margin-top:0; padding-top:0; border-top:0; }}
    .guide-section h2 {{ color:white; font-size:1.35rem; font-weight:800; margin-bottom:1rem; }}
    .guide-section p {{ color:#9ca3af; line-height:1.75; margin-bottom:1rem; }}
    .checks {{ display:grid; gap:.65rem; color:#d1d5db; }}
    .checks li {{ padding-left:.25rem; }}
    .steps {{ display:grid; gap:.75rem; color:#d1d5db; list-style:decimal; padding-left:1.25rem; }}
    .doc-card {{ display:block; border:1px solid #1f2937; border-radius:8px; background:rgba(17,24,39,.52); padding:1rem; transition:border-color 120ms ease, background-color 120ms ease; }}
    .doc-card:hover {{ border-color:rgba(6,182,212,.45); background:rgba(17,24,39,.82); }}
    .doc-card span {{ color:#22d3ee; font-size:.75rem; font-weight:800; text-transform:uppercase; letter-spacing:.04em; }}
    .doc-card strong {{ display:block; color:white; margin-top:.25rem; }}
    .doc-card p {{ color:#9ca3af; font-size:.875rem; margin-top:.45rem; line-height:1.6; }}
    .action {{ display:inline-flex; color:#020617; background:#22d3ee; padding:.6rem .85rem; border-radius:8px; font-weight:800; }}
  </style>
</head>
<body class="bg-gray-950 text-gray-100 antialiased">
{site_header}
  <div class="flex pt-16">
    <aside class="hidden md:block fixed top-16 left-0 max-h-[calc(100vh-4rem)] w-64 lg:w-72 border-r border-gray-800/60 bg-gray-950/80 overflow-y-auto z-20">
      <nav class="p-6 space-y-7">
        <div><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Guides</h3><ul class="space-y-0.5">{nav}</ul></div>
        <div class="border-t border-gray-800/60 pt-5"><h3 class="text-xs font-bold uppercase tracking-widest text-gray-600 mb-2.5">Deep Reference</h3><a href="https://github.com/valkyoth/fluxheim/tree/main/docs" target="_blank" rel="noopener" class="block px-3 py-2 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-gray-800/60">Full docs on GitHub</a></div>
      </nav>
    </aside>
    <main class="flex-1 md:ml-64 lg:ml-72 min-w-0">
      <div class="max-w-4xl mx-auto px-6 lg:px-10 py-12">
        <div class="flex items-center gap-2 text-sm text-gray-500 mb-8"><a href="../index.html" class="hover:text-gray-300">Fluxheim</a><span>/</span><a href="index.html" class="hover:text-gray-300">Docs</a><span>/</span><span class="text-gray-400">{title}</span></div>
        <header class="mb-10"><h1 class="text-4xl font-black text-white mb-4">{title}</h1><p class="text-gray-400 text-lg leading-relaxed max-w-2xl">{intro}</p></header>
{body}
      </div>
    </main>
  </div>
{site_footer}
  <script src="../assets/js/prism.min.js"></script>
  <script src="../assets/js/prism-bash.min.js"></script>
  <script src="../assets/js/prism-toml.min.js"></script>
  <script src="../assets/js/alpine.min.js" defer></script>
</body>
</html>
"""


def nav_class(current: str, href: str) -> str:
    base = "sidebar-link block px-3 py-2 rounded-lg text-sm transition-colors"
    if current == href:
        return f"{base} active pl-4"
    return f"{base} text-gray-400 hover:text-white hover:bg-gray-800/60"


def main() -> int:
    docs = ROOT / "docs"
    for page, (title, intro, body) in PAGES.items():
        (docs / page).write_text(render(page, title, intro, body), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
