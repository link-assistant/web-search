---
'@link-assistant/web-search': minor
---

Support both Rust and JavaScript as first-class implementations with full parity (issue #3).

- Add a descriptor-driven engine catalog and a single shared `GenericProvider`, plus shared HTML utilities (entity decoding, tag stripping, generic anchor-list parser), in both languages.
- Add a typed provider registry over the four `formal-ai` categories (`search`, `knowledge`, `papers`, `code`) powering discovery (`--list-providers`, `/providers`, `/providers?category=`, `/categories`) and instantiation. Both languages now report the same 22 providers (search 15, knowledge 2, papers 3, code 2).
- Add knowledge/papers/code and extra search providers: wikipedia, wikidata, searx, crossref, openalex, github, hackernews, arxiv, brave, mojeek, ecosia, startpage, yahoo and DuckDuckGo Lite.
- Integrate `@link-assistant/web-capture` as an optional component library via `wc:*` providers; it loads lazily and degrades gracefully when absent.
- Align `decodeHtmlEntities` across languages (decode `&hellip;`, `&mdash;`, `&ndash;`).
- Expand the test suites (115 JS tests; Rust integration tests for the registry, HTML utilities, and every parser) and document the catalog, categories, web-capture component, and registry in the README and the issue #3 case study.
