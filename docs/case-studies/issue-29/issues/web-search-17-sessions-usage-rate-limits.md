---
id: WS-17
repo: link-assistant/web-search
title: session_id, client_model hints, usage accounting, and optional rate limiting
depends_on: [WS-10]
labels: enhancement
---

## Summary

parallel.ai echoes or generates `session_id` (≤1000 chars) to link related
calls, accepts `client_model` for excerpt tuning, returns `usage[]{name,count}`
SKU counters, and enforces per-API rate limits with 429 + backoff guidance
(`search_search-quickstart.md`, `getting-started_rate-limits.md`).

## Scope

- Validate/echo `session_id`, generate one when absent; log `client_model`.
- `usage[]` counters (`sku_search`, `sku_extract`, `sku_images`, provider
  request counts) in every envelope.
- Optional token-bucket limiter (`express-rate-limit`, `tower_governor`) with
  per-key limits and `Retry-After`; presets mirroring parallel.ai's documented
  defaults (600/min web tools, 2000/min tasks, 300/min chat).

## Acceptance criteria

- Tests for echo/generate behaviour and 429 after the configured burst.

## Implementation follow-through

1. Validate session/client hints and operation/SKU counters through shared envelopes.
2. Separate usage from billing WS-34 and configurable per-key limiting from provider retry behavior.

## Additional verification

- Length limits, echo/generation, partial-failure usage, fake-clock 429/recovery, key isolation, and limiter disabled.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
