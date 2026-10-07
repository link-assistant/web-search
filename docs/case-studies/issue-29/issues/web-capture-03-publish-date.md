---
id: WC-03
repo: link-assistant/web-capture
title: Detect and expose publish_date and page metadata on search and capture results
depends_on: []
labels: enhancement
---

## Summary

Every parallel.ai Search/Extract result carries `publish_date` (YYYY-MM-DD), and
Search supports `source_policy.after_date` filtering on it
(`search_search-quickstart.md`, `resources_source-policy.md`). web-capture
captures pages but does not return a normalized publish date.

## Scope

- Metadata extractor returning `{ title, publish_date, modified_date, author, site_name, canonical_url, language }`.
- Date sources in priority order: `article:published_time`, JSON-LD
  `datePublished`, `<meta name="date">`/`dc.date`, `<time datetime>`,
  HTTP `Last-Modified`, URL path patterns (`/2026/09/18/`).
- Normalize to `YYYY-MM-DD` (UTC); return `null` when unknown.
- Expose on `/search` results, `/markdown` headers (`x-capture-publish-date`),
  the `captureResponse` receipt, and the WC-02 extract results.

## Acceptance criteria

- Fixture-based tests for each date source and for conflicting sources.
- JS and Rust produce the same date for the same fixtures.
- Documented in README under the response contract.

## References

- https://docs.parallel.ai/search/source-policy (after_date semantics)
- Reference libraries: `metascraper-date` (JS), `dom_smoothie`/`article_scraper` + `chrono` (Rust)
- Consumers: web-search WS-02 (after_date filter), WS-01 (result envelope)
