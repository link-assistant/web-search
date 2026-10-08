---
id: WC-11
repo: link-assistant/web-capture
title: Standalone /v1/extract contract, OpenAPI, hints, errors, and usage
depends_on: [WC-02]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Standalone /v1/extract contract, OpenAPI, hints, errors, and usage**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Mount POST /v1/extract backed by WC-02 and generate OpenAPI/tool schemas from shared types.
- Validate nested GA settings and reject beta-only top-level fields with typed 422 errors.
- Support extract_id/session_id/client_model/warnings/usage/partial errors/auth/configurable Retry-After limits.
- Keep legacy routes and explicit adapters, with excerpt max_chars_total independent of full_content limits.

## Solution alternatives and implementation plan

Select shared batch schemas behind versioned adapters. Reuse existing server/schema libraries and avoid duplicating WC-02.

1. Inspect the existing capture transport, conversion modules, server, CLI, and capture receipts. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- GA quickstart bodies work in JS/Rust while beta top-level fields fail on GA and translate through explicit legacy route.
- Empty/invalid inputs/Unicode/budgets/mixed errors/session hints have shared fixtures.
- A generated client calls standalone web-capture without needing web-search.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/api-reference/extract/extract
- https://docs.parallel.ai/extract/extract-migration-guide
- https://docs.parallel.ai/extract/advanced-extract-settings
- https://docs.parallel.ai/resources/warnings-and-errors
- https://docs.parallel.ai/getting-started/rate-limits
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
