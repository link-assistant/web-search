# Web Search (Rust)

[![crates.io crate](https://img.shields.io/crates/v/web-search?label=crates.io)](https://crates.io/crates/web-search)
[![crate downloads](https://img.shields.io/crates/d/web-search?label=downloads)](https://crates.io/crates/web-search)
[![docs.rs](https://img.shields.io/docsrs/web-search?label=docs.rs)](https://docs.rs/web-search)
[![Rust CI](https://github.com/link-assistant/web-search/actions/workflows/rust.yml/badge.svg)](https://github.com/link-assistant/web-search/actions/workflows/rust.yml)
[![Rust release tag](https://img.shields.io/badge/GitHub%20release-rust--v0.2.0-orange)](https://github.com/link-assistant/web-search/releases?q=rust-v)

Rust implementation of the `web-search` library, CLI, and HTTP service. It
mirrors the JavaScript package in `../js` with the same 40-provider catalog,
provider categories, merge strategies, and discovery surface.

## Install

```bash
cargo install web-search
```

As a library:

```toml
[dependencies]
web-search = "0.5"
```

From a local checkout before a crates.io release is visible:

```toml
[dependencies]
web-search = { path = "../web-search/rust" }
```

For deterministic result merging without providers, the CLI/server, or their
network and browser dependencies:

```toml
[dependencies]
web-search = { version = "0.6", default-features = false, features = ["merge"] }
```

This merge-only configuration excludes Axum, Reqwest, Tokio, `web-capture`,
and the native-TLS/OpenSSL dependency graph. With the server feature disabled,
this crate uses `no_std` + `alloc`. `SearchResult`, `MergeOptions`, all three
`MergeStrategy` variants, `merger::normalize_url`, and provider discovery remain
available. The core API also stays available with no features, preserving the
existing merge-only installation.

Use `alloc::collections::BTreeMap<String, Vec<SearchResult>>` for provider lists.
The same merge functions accept the server's `std::collections::HashMap`.
`MergeOptions::with_weights` accepts either map; its public `weights` field
retains a HashMap with `server` and uses BTreeMap without it. Providers, tied URLs,
and contributing sources are ordered deterministically in both builds.

`web_search::registry::PROVIDER_REGISTRY` exposes all 40 providers as static
metadata, including endpoint/body templates and capabilities. `get_registry`,
`get_provider_ids`, `get_default_provider_ids`, and `is_known_category` are also
available at the crate root. Replace `{query}` with percent-encoded query text,
`{language}` with a language code and `{limit}` with a bounded result count.
Hybrid entries describe their HTML fallback; component entries describe upstream
endpoints and use web-capture for retrieval. Capabilities describe HTTP method,
API/HTML support, optional credentials, and component backing. The catalog does
not perform HTTP requests; WASM callers supply their own transport.

The `merge` feature is prepared for the next release (0.6). See
[the no_std consumer](examples/no-std-consumer) and the three-provider
ranking tests in `tests/merge_core.rs`.

```sh
cargo build --no-default-features --features merge --target wasm32-unknown-unknown
```

## Library

```rust
use web_search::{MergeOptions, MergeStrategy, SearchOptions, WebSearchEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = WebSearchEngine::new();

    let results = engine
        .search_with_options(
            "graph neural networks",
            SearchOptions {
                limit: Some(10),
                ..Default::default()
            },
            Some(vec!["arxiv".to_string(), "crossref".to_string()]),
            Some(MergeOptions {
                strategy: MergeStrategy::Rrf,
                ..Default::default()
            }),
        )
        .await?;

    for result in results {
        println!("{} {}", result.title, result.url);
    }

    Ok(())
}
```

### Caller-owned transport and detailed outcomes

Implement `SearchTransport` for a cache or HTTP stack, then pass it to the
detailed API. Each `ProviderOutcome` retains an independent status/error and
the exact `TransportResponse` bytes (plus an optional opaque cache receipt).

```rust,ignore
use std::sync::Arc;
use web_search::{SearchOptions, SearchTransport, WebSearchEngine};

let transport: Arc<dyn SearchTransport> = Arc::new(MyCachedTransport::new());
let detailed = engine
    .search_detailed_with_options(
        "graph neural networks",
        SearchOptions::default(),
        Some(vec!["arxiv".into(), "github".into()]),
        None,
        transport,
    )
    .await;
```

Provider work is polled within the returned aggregate future instead of being
detached with `tokio::spawn`; dropping that future therefore drops all in-flight
requests. Single-provider callers can use `search_single_with_transport`, and
provider implementations expose `SearchProvider::search_with_transport`.

## CLI

```bash
web-search "rust async search" --limit 10
web-search "transformer architecture" --providers arxiv,crossref --format json
web-search --list-providers
```

## HTTP Service

```bash
web-search serve --port 3000

curl "http://localhost:3000/search?q=rust+programming&limit=10"
curl "http://localhost:3000/providers?category=papers"
curl "http://localhost:3000/categories"
```

## Providers

The live registry has 40 providers in four categories. The full catalog is
available through `get_registry()` in every feature configuration:

| Category    | Provider ids                                                                                                                                                                                                                   |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `search`    | `google`, `bing`, `duckduckgo`, `searx`, `brave`, `mojeek`, `ecosia`, `startpage`, `yahoo`, `yandex`, `lite`, `wc:wikipedia`, `wc:duckduckgo`, `wc:google`, `wc:bing`, `wc:brave`                                              |
| `knowledge` | `wikipedia`, `wikidata`, `wiktionary`, `wikinews`, `internet-archive`, `dbpedia`, `openlibrary`, `semantic-scholar`, `openalex`, `crossref`, `cambridge-dictionary`, `merriam-webster`, `dictionary-com`, `collins-dictionary` |
| `papers`    | `arxiv`, `europepmc`, `doaj`                                                                                                                                                                                                   |
| `code`      | `github`, `hackernews`, `gitlab`, `codeberg`, `gitee`, `bitbucket`, `gitflic`                                                                                                                                                  |

`google` and `bing` use official APIs when credentials are configured and fall
back to HTML parsing otherwise. `GITHUB_TOKEN` is optional and raises the GitHub
search rate limit.

## web-capture

`wc:wikipedia`, `wc:duckduckgo`, `wc:google`, `wc:bing`, and `wc:brave` delegate
to the published `web-capture` crate. Runtime errors from that component are
retained as provider errors by detailed aggregate searches, so other selected
providers can still return results without erasing the diagnostic.

```rust
use web_search::providers::{SearchOptions, SearchProvider, WebCaptureProvider};

let provider = WebCaptureProvider::new("wikipedia");
let results = provider.search("OpenAI", &SearchOptions::default()).await?;
```

## Release

The Rust workflow publishes `web-search` to crates.io from `main` after lint,
tests, doc tests, and package checks pass. GitHub releases are tagged as
`rust-v<version>` so they stay distinct from JavaScript `js-v<version>` releases.

The `web-capture 0.3.37` dependency and all direct dependencies track their latest
stable releases. This crate declares its supported minimum Rust version in
`Cargo.toml`.

## Development

```bash
cargo test --all-features
cargo test --doc
cargo test --no-default-features --features merge
cargo build --no-default-features --features merge --target wasm32-unknown-unknown
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo package --list --allow-dirty
```

Cross-language parity is checked from the repository root:

```bash
node js/scripts/check-js-rust-parity.mjs
```

## Dependency freshness

CI checks direct runtime, optional, build and development dependencies against
the latest non-yanked stable crates.io release, including versions outside the
manifest range. Run `python3 rust/scripts/check-dependency-freshness.py` from the
repository root. A blocked update must have an explanatory GitHub issue URL on
its dependency line, for example:

```toml
tower-http = "0.7.1" # Blocked by upstream API: https://github.com/owner/repo/issues/123
```

The checker verifies that the cited issue is open, has a description, and is not
a pull request. Unverifiable issues and registry errors fail the check. Release
jobs depend on freshness passing, and scheduled checks detect later releases.

## License

[Unlicense](../LICENSE) - Public Domain
