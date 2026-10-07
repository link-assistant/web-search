---
id: WC-02
repo: link-assistant/web-capture
title: Add a batch Extract API compatible with parallel.ai /v1/extract
depends_on: [WC-01, WC-03, WC-04]
labels: enhancement
---

## Summary

parallel.ai Extract (`POST /v1/extract`) takes up to 20 URLs with an optional
`objective` and `search_queries`, returns per-URL `{url, title, publish_date,
excerpts[], full_content?}` as markdown, and reports per-URL failures in
`errors[]{url, error_type}` instead of failing the batch
(`extract_extract-quickstart.md`, `api-reference_extract_extract.md`).
web-capture only has single-URL routes (`/markdown`, `/txt`, ...).

## Scope

- `POST /extract` (and `extract(urls, options, transport)` in the libraries) with
  request body: `urls` (1–20), `objective`, `search_queries` (≤5),
  `max_chars_total`, `advanced_settings{ fetch_policy, excerpt_settings{max_chars_per_result}, full_content: bool | {max_chars_per_result} }`.
- Response: `extract_id`, `results[]`, `errors[]` with `error_type` ∈
  `fetch_error | timeout | blocked | unsupported_content | too_large`,
  `warnings[]`, `usage[]`.
- Bounded concurrency per batch and per host; per-URL timeout; reuse the
  transport/receipt contract and `CachedTransport`.
- Markdown from existing converters; excerpts from WC-01; dates from WC-03;
  freshness from WC-04.
- CLI `web-capture extract <url...> --objective --json`.

## Acceptance criteria

- Integration test with a local fixture server: mixed batch of 3 good URLs,
  1 timeout, 1 404 returns 3 results and 2 typed errors with HTTP 200.
- `full_content: true` returns markdown; `{max_chars_per_result}` truncates.
- Legacy routes remain unchanged; JS/Rust parity test for request/response shapes.

## References

- https://docs.parallel.ai/extract/extract-quickstart
- https://docs.parallel.ai/api-reference/extract/extract
- Case study: https://github.com/link-assistant/web-search/blob/main/docs/case-studies/issue-29/README.md
- Consumers: web-search WS-04, WS-10, WS-14
