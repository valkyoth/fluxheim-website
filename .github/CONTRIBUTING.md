# Contributing to fluxheim-website

This is a fully static website — plain HTML5, a vendored Tailwind browser build, Alpine.js, and a small shared theme script. There is no build step, no Node.js, no package manager. All JavaScript and CSS assets are vendored locally under `assets/`.

Contributions that keep the site lightweight, accessible, and accurate are welcome.

## License

This project is licensed under the **European Union Public Licence 1.2 (EUPL-1.2)**. By contributing, you agree that your contribution is provided under the same licence.

## Development Setup

No tooling required. Open any HTML file directly in a browser, or run a local server from the repo root:

```bash
python3 -m http.server 8000
```

Then visit `http://localhost:8000`.

To test the full container build locally:

```bash
podman build -f container/Dockerfile -t fluxheim-website:local .
podman run -d -p 8080:8080 --name fluxheim-test fluxheim-website:local
# Visit http://localhost:8080 — clean up with:
podman rm -f fluxheim-test
```

## What Lives Where

| Path | Purpose |
|------|---------|
| `index.html`, `download.html`, `changelog.html` | Top-level site pages |
| `docs/` | Documentation pages — one file per topic |
| `assets/js/` | Vendored JavaScript (Tailwind, Alpine.js, Prism.js, theme toggle) |
| `assets/css/` | Vendored CSS (Prism theme and shared light/dark theme overrides) |
| `assets/img/` | Logo and images |
| `conf/fluxheim.toml` | Fluxheim vhost config for serving the site |
| `container/Dockerfile` | Multi-stage build: Rust builder → Wolfi runtime |
| `container/podman-compose.yml` | Compose file for local container testing |

## Making Changes

**Editing a page:** Edit the HTML file directly. Pages share a common nav and footer pattern — if you update one, update all. Sidebar active-link state in the docs pages is set via the `bg-cyan-500/8 text-cyan-400 border-l-2` class on the relevant `<a>` element.

**Adding a docs page:** Copy an existing docs page as your template. Add a link to the sidebar nav (present in every docs page) and to `docs/index.html` (the docs hub). Keep the file name lowercase with hyphens.

**Updating for a new Fluxheim release:** Version strings appear in the nav button, download cards, changelog timeline, install code blocks, and the container image tags. Search for the old version and replace throughout. Update the changelog timeline and the download page release table to match the new release artifacts.

**Adding a vendored asset:** Download the file into the appropriate `assets/` subdirectory. Do not reference external CDN URLs. Update the relevant HTML `<script>` or `<link>` tags to point to the local path.

## Checks Before Opening a PR

- [ ] Pages open without console errors in Firefox and Chrome
- [ ] Navigation links and sidebar links all resolve correctly
- [ ] Mobile menu and light/dark theme toggle work (test at <768px viewport width)
- [ ] Code blocks render with syntax highlighting in both themes
- [ ] No external CDN URLs introduced (check `<script src>` and `<link href>` attributes)
- [ ] Version strings are consistent across all pages

## Security-Sensitive Areas

- **Alpine.js directives:** Use `x-text` for user-controlled or dynamic content, never `x-html`, to avoid DOM-based XSS.
- **External scripts and styles:** All assets must be vendored locally. Do not add `<script src="https://...">` or `<link href="https://...">` references.
- **Container:** Changes to `container/Dockerfile` should preserve the non-root `fluxheim` user and the Wolfi runtime base.
- **Config:** `conf/fluxheim.toml` has `deny_dotfiles = true` and directory listing disabled — do not relax these.

Do not post vulnerability details in public issues. Follow [SECURITY.md](../SECURITY.md).

## Pull Requests

Keep PRs scoped to a single change. Include a clear summary of what changed and why. For release updates, list the version being targeted in the PR description.
