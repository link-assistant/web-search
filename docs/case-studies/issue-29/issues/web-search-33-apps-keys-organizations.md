---
id: WS-33
repo: link-assistant/web-search
title: Account apps, API key lifecycle, organization roles, and resource isolation
depends_on: [WS-32]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Account apps, API key lifecycle, organization roles, and resource isolation**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement account /service/v1/apps and app/key create/delete contracts from account OpenAPI.
- Scope runs/groups/monitors/memory/connectors/secrets to organization/app/key and expose key material only at creation.
- Enforce Member/Admin permissions for key ownership/history/billing/webhook-secret reveal and rotation.
- Integrate invitation/role administration with operator identity provider and audit retention/deletion rules.

## Solution alternatives and implementation plan

Select scoped storage/auth context over global key checks. Reuse established identity and key-verifier implementations.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Create app/key, call search, revoke key, and verify subsequent auth failure.
- A role matrix rejects cross-key history and admin-only changes for members.
- Deletion follows documented history rules and audit records contain no keys.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/resources/organization-roles-and-permissions
- https://docs.parallel.ai/service-api/apps/list-apps
- https://docs.parallel.ai/service-api/apps/create-app
- https://docs.parallel.ai/service-api/keys/create-key
- https://docs.parallel.ai/service-api/keys/delete-key
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
