# Research: `link-assistant/web-capture` as a component library

> Source: direct clone and inspection of `link-assistant/web-capture` (default branch),
> performed while researching issue #3. Nothing in that repo was modified.

## Summary

`web-capture` is a **polyglot monorepo** with two parallel, API-compatible
implementations:

- **JavaScript** — npm package `@link-assistant/web-capture` (`js/`), Express server +
  CLI, Node `>=22 <23`.
- **Rust** — crate `web-capture` (`rust/`), Axum server + CLI, both `[lib]` and `[[bin]]`.

Both are released independently (npm + crates.io) through a shared set of Node `.mjs`
release scripts in `scripts/`. Each ships a `Dockerfile` exposing port 3000.

Its tagline is "capture the web in required format": fetch a URL and render it as
Markdown, HTML, plain text, PNG/JPEG screenshot, ZIP archive, PDF, or DOCX.

## Why it matters for web-search

`web-capture` **already contains a first-class structured search subsystem** (its issue
#130), implemented identically in both languages:

- `rust/src/search.rs`, `js/src/search.js`
- Tests: `rust/tests/integration/search.rs`, `js/tests/unit/search.test.js`,
  `js/tests/integration/search-endpoint.test.js`

This is the most directly reusable foundation for our search API.

### Providers

`SEARCH_PROVIDERS = ["wikipedia", "duckduckgo", "google", "bing", "brave"]`
(default `wikipedia`, default limit `10`). Wikipedia uses its REST JSON API; the others
are best-effort HTML scrapes with CAPTCHA/bot-block detection surfaced via
`diagnostics.blockedByCaptcha`.

### Normalized result contract (camelCase JSON, identical across JS & Rust)

```json
{
  "query": "...",
  "provider": "...",
  "captureMode": "fetch",
  "capturedAt": "...",
  "results": [{ "rank": 1, "title": "...", "url": "...", "snippet": "..." }],
  "diagnostics": {
    "status": "...",
    "blockedByCors": false,
    "blockedByCaptcha": false,
    "sourceUrl": "...",
    "error": null
  }
}
```

### Three consumption surfaces already wired

1. **Library** — `search(...)`, plus a pure, network-free `parseSearchResults` /
   `parse_search_results` parser that is testable against fixtures.
2. **CLI** — `web-capture search <query> --provider <p> --limit <n>`.
3. **HTTP** — `GET /search?q=&provider=&limit=&format=json|markdown`, with
   `formatSearchAsMarkdown` rendering results as Markdown.

### Design points useful for building on top

- Injectable `fetchImpl` and `now` clock (JS) / injected `captured_at` (Rust) for
  deterministic tests.
- Transport failures are recorded in `diagnostics` rather than thrown, so the contract
  never partially fails.

### Gaps to be aware of when building on it

- No paging/offset (only `limit`).
- HTML-engine results are fragile (selector-based, easily blocked).
- No result dedup / cross-provider ranking-merge (this is exactly what web-search adds).
- `captureMode` exists but only `"fetch"` is implemented — browser-mode search would
  need wiring through `browser.rs` / `browser.js`.

## Browser markup + APIs (protocols/interfaces)

`web-capture` uses all three capture strategies, which the issue asks us to make sure are
available for "trying out services before buying an API":

1. **Browser automation (real headless Chromium)** — JS depends on both `puppeteer` and
   `playwright` plus `browser-commander` (engine selectable); Rust uses `browser-commander`
   (chromiumoxide over the Chrome DevTools Protocol / WebSocket).
2. **Plain HTTP scraping** — `reqwest` (Rust) / `node-fetch` (JS), parsed with `scraper`
   (Rust) / `cheerio` (JS).
3. **API-based capture** — native provider APIs: GitHub REST API, Google Docs export API,
   Wikipedia REST JSON API, StackOverflow via StackPrinter, xpaste raw endpoint.

## CI/CD highlights (to compare with this repo)

Two workflows: `.github/workflows/js.yml` and `.github/workflows/rust.yml`.

- Path-based `detect-changes` gating; concurrency with main-only cancellation.
- JS: changesets + **npm OIDC trusted publishing** (one workflow file constraint),
  hardened Playwright/Puppeteer Chromium installation, gated live integration tests.
- Rust: `RUSTFLAGS: -Dwarnings`, `cargo fmt --check` + `cargo clippy --all-targets
--all-features`, **3-OS test matrix** + doc tests, `cargo build --release` +
  `cargo package --list` + Docker build, **crates.io publish** with self-healing
  auto-bump, cargo registry caching.

## Integration recommendation for web-search

The cleanest path is to treat `web-capture`'s `search` module as the **capture/provider
layer** and have web-search focus on the **aggregation/reranking layer**:

1. Reuse `web-capture`'s normalized result + diagnostics contract so results flow through
   unchanged.
2. Optionally route HTML-engine queries through `web-capture`'s browser-automation layer
   to defeat CAPTCHA walls (the `captureMode: "browser"` path).
3. Reuse the dual npm/crates.io release machinery patterns already proven there.
