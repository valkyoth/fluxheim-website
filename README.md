<div align="center">
  <img src="assets/img/fluxheim-logo.webp" alt="Fluxheim" width="80">
  <h1>Fluxheim Website</h1>
  <p>
    The official project website for <a href="https://github.com/valkyoth/fluxheim">Fluxheim</a> —
    a high-performance edge server and reverse proxy built in Rust.
  </p>

  ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white)
  ![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=flat-square&logo=tailwind-css&logoColor=white)
  ![Alpine.js](https://img.shields.io/badge/Alpine.js-8BC0D0?style=flat-square&logo=alpine.js&logoColor=black)
  ![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue?style=flat-square)
</div>

---

## Overview

A fully static, multi-page project website — no build step, no Node.js, no framework. All JavaScript and CSS assets are vendored locally so the site works offline and can be served by any static file server, including Fluxheim itself.

The site is self-hosting: the included `container/Dockerfile` builds Fluxheim v1.6.4 from source and serves this website from inside a hardened Wolfi container.

## Pages

| Page | Path | Description |
|------|------|-------------|
| Landing | `index.html` | Hero, feature grid, quick-start tabs, architecture overview |
| Download | `download.html` | Release cards, install instructions, all-releases table |
| Changelog | `changelog.html` | Timeline-style release history |
| Docs — Hub | `docs/index.html` | Documentation landing with section cards |
| Docs — Installation | `docs/getting-started.html` | Tarball, container, and source install guides |
| Docs — Config Reference | `docs/configuration.html` | Full TOML configuration reference |
| Docs — Feature Matrix | `docs/features.html` | Cargo features and build profile table |
| Docs — TLS & ACME | `docs/tls-acme.html` | TLS backends, managed ACME, EAB issuers |
| Docs — Cache System | `docs/cache.html` | Memory/disk/tiered cache, range caching, admin API |
| Docs — Deployment | `docs/deployment.html` | Systemd, containers, Podman Quadlet |
| Docs — Observability | `docs/observability.html` | Prometheus, OpenTelemetry, structured logging |
| Docs — Advanced | `docs/advanced.html` | PHP-FPM, fluxheim-acme, config-tester, WAF, WASM |
| Docs — Source Reference | `docs/reference.html` | Vendored upstream Fluxheim Markdown docs from main |

## Tech Stack

All assets are vendored locally — no runtime CDN calls.

| Library | Version | Purpose |
|---------|---------|---------|
| Vendored Tailwind browser build | v3 | Utility-first styling, responsive dark and light themes |
| [Alpine.js](https://alpinejs.dev) | v3.14.1 | Mobile drawer, tabs, sidebar active state |
| [Prism.js](https://prismjs.com) | v1.29.0 | Syntax highlighting (TOML, Bash, Rust) |

**Design:** `#030712` (gray-950) dark background · `#22d3ee` (cyan-400) accent · violet secondary

## Running Locally

No build step — open any HTML file directly in a browser, or serve with any static file server:

```bash
# Python (built-in)
python3 -m http.server 8000

# Or with Fluxheim itself (see Docker section below)
```

## Docker / Podman

`container/Dockerfile` uses a three-stage build. It builds Fluxheim from the tagged upstream release and packages the website files from the local build context, so local container tests exercise the same files you are about to push:

1. **Builder** — `rust:1.96-slim-bookworm`: clones and compiles Fluxheim v1.6.4 with `profile-static-site`
2. **Site** — `alpine:3`: packages the local HTML, docs, assets, and config
3. **Runtime** — `cgr.dev/chainguard/wolfi-base`: hardened Wolfi image, non-root user, binary + site files baked in

### Build and run with Podman Compose

```bash
podman-compose -f container/podman-compose.yml up --build
```

The site will be available at **http://localhost:8080**.

### Build and run manually

```bash
# Build — context is repo root, Dockerfile is in container/
podman build -f container/Dockerfile -t fluxheim-website:1.6.4 .

# Run rootless on port 8080
podman run -d \
  --name fluxheim-website \
  --restart unless-stopped \
  -p 8080:8080 \
  fluxheim-website:1.6.4
```

### Override the config without rebuilding

```bash
podman run -d \
  --name fluxheim-website \
  -p 8080:8080 \
  -v ./conf/fluxheim.toml:/etc/fluxheim/fluxheim.toml:ro,z \
  fluxheim-website:1.6.4
```

## Project Structure

```
fluxheim-website/
├── index.html              # Landing page
├── download.html           # Downloads & install guide
├── changelog.html          # Release history
├── docs/
│   ├── index.html          # Docs hub
│   ├── getting-started.html
│   ├── configuration.html
│   ├── features.html
│   ├── tls-acme.html
│   ├── cache.html
│   ├── deployment.html
│   ├── observability.html
│   ├── advanced.html
│   ├── reference.html
│   └── source/               # Vendored Markdown plus rendered source-doc HTML
├── assets/
│   ├── css/
│   │   ├── prism-dark.min.css
│   │   └── theme.css
│   ├── js/
│   │   ├── tailwind.js
│   │   ├── theme.js
│   │   ├── alpine.min.js
│   │   ├── prism.min.js
│   │   ├── prism-bash.min.js
│   │   ├── prism-toml.min.js
│   │   └── prism-rust.min.js
│   └── img/
│       ├── fluxheim-logo.webp
│       └── fluxheim-overview.webp
├── conf/
│   └── fluxheim.toml       # Fluxheim vhost config for serving this site
├── container/
│   ├── Dockerfile          # Multi-stage: Rust/Debian builder → Wolfi runtime
│   └── podman-compose.yml
├── tools/
│   └── render-source-docs.py
├── .dockerignore
└── .containerignore
```

## Fluxheim Config

`conf/fluxheim.toml` configures Fluxheim to serve this site on port 8080 under the `fluxheim.eu` virtual host:

```toml
[server]
listen = ["0.0.0.0:8080"]
default_vhost = "site"

[[vhosts]]
name = "site"
hosts = ["fluxheim.eu"]

[vhosts.web]
root = "/srv/sites/fluxheim"
index_files = ["index.html"]
deny_dotfiles = true
```

## Contributing

1. Edit the HTML files directly — no compilation needed.
2. Keep all assets local (download and vendor any new JS/CSS into `assets/`).
3. Test by opening the page in a browser or running a local server.
4. The site targets the current stable Fluxheim release — update version strings in all pages when a new release ships.

## License

This website is part of the [Fluxheim](https://github.com/valkyoth/fluxheim) project and is licensed under the **[EUPL-1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)**.
