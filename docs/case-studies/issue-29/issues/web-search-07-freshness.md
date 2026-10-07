---
id: WS-07
repo: link-assistant/web-search
title: Fetch policy and freshness: max_age_seconds, captured_at, and live refresh
depends_on: [WS-04, web-capture WC-04]
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
