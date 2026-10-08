---
id: WS-25
repo: link-assistant/web-search
title: Remote MCP tools and authenticated private sources in research runs
depends_on: [WS-14]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Remote MCP tools and authenticated private sources in research runs**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Accept named Task mcp_servers and Responses-style mcp tool configuration with supported authentication.
- Discover/validate schemas, restrict allowed tools, bound calls/result sizes, and close connections on cancellation.
- Record mcp_tool_calls and attributable basis with typed failures and redacted credentials.
- Provide a Browser Use/private-web capture adapter owned by WC-09 without blocking independent public-web research.

## Solution alternatives and implementation plan

Use official @modelcontextprotocol/sdk and rmcp clients rather than manual JSON-RPC. Prefer operator-managed secret references for hosted instances.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- A local authenticated MCP server exposes tools and a run calls only permitted ones.
- Name collisions, malformed schema, unavailable server, oversized output, and cancellation are tested.
- No credential appears in logs/results/history and separate callers cannot share private source state.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/task-api/mcp-tool-call
- https://docs.parallel.ai/responses-api/features/mcp-tools
- https://docs.parallel.ai/integrations/browseruse
- https://docs.parallel.ai/resources/data-connectors
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
