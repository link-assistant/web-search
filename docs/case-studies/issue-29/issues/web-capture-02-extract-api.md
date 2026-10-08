---
id: WC-02
repo: link-assistant/web-capture
title: Add a batch Extract API compatible with parallel.ai /v1/extract
depends_on: [WC-01, WC-03, WC-04, WC-10]
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
- Case study: https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md
- Consumers: web-search WS-04, WS-10, WS-14

## Implementation follow-through

1. Implement one batch engine over current conversion/transport interfaces: WC-11 owns its versioned wrapper.
2. Preserve URL-to-input association, bound total/per-host work, and separate excerpt budgets from full_content caps.

## Additional verification

- Mixed success/404/timeout/blocked/binary, duplicates, cancellation, zero budgets, and full_content options.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
