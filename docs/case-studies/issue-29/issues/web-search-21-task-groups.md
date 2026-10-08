---
id: WS-21
repo: link-assistant/web-search
title: Task groups: bulk submission, aggregate status, pagination, and partial failures
depends_on: [WS-13, WS-14, WS-19]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Task groups: bulk submission, aggregate status, pagination, and partial failures**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement create/retrieve group, add/list runs, retrieve group run, and group events from current OpenAPI.
- Allow more runs while a group is active and preserve per-run text/JSON inputs and specification overrides.
- Define atomic admission or explicit per-item rejection, stable counters/cursors, and deterministic input association.
- Bound per-group/organization concurrency and retain successful results when another run fails.

## Solution alternatives and implementation plan

Compose WS-13 runs instead of building a second batch engine. Use storage transactions and indexed group membership with shared queue limits.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Add a second batch during execution and verify totals after mixed success/failure.
- Pagination returns each run once and a foreign group cannot access it.
- Restart and finite large batches respect limits and retain structured inputs exactly.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/task-api/group-api
- https://docs.parallel.ai/api-reference/tasks/add-runs-to-task-group
- https://docs.parallel.ai/api-reference/tasks/fetch-task-group-runs
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
