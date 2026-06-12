# Issue 9 Case Study: CI/CD and Repository Release Readiness

Issue: https://github.com/link-assistant/web-search/issues/9
Pull request: https://github.com/link-assistant/web-search/pull/10
Investigation date: 2026-06-12 UTC

## Source Data

Downloaded data is stored under this directory:

- `data/issue-9.json`, `data/issue-9-comments.json`
- `data/pr-10*.json`
- `data/recent-branch-runs.json`, `data/run-27445375811.json`, `data/run-27445375823.json`
- `ci-logs/run-27445375811.log`, `ci-logs/run-27445375823.log`
- `data/template-*-tree.json`, `data/template-js-release.yml`, `data/template-rust-release.yml`
- `data/npm-*.json`, `data/npm-*.stderr`, `data/cargo-*.txt`
- `research/*.log`

The raw CI logs are intentionally kept because the JavaScript release failure
was not reproducible from source alone; it depended on registry publish
permissions and npm trusted-publisher state.

## Timeline

| Time (UTC)       | Event                                                                                                                                   |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-06-12 21:57 | Main-branch JavaScript and Rust workflows started for release commit `5895f312...`.                                                     |
| 2026-06-12 21:59 | JavaScript release failed during npm publish. Log lines 9177-9180 show npm package metadata warnings followed by `E404`.                |
| 2026-06-12 22:02 | Rust CI completed successfully. Log lines 4556-4570 show `web-search@0.2.0` packaged under the crates.io size limit.                    |
| 2026-06-12 22:08 | Issue 9 was opened with README, release, web-capture, parity, template-comparison, and case-study requirements.                         |
| 2026-06-12 22:11 | PR branch checks were successful for the placeholder branch, but they did not yet address release readiness or the stale Rust provider. |

## Requirements

| Requirement                            | Finding                                                                                    | Change in this PR                                                                                                           |
| -------------------------------------- | ------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| Root README common to both languages   | Root README already had shared API content but no release/package badges.                  | Added root npm, crates.io, docs.rs, workflow, and language-specific release badges.                                         |
| Advanced language-specific READMEs     | `js/README.md` was a placeholder and `rust/README.md` only documented 3 providers.         | Replaced both READMEs with package-manager-focused install, API, CLI, server, provider, web-capture, release, and dev docs. |
| Actual npm and crate release readiness | npm publish failed with metadata warnings plus registry `E404`; Rust had no publish job.   | Fixed npm metadata/package contents and added Rust crates.io plus `rust-v*` GitHub release automation.                      |
| Separate GitHub releases by language   | JavaScript release scripts used generic `v<version>` tags; Rust had no release job.        | JavaScript now creates `js-v<version>` releases; Rust workflow creates `rust-v<version>` releases.                          |
| Latest web-capture libraries           | npm latest is `@link-assistant/web-capture@1.10.7`; crates.io has `web-capture@0.3.30`.    | Rust now depends on `web-capture = 0.3.30`; JS keeps lazy optional `@link-assistant/web-capture` behavior.                  |
| Same JS/Rust provider surface          | Registry parity existed, but Rust `wc:*` provider was a stub returning no results.         | Rust `WebCaptureProvider` delegates to the crate and adapts normalized results into the web-search result contract.         |
| Compare CI/CD templates                | JS workflow was close to the JS template; Rust workflow lacked publish/release automation. | Added Rust manual/main release path modeled on the Rust template's publish, wait, and GitHub release stages.                |
| Preserve evidence in case-study folder | No issue-9 case study existed.                                                             | Saved issue data, PR data, CI logs, registry lookups, package dry-runs, template files, and this analysis.                  |

## Root Causes

### JavaScript publish failure

The failed JavaScript release log shows three categories of problems:

- npm metadata warnings: `bin[web-search]` used `./bin/web-search.js` and the
  repository URL needed npm's `git+...git` form (`ci-logs/run-27445375823.log`
  lines 9177-9178).
- package lifecycle noise: the package-level `prepare` script ran Husky during
  release (`ci-logs/run-27445375823.log` line 9172).
- registry authorization or trusted-publisher state: npm returned `E404` for
  `@link-assistant/web-search@0.8.0` and failed after three attempts
  (`ci-logs/run-27445375823.log` lines 9180 and 9304).

The first two are source issues and are fixed here. The `E404` can still occur
if the npm organization/package trusted publisher is not configured for this
repository and workflow.

### JavaScript package contents

`npm pack --dry-run` showed the package had no `files` allowlist, so dev-only
assets such as tests, scripts, dotfiles, and changesets were included in the
package tarball. The regression test in
`research/js-package-metadata-test-before.log` failed on the bin shape and
missing files allowlist.

### Rust release gap

The Rust workflow had lint, test, doc-test, package-list, and crate-size checks,
but no crates.io publish or GitHub Release job. The downloaded Rust template has
dedicated publish, wait-for-crate, and GitHub release phases
(`data/template-rust-release.yml` lines 296-438 and 464-593). This PR adds the
same essential release path without importing the whole template script stack.

### Rust web-capture provider was stale

`rust/src/providers/web_capture.rs` said no published Rust crate existed and
returned no results for every non-empty query. `cargo search web-capture` and
`cargo info web-capture` showed that `web-capture 0.3.30` is published with
Rust `1.88` support (`data/cargo-search-web-capture.txt` line 1 and
`data/cargo-info-web-capture.txt` lines 6-11).

### web-capture transitive dependency drift

Adding `web-capture = 0.3.30` uncovered two fresh-resolution compile failures:

- `cookie 0.18.1` conflicted with latest `time 0.3.x` under Rust 1.96
  (`research/cargo-check-web-capture-after-first-pass.log` lines 465-479).
- `html-to-markdown-rs 3.6.1` changed the `InlineImage` dimensions shape
  expected by `web-capture 0.3.30`
  (`research/cargo-check-web-capture-after-time-pin.log` lines 168-181).

The workaround in this repo pins `time = 0.3.36` and
`html-to-markdown-rs = 3.5.7`, matching the dependency graph the component was
released against. `research/cargo-check-web-capture-after-pins.log` shows the
full Cargo check passing after those pins.

## Implemented Solution

- Added npm publish metadata tests for bin path, repository URL, public scoped
  publish config, no package `prepare` hook, and runtime-only `files`.
- Updated `js/package.json` to remove publish warnings and restrict the tarball
  to runtime assets.
- Changed JavaScript GitHub releases from `v<version>` to `js-v<version>`.
- Added `web-capture = 0.3.30` and deterministic transitive pins for the Rust
  dependency graph.
- Replaced the Rust `wc:*` stub with a real `web_capture::search` bridge and a
  result adapter that mirrors JavaScript's empty-result behavior on component
  errors.
- Added Rust integration tests for the web-capture provider contract, empty
  query behavior, and adapter behavior.
- Added a Rust release workflow job that publishes from `main`, skips already
  published crate versions, waits for crates.io visibility, and creates
  `rust-v<version>` GitHub releases.
- Reworked the root, JavaScript, and Rust READMEs with package-manager-facing
  docs and badges.

## Follow-ups

- Configure npm trusted publishing for `@link-assistant/web-search` in the npm
  organization so the next `main` release can get past the registry `E404`.
- Add `CARGO_REGISTRY_TOKEN` to repository secrets before enabling an actual
  Rust crates.io publish from `main`.
- Track the upstream `web-capture 0.3.30` dependency drift reported in
  https://github.com/link-assistant/web-capture/issues/137. Once fixed
  upstream, remove the local `time` and `html-to-markdown-rs` compatibility
  pins.
- Consider upgrading GitHub Actions to the template's newer Node 24/action
  versions after the current PR lands; this repo still uses `actions/*@v4` and
  Node 20 in the JS workflow.

## Upstream Template Check

No template bug was found. The useful template practices were release-path
coverage and explicit wait-for-registry behavior. The failures in this issue
were in this repository's release metadata/workflows and in the published
`web-capture` dependency graph, not in the four CI/CD templates.
