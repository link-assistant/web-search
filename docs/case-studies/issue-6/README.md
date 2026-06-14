# Issue 6 Case Study: Publish `web-search` packages (FormalAI adoption blocker)

- **Issue:** [link-assistant/web-search#6](https://github.com/link-assistant/web-search/issues/6) — _FormalAI adoption blocker: publish web-search packages_
- **Upstream driver:** [link-assistant/formal-ai#410](https://github.com/link-assistant/formal-ai/issues/410)
- **Pull request:** [link-assistant/web-search#14](https://github.com/link-assistant/web-search/pull/14)
- **Branch:** `issue-6-9ad68b60fc91`

## Executive summary

FormalAI cannot adopt `web-search` as a component while the packages are not
installable from public registries. When the issue was filed (2026-06-11) both
`npm view @link-assistant/web-search` and `cargo info web-search` failed
(`E404` / not found).

By the time this case study was reconstructed (2026-06-14) **both packages had
already been published manually**:

- npm `@link-assistant/web-search@0.8.2` (published 2026-06-13T13:43Z)
- crates.io `web-search@0.2.0` (published 2026-06-13)

So the registry-lookup symptom from the issue body is **stale**. The remaining,
still-live problem is that the **release automation does not reliably keep
publishing**, the **README install instructions were unversioned**, and **CI did
not prove the published artifacts are actually installable**. This PR fixes the
automation gaps and the documentation, and reports the shared CI/CD gap upstream
to the four pipeline templates.

## Timeline of events

| When (UTC)         | Event                                                                                                          |
| ------------------ | -------------------------------------------------------------------------------------------------------------- |
| 2026-06-11 17:19   | Issue #6 filed. `npm view` → `E404`, `cargo info` → not found. Both registries empty for `web-search`.         |
| 2026-06-11         | Two referenced CI runs (`27482761521`, `27482761505`) failed — publish never reached the registries.           |
| 2026-06-13 11:56   | GitHub Release `rust-v0.2.0` created; crate `web-search@0.2.0` published to crates.io (Rust pipeline works).   |
| 2026-06-13 13:43   | npm `@link-assistant/web-search@0.8.2` published (manual bootstrap publish; **no `js-v0.8.2` tag/release**).   |
| 2026-06-13 (later) | `package.json` bumped to `0.9.0` (commit `ce5f7b7`). The publish of `0.9.0` did **not** land on npm.           |
| 2026-06-14         | This PR: self-healing release detection, versioned README, install-from-package smoke tests, upstream reports. |

## Requirement breakdown

| Acceptance criterion (issue #6)                                                         | Status            | Implementation in this PR                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Publish `@link-assistant/web-search` to npm (or document the intended name)             | Met + hardened    | Package is published (`0.8.2`). The release job now **self-heals**: if `package.json` is ahead of npm with no changeset, it republishes (fixes the stuck `0.9.0`). See root cause #1.                                                                        |
| Publish the Rust crate to crates.io (or document alternative distribution)              | Met               | `web-search@0.2.0` is on crates.io; the Rust pipeline publishes correctly via `CARGO_REGISTRY_TOKEN`/`CARGO_TOKEN`. No change required beyond the smoke test.                                                                                                |
| Add release tags / GitHub releases mapping to published versions                        | Met going forward | `rust-v0.2.0` exists. JS releases create `js-v<version>` tags + GitHub releases on publish (`create-github-release.mjs`); the self-heal will mint `js-v0.9.0`. `0.8.2` was a manual bootstrap publish that predates tagging (documented under "Known gaps"). |
| Document library, CLI, and HTTP server entry points with **versioned** install commands | Met               | README gained an **Entry points** table + versioned `npm install …@0.9.0` / `cargo add …@0.2.0` / `cargo install` commands. See `README.md`.                                                                                                                 |
| Ensure CI exercises **install-from-package smoke tests** after publication              | Met               | `js/scripts/smoke-test-package.mjs` (npm) + a `Smoke-test published crate` step in `rust.yml` (crates.io) install the published artifact into a clean project and run all entry points.                                                                      |
| Compare all CI/CD files vs the 4 templates; report shared issues upstream               | Met               | The install-from-package gap is shared by all 4 templates → 4 upstream issues filed (see "Upstream reports").                                                                                                                                                |
| Download all logs/data and compile a case study                                         | Met               | This document + `data/` + `raw-data/`.                                                                                                                                                                                                                       |

## Root cause analysis

### Root cause #1 — JS `0.9.0` got stuck: a failed publish consumes the changeset

The JS release job gated publishing on "are there changeset files?". The flow is:

1. A changeset bumps `package.json` to a new version and is committed/consumed.
2. `changeset publish` runs. **If it fails** (missing auth, registry blip), the
   version is already bumped and the changeset is already gone.
3. The next push to `main` finds **no changeset**, so the release job skips —
   `package.json` says `0.9.0` but npm only has `0.8.2`, forever.

This is the live failure behind the issue's "JS flow failed" links.

**Fix:** ported the template's self-healing detection
(`js/scripts/check-release-needed.mjs`). It checks **npm — the source of truth —
not git tags**. With no changeset, if `package.json`'s version is not yet on npm,
it requests a release with `skip_bump=true` (republish without minting a new
version). The `release` job's publish condition was expanded to honor this.
Analogous to `check-release-needed.rs` in the Rust template and template issue
[#36](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/36).

### Root cause #2 — first publish of a brand-new package cannot be bootstrapped by OIDC

npm "trusted publishing" (OIDC) requires a trusted publisher to be **configured
on npmjs.org for an already-existing package**. The very first publish of a
brand-new package therefore cannot use OIDC and returns `E404` — which matches
the issue's original symptom. This is an **owner action**, not a code bug:
`publish-to-npm.mjs` already prints `buildAuthFailureGuidance()` explaining it,
and `js.yml` already passes `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` as the
bootstrap fallback. Now that `0.8.2` exists, the owner can configure the OIDC
trusted publisher and subsequent releases need no token.

### Root cause #3 — "published" was never proven, only "indexed"

Both pipelines confirmed the registry _indexed_ the new version, but never
installed it and ran it. A package can index yet be broken for consumers
(missing files, wrong `bin`/`[[bin]]` path, misconfigured `exports`). Fixed by
the install-from-package smoke tests (root requirement + Root cause #3 below).

### Root cause #4 — install docs were unversioned

The README install commands had no version pin and did not enumerate the three
entry points, so a downstream consumer could not reproduce a known-good install.
Fixed in `README.md`.

## Solution plan executed

1. **Self-healing JS release** — added `js/scripts/check-release-needed.mjs`;
   wired it into `.github/workflows/js.yml` and expanded the publish condition.
2. **Versioned docs** — added an _Entry points_ table and versioned npm/cargo
   install commands to `README.md`; added a `serve` subcommand to the JS CLI for
   parity with the Rust CLI so both share `web-search serve --port <port>`.
3. **Install-from-package smoke tests** — `js/scripts/smoke-test-package.mjs`
   (npm) and a `Smoke-test published crate` step in `rust.yml` (crates.io), each
   verifying library + CLI + HTTP-server entry points from a clean install.
4. **CLI parser unit tests** — `js/tests/cli.test.js` covers `--serve`, the
   `serve` subcommand, and that a query containing the word "serve" is not
   misinterpreted.
5. **Upstream reports** — filed the shared install-from-package gap to all four
   pipeline templates (see below).

## Template comparison (Task: compare all CI/CD files vs the 4 templates)

Raw template workflows are saved under `raw-data/templates/`:
`js-release.yml`, `rust-release.yml`, `python-release.yml`, `csharp-release.yml`.

| Capability                                          | js tmpl | rust tmpl | python tmpl | csharp tmpl | web-search (after this PR) |
| --------------------------------------------------- | :-----: | :-------: | :---------: | :---------: | :------------------------: |
| Self-healing release detection (check registry)     |   ✅    |    ✅     |     ✅      |     ✅      |    ✅ (now, JS + Rust)     |
| Wait for registry indexing after publish            |   ✅    |    ✅     |     ⚠️      |     ✅      |             ✅             |
| **Install-from-package smoke test of entry points** |   ❌    |    ❌     |     ❌      |     ❌      |      ✅ (added here)       |

**Shared gap:** none of the four templates install the _published_ artifact and
run it; they only confirm it is indexed. Reported upstream with reproducible
examples, workarounds, and a fix suggestion that links back to this repo's
implementation.

## Upstream reports

| Template repo                                                    | Issue                                                                                              |
| ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `link-foundation/js-ai-driven-development-pipeline-template`     | [#81](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/81)     |
| `link-foundation/rust-ai-driven-development-pipeline-template`   | [#73](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/73)   |
| `link-foundation/python-ai-driven-development-pipeline-template` | [#20](https://github.com/link-foundation/python-ai-driven-development-pipeline-template/issues/20) |
| `link-foundation/csharp-ai-driven-development-pipeline-template` | [#27](https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template/issues/27) |

See `upstream-reports.md` for the full filed bodies.

## Current published state (evidence)

- npm: `@link-assistant/web-search` versions = `["0.8.2"]` (package.json = `0.9.0`, pending self-heal).
- crates.io: `web-search` versions = `["0.2.0"]`.
- git tags: `rust-v0.2.0` (no `js-v*` yet).
- GitHub releases: `Rust web-search 0.2.0` (`rust-v0.2.0`).

Raw probes: `data/issue-6.json`, `data/pr-14.json`,
`raw-data/ci-logs/*.log`.

## Known gaps / remaining owner actions

- **npm auth:** subsequent publishes need either the `NPM_TOKEN` secret set or a
  configured OIDC trusted publisher on npmjs.org for the now-existing package.
  This is an owner action; the pipeline self-heals once auth is available.
- **`js-v0.8.2` tag:** `0.8.2` was a manual bootstrap publish before release
  tagging; the automation mints `js-v<version>` tags from `0.9.0` onward. A
  retroactive `js-v0.8.2` tag can be added by a maintainer if desired (this PR
  is restricted to the `issue-6-*` branch and does not push tags).

## Verification

```bash
# JavaScript
cd js
npm install
npm test          # 134 tests incl. js/tests/cli.test.js
npm run check     # eslint + script syntax + prettier + jscpd

# Self-healing detection (no changesets, version ahead of npm → should_release=true, skip_bump=true)
HAS_CHANGESETS=false node js/scripts/check-release-needed.mjs --js-root js

# Install-from-package smoke test (runs in CI after publish; locally needs the version on npm)
node js/scripts/smoke-test-package.mjs --package-version 0.8.2 --js-root js
```
