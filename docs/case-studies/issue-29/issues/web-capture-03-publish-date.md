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
  Publication remains unknown when only HTTP `Last-Modified` or URL date patterns are available; preserve those as modification/heuristic metadata.
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

## Implementation follow-through

1. Keep publication/modification/capture metadata distinct with source attribution and confidence.
2. Unknown publication stays null. Last-Modified or URL guesses cannot silently become confirmed publication dates.

## Additional verification

- Contradictory/malformed/future metadata, timezones, unknown dates, canonical/language fields, and parity.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
