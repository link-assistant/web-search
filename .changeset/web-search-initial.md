---
'@link-assistant/web-search': minor
---

Add web search microservice with multi-provider aggregation

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
