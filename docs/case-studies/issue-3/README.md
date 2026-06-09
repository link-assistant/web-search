# Case Study: Issue #3 — Support both Rust and JavaScript

- **Issue:** [link-assistant/web-search#3](https://github.com/link-assistant/web-search/issues/3)
- **Title:** We need to support both Rust and JavaScript
- **Labels:** documentation, enhancement
- **Author:** @konard
- **Pull Request:** [#4](https://github.com/link-assistant/web-search/pull/4)
- **Status:** In progress
- **Raw issue data:** [`data/issue-3.json`](./data/issue-3.json)

> Note: this folder previously contained leftover template content about an unrelated
> "release formatting script" bug (carried over from a pipeline template). That content
> did **not** describe this repository's issue #3 and has been removed and replaced by
> this case study.

## 1. Issue overview

The issue asks the `web-search` project to become a true **dual-language (Rust +
JavaScript) search API library** that:

1. Adopts best practices from `link-assistant/web-capture` for CI/CD and search, and uses
   `web-capture` as a **component library** so `web-search` can focus on the search API.
2. Ensures `web-capture` exposes everything needed for search API services across all
   protocols/interfaces (browser markup + APIs), so users can try services before buying
   an API.
3. Supports all features required by `link-assistant/formal-ai`, so `web-search` can be
   consumed by `formal-ai`.
4. Reuses best practices from the four `link-foundation` CI/CD pipeline templates (JS,
   Rust, Python, C#) to avoid future CI/CD errors.
5. Compiles the issue's research data into `./docs/case-studies/issue-3`, performs deep
   case-study analysis (including online research), lists every requirement, and proposes
   solution plans for each — checking known existing components/libraries.

## 2. Requirements breakdown

Each requirement is given a stable ID for cross-referencing.

| ID  | Requirement                                                                                                                                                               | Source phrase                                                                                                        |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| R1  | Support **both Rust and JavaScript** as first-class implementations                                                                                                       | issue title + body                                                                                                   |
| R2  | Adopt CI/CD best practices from `web-capture`                                                                                                                             | "support best practices from web-capture … on how CI/CD works"                                                       |
| R3  | Adopt search best practices from `web-capture` and use it as a **component library**                                                                                      | "how search is done, and use web-capture as component library, to focus on search API"                               |
| R4  | Ensure `web-capture` covers all protocols/interfaces for search API services (browser markup + APIs) so users can try before buying                                       | "make sure web-capture has everything we need … all possible protocols, interfaces … browser markup + APIs"          |
| R5  | Support all features required by `formal-ai`                                                                                                                              | "support all features required by formal-ai, so our web-search library can be used at formal-ai"                     |
| R6  | Reuse best practices from the four CI/CD pipeline templates; compare all files to avoid future CI/CD errors                                                               | "Use all the best practices from CI/CD templates …"                                                                  |
| R7  | Compile issue data into `./docs/case-studies/issue-3`, do deep analysis (incl. online research), list all requirements, propose solution plans, check existing components | "collect data … compile … deep case study analysis … list of each and all requirements … propose possible solutions" |
| R8  | Plan and execute everything in this single pull request (#4)                                                                                                              | "Please plan and execute everything in this single pull request"                                                     |

## 3. Research findings

Full reports live under [`research/`](./research):

- [`research/web-capture-analysis.md`](./research/web-capture-analysis.md)
- [`research/formal-ai-analysis.md`](./research/formal-ai-analysis.md)
- [`research/cicd-templates-analysis.md`](./research/cicd-templates-analysis.md)

### 3.1 Current state of this repository

`web-search` **already supports both Rust and JavaScript** (R1 is largely satisfied):

- **JavaScript** — npm package `@link-assistant/web-search` (`src/`): `WebSearchEngine`,
  providers (Google, DuckDuckGo, Bing, browser), merger (`src/merger.js`) with RRF /
  weighted / interleave strategies, REST server, and CLI. 22 tests pass
  (`node --test tests/`).
- **Rust** — crate `web-search` (`rust/`): mirrored providers, merger
  (`rust/src/merger.rs`), Axum server, and CLI. `cargo fmt`/`clippy`/`test`/`build` pass.
- CI: `.github/workflows/release.yml` (JS, mirrors the JS template) and
  `.github/workflows/rust.yml` (Rust).

### 3.2 `web-capture` (R3, R4)

`web-capture` is a polyglot monorepo (npm `@link-assistant/web-capture` + crate
`web-capture`) that **already ships a structured search subsystem** with providers
`wikipedia, duckduckgo, google, bing, brave`, a normalized camelCase result + diagnostics
contract, and three surfaces (library, CLI `web-capture search`, HTTP `/search`). It
covers all three capture strategies the issue asks about (R4): **browser automation**
(Playwright/Puppeteer/browser-commander over CDP), **HTTP scraping** (reqwest/node-fetch +
scraper/cheerio), and **API-based capture** (GitHub, Google Docs, Wikipedia, StackOverflow,
xpaste). See the full report for the exact contract and gaps.

### 3.3 `formal-ai` (R5)

`formal-ai` already implements web search internally in `src/web_search_core.rs`
(`no_std` + `alloc`): a typed **provider registry** across `Search | Knowledge | Papers |
Code`, **Reciprocal Rank Fusion with k = 60**, concurrency caps (5/category, top-10/
provider), a deterministic tab-separated WASM↔JS wire format, and evidence/trace hooks.
For `web-search` to be usable by `formal-ai`, it must match this contract. The current
`web-search` RRF `k = 60` already matches; the registry/categories, extra knowledge
providers, `no_std` core, wire format, and trace hooks do not yet exist.

### 3.4 CI/CD templates (R2, R6)

This repo already adopts most of the **JS** template (detect-changes gating, version-check,
changeset-check, fresh-merge simulation, jscpd duplication check, node/bun/deno × 3-OS
matrix, npm OIDC trusted publishing). The **Rust** workflow is leaner than the Rust
template. All four templates lack community-health files (dependabot, CODEOWNERS,
SECURITY.md, issue/PR templates).

## 4. Existing components / libraries to reuse

| Need                                  | Existing component                                                           | Notes                                                |
| ------------------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------------------- |
| Web capture across protocols (R3, R4) | `link-assistant/web-capture` `search` module                                 | normalized contract + 3 surfaces, dual npm/crates.io |
| RRF fusion contract (R5)              | `formal-ai` `web_search_core.rs` (`reciprocal_rank_fusion`, k=60)            | reference shape to match                             |
| CI/CD pipelines (R2, R6)              | the four `link-foundation` `*-ai-driven-development-pipeline-template` repos | proven workflow + scripts                            |
| Versioning/release (R6)               | `@changesets/cli` (JS), `changelog.d/` fragments (Rust template)             | already used on the JS side here                     |
| Duplication / quality gates           | `jscpd`, ESLint, Prettier, rustfmt, clippy                                   | already wired                                        |

## 5. Proposed solutions and solution plans (per requirement)

### R1 — Support both Rust and JavaScript ✅ (done; verified in this PR)

Both implementations exist, build, and test green. Plan: keep parity by mirroring every
new feature in both `src/` and `rust/src/`, and keep both covered in CI.

### R2 / R6 — CI/CD best practices

- **Done in this PR (Rust side, low-risk):** `Cargo.toml` now sets
  `[lints.rust] unsafe_code = "forbid"` (matches the 100% safe codebase and the
  `RUSTFLAGS: -Dwarnings` CI gate) and an optimized `[profile.release]`
  (`lto`/`codegen-units`/`strip`), both straight from the Rust template.
- **Deferred (tracked here):**
  - Full `[lints.clippy]` pedantic/nursery block — currently produces 73 warnings that
    would fail `-Dwarnings`; adopt incrementally with the template's allow-list, fixing
    warnings in small batches.
  - crates.io publish automation + `wait-for-crate`, coverage (`cargo-llvm-cov` →
    Codecov), crate-size guard + `include` allowlist, and `cargo doc` → GitHub Pages —
    these require new secrets/scripts and are best landed as focused follow-up PRs.
  - Community-health files (dependabot, CODEOWNERS, SECURITY.md, issue/PR templates) — a
    gap shared by all four templates.

### R3 — Use `web-capture` as a component library

Plan: depend on `@link-assistant/web-capture` (npm) and the `web-capture` crate, and have
`web-search` providers delegate single-provider fetching to `web-capture`'s `search`
module, reusing its normalized result + diagnostics contract. `web-search` keeps ownership
of the **aggregation/reranking** layer (merger strategies) and the multi-provider API. This
removes duplicated scraping/parsing logic from this repo.

### R4 — Ensure `web-capture` covers all protocols/interfaces

Finding: `web-capture` already covers browser automation + HTTP scraping + API-based
capture. Plan: file follow-up issues in `web-capture` only for concrete gaps surfaced when
wiring R3 (e.g. `captureMode: "browser"` for SERPs to bypass CAPTCHA walls, paging/offset).
No `web-capture` changes are made in this PR (out of scope for a single web-search PR).

### R5 — Support all features required by `formal-ai`

Plan (incremental, both languages):

1. Introduce a typed **provider registry** (`id, label, category, cors_readable,
default_for_category`) over `Search | Knowledge | Papers | Code`, mirroring
   `web_search_core.rs`.
2. Add knowledge/papers/code providers (wikipedia, wikidata, wiktionary, wikinews,
   internet-archive, openalex, crossref, arxiv, github) alongside the existing search
   engines.
3. Keep RRF at `k = 60` (already matches) and add the deterministic tab-separated wire
   format + 6-decimal score formatter for the WASM↔JS boundary.
4. Add evidence/trace hooks (`web_search:request:*`, `web_search:provider:*`,
   `web_search:combined:rrf:k=60`).
5. Provide a `no_std` + `alloc` core so the Rust crate can be reused by formal-ai's WASM
   worker.

These are the rows of the gap table in
[`research/formal-ai-analysis.md`](./research/formal-ai-analysis.md).

### R7 — Case study ✅ (this document)

Issue data compiled to [`data/issue-3.json`](./data/issue-3.json); deep analysis with
online/repo research captured in [`research/`](./research); all requirements listed (§2)
with solution plans (§5) and existing components (§4).

### R8 — Single pull request ✅

All work lands in PR #4 on branch `issue-3-8d54556db4de`.

## 6. What this PR changes

1. Replaces the incorrect leftover case-study content with this analysis plus the three
   research reports and the real issue data.
2. Hardens `rust/Cargo.toml` with two low-risk Rust-template best practices
   (`unsafe_code = "forbid"`, optimized release profile); verified with
   `cargo fmt --check`, `cargo clippy --all-targets --all-features` under
   `RUSTFLAGS=-Dwarnings`, `cargo test`, and `cargo build --release`.
3. Documents the remaining, larger items (R3/R4/R5 implementation, full CI/CD parity) as
   explicit, scoped follow-up plans above, so they can be executed as focused PRs without
   re-doing the research.

## 7. References

- Issue: <https://github.com/link-assistant/web-search/issues/3>
- `link-assistant/web-capture`: <https://github.com/link-assistant/web-capture>
- `link-assistant/formal-ai`: <https://github.com/link-assistant/formal-ai>
- JS template: <https://github.com/link-foundation/js-ai-driven-development-pipeline-template>
- Rust template: <https://github.com/link-foundation/rust-ai-driven-development-pipeline-template>
- Python template: <https://github.com/link-foundation/python-ai-driven-development-pipeline-template>
- C# template: <https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template>
- Reciprocal Rank Fusion (Cormack et al., 2009): <https://plg.uwaterloo.ca/~gvcormac/cormacksigir09-rrf.pdf>
