> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# FindAll API Streaming Events

> Receive real-time updates on FindAll runs using Server-Sent Events (SSE)

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## Overview

The `/v1beta/findall/runs/{findall_id}/events` endpoint provides real-time updates on candidates as they are discovered and evaluated using Server-Sent Events (SSE). Run, candidate, and schema events are delivered in chronological order with `event_id`, `timestamp`, `type`, and `data`. Error events instead contain `type` and `error`.

**Resumability**: Use the `last_event_id` query parameter to resume after disconnections. Each run, candidate, or schema event has an `event_id`. The `/result` endpoint returns the ID of the latest persisted event as `last_event_id`; heartbeat status events are generated in memory and are not used for that result cursor. If no ID is supplied, the stream starts from the beginning.

**Duration**: Streams remain open while the run is active or until an optional `timeout` is reached. `timeout` is measured in seconds and must be between 0 and 6,000, inclusive. A `findall.status` heartbeat is sent every 10 seconds to keep connections alive.

## Accessing the Event Stream

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -N -X GET "https://api.parallel.ai/v1beta/findall/runs/findall_40e0ab8c10754be0b7a16477abb38a2f/events" \
    -H "x-api-key: ${PARALLEL_API_KEY}" \
    -H "Accept: text/event-stream"
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])
  findall_id = "findall_40e0ab8c10754be0b7a16477abb38a2f"

  print(f"Streaming events for FindAll run {findall_id}:")

  events = client.beta.findall.events(
      findall_id=findall_id,
      # last_event_id="some_previous_event_id",
      # api_timeout=30.0,
  )

  for event in events:
      print(f"Event [{event.type}]: {event.model_dump_json()}")
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from 'parallel-web';

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY
  });

  const findallId = "findall_40e0ab8c10754be0b7a16477abb38a2f";

  console.log(`Streaming events for FindAll run ${findallId}:`);

  const stream = await client.beta.findall.events(findallId, {
    // last_event_id: "some_previous_event_id",
    // timeout: 30.0,
  });

  for await (const event of stream) {
    // Events are already parsed JSON objects
    if ('type' in event) {
      console.log(`Event [${event.type}]: ${JSON.stringify(event)}`);
    }
  }
  ```
</CodeGroup>

## Event Types

The SSE endpoint emits the following event types:

| Event Type | Description |
| - | - |
| `findall.status` | Heartbeat of FindAllRun object every 10 seconds, or when FindAll status changes |
| `findall.candidate.generated` | Emitted when a new candidate is discovered, before evaluation |
| `findall.candidate.matched` | Emitted when a candidate successfully matches all match conditions |
| `findall.candidate.unmatched` | Emitted when a candidate fails to match all conditions |
| `findall.candidate.discarded` | Supported by the stream contract for a candidate removed from further evaluation; current FindAll processing does not produce this event |
| `findall.candidate.enriched` | Emitted when enrichment data has been extracted for a candidate |
| `findall.schema.updated` | Emitted when an enrichment or extension changes the run schema |
| `error` | Reports a stream error using an `error` object rather than `event_id`, `timestamp`, and `data` |

<Note>
  For a complete guide to candidate object structure, states, and fields, see [Candidates](/findall-api/core-concepts/findall-candidates).
</Note>

## Event Payloads

**findall.status** — Heartbeat of FindAllRun object every 10 seconds, or when FindAll status changes.

```json theme={"system"}
{
  "type": "findall.status",
  "timestamp": "2025-11-04T18:45:43.223633Z",
  "event_id": "641eebfb0d81f",
  "data": {
    "findall_id": "findall_40e0ab8c10754be0b7a16477abb38a2f",
    "status": {
      "status": "running",
      "is_active": true,
      "metrics": {
        "generated_candidates_count": 4,
        "matched_candidates_count": 0
      }
    },
    "generator": "core",
    "metadata": {},
    "created_at": "2025-11-04T18:40:02.123456Z",
    "modified_at": "2025-11-04T18:45:41.987654Z"
  }
}
```

**findall.candidate.\*** — Emitted as candidates are generated and evaluated:

<CodeGroup>
  ```json findall.candidate.generated [expandable] theme={"system"}
  {
    "type": "findall.candidate.generated",
    "timestamp": "2025-11-04T18:46:52.952095Z",
    "event_id": "641eebe8d11af",
    "data": {
      "candidate_id": "candidate_a062dd17-d77a-4b1b-ad0e-de113e82f838",
      "name": "Adept AI",
      "url": "https://adept.ai",
      "description": "Adept AI is a company founded in 2021...",
      "match_status": "generated"
    }
  }
  ```

  ```json findall.candidate.matched [expandable] theme={"system"}
  {
    "type": "findall.candidate.matched",
    "timestamp": "2025-11-04T18:48:22.366975Z",
    "event_id": "641eec0cb2ccf",
    "data": {
      "candidate_id": "candidate_ae13884c-dc93-4c62-81f2-1308a98e2621",
      "name": "Traba",
      "url": "https://traba.work/",
      "description": "Traba is a company founded in 2021...",
      "match_status": "matched",
      "output": {
        "founded_after_2020_check": {
          "value": "2021",
          "type": "match_condition",
          "is_matched": true
        }
      },
      "basis": [
        {
          "field": "founded_after_2020_check",
          "citations": [
            {
              "title": "Report: Traba Business Breakdown & Founding Story",
              "url": "https://research.contrary.com/company/traba",
              "excerpts": ["Traba, a labor marketplace founded in 2021..."]
            }
          ],
          "reasoning": "Multiple sources state that Traba was founded in 2021...",
          "confidence": "high"
        }
      ]
    }
  }
  ```

  ```json findall.candidate.unmatched [expandable] theme={"system"}
  {
    "type": "findall.candidate.unmatched",
    "timestamp": "2025-11-04T18:48:30.341999Z",
    "event_id": "641eebefb327f",
    "data": {
      "candidate_id": "candidate_76489c89-956e-4b5d-8784-e84a0abf3cbe",
      "name": "Twelve",
      "url": "https://www.capitaly.vc/blog/khosla-ventures-investment...",
      "description": "Twelve is a company that Khosla Ventures has invested in...",
      "match_status": "unmatched",
      "output": {
        "founded_after_2020_check": {
          "value": "2015",
          "type": "match_condition",
          "is_matched": false
        }
      },
      "basis": [
        {
          "field": "founded_after_2020_check",
          "citations": [],
          "reasoning": "The search results consistently indicate that Twelve was founded in 2015...",
          "confidence": "high"
        }
      ]
    }
  }
  ```

  ```json findall.candidate.enriched [expandable] theme={"system"}
  {
    "type": "findall.candidate.enriched",
    "timestamp": "2025-11-04T18:49:14.474959Z",
    "event_id": "642c949cfbdcf",
    "data": {
      "candidate_id": "candidate_5e30951e-435f-4785-b253-4b29f85ded9d",
      "name": "Liquid AI",
      "url": "https://www.liquid.ai/",
      "description": "Liquid AI is an AI company that raised $250 million in a Series A funding round...",
      "match_status": "matched",
      "output": {
        "ceo_name": {
          "value": "Ramin Hasani",
          "type": "enrichment"
        },
        "cto_name": {
          "value": "Mathias Lechner",
          "type": "enrichment"
        }
      },
      "basis": [
        {
          "field": "ceo_name",
          "citations": [
            {
              "title": "Ramin Hasani",
              "url": "https://www.liquid.ai/team/ramin-hasani",
              "excerpts": ["Ramin Hasani is the Co-founder and CEO of Liquid AI..."]
            }
          ],
          "reasoning": "The search results consistently identify Ramin Hasani as the CEO of Liquid AI...",
          "confidence": "high"
        },
        {
          "field": "cto_name",
          "citations": [
            {
              "title": "Mathias Lechner",
              "url": "https://www.liquid.ai/team/mathias-lechner",
              "excerpts": ["Mathias Lechner", "Co-founder & CTO"]
            }
          ],
          "reasoning": "The search results consistently identify Mathias Lechner as the CTO of Liquid AI...",
          "confidence": "high"
        }
      ]
    }
  }
  ```

  ```json findall.schema.updated [expandable] theme={"system"}
  {
    "type": "findall.schema.updated",
    "timestamp": "2025-11-04T18:50:00.123456Z",
    "event_id": "642c94a12bcde",
    "data": {
      "generator": "core",
      "match_limit": 60,
      "entity_type": "companies",
      "objective": "Find all portfolio companies of Khosla Ventures",
      "match_conditions": [
        {
          "name": "khosla_ventures_portfolio_check",
          "description": "Company must be a portfolio company of Khosla Ventures."
        }
      ]
    }
  }
  ```

  ```json error [expandable] theme={"system"}
  {
    "type": "error",
    "error": {
      "ref_id": "findall_40e0ab8c10754be0b7a16477abb38a2f",
      "message": "FindAll run failed due to an unexpected error. Please contact support or try again later.",
      "detail": null
    }
  }
  ```
</CodeGroup>

## Related Topics

* **[Preview](/findall-api/features/findall-preview)**: Test queries with 5–10 evaluated candidates before running full searches
* **[Generators and Pricing](/findall-api/core-concepts/findall-generator-pricing)**: Understand generator options and pricing
* **[Enrichments](/findall-api/features/findall-enrich)**: Extract additional structured data for matched candidates
* **[Extend Runs](/findall-api/features/findall-extend)**: Increase match limits without paying new fixed costs
* **[Webhooks](/findall-api/features/findall-webhook)**: Configure HTTP callbacks for run completion and matches
* **[Run Lifecycle](/findall-api/core-concepts/findall-lifecycle)**: Understand run statuses and how to cancel runs
* **[API Reference](/api-reference/findall/stream-findall-events)**: Complete endpoint documentation
