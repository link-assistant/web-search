---
id: WS-19
repo: link-assistant/web-search
title: Replayable SSE for task runs, groups, FindAll, and Responses
depends_on: [WS-13]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Replayable SSE for task runs, groups, FindAll, and Responses**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Persist ordered resource events with stable IDs/types/payloads and expose the documented product-specific event adapters.
- Bound subscriber buffers, send heartbeats, release subscriptions on disconnect, and handle completed/failed/cancelled streams.
- Support Last-Event-ID where the product contract permits it and report retention gaps explicitly.
- Keep Monitor events as paginated JSON history plus webhooks: its /events route is not documented as SSE.

## Solution alternatives and implementation plan

Compare ephemeral pub/sub with a durable event log and select the latter. Reuse native JS streams or better-sse and axum::response::sse with WS-13 storage.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Reconnect after event 2 preserves event order without missed terminal updates.
- Slow/disconnected subscribers cannot grow buffers or keep heartbeat timers alive.
- JS/Rust fixtures match framing and payloads for success/failure/cancellation.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/task-api/task-sse
- https://docs.parallel.ai/task-api/group-api
- https://docs.parallel.ai/findall-api/features/findall-sse
- https://docs.parallel.ai/responses-api/features/streaming-events
- https://docs.parallel.ai/monitor-api/monitor-events
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
