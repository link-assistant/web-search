> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitor Migration Guide: Alpha to GA

> Migrate from the Alpha Monitor API (/v1alpha) to the GA version (/v1)

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

V1 is the same Monitor product on a **new HTTP contract**. Alpha-created monitors keep their IDs, schedule, webhook URL, and execution history — retrieve them at `GET /v1/monitors/{id}` as `type: "event_stream"`. Replacing `v1alpha` with `v1` in a URL is not enough: request shapes, response fields, pagination, and event JSON all changed.

<Note>
  All ongoing development targets V1. The Alpha endpoints remain reachable but receive no new features:

  * Capabilities introduced after Alpha — `snapshot` monitors, structured `output` with `basis`, `advanced_settings.location`, and `processor` selection — are V1-only.
  * The Python and TypeScript SDKs expose typed bindings (`client.monitor.*`) only for V1. Alpha is reachable solely via the low-level HTTP client (`client.post("/v1alpha/monitors", ...)`).
  * The [Parallel CLI](/integrations/cli) targets V1 endpoints exclusively.
</Note>

## What stays the same

* Monitor IDs, event group IDs, frequency, webhook URL, and metadata.
* The execution backend. Detection quality does not change because a client switches paths.
* Webhook **event types** (`monitor.event.detected`, `monitor.execution.completed`, `monitor.execution.failed`) and the envelope (`type`, `timestamp`, `data.monitor_id`, `data.event.event_group_id`, `data.metadata`).
* You do **not** need to recreate monitors or re-register webhooks.

Processor (`lite` / `base`), geo `location`, and `snapshot` monitors apply to **new** V1 creates. Alpha create has no processor field.

## Highlights

* **Not a drop-in path swap** — same monitors, breaking client contract. Update parsers, pagination, and field paths.
* **Required `type` discriminant** — `"event_stream"` (Alpha behavior) or `"snapshot"`. See [Snapshot Quickstart](/monitor-api/quickstart-snapshot).
* **Nested `settings` / `advanced_settings`** — `query`, `output_schema`, `include_backfill` move under `settings`; `source_policy` and `location` move under `settings.advanced_settings`.
* **Unified events endpoint** — `GET /v1/monitors/{id}/events` replaces both Alpha `/events` and `/event_groups/{id}`, with cursor pagination and an optional `event_group_id` filter.
* **Restructured event payload** — `event_id`, `event_type`, and typed `output` with `basis` replace the string `output`, `result`, and `source_urls`.
* **`simulate_event` removed** — closest analogue is `POST /{id}/trigger`, which enqueues a real run rather than a synthetic webhook.
* **V1-only SDKs and CLI** — typed `client.monitor.*` bindings and the [Parallel CLI](/integrations/cli).

## Pick your migration path

Most Alpha traffic is fetch-after-webhook or lookback polling, not create. Effort depends on which endpoints you call:

| If you currently... | V1 equivalent | Drop-in? |
| - | - | - |
| `GET /{id}/event_groups/{event_group_id}` after a webhook | `GET /v1/monitors/{id}/events?event_group_id=` | **Path yes, body no** — remap event JSON |
| `GET /{id}/events?lookback_period=10d` | `GET /v1/monitors/{id}/events?cursor=&limit=` | **No** — add pagination, completions are opt-in, rewrite the event model |
| `GET /v1alpha/monitors/list` | `GET /v1/monitors` | **Almost** — `data` → `monitors`; V1 defaults to `status=active` |
| `GET /v1alpha/monitors/{id}` | `GET /v1/monitors/{id}` | **Body reshape** — settings nest; `canceled` → `cancelled` |
| `POST /{id}/simulate_event` | Removed. `POST /{id}/trigger` runs a real execution | **No** — no synthetic webhook |

Webhook-then-fetch clients keep the webhook, swap the GET URL, and remap `result` / `source_urls` → `output` / `basis`. Lookback pollers also have to page instead of dumping a 10-day window. `simulate_event` is a product gap, not a rename.

## Endpoints

| Operation | Alpha | V1 |
| - | - | - |
| **Base path** | `/v1alpha/monitors` | `/v1/monitors` |
| **Create** | `POST /v1alpha/monitors` | `POST /v1/monitors` |
| **List (paginated)** | `GET /v1alpha/monitors/list` (`data` + `next_cursor`) | `GET /v1/monitors` (`monitors` + `next_cursor`); `type` and `status` filters; defaults to `status=active` |
| **List (unpaginated)** | `GET /v1alpha/monitors` (JSON array, all statuses) | Use `GET /v1/monitors` with pagination |
| **Retrieve** | `GET /v1alpha/monitors/{monitor_id}` | `GET /v1/monitors/{monitor_id}` |
| **Update** | `POST /v1alpha/monitors/{monitor_id}` | `POST /v1/monitors/{monitor_id}/update` |
| **Cancel** | `DELETE /v1alpha/monitors/{monitor_id}` | `POST /v1/monitors/{monitor_id}/cancel` |
| **Trigger one-off run** | — | `POST /v1/monitors/{monitor_id}/trigger` |
| **List events** | `GET /v1alpha/monitors/{monitor_id}/events?lookback_period=10d` | `GET /v1/monitors/{monitor_id}/events?cursor=&limit=&include_completions=` |
| **Single execution events** | `GET /v1alpha/monitors/{monitor_id}/event_groups/{event_group_id}` | `GET /v1/monitors/{monitor_id}/events?event_group_id=...` |
| **Simulate event** | `POST /v1alpha/monitors/{monitor_id}/simulate_event` | Removed; `POST /{monitor_id}/trigger` executes a real run instead of dispatching a synthetic event |

## Events

V1 unifies Alpha `/events` and `/event_groups/{id}` on `GET /v1/monitors/{id}/events`. Changing the URL is not enough — each event is a different JSON object. A client that only checks `events.length` may not notice; anything that renders text or citations will break until field paths change.

### List events (lookback pollers)

This is the largest break for clients that poll a window of history.

| Behavior | Alpha | V1 |
| - | - | - |
| Window | `lookback_period` (default `10d`); up to 300 event groups, whichever is less | No lookback parameter. Cursor pagination (`limit` default 20, max 100), newest first |
| Completions | Always included (`type: "completion"`) | Off unless `include_completions=true` |
| Errors | Always included | Always included |
| Order | Reverse chronological; events from a group flattened into the list | Newest first |

There is no V1 equivalent of "give me the last 10 days." Page with `next_cursor` until you have enough history.

<CodeGroup>
  ```bash cURL theme={"system"}
  # Alpha
  curl "https://api.parallel.ai/v1alpha/monitors/${MONITOR_ID}/events?lookback_period=10d" \
    -H "x-api-key: $PARALLEL_API_KEY"

  # V1 — page until next_cursor is absent
  curl "https://api.parallel.ai/v1/monitors/${MONITOR_ID}/events?limit=100" \
    -H "x-api-key: $PARALLEL_API_KEY"
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  # Alpha: client.get(f"/v1alpha/monitors/{monitor_id}/events", ...)
  page = client.monitor.events(monitor_id, limit=100)
  while True:
      for event in page.events:
          # completion/error events have event_type and timestamp only — no event_id
          print(getattr(event, "event_id", None), event.event_type)
      if not page.next_cursor:
          break
      page = client.monitor.events(monitor_id, limit=100, cursor=page.next_cursor)
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({ apiKey: process.env.PARALLEL_API_KEY });

  // Alpha: client.get(`/v1alpha/monitors/${monitorId}/events`, ...)
  let cursor: string | undefined;
  do {
    const page = await client.monitor.events(monitorId, { limit: 100, cursor });
    for (const event of page.events) {
      // completion/error events have event_type and timestamp only — no event_id
      const eventId = "event_id" in event ? event.event_id : undefined;
      console.log(eventId, event.event_type);
    }
    cursor = page.next_cursor ?? undefined;
  } while (cursor);
  ```
</CodeGroup>

Pass `include_completions=true` if you relied on Alpha’s completion placeholders to audit runs that detected nothing.

### Fetch by event group (webhook-then-fetch)

Webhooks still fire with `event_group_id`. Resolve it on the unified events endpoint:

```
GET /v1/monitors/{id}/events?event_group_id={event_group_id}
```

Pagination params are ignored when `event_group_id` is set. The response is **only detected events** for that run — no completion placeholder, and no Alpha `simulate_event` dummy payload.

<CodeGroup>
  ```bash cURL theme={"system"}
  # Alpha
  curl "https://api.parallel.ai/v1alpha/monitors/${MONITOR_ID}/event_groups/${EVENT_GROUP_ID}" \
    -H "x-api-key: $PARALLEL_API_KEY"

  # V1
  curl "https://api.parallel.ai/v1/monitors/${MONITOR_ID}/events?event_group_id=${EVENT_GROUP_ID}" \
    -H "x-api-key: $PARALLEL_API_KEY"
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  # Alpha: client.get(f"/v1alpha/monitors/{monitor_id}/event_groups/{event_group_id}", ...)
  result = client.monitor.events(monitor_id, event_group_id=event_group_id)
  for event in result.events:
      if event.event_type == "event_stream":
          print(event.output.content)
      elif event.event_type == "snapshot":
          print(event.changed_output)
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({ apiKey: process.env.PARALLEL_API_KEY });

  // Alpha: client.get(`/v1alpha/monitors/${monitorId}/event_groups/${eventGroupId}`, ...)
  const result = await client.monitor.events(monitorId, {
    event_group_id: eventGroupId,
  });
  for (const event of result.events) {
    if (event.event_type === "event_stream") {
      console.log(event.output.content);
    } else if (event.event_type === "snapshot") {
      console.log(event.changed_output);
    }
  }
  ```
</CodeGroup>

### Event JSON rewrite

Same run, same `event_group_id`. The object inside `events[]` changed.

<CodeGroup>
  ```json Alpha theme={"system"}
  {
    "events": [
      {
        "type": "event",
        "event_group_id": "mevtgrp_b0079f70195e4258eab1e7284340f1a9ec3a8033ed236a24",
        "output": "New product launch announced",
        "event_date": "2025-01-15",
        "source_urls": ["https://example.com/news"],
        "result": {
          "type": "text",
          "content": "New product launch announced"
        }
      }
    ]
  }
  ```

  ```json V1 theme={"system"}
  {
    "events": [
      {
        "event_type": "event_stream",
        "event_id": "mevt_323b37562d1bec451c5bab674ee5afaf2ddd17674e99cd5f4e99cd5f",
        "event_group_id": "mevtgrp_b0079f70195e4258eab1e7284340f1a9ec3a8033ed236a24",
        "event_date": "2025-01-15",
        "output": {
          "type": "text",
          "content": "New product launch announced",
          "basis": [
            {
              "field": "output",
              "citations": [{ "url": "https://example.com/news" }],
              "reasoning": "Article announces the product launch.",
              "confidence": "high"
            }
          ]
        }
      }
    ]
  }
  ```
</CodeGroup>

| Alpha field | V1 field |
| - | - |
| `type: "event"` | `event_type: "event_stream"` (or `"snapshot"`) |
| — | `event_id` (new; stable; use for idempotent dedup) |
| `event_group_id` | unchanged |
| `result.content` | `output.content` |
| `result.type` | `output.type` |
| `source_urls[]` | `output.basis[].citations[].url` |
| string `output` | removed (V1 `output` is an object) |
| `event_date` | **not the same value** — Alpha is the extracted real-world event date; V1 is the **run date** |

Clients that filter or display `event_date` as “when the news happened” will see different values after migrating.

Completion and error rows in a list-events response also renamed:

| Alpha | V1 |
| - | - |
| `type: "completion"`, `monitor_ts` | `event_type: "completion"`, `timestamp` |
| `type: "error"`, `error`, `id`, `date` | `event_type: "error"`, `error_message`, `timestamp` |

See [Events](/monitor-api/monitor-events) and [Research Basis](/task-api/guides/access-research-basis) for the full V1 schemas.

## List monitors

`GET /v1alpha/monitors/list` → `GET /v1/monitors`. Still cursor-paginated, newest first.

| Behavior | Alpha `/list` | V1 |
| - | - | - |
| Collection field | `data` | `monitors` |
| Cursor | `next_cursor` | `next_cursor` (unchanged) |
| Status filter | not a query param | defaults to **`status=active` only**; pass `status=cancelled` or both values to include cancelled monitors |
| Type filter | — | optional `type=event_stream` / `type=snapshot` |

Alpha also had `GET /v1alpha/monitors` (no `/list`), which returned a **bare JSON array** of every status. That shape does not exist on V1 — always read `monitors` from the paginated object.

Status spelling is `cancelled` (two L’s) on V1, vs Alpha `canceled`.

## Simulate event

`POST /v1alpha/monitors/{id}/simulate_event` is **gone**. It dispatched a synthetic webhook (and a dummy event group you could GET) without running the monitor.

`POST /v1/monitors/{id}/trigger` enqueues a **real** off-schedule execution. A webhook fires only if that run detects a change, completes with no detections, or fails — not a canned payload. Cancelled monitors cannot be triggered.

If your integration tests depended on dummy `event_group_id` payloads, stub the webhook yourself or trigger a real run against a test monitor. See the Alpha-only [Simulate Event](/monitor-api/monitor-simulate-event) page for the old contract.

## Create, update, and retrieve

Existing monitors do not need to be recreated. Create mappings are below; updates have a distinct shape and are **not** the nested create body with a different URL.

### Create request

| Concept | Alpha | V1 |
| - | - | - |
| **Monitor type** | implicit; search-query monitors only | `type: "event_stream"` or `type: "snapshot"` (required) |
| **Search query** | top-level `query` | `settings.query` (event\_stream only) |
| **Output schema** | top-level `output_schema` | `settings.output_schema` (event\_stream only) |
| **Backfill** | top-level `include_backfill` | `settings.include_backfill` (event\_stream only) |
| **Source policy** | top-level `source_policy` | `settings.advanced_settings.source_policy` (event\_stream only) |
| **Geo (`location`)** (new) | — | `settings.advanced_settings.location` (ISO 3166-1 alpha-2, e.g. `"us"`, `"gb"`) |
| **Snapshot baseline** (new) | — | `settings.task_run_id` (snapshot only) |
| **Processor** (new) | — | top-level `processor: "lite" \| "base"` (defaults to `"lite"`) |
| **Frequency** | top-level `frequency` (`1h`–`30d`) | unchanged |
| **Webhook** | top-level `webhook` | unchanged |
| **Metadata** | top-level `metadata` | unchanged |

### Update request

Alpha `POST /{id}` accepted top-level `query`, `source_policy`, `frequency`, `webhook`, and `metadata`. V1 `POST /{id}/update` only changes fields you include; omit a field to leave it unchanged. Empty updates fail validation.

| Concept | Alpha | V1 |
| - | - | - |
| **Query** | top-level `query` | `settings.query` — also send `type: "event_stream"` |
| **Source policy** | top-level `source_policy` | `settings.advanced_settings.source_policy` — also send `type: "event_stream"` |
| **Frequency / webhook / metadata** | top-level | unchanged; omit `type` and `settings` |
| **Processor** (new) | — | top-level `processor` |
| **`type` discriminant** | — | **Required whenever `settings` is present.** Must be `"event_stream"` (snapshot monitors have no updatable type-specific settings). Omit `type` when you are not sending `settings`. |

`null` clears only `webhook`, `metadata`, and `settings.advanced_settings`. Every other field rejects `null` — omit it instead.

Updating a query without `type` returns a validation error:

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/monitors/${MONITOR_ID}/update \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "type": "event_stream",
      "settings": {
        "query": "AI startup funding announcements"
      }
    }'
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  client.monitor.update(
      monitor_id,
      type="event_stream",
      settings={"query": "AI startup funding announcements"},
  )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({ apiKey: process.env.PARALLEL_API_KEY });

  await client.monitor.update(monitorId, {
    type: "event_stream",
    settings: { query: "AI startup funding announcements" },
  });
  ```
</CodeGroup>

Frequency, webhook, or metadata updates do not need `type`:

```python theme={"system"}
client.monitor.update(monitor_id, frequency="1w")
```

### Response

| Field | Alpha | V1 |
| - | - | - |
| `type` | — | new — `"event_stream"` or `"snapshot"` |
| `query` | top-level | now at `settings.query` |
| `output_schema` | top-level | now at `settings.output_schema` |
| `source_policy` | top-level | now at `settings.advanced_settings.source_policy` |
| `include_backfill` | top-level | now at `settings.include_backfill` |
| `cadence` | top-level (deprecated; `daily`/`weekly`/etc.) | removed; use `frequency` |
| `status` | `"active"` \| `"canceled"` (single-`l` spelling) | `"active"` \| `"cancelled"` (double-`l` spelling) |
| `last_run_at` | present | unchanged |
| `output` (snapshot only) | — | new; latest snapshot value for `type=snapshot` monitors |

## SDK and CLI surface

V1 exposes typed bindings in both the Python and TypeScript SDKs and is the only version supported by the [Parallel CLI](/integrations/cli). Alpha has no typed or CLI surface — it is reachable only via the low-level HTTP client.

| Operation | Alpha (Python) | V1 (Python) |
| - | - | - |
| Create | `client.post("/v1alpha/monitors", body=...)` | `client.monitor.create(...)` |
| List | `client.get("/v1alpha/monitors/list", ...)` | `client.monitor.list(...)` |
| Retrieve | `client.get("/v1alpha/monitors/{id}", ...)` | `client.monitor.retrieve(monitor_id)` |
| Update | `client.post("/v1alpha/monitors/{id}", body=...)` | `client.monitor.update(monitor_id, ...)` |
| Cancel | `client.delete("/v1alpha/monitors/{id}", ...)` | `client.monitor.cancel(monitor_id)` |
| Trigger | — | `client.monitor.trigger(monitor_id)` |
| Events | `client.get("/v1alpha/monitors/{id}/events", ...)` | `client.monitor.events(monitor_id, ...)` |

## Migration example: create

### Before (Alpha)

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1alpha/monitors \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "query": "AI startup funding announcements",
      "frequency": "1d",
      "include_backfill": false,
      "source_policy": {
        "include_domains": ["techcrunch.com", "bloomberg.com"]
      },
      "webhook": {
        "url": "https://example.com/webhook",
        "event_types": ["monitor.event.detected"]
      },
      "metadata": { "external_id": "acme-monitor-001" }
    }'
  ```

  ```python Python theme={"system"}
  import os
  from httpx import Response
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  monitor = client.post(
      "/v1alpha/monitors",
      cast_to=Response,
      body={
          "query": "AI startup funding announcements",
          "frequency": "1d",
          "include_backfill": False,
          "source_policy": {
              "include_domains": ["techcrunch.com", "bloomberg.com"],
          },
          "webhook": {
              "url": "https://example.com/webhook",
              "event_types": ["monitor.event.detected"],
          },
          "metadata": {"external_id": "acme-monitor-001"},
      },
  ).json()

  print(f"Monitor ID: {monitor['monitor_id']}")
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({ apiKey: process.env.PARALLEL_API_KEY });

  const monitor = (await client.post("/v1alpha/monitors", {
    body: {
      query: "AI startup funding announcements",
      frequency: "1d",
      include_backfill: false,
      source_policy: {
        include_domains: ["techcrunch.com", "bloomberg.com"],
      },
      webhook: {
        url: "https://example.com/webhook",
        event_types: ["monitor.event.detected"],
      },
      metadata: { external_id: "acme-monitor-001" },
    },
  })) as { monitor_id: string; status: string };

  console.log(`Monitor ID: ${monitor.monitor_id}`);
  ```
</CodeGroup>

### After (V1)

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/monitors \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "type": "event_stream",
      "frequency": "1d",
      "processor": "lite",
      "settings": {
        "query": "AI startup funding announcements",
        "include_backfill": false,
        "advanced_settings": {
          "source_policy": {
            "include_domains": ["techcrunch.com", "bloomberg.com"]
          },
          "location": "us"
        }
      },
      "webhook": {
        "url": "https://example.com/webhook",
        "event_types": ["monitor.event.detected"]
      },
      "metadata": { "external_id": "acme-monitor-001" }
    }'
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  monitor = client.monitor.create(
      type="event_stream",
      frequency="1d",
      processor="lite",
      settings={
          "query": "AI startup funding announcements",
          "include_backfill": False,
          "advanced_settings": {
              "source_policy": {
                  "include_domains": ["techcrunch.com", "bloomberg.com"],
              },
              "location": "us",
          },
      },
      webhook={
          "url": "https://example.com/webhook",
          "event_types": ["monitor.event.detected"],
      },
      metadata={"external_id": "acme-monitor-001"},
  )

  print(f"Monitor ID: {monitor.monitor_id}")
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({ apiKey: process.env.PARALLEL_API_KEY });

  const monitor = await client.monitor.create({
    type: "event_stream",
    frequency: "1d",
    processor: "lite",
    settings: {
      query: "AI startup funding announcements",
      include_backfill: false,
      advanced_settings: {
        source_policy: {
          include_domains: ["techcrunch.com", "bloomberg.com"],
        },
        location: "us",
      },
    },
    webhook: {
      url: "https://example.com/webhook",
      event_types: ["monitor.event.detected"],
    },
    metadata: { external_id: "acme-monitor-001" },
  });

  console.log(`Monitor ID: ${monitor.monitor_id}`);
  ```
</CodeGroup>

## Migration checklist

### Required changes

* Keep existing monitor IDs; do not recreate monitors solely to move to V1.
* Update the base path from `/v1alpha/monitors` to `/v1/monitors`.
* Add the `type` discriminant (`"event_stream"` or `"snapshot"`) to every `CreateMonitorRequest`.
* Move `query`, `output_schema`, and `include_backfill` from top-level into `settings`.
* Move `source_policy` from top-level into `settings.advanced_settings.source_policy`.
* Migrate Update calls from `POST /{id}` to `POST /{id}/update`.
* When an update includes `settings` (query or source policy), also send `type: "event_stream"`. Omit `type` when you are only changing frequency, webhook, or metadata.
* Migrate Cancel calls from `DELETE /{id}` to `POST /{id}/cancel`.
* Replace `GET /{id}/event_groups/{event_group_id}` with `GET /{id}/events?event_group_id=...`.
* Replace `lookback_period` with cursor pagination (`cursor`, `limit`). Do not assume a single response covers 10 days.
* Treat completions as opt-in (`include_completions=true`); Alpha always returned them.
* Remap list responses from `data` to `monitors`. Pass `status` if you need cancelled monitors.
* Update the status enum check from `"canceled"` to `"cancelled"` (double `l`).
* Replace reads of `result.content` and `source_urls` with `output.content` and `output.basis[].citations[].url`.
* Drop the deprecated top-level string `output` field on event records.
* Stop treating `event_date` as the extracted real-world date; V1 uses the run date.
* Remove `simulate_event` calls. `POST /{id}/trigger` is a real run, not a fake webhook.

### Optional enhancements

* Set `processor: "base"` for harder queries that need higher recall and breadth.
* Set `settings.advanced_settings.location` to scope retrieval to a single country.
* Replace low-level `client.post(...)` calls with `client.monitor.*` SDK bindings.
* Use `event_id` for idempotent client-side dedup across pagination and webhook retries.
* Consume `output.basis` for per-field citations, reasoning, and confidence.
* Issue `POST /{id}/trigger` for off-schedule one-off executions.
* Pass `include_completions=true` to enumerate executions that produced no events (useful for audit traces).
* Adopt `type=snapshot` for field-level diffing of structured Task Run outputs (see [Snapshot Quickstart](/monitor-api/quickstart-snapshot)).

## Additional resources

* [Monitor Quickstart](/monitor-api/monitor-quickstart) — V1 monitor lifecycle and webhook walkthrough
* [Snapshot Quickstart](/monitor-api/quickstart-snapshot) — `type=snapshot` walkthrough
* [Events](/monitor-api/monitor-events) — V1 event schemas and retrieval semantics
* [Webhooks](/monitor-api/monitor-webhooks) — payload schemas and HMAC verification
* [Simulate Event](/monitor-api/monitor-simulate-event) — Alpha-only synthetic webhook (removed in V1)
* [V1 API Reference](/api-reference/monitor/create-monitor) — generated from `public-openapi.json`
* [Legacy Alpha API Reference](/api-reference/legacy/monitor-alpha/create-monitor) — `/v1alpha/monitors` endpoint reference

Questions? Contact [support@parallel.ai](mailto:support@parallel.ai).
