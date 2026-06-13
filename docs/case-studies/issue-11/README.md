# Issue 11 Case Study: CI/CD False Positives Block Real Releases

> Fix all false positives and errors in CI/CD, to actually produce releases in
> Package Managers and GitHub Releases.
> — [link-assistant/web-search#11](https://github.com/link-assistant/web-search/issues/11)

This case study reconstructs the failures from the two CI runs referenced in the
issue, identifies the root cause of each, maps every requirement of the issue to
a concrete fix, and records the evidence (downloaded logs + live registry
probes) used to reach those conclusions.

## Source Data

All raw data backing this analysis lives next to this document:

| Path | What it is |
| --- | --- |
| `logs/rust-publish-failed.log` | Full log of the failed `Rust - Publish and Release` job (run 27448599745) |
| `logs/js-release-failed.log` | Full log of the failed `JS - Release` job (run 27448599744) |
| `data/issue-11.json` | The issue body, author and metadata |
| `data/run-27448599745-rust.json` | Job/step timeline for the Rust CI run |
| `data/run-27448599744-js.json` | Job/step timeline for the JavaScript CI run |
| `research/live-registry-probes.txt` | Live crates.io / npm probes that reproduce the false positive |
| `data/templates/*` | Template workflow + script files used for the comparison |

## Timeline

Both failing runs were triggered by the **same commit** — the merge of PR #10
(the previous release-readiness fix from issue #9) into `main`.

| Time (UTC) | Event |
| --- | --- |
| 2026-06-12 23:18:10 | `c67aef6` "Merge pull request #10 …" pushed to `main`; both release workflows start |
| 2026-06-12 23:19:12 | `JS - Release` job starts |
| 2026-06-12 23:19:35 | `publish-to-npm.mjs` begins; npm `PUT` returns **E404** for `@link-assistant/web-search` |
| 2026-06-12 23:20:03 | npm publish fails for the 3rd time → "Failed to publish after 3 attempts" → job red |
| 2026-06-12 23:27:35 | `Rust - Publish and Release` job starts (after build) |
| 2026-06-12 23:27:39 | Pre-flight `curl -fsS` to crates.io returns **HTTP 403** (no User-Agent) → "not published yet" |
| 2026-06-12 23:27:39 | Publish step finds `CARGO_REGISTRY_TOKEN` empty → `::error::CARGO_REGISTRY_TOKEN is required` → job red |
| 2026-06-13 08:49:06 | Issue #11 filed referencing both failed runs |

Every other job in both workflows (detect-changes, lint, the full test matrix,
build) passed. The failure is isolated to the **publish/release** jobs — exactly
the jobs that have to talk to an external package registry.

## Requirements

Extracted verbatim from the issue body (`data/issue-11.json`):

| # | Requirement | Status |
| --- | --- | --- |
| R1 | Fix the false positives/errors in CI/CD so releases actually publish to package managers + create GitHub Releases | ✅ Addressed (see Root Causes & Implemented Solution) |
| R2 | Compare **all** workflow / CI-CD script files against the four `link-foundation` pipeline templates and reuse their best practices | ✅ Done (see Template Comparison) |
| R3 | If the same issue exists in a template, file an issue upstream (with reproducible example, workaround, code-level fix suggestion) | ✅ Done for the JS template (see Upstream Reports); crates.io bug is repo-specific |
| R4 | Download all logs/data into `./docs/case-studies/issue-11` and write a deep case study (timeline, requirements, root causes, solution plans); search online for additional facts | ✅ This document |
| R5 | If data is insufficient to find a root cause, add debug/verbose output (default off) for the next iteration | ✅ Not needed — logs were conclusive; the publish scripts already emit verbose, actionable diagnostics |
| R6 | File issues on any other related repositories where issues can be filed | ✅ Covered by R3 (the templates are the related repos) |
| R7 | Apply each fix to the **entire** codebase — fix every place an issue appears | ✅ Both publish paths in each workflow were fixed (release + instant-release for JS; the single release job for Rust) |
| R8 | Do everything in the single PR #12 | ✅ All work is on branch `issue-11-319b016aadbe` / PR #12 |

## Root Causes

### RC1 — crates.io returns HTTP 403 without a User-Agent, defeating the publish skip (Rust)

The Rust release job from PR #10 used an **inline** pre-flight check:

```sh
if curl -fsS "https://crates.io/api/v1/crates/web-search/$VERSION" >/dev/null; then
  echo "published=true" ...
else
  echo "published=false" ...
fi
```

crates.io **rejects any request without a descriptive `User-Agent` header with
HTTP 403** (it is part of their [crawler policy](https://crates.io/policies)).
`curl`'s default `User-Agent` (`curl/8.x`) is treated as missing, so the request
returned 403, `curl -f` exited non-zero, and the `else` branch declared the
version "not published yet" on **every** run.

This is visible verbatim in `logs/rust-publish-failed.log`:

```
curl: (22) The requested URL returned error: 403
web-search 0.2.0 is not published on crates.io yet
```

Reproduced live (`research/live-registry-probes.txt`):

```
crates.io web-search/0.2.0 WITHOUT User-Agent  -> HTTP 403
crates.io web-search/0.2.0 WITH    User-Agent  -> HTTP 404 (correct: not yet published)
curl -fsS (no UA)                              -> exit 22 (treated as NOT published)
```

Consequence: the skip could never fire, so the job always proceeded to
`cargo publish`. Once the crate *is* published, that path would fail with
"crate already exists" — a permanent false-positive red build on every
subsequent no-op push.

### RC2 — `CARGO_REGISTRY_TOKEN` was empty at publish time (Rust)

Because RC1 forced the publish branch to run, the job hit its own token guard:

```
CARGO_REGISTRY_TOKEN:
##[error]CARGO_REGISTRY_TOKEN is required to publish the Rust crate
```

The token was empty. Two contributing factors:

1. The secret was not exposed to the job (no job/step-level `env` mapping a
   `secrets.*` value onto `CARGO_REGISTRY_TOKEN`), and
2. there was no fallback for an organisation secret named `CARGO_TOKEN`.

This is the **proximate** cause of the red Rust build; RC1 is the latent cause
that would keep the job red even after the token is supplied.

### RC3 — npm OIDC trusted publishing cannot bootstrap a brand-new package (JS)

The JS job publishes via npm **OIDC trusted publishing** (no token). The
registry returned:

```
npm error code E404
npm error 404 Not Found - PUT https://registry.npmjs.org/@link-assistant%2fweb-search - Not found
The requested resource '@link-assistant/web-search@0.8.1' could not be found or you do not have permission to access it.
```

`@link-assistant/web-search` has **never been published** — confirmed live:

```
npm @link-assistant/web-search  -> HTTP 404
```

Trusted publishing has a chicken-and-egg constraint: a *trusted publisher* can
only be configured on npmjs.com for a package that **already exists**. The very
first publish of a new package therefore cannot use OIDC — it needs a classic
automation token (`NODE_AUTH_TOKEN`). The workflow provided neither, and
`setup-node` injected its placeholder token (`XXXXX-XXXXX-XXXXX-XXXXX`), so the
`PUT` was unauthenticated → 404.

### RC4 — permanent failures were retried 3× and surfaced as a generic error (JS)

`publish-to-npm.mjs` classified the E404 as a generic "publish failure" and
retried it 3 times with 10 s waits (23:19:35 → 23:20:03 ≈ 28 s wasted) before
exiting 1 with "Failed to publish after 3 attempts". A first-publish E404 (and
E401/E403 auth failures) are **permanent** — retrying cannot help, and the
generic message hides the actual fix (configure a trusted publisher or supply
`NPM_TOKEN`).

### RC5 — GitHub Release creation was not idempotent

Both release jobs created the GitHub Release unconditionally. On any re-run after
the tag already exists (e.g. a transient failure in a later step), `gh release
create` / `gh api … releases` would fail with `already_exists` and turn the job
red — another false positive class the issue asks us to eliminate.

## Implemented Solution

Each root cause maps to a concrete change on this branch. The fixes were taken
from / aligned with the `link-foundation` pipeline templates wherever the
template already encodes the best practice.

| Root cause | Fix | Where |
| --- | --- | --- |
| RC1 | Pre-flight crates.io check now sends a descriptive `User-Agent` (`curl -A …`) and maps 200→published, 404→not-published, anything else→hard error; the publish step is gated on `published == 'false'` | `.github/workflows/rust.yml` (`Check crates.io version` step) |
| RC1/RC2 | Publishing delegated to `rust/scripts/publish-crate.rs` (verbatim from the Rust template): classifies failures (`already_exists` / `auth_failed` / `rate_limited` / `failed`), treats a 429 rate-limit as a *deferred* success (exit 0), reads the token from `CARGO_REGISTRY_TOKEN \|\| CARGO_TOKEN`; `rust/scripts/wait-for-crate.rs` confirms visibility with a `User-Agent` | `rust/scripts/publish-crate.rs`, `rust/scripts/wait-for-crate.rs` |
| RC2 | Job-level `env` maps `CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN \|\| secrets.CARGO_TOKEN }}` and `CARGO_TOKEN: ${{ secrets.CARGO_TOKEN }}` | `.github/workflows/rust.yml` (`release` job) |
| RC3 | `Publish to npm` steps now pass `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` as a **bootstrap fallback** — used for the first publish; once the package exists and a trusted publisher is configured, OIDC takes over even if `NPM_TOKEN` is unset. Publishing jobs bumped to Node `24.x` (≥ 22.14.0, npm ≥ 11.5.1, required for OIDC) | `.github/workflows/js.yml` (both `release` and `instant-release`) |
| RC3/RC4 | `publish-to-npm.mjs` classifies non-retryable failures (404/401/403/E404/E401/E403/`access token expired`/`eneedauth`/`you must be logged in`/`unable to authenticate`) and exits immediately with `buildAuthFailureGuidance()` — an actionable message explaining the first-publish bootstrap and pointing here — instead of retrying 3× | `js/scripts/publish-to-npm.mjs` |
| RC3 | `setup-npm.mjs` replaced with the template's multi-strategy upgrader (standard install → curl tarball → npx → corepack) and Node/npm minimum-version validation | `js/scripts/setup-npm.mjs` |
| RC5 | GitHub Release creation made idempotent in both languages: Rust checks `gh release view "$TAG"` first; JS treats an `already_exists` response from `gh api … releases` as success | `.github/workflows/rust.yml`, `js/scripts/create-github-release.mjs` |

### Why the no-op push is now green

After the fix, a push to `main` that does **not** bump a version:

- Rust: `Check crates.io version` returns 200 (with UA) → `published == 'true'`
  → publish step skipped → GitHub Release step is idempotent → **green**.
- JS: `npm view` finds the version already published → publish is skipped →
  **green**.

And the *first real release* now succeeds because the token/bootstrap paths
exist (RC2, RC3), while a genuinely missing token produces a single, actionable
error instead of three silent retries (RC4).

## Template Comparison

The issue requires comparing every CI/CD file with the four templates and
reusing their best practices. Findings:

| Concern | Template behaviour | This repo (before) | This repo (after) |
| --- | --- | --- | --- |
| crates.io existence check | `scripts/check-release-needed.rs` uses `ureq … .set("User-Agent", "rust-script-check-release")` ✅ | inline `curl -fsS` **without** UA ❌ | `curl -A …` with UA ✅ |
| crate publish | `scripts/publish-crate.rs` with failure classification + deferred rate-limit ✅ | inline `cargo publish` + ad-hoc token guard ❌ | `publish-crate.rs` (verbatim) ✅ |
| crate visibility wait | `scripts/wait-for-crate.rs` with UA ✅ | inline retry loop ❌ | `wait-for-crate.rs` (verbatim) ✅ |
| npm version upgrade | `scripts/setup-npm.mjs` multi-strategy ✅ | older single-strategy ⚠️ | template version ✅ |
| npm publish retries | `scripts/publish-to-npm.mjs` retries **all** failures, incl. permanent E404/auth ❌ (shared bug) | same ❌ | non-retryable classification + actionable guidance ✅ |
| npm first-publish bootstrap | release.yml publish step has **no** `NODE_AUTH_TOKEN` ❌ (shared bug) | same ❌ | `NODE_AUTH_TOKEN: secrets.NPM_TOKEN` fallback ✅ |
| GitHub Release idempotency | template create steps are not explicitly idempotent ⚠️ | not idempotent ❌ | idempotent in both languages ✅ |

The Python and C# templates are different language pipelines and share no
publish scripts with this repo; their workflow structure (detect-changes →
lint → test matrix → build → gated release on `main`) matches what this repo
already uses, so no structural changes were needed from them.

Net: the **crates.io 403/User-Agent bug is repo-specific** — it was introduced
by the repo's own inline `curl` during issue #9 / PR #10 and is *not* present in
the template (the template already sends a User-Agent). The **npm retry and
first-publish-bootstrap gaps are shared** with the JS template and are reported
upstream (below).

## Upstream Reports

Per R3/R6, the shared bugs are reported to
`link-foundation/js-ai-driven-development-pipeline-template` as
[issue #77](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/77).
See `upstream/js-template-publish-to-npm-bugs.md` for the exact report body
(reproducible example + workaround + code-level fix suggestion). It covers:

1. **`publish-to-npm.mjs` retries permanent failures and hides the cause** —
   the retry loop re-attempts E404 (first publish of a non-existent package) and
   E401/E403 (auth) failures `MAX_RETRIES` times even though they cannot
   succeed, then exits with a generic "Failed to publish after N attempts".
   Suggested fix: classify non-retryable patterns and exit immediately with
   actionable guidance (mirrors `js/scripts/publish-to-npm.mjs` in this PR).

2. **No first-publish bootstrap path** — `release.yml`'s `Publish to npm` step
   relies solely on OIDC trusted publishing, which cannot be configured for a
   package that does not yet exist, so the template's *first ever* release of a
   new package fails with E404. Suggested fix: add
   `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` as an optional bootstrap fallback
   on the publish steps.

## Maintainer Action Required

Two fixes need a repository secret, which only a maintainer can set (the
automation cannot create secrets):

- **Rust:** set `CARGO_REGISTRY_TOKEN` (or `CARGO_TOKEN`) to a crates.io API
  token with publish scope.
- **JS (first release only):** set `NPM_TOKEN` to an npm automation token to
  bootstrap the first publish of `@link-assistant/web-search`. After the first
  publish, configure a trusted publisher on npmjs.com and the token becomes
  optional.

## References

- crates.io crawler / User-Agent policy: https://crates.io/policies
- npm trusted publishing (OIDC): https://docs.npmjs.com/trusted-publishers
- GitHub Actions OIDC: https://docs.github.com/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect
- `link-foundation/rust-ai-driven-development-pipeline-template`
- `link-foundation/js-ai-driven-development-pipeline-template`
- Predecessor: issue #9 / PR #10 (`docs/case-studies/issue-9`)
