---
id: WS-31
repo: link-assistant/web-search
title: Typed HTTP clients for Python, TypeScript, and Rust with polling and streams
depends_on: [WS-10, WS-18, WS-19, WS-21, WS-22, WS-23, WS-28, WS-29]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Typed HTTP clients for Python, TypeScript, and Rust with polling and streams**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Generate or maintain typed service clients for all product routes with a configurable base URL and auth.
- Provide cancellation, finite polling, pagination, Retry-After retries, SSE, injectable transport, and typed errors.
- Keep existing native JS/Rust search libraries independent and add installed Python/client-package examples.
- Define schema/client drift checks and client release/version/runtime support.

## Solution alternatives and implementation plan

Evaluate OpenAPI Generator/openapi-typescript and select generation where streams are supported, with tested thin polling helpers. Native libraries are not replaced by HTTP SDKs.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Installed clients run local search/extract/task/stream quickstarts.
- 429, partial failure, pagination, malformed response, and cancellation have fixture tests.
- Generated drift fails CI and no client requires a repository checkout.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/getting-started/overview
- https://docs.parallel.ai/integrations/developer-quickstart
- https://docs.parallel.ai/integrations/mcp/programmatic-use
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
