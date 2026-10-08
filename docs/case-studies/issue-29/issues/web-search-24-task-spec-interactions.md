---
id: WS-24
repo: link-assistant/web-search
title: Task schemas, processor budgets, Ingest, and interaction chains
depends_on: [WS-14]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Task schemas, processor budgets, Ingest, and interaction chains**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Support task_spec input_schema/output_schema with text/JSON validation and field-level research basis.
- Resolve lite/base/core/core2x/pro/ultra/ultra2x/ultra4x/ultra8x into configurable finite research budgets and report measured capabilities.
- Implement documented POST /v1beta/findall/ingest objective-to-spec inference and GET run/schema retrieval rather than inventing POST /findall/schema.
- Return interaction_id and accept previous_interaction_id for cross-processor follow-ups with caller isolation and retention controls.

## Solution alternatives and implementation plan

Support caller schemas plus injectable model inference through ajv/jsonschema. Reuse WS-14 backend and WS-13 ancestry storage.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- A stub model infers schemas and produces valid cited fields while invalid outputs fail explicitly.
- Every processor has finite-budget and exhaustion tests.
- Interaction chains use interaction IDs rather than previous_run_id, reject cross-caller ancestry, and honor retention-disabled mode.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/task-api/guides/specify-a-task
- https://docs.parallel.ai/task-api/guides/choose-a-processor
- https://docs.parallel.ai/task-api/guides/interactions
- https://docs.parallel.ai/task-api/ingest-api
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
