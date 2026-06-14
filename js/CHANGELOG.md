# Changelog

## 0.10.2

### Patch Changes

- a2089ed: Make GitHub release tags short and self-heal stuck JavaScript releases (issue #17):
  - Shorten the JavaScript release tag to `js-<version>` in multi-language repos and a bare `<version>` in single-language repos: no `v` prefix and a single `-` separator, replacing the previous `js_v<version>` spelling. `normalizeVersion` and `buildReleaseTag` accept every legacy spelling so existing releases keep resolving, and the shared `release-naming.mjs` helper centralizes the convention with unit tests.
  - Self-heal a release that bumped `package.json` but never published: when `version-and-commit.mjs` finds nothing to commit (the changeset was already consumed by a prior failed run), it now signals `already_released` so the idempotent npm publish step still verifies the registry and publishes the missing version, instead of leaving it stuck on `main` (observed as `0.10.1`).
  - Bump pinned GitHub Actions to versions whose runtime is Node 20+, clearing the deprecation warnings on the release workflows.

## 0.10.1

### Patch Changes

- f8f3b1e: Fix multi-language release metadata so JavaScript releases are unambiguous and verifiable (issue #15):
  - Namespace the git tag as `js_v<version>` in multi-language repos (was `js-v<version>`) and keep a plain `v<version>` tag in single-language repos, auto-detected from the package layout.
  - Title GitHub releases `[JavaScript] <version>` in multi-language repos so they no longer collide with `[Rust] <version>` releases in the shared list.
  - Link the npm shields.io badge to the exact published version page (`/package/<pkg>/v/<version>`), fixing a normalization bug that previously produced a `js-v…` link.
  - Centralize the tag/title/badge conventions in a shared `release-naming.mjs` helper with unit tests.
  - Import Node built-ins via the `node:` specifier in `js-paths.mjs` so the Deno test job type-checks the new helper's import graph (bare `fs`/`path` imports failed Deno's `TS2307 not a dependency` check).

## 0.10.0

### Minor Changes

- f4cc1c5: Make `web-search` reliably consumable as a published package (issue #6,
  FormalAI adoption blocker).
  - Add a `serve` subcommand to the CLI (`web-search serve --port <port>`) as a
    parity alias for the existing `--serve` flag, matching the Rust CLI so both
    ecosystems share the same HTTP-server entry-point syntax.
  - Self-healing JS release detection (`check-release-needed.mjs`): when
    `package.json` is ahead of npm but no changeset exists (e.g. a previous
    publish failed after consuming its changeset), the next push republishes the
    unpublished version instead of silently skipping.
  - Install-from-package smoke test (`smoke-test-package.mjs`) run after publish:
    installs the released package from npm into a clean project and verifies the
    library, CLI, and HTTP-server entry points actually work.
  - Document the library, CLI, and HTTP-server entry points in the README with
    versioned npm/cargo install commands.

## 0.9.0

### Minor Changes

- d54823c: Align the provider registry with FormalAI's `web_search_core` (issue #5): the
  catalog is now a documented superset of FormalAI's provider IDs (40 providers),
  the default category for `search` is DuckDuckGo, and the live default plan is
  `duckduckgo`, `internet-archive`, `wikipedia`, `wikidata`, `wiktionary`,
  `wikinews`. Adds a shared compatibility map and JS/Rust provider-parity tests.

## 0.8.2

### Patch Changes

- d9f20c9: Fix npm release CI/CD false positives: exit immediately with actionable guidance on permanent publish failures (E404/E401/E403) instead of retrying, add an optional `NPM_TOKEN` bootstrap fallback for the first publish of the package, make GitHub Release creation idempotent, and adopt the multi-strategy npm upgrade from the pipeline template.

## 0.8.1

### Patch Changes

- 9c6864f: Fix npm publish metadata, restrict published package assets, and use `js-v*` GitHub release tags.

## 0.8.0

### Minor Changes

- 9838960: Move the JavaScript package into `js/`, split JavaScript/Rust CI workflows, and add layout plus provider parity checks.

## 0.7.0

### Minor Changes

- bd74868: Support both Rust and JavaScript as first-class implementations with full parity (issue #3).
  - Add a descriptor-driven engine catalog and a single shared `GenericProvider`, plus shared HTML utilities (entity decoding, tag stripping, generic anchor-list parser), in both languages.
  - Add a typed provider registry over the four `formal-ai` categories (`search`, `knowledge`, `papers`, `code`) powering discovery (`--list-providers`, `/providers`, `/providers?category=`, `/categories`) and instantiation. Both languages now report the same 22 providers (search 15, knowledge 2, papers 3, code 2).
  - Add knowledge/papers/code and extra search providers: wikipedia, wikidata, searx, crossref, openalex, github, hackernews, arxiv, brave, mojeek, ecosia, startpage, yahoo and DuckDuckGo Lite.
  - Integrate `@link-assistant/web-capture` as an optional component library via `wc:*` providers; it loads lazily and degrades gracefully when absent.
  - Align `decodeHtmlEntities` across languages (decode `&hellip;`, `&mdash;`, `&ndash;`).
  - Expand the test suites (115 JS tests; Rust integration tests for the registry, HTML utilities, and every parser) and document the catalog, categories, web-capture component, and registry in the README and the issue #3 case study.

## 0.6.0

### Minor Changes

- 03e7911: Add web search microservice with multi-provider aggregation

  **Features:**
  - Multi-provider search aggregation (Google, DuckDuckGo, Bing)
  - Multiple merge strategies: Reciprocal Rank Fusion (RRF), weighted scoring, interleaving
  - Configurable provider weights for reranking
  - URL normalization for proper deduplication across providers
  - API-first design with fallback to web scraping
  - browser-commander integration for direct browser search testing

  **JavaScript Library:**
  - Search provider interfaces with API support and scraping fallback
  - Result merger with RRF, weighted, and interleave strategies
  - WebSearchEngine class for multi-provider search
  - Express.js REST API server
  - CLI tool for command-line usage

  **Rust Library:**
  - Async search providers using reqwest and scraper
  - Result merger with same strategies as JavaScript version
  - WebSearchEngine with async search
  - Axum REST API server
  - CLI tool with clap

  **REST API Endpoints:**
  - GET /search?q=<query> - Search all providers
  - POST /search - Search with options in body
  - GET /search/:provider?q=<query> - Search single provider
  - GET /providers - List available providers
  - GET /health - Health check

  **CI/CD:**
  - Added rust.yml workflow for Rust CI (lint, test matrix, build)

## 0.5.0

### Minor Changes

- 66211b5: Add fresh merge simulation to CI/CD to prevent stale merge preview issues
  - Add "Simulate fresh merge with base branch" step to lint and test jobs
  - This ensures PR CI validates the actual merge result, not a stale snapshot
  - Prevents CI failures on main branch after merging PRs that sat open for days
  - Add case study documentation for issue #23 with root cause analysis
  - Add ignore patterns for case study data files in ESLint and Prettier

  See docs/case-studies/issue-23 for detailed analysis of the stale merge preview problem.

  Fixes #23

## 0.4.0

### Minor Changes

- e6c2691: Add multi-language repository support for CI/CD scripts
  - Add `scripts/js-paths.mjs` utility for automatic JavaScript package root detection
  - Support both `./package.json` (single-language) and `./js/package.json` (multi-language repos)
  - Add `--legacy-peer-deps` flag to npm install commands in release scripts to fix ERESOLVE errors
  - Save and restore working directory after `cd` commands to fix `command-stream` library's `process.chdir()` behavior
  - Add case study documentation with root cause analysis in `docs/case-studies/issue-21/`

## 0.3.0

### Minor Changes

- 80d9c84: Add CI check to prevent manual version modification in package.json
  - Added `check-version.mjs` script that detects manual version changes in PRs
  - Added `check-changesets.mjs` script to check for pending changesets (converted from inline shell)
  - Added `version-check` job to release.yml workflow
  - Automated release PRs (changeset-release/_ and changeset-manual-release-_) are automatically skipped

## 0.2.2

### Patch Changes

- 9a12139: Fix CI/CD check differences between pull request and push events

  Changes:
  - Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
  - Make lint job independent of changeset-check (runs based on file changes only)
  - Allow docs-only PRs without changeset requirement
  - Handle changeset-check 'skipped' state in dependent jobs
  - Exclude `.changeset/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

## 0.2.1

### Patch Changes

- 55aef41: Make Bun the primary runtime choice throughout the template
  - Update all shebangs from `#!/usr/bin/env node` to `#!/usr/bin/env bun` in scripts, experiments, and case studies
  - Update README.md to prioritize Bun in all sections (features, development, runtime support, package managers, scripts reference)
  - Update examples to list Bun first
  - Bun now described as "Primary runtime with highest performance" and "Primary choice" for package management
  - Maintains full compatibility with Node.js and Deno

## 0.2.0

### Minor Changes

- d3f7fcd: Improve changeset CI/CD robustness for concurrent PRs
  - Update validate-changeset.mjs to only check changesets ADDED by the current PR (not pre-existing ones)
  - Add merge-changesets.mjs script to combine multiple pending changesets during release
  - Merged changesets use highest version bump type (major > minor > patch) and combine descriptions chronologically
  - Update release workflow to pass SHA environment variables and add merge step
  - Add comprehensive case study documentation for the CI/CD improvement
  - This prevents PR failures when multiple PRs merge before a release cycle completes

## 0.1.4

### Patch Changes

- e9703b9: Add ESLint complexity rules with reasonable thresholds

## 0.1.3

### Patch Changes

- 0198aaa: Add case study documentation comparing best practices from effect-template

  This changeset adds comprehensive documentation analyzing best practices from
  ProverCoderAI/effect-template repository, identifying gaps in our current setup,
  and providing prioritized recommendations for improvements.

  Key findings include missing best practices like code duplication detection (jscpd),
  ESLint complexity rules, VS Code settings, and test coverage thresholds.

## 0.1.2

### Patch Changes

- 2ea9b78: Enforce strict no-unused-vars ESLint rule without exceptions. All unused variables, arguments, and caught errors must now be removed or used. The `_` prefix no longer suppresses unused variable warnings.

## 0.1.1

### Patch Changes

- 042e877: Fix GitHub release formatting to support Major/Minor/Patch changes

  The release formatting script now correctly handles all changeset types (Major, Minor, Patch) instead of only Patch changes. This ensures that:
  - Section headers are removed from release notes
  - PR detection works for all release types
  - NPM badges are added correctly

## 0.1.0

### Minor Changes

- 65d76dc: Initial template setup with complete AI-driven development pipeline

  Features:
  - Multi-runtime support for Node.js, Bun, and Deno
  - Universal testing with test-anywhere framework
  - Automated release workflow with changesets
  - GitHub Actions CI/CD pipeline with 9 test combinations
  - Code quality tools: ESLint + Prettier with Husky pre-commit hooks
  - Package manager agnostic design

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
