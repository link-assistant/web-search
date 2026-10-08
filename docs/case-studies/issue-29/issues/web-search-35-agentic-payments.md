---
id: WS-35
repo: link-assistant/web-search
title: Optional MPP and x402 payment gateway for machine clients
depends_on: [WS-34, WS-24]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Optional MPP and x402 payment gateway for machine clients**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Expose paid Search/Extract/Task creation and free polling through an optional gateway.
- Handle HTTP 402 challenge/proof verification/replay protection/idempotency using provider adapters.
- Publish prices/currencies/spend limits and test-mode Stripe/Tempo/Base settlement contracts.
- Provide mppx/purl examples and a skill without adding live settlement to default tests.

## Solution alternatives and implementation plan

Reuse maintained MPP/x402 reference SDKs and settlement adapters. Do not implement a blockchain or store user wallets in core search.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Fake settlement tests challenge/accepted/expired/wrong/reused proof.
- Retried paid requests produce one task and one ledger debit.
- Free polling and disabled gateway leave ordinary API behavior intact.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/agentic-payments
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
