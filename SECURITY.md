# Security Policy

This frontend application is client-facing and designed with a strict emphasis on user data sovereignty, client-side integrity, and defensive architecture. Treat Alpine.js reactive state changes, utility class overrides, and third-party npm dependency updates as high-risk until thoroughly validated.

## Routine Checks

Run these security audits regularly during local development and strictly enforced before any production release:

```bash
# Audit the frontend dependency tree for known vulnerabilities
npm audit

# Validate formatting and look for suspicious code patterns/linters
npm run lint
npm run format -- --check

# Execute local asset compilation to catch configuration or build bugs
npm run build

# Run custom repository integrity and validation preflights
scripts/validate-assets.sh
scripts/release_checks.sh
```

GitHub Actions run our continuous integration (CI) pipeline. The GitHub CodeQL default setup should be explicitly enabled in the repository security settings to scan for client-side vulnerabilities (such as DOM-based XSS or open redirects). 

*Note: Maintain only one active CodeQL configuration. GitHub will reject SARIF uploads if both the default setup and an advanced workflow file try to analyze the same frontend repository simultaneously.*

The full build gate is managed via our local release workflows. Utilize it before pushing static assets to production hosting, updating dependency ranges, or refactoring client-side state architecture.

## Dependency Policy

We enforce strict control over our client-side software supply chain to preserve data privacy and prevent malicious script injection. 

- **Local Compilations Only:** Pulling dependencies via untracked external CDNs (such as unpinned `<script>` tags) is heavily restricted to prevent supply-chain tampering and protect user privacy. All packages must be pulled locally via `npm` and bundled.
- **Lockfile Enforcement:** The `package-lock.json` (or respective project lockfile) must be cryptographically locked and committed. Floating dependencies are denied.
- **Reviewed Advisories:** If an upstream dependency triggers an `npm audit` warning and cannot be immediately patched due to breaking changes, it must be audited to ensure the vulnerable path is unreachable in our frontend code. 

Current reviewed client-side advisory exceptions:
- *No active exceptions.* Every package must clear `npm audit` with zero high or critical vulnerabilities before a deployment tag is cut.

## Content Security & Token Policy

Because this project runs entirely within the user's browser, preventing Cross-Site Scripting (XSS) and accidental data exposure is paramount.

- **Alpine.js Directives:** Never use `x-html` with user-supplied input or unvalidated URL parameters, as this introduces direct DOM-based XSS vectors. Always default to `x-text` for reactive string binding.
- **Secrets Exposure:** Hardcoding backend API tokens, staging authorization headers, or private service credentials inside HTML markup or Alpine components is strictly prohibited. Use environment variables during the build process if configuration is required, ensuring no private keys are exposed to the public bundle.
- **Content Security Policy (CSP):** This application is architected to align with a strict Content Security Policy. Avoid inline script blocks or unsanctioned inline styles that require `'unsafe-inline'` or `'unsafe-eval'` overrides.

## Reporting

Do not publish exploitable security details, proof-of-concept scripts, or client-side vulnerability findings in public GitHub issues. Please open a private security advisory through the repository’s native Security tab or contact the maintainers directly to coordinate a coordinated patch release.
