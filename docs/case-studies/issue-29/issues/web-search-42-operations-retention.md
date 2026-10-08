---
id: WS-42
repo: link-assistant/web-search
title: Production operations, status, retention controls, and failure diagnostics
depends_on: [WS-13, WS-17, WS-20]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Production operations, status, retention controls, and failure diagnostics**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Expose correlated provider/run timings, queue/outbox health, limit counters, and optional redacted tracing/metrics.
- Document readiness/liveness/graceful shutdown/storage migrations/backups/recovery and status-page integration.
- Configure expiry/deletion for inputs/outputs/events/memory/secrets and retention-disabled operation.
- Publish limits/overload/data handling/deployment examples without claiming unaudited certifications.

## Solution alternatives and implementation plan

Reuse OpenTelemetry/tracing and storage policies with tracing off by default. Keep a hosted observability vendor optional.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Controlled failures/restarts yield correlated actionable diagnostics without credentials/content leaks.
- Expiry removes scoped history/context and disabled retention does not persist interactions.
- Shutdown drains/requeues work safely and finite load fixtures obey queue bounds.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/resources/status
- https://docs.parallel.ai/resources/faqs
- https://docs.parallel.ai/resources/warnings-and-errors
- https://docs.parallel.ai/getting-started/rate-limits
- https://docs.parallel.ai/task-api/guides/interactions
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
