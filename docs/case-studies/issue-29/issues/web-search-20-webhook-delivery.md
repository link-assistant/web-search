---
id: WS-20
repo: link-assistant/web-search
title: Durable Standard Webhooks delivery, retries, replay, and secret rotation
depends_on: [WS-13]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Durable Standard Webhooks delivery, retries, replay, and secret rotation**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement documented webhook-id/timestamp/signature headers and HMAC-SHA256 of exact raw bytes using versioned secrets.
- Persist delivery attempts in an outbox with finite retry horizon, jitter, rate-limit handling, operator replay, and dead-letter diagnostics.
- Provide per-product event subscriptions, stable delivery IDs, and documented at-least-once semantics.
- Validate outbound destinations through configured transport policy and redact credentials, including secret-rotation overlap.

## Solution alternatives and implementation plan

Use official standardwebhooks libraries instead of inventing signing. Compare a hosted queue with a storage-backed outbox and select the outbox for self-hosting.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- A reference verifier accepts emitted requests and rejects altered bytes/wrong secret/expired timestamps.
- A local receiver returning 500 then 429 then 200 receives one stable event ID.
- Worker restart, dead-letter replay, and rotation preserve pending delivery without real notifications.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/resources/webhook-setup
- https://docs.parallel.ai/task-api/webhooks
- https://docs.parallel.ai/findall-api/features/findall-webhook
- https://docs.parallel.ai/monitor-api/monitor-webhooks
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
