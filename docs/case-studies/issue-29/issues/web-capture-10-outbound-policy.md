---
id: WC-10
repo: link-assistant/web-capture
title: Capture target/redirect policy and bounded document processing
depends_on: []
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Capture target/redirect policy and bounded document processing**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Define injectable target policy for schemes/hosts/networks/redirects/DNS changes and credential forwarding.
- Hosted defaults block unintended private-network access with explicit intranet opt-in for authorized connectors.
- Bound bytes/decompression/redirects/browser concurrency/PDF conversion using existing transport limits.
- Return typed per-URL target/size errors with secret-safe rejected-destination diagnostics.

## Solution alternatives and implementation plan

Select one transport policy over scattered route checks. Reuse URL parsing/resource limits and evaluate isolated converter workers.

1. Inspect the existing capture transport, conversion modules, server, CLI, and capture receipts. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Local DNS/transport fixtures cover blocked schemes/networks/redirects/rebinding/credential stripping.
- Finite oversized/compressed/malformed PDF probes obey process/memory limits.
- Authorized intranet policy works while anonymous hosted capture remains restricted.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/extract/extract-quickstart
- https://docs.parallel.ai/resources/faqs
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
