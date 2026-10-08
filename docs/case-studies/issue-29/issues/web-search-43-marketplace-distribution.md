---
id: WS-43
repo: link-assistant/web-search
title: AWS and Google Cloud marketplace deployment and subscription plans
depends_on: [WS-33, WS-34, WS-42]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **AWS and Google Cloud marketplace deployment and subscription plans**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Prepare reproducible container/cloud infrastructure templates with local build validation.
- Define entitlement/provisioning/organization mapping/metering/cancellation adapters.
- Prepare listing assets/support information and explicit partner account/approval prerequisites.
- Track submission/activation as external milestones requiring public listing and subscription smoke evidence.

## Solution alternatives and implementation plan

Compare SaaS fulfillment with deploy-your-own containers against operator capacity and official cloud contracts. Reuse billing/operating infrastructure.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Local templates build and mocked entitlement callbacks provision/revoke idempotently.
- Metering agrees with the ledger and retry does not duplicate usage.
- Issue records review-ready artifacts and each external blocker without claiming a live listing.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/aws-marketplace
- https://docs.parallel.ai/integrations/google-cloud-marketplace
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
