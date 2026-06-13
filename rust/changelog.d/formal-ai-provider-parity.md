---
bump: minor
---

### Added

- Expand the provider registry to a documented superset of FormalAI's `web_search_core` IDs (40 providers): new `knowledge` engines (wiktionary, wikinews, internet-archive, dbpedia, openlibrary, semantic-scholar, openalex, and four dictionaries), `papers` engines (europepmc, doaj), and `code` engines (gitlab, codeberg, gitee, bitbucket, gitflic), plus the `yandex` search engine.
- Add a `formal_ai_compat` integration suite that reads the shared `docs/case-studies/issue-5/formal-ai-compatibility.json` map and enforces FormalAI parity in lockstep with the JavaScript suite.

### Changed

- Default the `search` category to DuckDuckGo and return FormalAI's live default plan (`duckduckgo`, `internet-archive`, `wikipedia`, `wikidata`, `wiktionary`, `wikinews`) from `get_default_provider_ids`.
