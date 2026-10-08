---
id: WC-08
repo: link-assistant/web-capture
title: web_fetch tool schemas and installed CLI JSON output
depends_on: [WC-02, WC-11]
labels: enhancement
---

## Summary

Parallel Search MCP pairs web_search with web_fetch. Publish capture-owned tool definitions and CLI output from the standalone batch Extract schema. Authentication is independently required work in WC-09.

## Scope

- MCP and model function tool formats generated from the WC-11 Extract types and published in installed packages.
- web-capture extract URL... --objective --full-content --json delegates to WC-02 with warnings/per-URL failures.
- Preserve excerpt/full_content budget semantics and typed errors in tool results.
- Document WS-11 mounting and standalone capture usage without requiring search.

## Acceptance criteria

- SDK schema/call, mixed URL failures, CLI stdout/stderr, invalid args, and JS/Rust parity.
- Existing unit/integration and JS/Rust parity checks pass.

## References

- https://docs.parallel.ai/integrations/mcp/search-mcp
- Consumer: web-search WS-11

## Implementation follow-through

1. Generate web_fetch schemas from WC-11 and delegate CLI JSON to WC-02.
2. WC-09 owns authenticated capture, and tool wrappers preserve errors/budgets/full_content across formats.

## Additional verification

- SDK schema/call, mixed URL failures, CLI stdout/stderr, invalid args, and JS/Rust parity.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
