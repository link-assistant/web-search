# Web Search (Rust)

[![crates.io crate](https://img.shields.io/crates/v/web-search?label=crates.io)](https://crates.io/crates/web-search)
[![crate downloads](https://img.shields.io/crates/d/web-search?label=downloads)](https://crates.io/crates/web-search)
[![docs.rs](https://img.shields.io/docsrs/web-search?label=docs.rs)](https://docs.rs/web-search)
[![Rust CI](https://github.com/link-assistant/web-search/actions/workflows/rust.yml/badge.svg)](https://github.com/link-assistant/web-search/actions/workflows/rust.yml)
[![Rust release tag](https://img.shields.io/badge/GitHub%20release-rust--v0.2.0-orange)](https://github.com/link-assistant/web-search/releases?q=rust-v)

Rust implementation of the `web-search` library, CLI, and HTTP service. It
mirrors the JavaScript package in `../js` with the same 22-provider catalog,
provider categories, merge strategies, and discovery surface.

## Install

```bash
cargo install web-search
```

As a library:

```toml
[dependencies]
web-search = "0.2"
```

From a local checkout before a crates.io release is visible:

```toml
[dependencies]
web-search = { path = "../web-search/rust" }
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

The live registry has 22 providers in four categories:

| Category    | Provider ids                                                                                             |
| ----------- | -------------------------------------------------------------------------------------------------------- |
| `search`    | `google`, `bing`, `duckduckgo`, `searx`, `brave`, `mojeek`, `ecosia`, `startpage`, `yahoo`, `lite`, `wc:*` |
| `knowledge` | `wikipedia`, `wikidata`                                                                                  |
| `papers`    | `crossref`, `openalex`, `arxiv`                                                                          |
| `code`      | `github`, `hackernews`                                                                                   |

`google` and `bing` use official APIs when credentials are configured and fall
back to HTML parsing otherwise. `GITHUB_TOKEN` is optional and raises the GitHub
search rate limit.

## web-capture

`wc:wikipedia`, `wc:duckduckgo`, `wc:google`, `wc:bing`, and `wc:brave` delegate
to the published `web-capture` crate. Runtime errors from that component are
logged and converted to an empty result set so other selected providers can
still return results.

```rust
use web_search::providers::{SearchOptions, SearchProvider, WebCaptureProvider};

let provider = WebCaptureProvider::new("wikipedia");
let results = provider.search("OpenAI", &SearchOptions::default()).await?;
```

## Release

The Rust workflow publishes `web-search` to crates.io from `main` after lint,
tests, doc tests, and package checks pass. GitHub releases are tagged as
`rust-v<version>` so they stay distinct from JavaScript `js-v<version>` releases.

The current `web-capture 0.3.31` dependency requires Rust 1.96 or newer, which
is declared as this crate's MSRV in `Cargo.toml`.

## Development

```bash
cargo test --all-features
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo package --list --allow-dirty
```

Cross-language parity is checked from the repository root:

```bash
node js/scripts/check-js-rust-parity.mjs
```

## License

[Unlicense](../LICENSE) - Public Domain
