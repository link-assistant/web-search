---
id: WS-15
repo: link-assistant/web-search
title: GA Monitor event streams, scheduling, history, and control routes
depends_on: [WS-13, WS-02, WS-14, WS-20]
labels: enhancement
---

## Summary

Parallel GA Monitor runs periodic research queries and exposes paginated JSON event history and webhooks. This issue owns event-stream monitoring. Snapshot diffs and follow-up Tasks are WS-29.

## Scope

- GA POST/list/retrieve/stats/cancel/trigger/update Monitor routes and GET /events JSON pagination with event_group_id and include_completions.
- Persist schedules, checkpoint prior evidence, and deduplicate material events with stable IDs and basis rather than URL presence alone.
- Use finite interval presets and configured scheduler, preventing manual/scheduled overlap and recovering missed runs after restart.
- Emit mutually exclusive detection/completion/error execution outcomes through WS-20 webhooks. No Monitor SSE claim.

## Acceptance criteria

- Fake-clock no-change/detection/failure, missed schedule recovery, pagination, update/cancel, and signed notifications.
- Existing unit/integration and JS/Rust parity checks pass.

## References

- https://docs.parallel.ai/monitor-api/monitor-quickstart

## Implementation follow-through

1. Implement GA event-stream Monitor CRUD/stats/trigger/schedule with durable checkpoints and paginated JSON events.
2. Deduplicate material detections, not just URLs, and prevent overlapping scheduled/manual runs. WS-29 owns snapshot/follow-up work.

## Additional verification

- Fake-clock no-change/detection/failure, missed schedule recovery, pagination, update/cancel, and signed notifications.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
