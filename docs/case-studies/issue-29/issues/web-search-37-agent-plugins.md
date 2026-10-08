---
id: WS-37
repo: link-assistant/web-search
title: Agent skills and editor plugins for search, extraction, and research
depends_on: [WS-11, WS-30, WS-32]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Agent skills and editor plugins for search, extraction, and research**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Publish versioned shared skills and stdio/HTTP MCP configuration with uninstall steps.
- Add native manifests for Claude Code/Cursor/OpenCode/Pi/OpenClaw-ClawHub and compatibility recipes for other agents.
- Support self-hosted URLs, static keys/device login, finite polling, and JSON.
- Prepare distribution artifacts and track external marketplace approvals separately.

## Solution alternatives and implementation plan

Select shared skill content with thin native wrappers and official installation mechanisms rather than bespoke installers.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Schema lint and installation smoke tests use package artifacts and mocked local service.
- Each supported agent has search/extract/research-result examples with citations.
- Upgrade/uninstall preserves unrelated settings and secret storage.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/agent-skills
- https://docs.parallel.ai/integrations/claude-code-marketplace
- https://docs.parallel.ai/integrations/cursor-marketplace
- https://docs.parallel.ai/integrations/opencode-plugin
- https://docs.parallel.ai/integrations/pi-extension
- https://docs.parallel.ai/integrations/clawhub
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
