---
id: WC-04
repo: link-assistant/web-capture
title: Expose fetch_policy (max_age_seconds, live vs cached) on the public API
depends_on: []
labels: enhancement
---

## Summary

parallel.ai lets callers choose cached index content or a live fetch through
`fetch_policy{ max_age_seconds, disable_cache_fallback, timeout_seconds }`
(`search_advanced-search-settings.md`, `extract_advanced-extract-settings.md`).
web-capture has a content-addressed `CaptureStore`/`CachedTransport` (issue #156)
but no request-level knob to control freshness.

## Scope

- Accept `fetch_policy` on `/fetch`, `/markdown`, `/search`, and the WC-02
  `/extract` routes and in the library options.
- GA `max_age_seconds` has minimum 600; serve cache within the age budget and attempt live refresh otherwise. Force-live is a separately documented local extension.
- `disable_cache_fallback: true` returns an error when live refresh fails; false permits stale fallback. `timeout_seconds` bounds live work.
- Receipt/diagnostics gain `captured_at`, `cache_hit`, `fetch_policy_applied`.

## Acceptance criteria

- Tests with a mocked clock covering fresh hit, stale miss, forced live, and timeout fallback.
- Default behaviour (no policy) is unchanged.

## References

- https://docs.parallel.ai/search/indexed-content-for-agents
- Consumers: web-search WS-07 (freshness), WS-15 (monitor dedupe), WC-02

## Implementation follow-through

1. Extend CaptureStore/CachedTransport policy resolution while preserving default/caller-owned behavior.
2. GA max_age_seconds minimum is 600, with timeout_seconds and disable_cache_fallback. Force-live/cache-bypass are separate local extensions.

## Additional verification

- Age boundary, stale fallback on/off, live failure, conditional revalidation, cache scope, and refresh dedupe.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
