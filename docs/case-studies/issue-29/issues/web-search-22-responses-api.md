---
id: WS-22
repo: link-assistant/web-search
title: Responses API statefulness, citations, structured output, and stream compatibility
depends_on: [WS-14, WS-19, WS-24]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Responses API statefulness, citations, structured output, and stream compatibility**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Mount POST /v1/responses with documented input forms, Bearer auth, model alias, reasoning.effort, previous_response_id, text.format, web_search tools, and stream.
- Map research traces to message/web_search_call items and url_citation annotations with correct Unicode offsets.
- Persist caller-scoped ancestry and handle missing/deleted context explicitly.
- Validate JSON-schema output and documented unsupported fields, with extension hooks for WS-25 MCP and WS-26 data_sources.

## Solution alternatives and implementation plan

Select a format adapter over WS-14/24 rather than another agent loop. Use ajv/jsonschema and WS-19 event adapters.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Current request fixtures work against a local compatible client in streaming and nonstreaming modes.
- Unicode citation offsets reference evidence actually used and schema errors are typed.
- Follow-ups reuse context only within caller scope and missing ancestry returns an explicit error.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/responses-api/responses-quickstart
- https://docs.parallel.ai/responses-api/openai-compatibility
- https://docs.parallel.ai/responses-api/features/statefulness
- https://docs.parallel.ai/responses-api/features/citations
- https://docs.parallel.ai/responses-api/features/structured-outputs
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
