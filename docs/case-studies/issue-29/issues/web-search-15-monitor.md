---
id: WS-15
repo: link-assistant/web-search
title: Monitor API: scheduled searches with event streams, snapshot diffs, and webhooks
depends_on: [WS-13, WS-02]
labels: enhancement
---

## Summary

parallel.ai Monitor (GA May 2026) runs a query on a `1h`/`1d`/`1w` schedule,
emits new-event notifications (`event_stream`) or diffs of a structured
snapshot (`snapshot` with `task_run_id`), supports `source_policy`, SSE
`/events`, `trigger`, `update`, `cancel`, `stats`, and webhook events
`monitor.event.detected`, `monitor.execution.completed`,
`monitor.execution.failed` (`monitor-api_monitor-quickstart.md`,
`monitor-api_monitor-events.md`).

## Scope

- `POST /v1/monitors` and the list/retrieve/stats/cancel/trigger/update/events
  routes backed by the WS-13 store; scheduler `croner` (JS) /
  `tokio-cron-scheduler` (Rust); runs once at creation.
- `event_stream`: dedupe against prior results by canonical URL and content
  hash (WC-04 `captured_at`), emit events with `basis` (url, excerpts).
- `snapshot`: rerun a WS-14 structured response and diff fields
  (`previous_output`/`changed_output`).
- References studied: changedetection.io, Huginn, Kibitzr, Argus.

## Acceptance criteria

- Tests with a fake clock: first run emits all results, second run emits only new ones; webhook delivered.

## References

- https://docs.parallel.ai/monitor-api/monitor-quickstart
