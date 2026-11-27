# Assessment: GHSA-587h-7wmj-h5pm (CVE-2026-112942), vite 8.3.1

- Date: 2026-11-27
- Author: Security auditor (GRC)
- Flow: dependency-vulnerability, step assess
- Status: open, fix to be done by the software engineer

## Facts

| Item | Value |
| --- | --- |
| Package | `vite` (npm), transitive dependency of `astro` 7.3.5 (`"vite": "^8.0.13"`) |
| Where | `site/package-lock.json` line 6064 (`node_modules/vite`, version 8.3.1) only |
| Not affected | `bindings/node`, `harness/parity/node` (no vite in their lockfiles); Rust crates |
| Fixed in | 8.3.3 |
| Advisory summary | `launch-editor`: the `fs.existsSync()` existence check can be bypassed on Windows (bundled in vite, no separate lockfile entry) |
| Advisory rating | High |
| Weakness class | CWE-22 / CWE-78 family (path or argument check bypass before launching a local editor) |

The advisory text was treated as untrusted data; nothing in it was executed or followed.

## Reachability

- `launch-editor` is used only by the vite **dev server** middleware (`/__open-in-editor`), started by `npm run dev` (`astro dev`) in `site/`.
- The production site is a static build (`astro build`) deployed to GitHub Pages by `.github/workflows/site.yml`. No vite server runs in production; vite code is not shipped to visitors.
- CI (`ubuntu-24.04`) only runs `npm ci`, `npm test`, `npm run build`: no dev server, and not Windows.
- The flaw is Windows-only. Exposure is therefore a contributor running `astro dev` on Windows, with the dev server reachable by the attacker (default bind is localhost; `--host` widens it) or a request driven from a malicious page opened in the same browser.

Worst case on such a machine: the attacker gets the editor launcher to run with a path or argument the existence check should have refused, i.e. code execution or file opening on the contributor's workstation (supply-chain risk to the repository through that machine).

## Real risk

- Advisory severity: High. Real exposure in this product: **Low** (dev-only, Windows-only, no production surface, no user data).
- Decision: **fix now**; the fix is a lockfile bump inside the range astro already accepts. No risk acceptance is needed, so no founder decision.
- Deadline: HQ default for High is 30 days, i.e. **2026-12-27** at the latest; target this PR.

## Fix for the software engineer

1. In `site/`: `npm update vite` (keep vite a transitive dependency, do not add it to `dependencies`). Check `npm ls vite` shows `8.3.3` or later, deduplicated under `astro`.
2. If npm keeps 8.3.1, add `"overrides": { "vite": "^8.3.3" }` to `site/package.json`, then `npm install`.
3. Commit only `site/package-lock.json` (and `package.json` if the override was needed), e.g. `fix(site): bump vite to 8.3.3 for GHSA-587h-7wmj-h5pm`.

## What to test

- `npm ci && npm test && npm run build` in `site/` (the Site workflow) is green.
- `npm audit --omit=optional` in `site/` no longer reports GHSA-587h-7wmj-h5pm.
- `npm run dev` starts and serves the home page (smoke check of the dev server after the bump).
- Re-run the HQ dependency scan (Trivy) on the branch; close the finding with that evidence after merge.

## Side notes (separate findings, not blocking)

- Low: `.github/workflows/site.yml` pins actions by tag (`actions/checkout@v7`, `actions/setup-node@v7`, `actions/upload-pages-artifact@v5`, `actions/deploy-pages@v5`), not by commit SHA as the HQ pipeline rule requires. Fix: pin each to its SHA with a version comment; let Dependabot or Renovate update them.
- Low: no Dependabot or Renovate for `site/` was found to catch such bumps automatically; consider a weekly npm update for `site/`.
