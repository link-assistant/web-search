# @link-assistant/web-search

A web search microservice that aggregates results from multiple search engines with intelligent result merging and reranking.

## Features

- **Multi-provider search**: Aggregate results from Google, DuckDuckGo, and Bing
- **Result merging**: Combine results using RRF, weighted scoring, or interleaving
- **Configurable weights**: Adjust provider weights for custom reranking
- **URL deduplication**: Automatic normalization and deduplication across providers
- **Browser testing**: Integration with browser-commander for direct browser search
- **Multi-runtime support**: Works with Bun, Node.js, and Deno

## Installation

```bash
# With npm
npm install @link-assistant/web-search

# With bun
bun add @link-assistant/web-search

# With yarn
yarn add @link-assistant/web-search
```

## Quick Start

### As a Library

```javascript
import {
  WebSearchEngine,
  createSearchEngine,
} from '@link-assistant/web-search';

// Create a search engine
const engine = createSearchEngine();

// Search across all providers
const results = await engine.search('artificial intelligence');

// Search with options
const results = await engine.search('machine learning', {
  limit: 20,
  providers: ['google', 'duckduckgo'],
  strategy: 'rrf',
  weights: { google: 1.5, duckduckgo: 1.0 },
});

// Search single provider
const googleResults = await engine.searchSingle('deep learning', 'google');
```

### As a REST API Server

```bash
# Start the server
npx web-search serve --port 3000

# Or with bun
bunx web-search serve --port 3000
```

API Endpoints:

- `GET /search?q=<query>` - Search all providers
- `POST /search` - Search with options in body
- `GET /search/:provider?q=<query>` - Search single provider
- `GET /providers` - List available providers
- `GET /health` - Health check

Example:

```bash
curl "http://localhost:3000/search?q=rust+programming&limit=10&strategy=rrf"
```

### As a CLI Tool

```bash
# Search from command line
npx web-search "artificial intelligence"

# With options
npx web-search "machine learning" --limit 20 --providers google,bing --format json

# Output just URLs
npx web-search "deep learning" --format urls
```

## Merge Strategies

### Reciprocal Rank Fusion (RRF)

Default strategy. Combines results by their rank positions across providers.

```javascript
const results = await engine.search(query, { strategy: 'rrf' });
```

### Weighted Scoring

Score results based on provider weights and rank positions.

```javascript
const results = await engine.search(query, {
  strategy: 'weighted',
  weights: { google: 2.0, duckduckgo: 1.0, bing: 0.5 },
});
```

### Interleaving

Round-robin style interleaving of results from each provider.

```javascript
const results = await engine.search(query, { strategy: 'interleave' });
```

## Search Providers

### Google

- Uses Custom Search API when credentials are configured
- Falls back to web scraping otherwise

```javascript
import { GoogleProvider } from '@link-assistant/web-search';

const provider = new GoogleProvider({
  apiKey: 'your-api-key',
  searchEngineId: 'your-cx-id',
});
```

### DuckDuckGo

- Uses HTML scraping (no API required)

```javascript
import { DuckDuckGoProvider } from '@link-assistant/web-search';

const provider = new DuckDuckGoProvider();
```

### Bing

- Uses Web Search API when configured
- Falls back to web scraping otherwise

```javascript
import { BingProvider } from '@link-assistant/web-search';

const provider = new BingProvider({
  apiKey: 'your-bing-api-key',
});
```

### Browser-Based Search

- Uses browser-commander for direct browser search
- Useful for testing and when scraping is blocked

```javascript
import { createBrowserProvider } from '@link-assistant/web-search';

const provider = createBrowserProvider({
  engine: 'google',
  browserOptions: { headless: true },
});
```

## API Reference

### WebSearchEngine

```javascript
const engine = new WebSearchEngine(config);

// Search methods
await engine.search(query, options);
await engine.searchSingle(query, providerName, options);

// Provider management
engine.getAvailableProviders();
engine.getProviderStatus();
engine.setProviderWeight(name, weight);
engine.setProviderEnabled(name, enabled);
engine.getProvider(name);
```

### Merge Functions

```javascript
import {
  mergeResults,
  mergeWithRRF,
  mergeWithWeights,
  mergeWithInterleave,
} from '@link-assistant/web-search';

// Merge results from multiple providers
const merged = mergeResults(resultsByProvider, {
  strategy: 'rrf',
  weights: { google: 1.5 },
  rrfK: 60,
  removeDuplicates: true,
});
```

## Rust Library

A Rust implementation is also available in the `rust/` directory.

```bash
cd rust
cargo build --release
```

### Rust CLI

```bash
# Search
./target/release/web-search "artificial intelligence" --limit 10

# Start server
./target/release/web-search serve --port 3000
```

### Rust Library Usage

```rust
use web_search::{WebSearchEngine, SearchOptions, MergeStrategy};

let engine = WebSearchEngine::new();

let results = engine.search_with_options(
    "machine learning",
    SearchOptions { limit: Some(10), ..Default::default() },
    None,
    Some(MergeOptions { strategy: MergeStrategy::Rrf, ..Default::default() })
).await?;
```

## Development

```bash
# Install dependencies
bun install

# Run tests
bun test

# Run with other runtimes
npm test
deno test --allow-read --allow-net

# Lint code
bun run lint

# Format code
bun run format
```

### Rust Development

```bash
cd rust

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Environment Variables

- `GOOGLE_API_KEY` - Google Custom Search API key
- `GOOGLE_SEARCH_ENGINE_ID` - Google Custom Search Engine ID
- `BING_API_KEY` - Bing Web Search API key

## License

[Unlicense](LICENSE) - Public Domain
