---
id: WS-30
repo: link-assistant/web-search
title: Task MCP tools for research, enrichment, and async retrieval
depends_on: [WS-11, WS-21, WS-24]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Task MCP tools for research, enrichment, and async retrieval**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Publish research/create/status/result and batch-enrichment tool schemas from documented Task MCP inventory.
- Reuse WS-11 stdio/HTTP hosting with caller auth forwarded to tasks/groups.
- Return run IDs, polling guidance, structured output, and field-level basis.
- Publish Task skills and agent configuration with finite polling and sanitized errors.

## Solution alternatives and implementation plan

Reuse inbound MCP hosting and group execution. Prefer async run-ID tools over blocking long research calls.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- An MCP SDK client creates a fixture Task and retrieves cited results without one long-held call.
- Tool argument/error schemas validate and caller isolation is tested.
- Group enrichment returns partial failures without discarding good rows.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/mcp/task-mcp
- https://docs.parallel.ai/task-api/task-mcp
- https://docs.parallel.ai/integrations/mcp/programmatic-use
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
