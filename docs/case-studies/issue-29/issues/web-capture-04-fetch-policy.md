---
id: WC-04
repo: link-assistant/web-capture
title: Expose fetch_policy (max_age_seconds, live vs cached) on the public API
depends_on: []
labels: enhancement
---

## Summary

parallel.ai lets callers choose cached index content or a live fetch through
`fetch_policy{ max_age_seconds, disable_cached_content, timeout_seconds }`
(`search_advanced-search-settings.md`, `extract_advanced-extract-settings.md`).
web-capture has a content-addressed `CaptureStore`/`CachedTransport` (issue #156)
but no request-level knob to control freshness.

## Scope

- Accept `fetch_policy` on `/fetch`, `/markdown`, `/search`, and the WC-02
  `/extract` routes and in the library options.
- `max_age_seconds: 0` forces a live fetch; otherwise serve the cached
  capture when `now - capturedAt <= max_age_seconds`.
- `disable_cached_content: true` bypasses the cache entirely; `timeout_seconds`
  caps the live fetch and falls back to cache when allowed.
- Receipt/diagnostics gain `captured_at`, `cache_hit`, `fetch_policy_applied`.

## Acceptance criteria

- Tests with a mocked clock covering fresh hit, stale miss, forced live, and timeout fallback.
- Default behaviour (no policy) is unchanged.

## References

- https://docs.parallel.ai/search/indexed-content-for-agents
- Consumers: web-search WS-07 (freshness), WS-15 (monitor dedupe), WC-02
