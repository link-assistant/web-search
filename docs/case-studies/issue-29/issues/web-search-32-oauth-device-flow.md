---
id: WS-32
repo: link-assistant/web-search
title: Device OAuth login, refresh, and revoke for CLI and agent clients
depends_on: [WS-10]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Device OAuth login, refresh, and revoke for CLI and agent clients**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Support RFC 8628 client registration, device/user codes, verification URLs, polling, refresh, and revoke with configurable identity provider.
- Handle authorization_pending/slow_down/access_denied/expired_token and finite cancellation-aware polling.
- Add CLI login/logout with secure storage and Bearer access distinct from research API keys.
- Keep identity integration optional for static-key self-hosted instances.

## Solution alternatives and implementation plan

Prefer established providers such as Keycloak plus openid-client/oauth2 adapters instead of building an identity provider. Use RFC 8628 fixtures.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- A fake auth server covers every polling outcome, rotation, expiry, logout, and cancellation.
- Installed CLI sign-in stores tokens securely without diagnostics leaks.
- Static-key service works without an OAuth dependency.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/account-api
- https://docs.parallel.ai/integrations/oauth-provider
- https://docs.parallel.ai/integrations/cli
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
