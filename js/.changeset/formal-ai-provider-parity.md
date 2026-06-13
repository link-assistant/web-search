---
'@link-assistant/web-search': minor
---

Align the provider registry with FormalAI's `web_search_core` (issue #5): the
catalog is now a documented superset of FormalAI's provider IDs (40 providers),
the default category for `search` is DuckDuckGo, and the live default plan is
`duckduckgo`, `internet-archive`, `wikipedia`, `wikidata`, `wiktionary`,
`wikinews`. Adds a shared compatibility map and JS/Rust provider-parity tests.
