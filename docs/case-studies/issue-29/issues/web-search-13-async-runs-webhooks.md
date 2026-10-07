---
id: WS-13
repo: link-assistant/web-search
title: Async runs, task groups, SSE events, and Standard Webhooks delivery
depends_on: [WS-10]
labels: enhancement
---

## Summary

Task, FindAll, and Monitor on parallel.ai share one asynchronous model: create
a run, poll `GET …/{id}`, stream `/events` (SSE), group runs, and receive
webhooks signed per the Standard Webhooks spec (`webhook-id`,
`webhook-timestamp`, `webhook-signature: v1,<base64 HMAC-SHA256>` over
`id.timestamp.body` with a `whsec_` secret) (`task-api_task-quickstart.md`,
`resources_webhook-setup.md`). This is the substrate for WS-14/15/16/18.

## Scope

- Run store abstraction (in-memory default; SQLite via `better-sqlite3`/`sqlx` optional)
  with statuses queued → running → completed/failed/cancelled, `created_at`, `metadata`.
- `POST /v1/tasks/runs` for long searches (e.g. `advanced` mode with large
  `max_results`), `GET /v1/tasks/runs/{id}`, `/result` with blocking `timeout`,
  `/input`, `/events` SSE (`better-sse`, `axum` SSE), `POST /v1/tasks/groups` + add-runs.
- Webhook sender with Standard Webhooks signing and retry/backoff; verifiers
  from `standard-webhooks`/`svix` used in tests.
- Config limits for concurrency and retention.

## Acceptance criteria

- Tests: run lifecycle, SSE replay with `Last-Event-ID`, webhook signature verified by the Svix library.

## References

- https://docs.parallel.ai/resources/webhook-setup
- https://www.standardwebhooks.com/
