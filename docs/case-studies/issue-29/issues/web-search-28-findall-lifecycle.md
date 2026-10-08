---
id: WS-28
repo: link-assistant/web-search
title: FindAll preview, exclusions, extend, cancel, and refresh lifecycle
depends_on: [WS-16, WS-19, WS-27]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **FindAll preview, exclusions, extend, cancel, and refresh lifecycle**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement preview/base/core/pro budgets, terminal reasons, and applicable action_required/cancelling states.
- Extend without repeating completed matches and cancel generation/enrichment while retaining partial results.
- Refresh creates a new run from the original query with stable deduplicated exclude_list history.
- Retain every historical matched entity across reordered pages and explicitly reapply saved enrichment payloads.

## Solution alternatives and implementation plan

Select documented new-run refresh over mutation of a completed run. Use canonical entity IDs and storage transactions.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Preview/extend retains candidate IDs and adds only new matches.
- Two refresh cycles cannot reintroduce old matched entities or silently exclude unmatched ones.
- Cancellation races, restart, and budget exhaustion preserve partial results and correct status events.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/findall-api/features/findall-preview
- https://docs.parallel.ai/findall-api/features/findall-extend
- https://docs.parallel.ai/findall-api/features/findall-cancel
- https://docs.parallel.ai/findall-api/features/findall-refresh
- https://docs.parallel.ai/findall-api/core-concepts/findall-lifecycle
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
