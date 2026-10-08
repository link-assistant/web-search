---
id: WS-39
repo: link-assistant/web-search
title: Workflow integrations: n8n, Zapier, Sheets, Render, Vercel, and Superhuman
depends_on: [WS-10, WS-20, WS-31]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Workflow integrations: n8n, Zapier, Sheets, Render, Vercel, and Superhuman**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Publish common search/extract/task/result actions and signed webhook triggers.
- Provide n8n node and Zapier action templates plus Google Sheets function/batch examples.
- Add Render Workflows/Vercel deployment and AI-tool templates and Superhuman recipe using available extension contract.
- Document secrets, endpoint reachability, retry/idempotency, and third-party approval requirements.

## Solution alternatives and implementation plan

Use platform-native thin actions over HTTP clients and webhook delivery. Avoid building another automation engine.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Each template has mocked actions and sample cited output, and invalid webhook signatures fail.
- Duplicate trigger cannot repeat Task enrichment and Sheets preserves row alignment.
- Deployment examples have finite execution limits and record unavailable external prerequisites.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/n8n
- https://docs.parallel.ai/integrations/zapier
- https://docs.parallel.ai/integrations/gsuite
- https://docs.parallel.ai/integrations/render
- https://docs.parallel.ai/integrations/vercel
- https://docs.parallel.ai/integrations/superhuman
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
