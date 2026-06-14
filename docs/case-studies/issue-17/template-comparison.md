# Issue #17 — CI/CD template comparison

Comparison of the `link-assistant/web-search` release pipeline against the four
`link-foundation` AI-driven-development pipeline templates. Goal: validate the
issue #17 SHORT tag format (`rust-0.0.1`, `js-0.0.1`), reuse template best
practices, and surface bugs shared with the templates so they can be reported
upstream.

Repos compared:

- web-search: `/tmp/gh-issue-solver-1781438390962`
  (`.github/workflows/js.yml`, `rust.yml`, `parity.yml`; `js/scripts/*.mjs`, `rust/scripts/*.rs`)
- tpl-js: `https://github.com/link-foundation/js-ai-driven-development-pipeline-template`
- tpl-rust: `https://github.com/link-foundation/rust-ai-driven-development-pipeline-template`
- tpl-python: `https://github.com/link-foundation/python-ai-driven-development-pipeline-template`
- tpl-csharp: `https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template`

---

## Template tag conventions

All four templates use an **underscore + `v`** prefix for multi-language repos
and a **bare `v`** prefix for single-language repos. None use a hyphen; none
drop the `v`. Multi-language namespacing is by a per-language word prefix.

- **tpl-js** — multi: `js_v1.2.3`, single: `v1.2.3`. Source: `scripts/release-naming.mjs:14-15` (`MULTI_LANGUAGE_TAG_PREFIX = 'js_v'`, `SINGLE_LANGUAGE_TAG_PREFIX = 'v'`), `buildReleaseTag` line 55.
- **tpl-rust** — multi: `rust_v1.2.3`, single: `v1.2.3`. Source: `scripts/release-naming.rs:6-7` (`RUST_MULTI_LANGUAGE_TAG_PREFIX = "rust_v"`, `SINGLE_LANGUAGE_TAG_PREFIX = "v"`).
- **tpl-python** — multi: `py_v1.2.3`, single: `v1.2.3`. Source: `scripts/release_naming.py:14-15` (`MULTI_LANGUAGE_TAG_PREFIX = "py_v"`).
- **tpl-csharp** — multi: `cs_v1.2.3`, single: `v1.2.3`. Source: `scripts/release-naming.mjs:200-202` (`getTagPrefix` → `'cs_v'` / `'v'`); the C# release workflow passes `--language "C#"` explicitly (`.github/workflows/release.yml:557,720`) but relies on the default `cs_v` tag prefix (no `--tag-prefix` flag is actually passed).

Title convention (shared by all four): multi-language → `[Language] 1.2.3`,
single-language → `<PackageName> 1.2.3`.

Multi-language detection is layout-based in every template: a manifest at the
repo root (`package.json` / `Cargo.toml` / `pyproject.toml` / `*.csproj`) means
single-language (`v` prefix); a manifest under a language subdir
(`js/`, `rust/`, `python/`, `csharp/`) means multi-language (namespaced prefix).

---

## Best practices to adopt

Items present in the templates' workflows/scripts but missing (or only inlined
and untested) in web-search. References are to the TEMPLATE source.

JavaScript:

- **CHANGELOG regex escaping** — `tpl-js/scripts/create-github-release.mjs:89,96` adds `escapeRegex(version)` before building the header regex and factors extraction into a testable `extractReleaseNotes()`. web-search interpolates `version` raw (`js/scripts/create-github-release.mjs:131`), so the dots act as regex wildcards.
- **`package-info.mjs`** centralizes package-name/version reads. web-search hardcodes `@link-assistant/web-search` in several scripts (`js/scripts/publish-to-npm.mjs:34`, `validate-changeset.mjs:21`, `merge-changesets.mjs:30`). Template scripts read it from `package.json` (`tpl-js/scripts/check-release-needed.mjs:66`).
- **Failure classifier as a tested module** — `tpl-js/scripts/publish-failure-classifier.mjs`. web-search has equivalent logic inlined and untested in `js/scripts/publish-to-npm.mjs:91-152`.
- **Windows-safe path handling** — `tpl-js/scripts/validate-changeset.mjs:48-50` normalizes backslashes; web-search `validate-changeset.mjs:24` does not.
- **Quote-tolerant changeset frontmatter parsing** — `tpl-js/scripts/validate-changeset.mjs:126-128` (`requireQuotes:false`); web-search hardcodes quote expectations.
- **`timeout-minutes` on every job** — `tpl-js/.github/workflows/release.yml:51,375` (and throughout). web-search `js.yml` has **no `timeout-minutes` at all**; web-search `rust.yml` does set timeouts.
- Extra helper scripts shipped by tpl-js and absent from web-search: `check-docker-publish.mjs`, `check-file-line-limits.sh`, `check-mjs-syntax.sh`, `check-web-archive.mjs`, `format-release-notes-helpers.mjs`, `package-info.mjs`, `publish-failure-classifier.mjs`, `simulate-fresh-merge.sh`, `update-preview-images.mjs`, `wait-for-npm.mjs`.

Rust (web-search inlines the whole release in `rust.yml:344-383` instead of using scripts):

- **Release-body byte cap** — `tpl-rust/scripts/create-github-release.rs:43,217-247` caps the body (~125k) and appends a "full changelog" link. web-search's `gh release create --notes "$NOTES"` (`rust.yml:380-383`) has no byte-limit handling. (The JS side of web-search *does* implement this, in `js/scripts/create-github-release.mjs:71-119`.)
- **Robust idempotency** — `tpl-rust/scripts/create-github-release.rs:176-200` parses the HTTP 422 JSON and checks `resource==Release && code==already_exists && field==tag_name` (tests at lines 491-512). web-search uses a `gh release view "$TAG"` then `gh release create` sequence (`rust.yml:360-383`) — a TOCTOU gap, and a tag existing does not prove the crate/artifacts published.
- **Self-healing release check** — `tpl-rust/scripts/check-release-needed.rs` reconciles partially-completed releases; web-search only does an inline crates.io 200/404 check.
- **Badge/title helpers and unit tests** — the template ships `release-naming.rs` + `#[cfg(test)]` tests; web-search builds the tag/title/badge inline in YAML with no tests.
- Scripts shipped by tpl-rust and missing from web-search: `bump-version.rs`, `check-cargo-lock.rs`, `check-release-needed.rs`, `collect-changelog.rs`, `create-changelog-fragment.rs`, `create-github-release.rs`, `get-bump-type.rs`, `get-version.rs`, `git-config.rs`, `release-naming.rs`, `smoke-test-published-crate.rs`, `version-and-commit.rs`.

Cross-cutting:

- **Newer action pins** — templates pin `actions/checkout@v6`, `actions/setup-node@v6`, `actions/upload-artifact@v7`, `actions/cache@v5`, `peter-evans/create-pull-request@v8`. web-search pins `checkout@v4`, `setup-node@v4`, `cache@v4`, `create-pull-request@v7`. Upgrading avoids the Node 20 action-runtime deprecation (see Node section).

---

## Shared bugs to report upstream (with file:line and repro)

These defects exist in BOTH web-search and a template — candidates to file
against the upstream template repos.

1. **CHANGELOG `## 1.2` vs `## 1.2.3` prefix collision (JS, residual after escape fix).**
   - web-search: `js/scripts/create-github-release.mjs:131` — `## ${version}[\s\S]*?(?=## \d|$)` (also unescaped: dots are wildcards).
   - tpl-js: `scripts/create-github-release.mjs:96` — escaped, but still no end-anchor after the version.
   - Repro: CHANGELOG with `## 1.2.3` above `## 1.2`. Extracting notes for `1.2` matches starting at the `## 1.2.3` header (it is a prefix), publishing the wrong section. Fix: anchor the version end, e.g. `## ${escapeRegex(version)}(?=\s|$)`.
   - tpl-rust is immune: `scripts/create-github-release.rs:117-119` uses `(?m)^## \[{escaped}\]` (escaped, bracket-delimited, line-anchored).

2. **Changeset detection counts arbitrary `.md` files.**
   - web-search: `js/scripts/check-changesets.mjs:48-50`. tpl-js: `scripts/check-changesets.mjs:45-47`.
   - Filter is `file.endsWith('.md') && file !== 'README.md'`. Repro: a stray `.changeset/NOTES.md` doc file is counted as a pending changeset and triggers/poisons a release or fails validation. Fix: match the changeset filename pattern and/or require valid frontmatter.

3. **`merge-changesets` silently drops malformed changesets.**
   - web-search: `js/scripts/merge-changesets.mjs:133-136`. tpl-js: `scripts/merge-changesets.mjs:132-135`.
   - Repro: two changesets, one with broken frontmatter — the valid one merges, the broken one is skipped with only a warning, so its bump/notes silently vanish from the release.

4. **Published-version check uses `.includes(version)` substring match.**
   - web-search: `js/scripts/check-release-needed.mjs:89` and `publish-to-npm.mjs:189`. tpl-js: `scripts/check-release-needed.mjs:81`.
   - `npm view "pkg@1.2" version` returns a best match (e.g. `1.2.9`), and `"1.2.9".includes("1.2")` is true → can mis-detect a version as already published. Low real-world risk (callers pass exact versions) but a false-positive-prone equality check shared upstream.

5. **Redundant/partial shell-quote escaping in `version-and-commit`.**
   - web-search: `js/scripts/version-and-commit.mjs:269`. tpl-js: `scripts/version-and-commit.mjs:264`.
   - Commit message runs through `replace(/"/g, '\\"')` only (no backtick/`$()` handling) on top of command-stream's own quoting — redundant, and an injection risk if ever passed to a raw shell.

Not bugs (verified correct in templates): the Rust `already_exists` 422 handling
(`tpl-rust/scripts/create-github-release.rs:176-200`) and the Rust CHANGELOG
extraction regex.

---

## Node version status per template

GitHub Actions deprecates the Node 20 *action runtime* (the runtime that
`uses:` JavaScript actions execute on); `checkout@v6` / `setup-node@v6` run on
Node 24. This is separate from the `node-version:` used to run project code.

- **tpl-js** — actions on Node 24 runtime (`checkout@v6`, `setup-node@v6`, `upload-artifact@v7`). Project `node-version`: mostly `24.x` (e.g. `release.yml:155,212,288`), one matrix leg pins `20.x` (`example-app.yml:112`) deliberately.
- **tpl-rust** — `checkout@v6` (Node 24 runtime). No `setup-node` (Rust-only pipeline).
- **tpl-python** — `checkout@v6`, `upload-artifact@v7` / `download-artifact@v7` (Node 24 runtime). No `setup-node`.
- **tpl-csharp** — `checkout@v6`, `upload-artifact@v7` (Node 24 runtime). No `setup-node`.
- **web-search (for contrast)** — `checkout@v4`, `setup-node@v4`, `cache@v4` (Node 20 action runtime, deprecation-prone). Project `node-version` is mixed: `20.x` in most JS jobs (`js.yml:99,182,482`, `parity.yml:23`) and `24.x` in the publish/smoke jobs (`js.yml:328,428`).

Recommendation: bump web-search to `checkout@v6` / `setup-node@v6` / `cache@v5`
(and `create-pull-request@v8`) to match the templates and clear the Node 20
action-runtime deprecation. Standardizing project `node-version` on `24.x`
(keeping any intentional `20.x` compatibility legs) is also advisable.

---

## Recommendation for issue #17 tag format

Issue #17 requests SHORT tags: `rust-0.0.1` and `js-0.0.1` — no `v` prefix, a
single hyphen separator. This **diverges intentionally from all four templates**,
which use `js_v` / `rust_v` / `py_v` / `cs_v` (underscore + `v`).

Status in web-search (already implemented on this branch):

- **JS**: `js/scripts/release-naming.mjs:49-51` — `getTagPrefix()` returns `'js-'` (multi) or `''` (single, bare semver). `buildReleaseTag('1.2.3')` → `js-1.2.3`; single-language → `1.2.3`.
- **Rust**: `.github/workflows/rust.yml:289-296` — multi-language `TAG="rust-$VERSION"` (e.g. `rust-1.2.3`), single-language `TAG="$VERSION"` (bare semver).

This satisfies issue #17. Backward compatibility is preserved on input:
`normalizeVersion()` (`js/scripts/release-naming.mjs:84-95`) still strips legacy
`v`, `js-v`, `js_v`, `rust-v`, `rust_v` prefixes, so old tags remain valid
inputs even though new tags are produced in the short form.

Recommended final convention for issue #17:

- Multi-language repo (web-search layout): `js-<semver>` and `rust-<semver>` (e.g. `js-0.0.1`, `rust-0.0.1`).
- Single-language repo: bare `<semver>` (e.g. `0.0.1`).
- Titles unchanged: `[JavaScript] 0.0.1` / `[Rust] 0.0.1` (multi), `<name> 0.0.1` (single).

Note for the upstream templates: if the link-foundation org wants to standardize
on this shorter, more readable scheme, it would be a coordinated change across
all four `release-naming.*` files (and the C# workflow's reliance on the default
prefix). Until then, web-search's hyphen/no-`v` format is a deliberate local
deviation, not a defect.
