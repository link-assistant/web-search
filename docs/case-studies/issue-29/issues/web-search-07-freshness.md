---
id: WS-07
repo: link-assistant/web-search
title: Fetch policy and freshness: max_age_seconds, captured_at, and live refresh
depends_on: [WS-02, WS-04, WC-04]
labels: enhancement
---

## Summary

parallel.ai serves indexed content by default and refetches when
`fetch_policy.max_age_seconds` demands it (`search_indexed-content-for-agents.md`).
web-search has no caching layer of its own and cannot express freshness.

## Scope

- Pass `advanced_settings.fetch_policy` through to WC-04 for excerpt fetches.
- Record `captured_at` and `cache_hit` per result; optional response-level
  provider result cache with TTL keyed by (provider, query, options).
- `after_date` from WS-02 combined with `publish_date`.

## Acceptance criteria

- Tests with a fake clock: cached excerpt reused under the limit, refetched above it.

## Implementation follow-through

1. Reuse WC-04 receipts and scope query-cache keys by policy/locale/mode/caller/transport.
2. Report stale fallback and implement GA min age 600 and disable_cache_fallback. A zero-age force-live option is a named local extension.

## Additional verification

- Exact age boundary, fallback enabled/disabled, isolated keys, concurrent refresh, and publication versus capture date.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
