# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- changelog-insert-here -->

## [0.3.0] - 2026-06-14

### Fixed

- Send a descriptive `User-Agent` on the crates.io pre-flight existence check so already-published versions are detected (crates.io returns HTTP 403 without one), eliminating false-positive republish attempts on no-op pushes.
- Read the crates.io token from `CARGO_REGISTRY_TOKEN` with a `CARGO_TOKEN` fallback, and delegate publishing to `publish-crate.rs` with failure classification and deferred rate-limit handling.
- Make GitHub Release creation idempotent so re-runs over an existing tag stay green.

### Added

- Expand the provider registry to a documented superset of FormalAI's `web_search_core` IDs (40 providers): new `knowledge` engines (wiktionary, wikinews, internet-archive, dbpedia, openlibrary, semantic-scholar, openalex, and four dictionaries), `papers` engines (europepmc, doaj), and `code` engines (gitlab, codeberg, gitee, bitbucket, gitflic), plus the `yandex` search engine.
- Add a `formal_ai_compat` integration suite that reads the shared `docs/case-studies/issue-5/formal-ai-compatibility.json` map and enforces FormalAI parity in lockstep with the JavaScript suite.

### Changed

- Default the `search` category to DuckDuckGo and return FormalAI's live default plan (`duckduckgo`, `internet-archive`, `wikipedia`, `wikidata`, `wiktionary`, `wikinews`) from `get_default_provider_ids`.

### Changed
- Added Rust-local CI/CD guard scripts and workflow checks for the split JavaScript/Rust repository layout.

### Fixed

- Commit `Cargo.lock` (it was `.gitignore`d) so the binary crate builds from a pinned, reproducible dependency graph. With the lockfile unpinned, a freshly published `alloc-no-stdlib 3.0.0` split the `brotli` encoder's allocator types (`brotli` resolved `alloc-no-stdlib 2.0.4` for its own code while `alloc-stdlib` pulled `3.0.0`), producing `error[E0277]: StandardAlloc: alloc::Allocator<ZopfliNode> is not satisfied` on fresh CI resolution. The same commit built one moment and failed the next with no source change; the committed, unified-on-`2.0.4` lockfile makes the Rust build deterministic again (issue #15).
- Stop the published-crate smoke test from failing on a healthy CLI: it piped `web-search --list-providers` straight into `head`, which closed the pipe and made the binary abort with `failed printing to stdout: Broken pipe` (exit 101) under `set -o pipefail`. The step now captures the output once and paginates the captured text, removing the false-positive release failure (issue #15).
- Make `web-search --list-providers` itself broken-pipe-safe: the listing is rendered into a single buffer and written in one fallible call so a closed reader (`| head`) results in a clean exit instead of a panic.

### Changed

- Auto-detect single-language vs multi-language repository layout for the Rust release. Multi-language repos now tag releases `rust_v<version>` (was `rust-v<version>`) and title them `[Rust] <version>`; single-language repos use a plain `v<version>` tag and `<crate> <version>` title.
- Add a crates.io shields.io badge to the GitHub release body that links to the exact published version's crate page.

### Added

- Auto-bump the crate version from `changelog.d/` fragments at release time
  (issue #17). The Rust release job previously had no version-bump step, so the
  crate version never advanced past its initial value and no new crates.io or
  GitHub releases were ever produced. New `get-bump-type.rs`, `bump-version.rs`
  and `collect-changelog.rs` scripts (mirroring the JavaScript changeset flow and
  the link-foundation rust template) now read the fragments, pick the highest
  declared bump, rewrite `Cargo.toml`, fold the fragments into `CHANGELOG.md`, and
  commit the result back to `main` before publishing — the same pattern the
  JavaScript side already uses.

### Changed

- Shorten the Rust release tag to `rust-<version>` in multi-language repos and a
  bare `<version>` in single-language repos (issue #17): no `v` prefix and a
  single `-` separator, replacing the previous `rust_v<version>` / `rust-v<version>`
  spellings. `[Rust] <version>` release titles and the crates.io badge are
  unchanged.

### Fixed

- Unify the duplicated `alloc-no-stdlib` major version in the published-crate
  library smoke test (issue #17). A fresh `cargo add web-search` resolve pulled
  both `alloc-no-stdlib` 2.0.4 (required by `brotli`) and 3.0.0 (pulled by
  `brotli-decompressor`); with both majors present, `StandardAlloc` from 2.0.4 no
  longer satisfied `brotli`'s `Allocator` bound from 3.0.0, so the downstream
  consumer failed to compile with `error[E0277]`. The smoke test now collapses the
  3.0.0 instance to 2.0.4 (a no-op once upstream realigns), proving the published
  crate is installable from a clean resolve.

### Changed

- Wire the Rust `wc:*` provider namespace to the published `web-capture` crate and add release automation for crates.io plus `rust-v*` GitHub releases.

