---
id: WS-13
repo: link-assistant/web-search
title: Durable async task runtime, run storage, cancellation, and recovery
depends_on: [WS-10]
labels: enhancement
---

## Summary

Parallel Task runs and later FindAll/Monitor products need a common asynchronous execution substrate. This issue owns the store and workers. SSE, webhooks, and groups are separately tracked in WS-19, WS-20, and WS-21.

## Scope

- Run store with queued/running/completed/failed/cancelled states, timestamps, metadata, scoped inputs/results, and an in-memory plus optional SQLite implementation.
- POST /v1/tasks/runs and GET status/input/result with finite blocking-result timeout. WS-14 plugs research execution into the worker interface.
- Bound concurrency/queue size and preserve typed errors, cancellation, restart recovery, retention, and caller transport.
- Use existing search/transport APIs, SQLite/sqlx candidates, and injected workers. Do not advertise groups or streams until their issues ship.

## Acceptance criteria

- All state transitions, cancellation races, restart, blocking-result timeout, input round-trip, overload, and expiry.
- Existing unit/integration and JS/Rust parity checks pass.

## References

- https://docs.parallel.ai/resources/webhook-setup
- https://www.standardwebhooks.com/

## Implementation follow-through

1. Implement run/store/worker abstraction with in-memory fixtures and optional durable SQLite.
2. Recover queued/running work after restart with scope, finite concurrency, typed failure, cancellation, and retention. WS-19/20/21 own protocols/groups.

## Additional verification

- All state transitions, cancellation races, restart, blocking-result timeout, input round-trip, overload, and expiry.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
