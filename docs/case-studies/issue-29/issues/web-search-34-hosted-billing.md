---
id: WS-34
repo: link-assistant/web-search
title: Optional hosted usage ledger, credit balance, spend limits, and reload
depends_on: [WS-17, WS-33]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Optional hosted usage ledger, credit balance, spend limits, and reload**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement account balance GET/add POST and immutable request/run/connector ledger entries.
- Support configurable prices, granted credits, app/organization monthly caps, payment references, history, and controlled auto-reload.
- Reserve/refund async costs and settle retries exactly once, separating free polling/connectors.
- Provide test-mode payment adapter and billing-disabled self-hosted operation.

## Solution alternatives and implementation plan

Select a transactional ledger and optional payment-provider adapter rather than a bespoke billing platform. Use decimal/integer minor units.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Concurrent fixture requests cannot exceed spend caps or double charge.
- Failed/cancelled work settles by documented rules and free calls never debit.
- Fake provider tests cover add/reload retries and billing-disabled operation.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/getting-started/pricing
- https://docs.parallel.ai/service-api/balance/get-balance
- https://docs.parallel.ai/service-api/balance/add-to-balance
- https://docs.parallel.ai/resources/organization-roles-and-permissions
- https://docs.parallel.ai/resources/faqs
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
