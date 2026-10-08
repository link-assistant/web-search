---
id: WS-01
repo: link-assistant/web-search
title: Search request v2: objective, multiple search_queries, and a warnings/usage response envelope
depends_on: []
labels: enhancement
---

## Summary

parallel.ai Search accepts `search_queries` (1–5, ≤200 chars each) plus an
`objective` (≤5000 chars), `max_results` (default 10, cap 20), `session_id`,
`client_model`, and returns `{search_id, results[], warnings, usage,
session_id}` where each result is `{url, title, publish_date, excerpts[]}`
(`api-reference_search_search.md`, `search_search-quickstart.md` in
`docs/case-studies/issue-29/data/parallel-docs/`). web-search takes a single
`q` and returns a bare array.

## Scope

- `search(request)` overload in JS and Rust accepting
  `{ search_queries, objective, max_results, session_id, client_model, ...existing options }`.
- Fan out one provider call per query; merge all (provider × query) lists with
  the existing RRF/weighted/interleave strategies; dedupe by canonical URL.
- Response envelope `{ search_id, results, warnings, usage, session_id,
providers (existing receipts) }` with `warnings[]{type, message}` for: >5
  queries dropped, `max_results` clamped to 20, unsupported option per provider.
- `publish_date` field populated when a provider or web-capture (WC-03) supplies it, else `null`.
- `excerpts` initially `[snippet]`; WS-04 replaces it with real excerpts.
- Keep `GET /search?q=` and `search(query, options)` working (legacy wrappers).

## Acceptance criteria

- Unit tests with mocked transport: 3 queries × 2 providers merge into one
  ranked list; warnings emitted for 6 queries and `max_results: 50`.
- JS/Rust parity fixture for the envelope; `index.d.ts` updated.

## References

- https://docs.parallel.ai/api-reference/search/search
- Case study: docs/case-studies/issue-29/README.md

## Implementation follow-through

1. Extend current engine options and merger with shared validation/envelope fixtures, retaining canonical dedupe and provider receipts across queries.
2. Bound total provider×query calls and cancellation through existing transport. Objective-only requests need injectable query planning or an explicit deterministic fallback.

## Additional verification

- Objective-only/query-only/invalid input, duplicate URLs, all-provider failure, finite fan-out, cancellation, and legacy wrappers.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
