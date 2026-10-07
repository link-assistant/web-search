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
