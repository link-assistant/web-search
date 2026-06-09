# Research: `link-assistant/formal-ai` web-search requirements

> Source: direct clone and inspection of `link-assistant/formal-ai` (default branch,
> release `v0.182.0`), performed while researching issue #3. Nothing in that repo was
> modified.

## Summary

`formal-ai` is a **symbolic, deterministic AI assistant implemented primarily in Rust**
(crate `formal-ai`), with a JS browser demo (bundled with Bun), a Rust→WASM worker, an
Electron desktop app, and a VS Code extension. It exposes OpenAI-shaped interfaces with no
neural-network inference.

Crucially, **web search is a first-class, fully implemented capability inside `formal-ai`
itself** — not an external dependency. So for our `web-search` library to be "usable at
formal-ai," it must match the contract that formal-ai already defines internally, so it can
become a drop-in refactor target rather than a missing piece.

## The contract web-search must satisfy

The shared symbolic core lives in `src/web_search_core.rs` (`no_std` + `alloc`,
`#![forbid(unsafe_code)]`). Key constants and shapes:

```rust
pub const WEB_SEARCH_RRF_K: u32 = 60;                       // Reciprocal Rank Fusion k
pub const WEB_SEARCH_CONCURRENCY_PER_CATEGORY: u32 = 5;
pub const WEB_SEARCH_PROVIDER_LIMIT: u32 = 10;              // top-10 per provider
pub const WEB_SEARCH_PROVIDER_REGISTRY: &[ProviderSpec] = &[ /* duckduckgo, google,
    bing, brave, yahoo, yandex, ecosia, mojeek, startpage, wikipedia, wikidata,
    wiktionary, wikinews, internet-archive, openalex, crossref, arxiv, github, ... */ ];
pub const WEB_SEARCH_PROVIDERS: &[&str] =                   // live CORS-readable plan
    &["duckduckgo","internet-archive","wikipedia","wikidata","wiktionary","wikinews"];
pub fn reciprocal_rank_fusion(entries: &[ProviderRanking], k: u32) -> Vec<FusedEntry>;
```

To be usable by formal-ai, a web-search library would need to provide:

1. **A provider registry** of typed specs —
   `ProviderSpec { id, label, category, cors_readable, default_for_category }` over
   categories `Search | Knowledge | Papers | Code`. Order-significant: defaults first.
2. **A default plan** — `default_search_plan_ids() -> Vec<String>` returning the CORS-safe
   provider list.
3. **Per-provider query execution** returning ranked results. In the browser the contract
   is `searchX(query, language, limit) -> ranked list` and
   `fetchProviderJson(providerId, url, options) -> { ok, status, data, finalUrl }` with
   CORS/network error classification. Endpoints must be **keyless, CORS-readable JSON** to
   run in-browser.
4. **Reciprocal Rank Fusion merge** —
   `reciprocal_rank_fusion(entries: &[ProviderRanking{provider_id, rank, url, title,
excerpt}], k=60) -> Vec<FusedEntry{url, title, excerpt, score, providers}>`,
   deterministic with a defined tie-break (score, then provider count).
5. **Concurrency control** — cap of 5 providers per category, top-10 per provider.
6. **A deterministic line/tab wire format** for the WASM↔JS boundary:
   `provider_id\trank\turl\ttitle\texcerpt`, with a fixed 6-decimal score formatter so
   Rust and JS agree byte-for-byte.
7. **Evidence/trace hooks** — `build_request_evidence(query, language) -> Vec<String>`
   emitting `web_search:request:*`, `web_search:provider:*`,
   `web_search:combined:rrf:k=60` lines.
8. **`no_std` + `alloc` compatibility** so it can be reused by the WASM crate.
9. **Language awareness** — per-language host selection / labels (e.g. `*.wikipedia.org`).

## Where it is used inside formal-ai

- Intent recognition: `src/solver_handlers/web_search_intent.rs`.
- Offline symbolic trace: `src/solver_handlers/web_requests.rs` (`try_web_search`,
  `try_http_fetch`).
- WASM bridge: `src/web/wasm-worker/src/lib.rs` re-exports `web_search_fuse`,
  `web_search_plan`, `web_search_rrf_k`, `web_search_registry_dump`.
- Live browser fetch: `src/web/formal_ai_worker.js` (`fetchProviderJson`,
  `searchDuckDuckGo`, `searchWikipediaPages`, `wikidataSearchEntity`, …).
- Connectivity dashboard: `src/web/tests/connectivity.js`.
- Data-driven lexicon: `data/seed/meanings-web-search.lino` (the `web_search` concept and
  its surface forms across en/ru/hi/zh).

## Gap vs. current web-search implementation

| formal-ai expectation                                                                     | current web-search (this repo)                                              |
| ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| RRF with `k = 60`                                                                         | ✅ implemented in both JS (`src/merger.js`) and Rust (`rust/src/merger.rs`) |
| Provider registry w/ categories + `cors_readable` flag                                    | ❌ flat list `['google','duckduckgo','bing']`                               |
| Knowledge/Papers/Code providers (wikipedia, wikidata, arxiv, openalex, crossref, github…) | ❌ only Google/DuckDuckGo/Bing/browser                                      |
| `no_std` + `alloc` core for WASM reuse                                                    | ❌ Rust core uses `std`/tokio                                               |
| Deterministic tab-separated wire format + 6-decimal score                                 | ❌ not present                                                              |
| Evidence/trace hooks (`web_search:*`)                                                     | ❌ not present                                                              |

These rows form the backlog for "support all features required by formal-ai" and are
captured as solution plans in the case study `README.md`.

## Dependency manifests

`formal-ai`'s `Cargo.toml` already depends on several link-ecosystem crates from crates.io
(`lino-arguments`, `lino-objects-codec`, `link-calculator`, `doublets`, `platform-mem`),
so consuming a published `web-search` crate would fit its existing model — provided
web-search ships a `no_std`-compatible core and a matching RRF contract.
