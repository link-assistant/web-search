---
id: WS-29
repo: link-assistant/web-search
title: Snapshot monitors and follow-up Task enrichment for detected events
depends_on: [WS-15, WS-24, WS-20]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Snapshot monitors and follow-up Task enrichment for detected events**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Create snapshot monitors from task_run_id with retained specification/processor/source policy.
- Compare normalized structured outputs and emit partial changed_output plus full previous_output with basis.
- Represent event_stream/snapshot/completion/error as mutually exclusive execution outcomes with event_group_id and paginated history.
- Provide event-to-Task helper with bounded fan-out, provenance, and deduplication.

## Solution alternatives and implementation plan

Use structured semantic diffs instead of raw text hashes for snapshots. Compose the Task engine, existing scheduler, and WS-20 delivery.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Fake-clock first/no-change/changed/failure runs produce correct outcomes and one-field diffs.
- Duplicate event/webhook starts one follow-up Task without losing original events.
- Restart/update/cancel avoid overlapping runs and include_completions pagination works.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/monitor-api/quickstart-snapshot
- https://docs.parallel.ai/monitor-api/monitor-task
- https://docs.parallel.ai/monitor-api/monitor-events
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
