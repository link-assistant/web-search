---
id: WS-18
repo: link-assistant/web-search
title: Scoped Memory API with ranked/recent retrieval, evict, and clear
depends_on: [WS-13, WC-01]
labels: enhancement, documentation
---

## Summary

Parallel Memory reuses Task, Monitor, and FindAll findings through a memory_scope_key. This issue owns retrieval/eviction/clear; independent migration documentation is WS-40.

## Scope

- POST /v1beta/memory/retrieve, /evict, /clear over the run store and scoped memory visibility indexes.
- Optional query relevance versus recent-first ordering using WC-01 ranking, with stable results and documented limits.
- Eviction/clear preserves original resources, and expiry/retention-disabled behavior remains explicit.
- Isolation by caller/organization and memory_scope_key, with lifecycle hooks for Task/Monitor/FindAll adapters.

## Acceptance criteria

- Ranked/recent retrieval, cross-scope isolation, preserved runs after clear, missing IDs, restart, and expiry.
- Existing unit/integration and JS/Rust parity checks pass.

## Implementation follow-through

1. Implement caller-scoped Memory query relevance/recent ordering and retrieve/evict/clear over stored results.
2. Evict/clear affects memory visibility without deleting underlying runs. WS-40 owns migration guides and disabled retention is explicit.

## Additional verification

- Ranked/recent retrieval, cross-scope isolation, preserved runs after clear, missing IDs, restart, and expiry.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
