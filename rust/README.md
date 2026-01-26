# Web Search (Rust)

A multi-provider web search aggregator written in Rust with support for result merging and reranking.

## Features

- **Multiple Search Providers**: Google, DuckDuckGo, and Bing
- **API Support**: Uses official APIs when credentials are provided, falls back to scraping
- **Result Merging**: Combine results from multiple providers with deduplication
- **Reranking Strategies**: RRF (Reciprocal Rank Fusion), weighted scoring, or interleaving
- **REST API Server**: Built with Axum for high performance
- **CLI Tool**: Command-line interface for quick searches

## Installation

### From Source

```bash
cd rust
cargo build --release
```

### As Library

Add to your `Cargo.toml`:

```toml
[dependencies]
web-search = { git = "https://github.com/link-assistant/web-search", path = "rust" }
```

## Usage

### CLI

```bash
# Basic search
web-search "rust programming"

# Search with specific providers
web-search "rust async" --providers google,duckduckgo

# Output as JSON
web-search "web scraping" --format json

# Start API server
web-search serve --port 8080
```

### Library

```rust
use web_search::{WebSearchEngine, SearchOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = WebSearchEngine::new();

    let results = engine
        .search("rust programming", SearchOptions::default())
        .await?;

    for result in results {
        println!("{}: {}", result.title, result.url);
    }

    Ok(())
}
```

### REST API

```bash
# Start server
web-search serve --port 3000

# Search all providers
curl "http://localhost:3000/search?q=rust+programming"

# Search single provider
curl "http://localhost:3000/search/duckduckgo?q=rust+programming"

# Get provider status
curl "http://localhost:3000/providers"
```

## API Endpoints

| Endpoint                      | Method | Description              |
| ----------------------------- | ------ | ------------------------ |
| `/search?q=<query>`           | GET    | Search all providers     |
| `/search/:provider?q=<query>` | GET    | Search single provider   |
| `/providers`                  | GET    | List available providers |
| `/health`                     | GET    | Health check             |

### Query Parameters

| Parameter    | Description                                | Default |
| ------------ | ------------------------------------------ | ------- |
| `q`          | Search query (required)                    | -       |
| `providers`  | Comma-separated provider list              | all     |
| `limit`      | Max results per provider                   | 10      |
| `strategy`   | Merge strategy (rrf, weighted, interleave) | rrf     |
| `language`   | Language code (e.g., en)                   | -       |
| `region`     | Region code (e.g., us)                     | -       |
| `safeSearch` | Enable safe search                         | false   |

## Environment Variables

| Variable         | Description                    |
| ---------------- | ------------------------------ |
| `GOOGLE_API_KEY` | Google Custom Search API key   |
| `GOOGLE_CX`      | Google Custom Search Engine ID |
| `BING_API_KEY`   | Bing Search API key            |

## License

[Unlicense](../LICENSE) - Public Domain
