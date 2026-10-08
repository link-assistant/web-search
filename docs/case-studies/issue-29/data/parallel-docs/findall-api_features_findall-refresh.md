> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Refresh Runs

> Rerun the same FindAll query with exclude_list to discover net new entities over time

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## Overview

Scheduled jobs allow you to run the same FindAll query on a regular basis to discover newly emerging entities. This is useful for ongoing discovery workflows such as market intelligence, lead generation, and competitive tracking.

Rather than manually re-running queries, you can programmatically create new FindAll runs using a previous run's schema, while excluding candidates you've already discovered.

## Use Cases

Scheduled FindAll jobs are particularly useful for:

* **Market monitoring**: Track new companies entering a market space over time
* **Lead generation**: Continuously discover new potential customers matching your criteria
* **Competitive intelligence**: Discover emerging competitors
* **Investment research**: Track new companies meeting specific investment criteria
* **Regulatory compliance**: Discover new entities that may require compliance review

## How It Works

Creating a scheduled FindAll job involves three steps:

1. **Retrieve the search schema** from a previous run and load the original enrichment request payloads from your own storage
2. **Create a new run** using that schema, with an exclude list of previously discovered candidates
3. **Reapply enrichments** using the original saved request payloads through the enrichment endpoint

This approach ensures:

* **Consistent criteria**: Reuse the same objective, entity type, and match conditions across runs
* **Fewer repeats**: Exclude candidates returned by earlier runs
* **Focused discovery**: Direct each new run toward candidates that are not already in your saved exclusion set

## Step 1: Retrieve the Search Schema

Get the schema from an existing FindAll run to reuse its `objective`, `entity_type`, `match_conditions`, `generator`, and `match_limit`:

<Warning>The schema response is not a lossless store of the original `/enrich` requests. In particular, MCP server configuration can be omitted or redacted. When you first call `/enrich`, persist the exact request payload in secure application storage and replay that saved payload for refresh runs. Do not treat `schema.enrichments` as the source of truth for requests that may contain `mcp_servers` or other sensitive configuration.</Warning>

<Note>The schema response also omits the original run's `metadata` and `webhook`. Supply new values when creating the refreshed run if you need them.</Note>

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -X GET "https://api.parallel.ai/v1beta/findall/runs/findall_40e0ab8c10754be0b7a16477abb38a2f/schema" \
    -H "x-api-key: $PARALLEL_API_KEY"
  ```

  ```python Python theme={"system"}
  from parallel import Parallel

  client = Parallel(api_key="YOUR_API_KEY")

  schema = client.beta.findall.schema(
      findall_id="findall_40e0ab8c10754be0b7a16477abb38a2f"
  )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from 'parallel-web';

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY
  });

  const schema = await client.beta.findall.schema("findall_40e0ab8c10754be0b7a16477abb38a2f");
  ```
</CodeGroup>

**Response:**

```json theme={"system"}
{
  "objective": "Find all portfolio companies of Khosla Ventures founded after 2020",
  "entity_type": "companies",
  "match_conditions": [
    {
      "name": "khosla_ventures_portfolio_check",
      "description": "Company must be a portfolio company of Khosla Ventures."
    },
    {
      "name": "founded_after_2020_check",
      "description": "Company must have been founded after 2020."
    }
  ],
  "enrichments": [
    {
      "processor": "core",
      "output_schema": {
        "type": "json",
        "json_schema": {
          "type": "object",
          "properties": {
            "funding_amount": {
              "type": "string",
              "description": "Total funding raised by the company in USD"
            }
          }
        }
      }
    }
  ],
  "generator": "core",
  "match_limit": 50
}
```

## Step 2: Create a New Run and Replay Saved Enrichment Requests

Use the retrieved search criteria to create a new FindAll run, adding an `exclude_list` parameter to skip candidates you've already discovered. The create endpoint does not accept `enrichments`; after creation, send each original enrichment request that you persisted in your application.

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -X POST "https://api.parallel.ai/v1beta/findall/runs" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "objective": "Find all portfolio companies of Khosla Ventures founded after 2020",
      "entity_type": "companies",
      "match_conditions": [
        {
          "name": "khosla_ventures_portfolio_check",
          "description": "Company must be a portfolio company of Khosla Ventures."
        },
        {
          "name": "founded_after_2020_check",
          "description": "Company must have been founded after 2020."
        }
      ],
      "generator": "core",
      "match_limit": 50,
      "exclude_list": [
        {
          "name": "Anthropic",
          "url": "https://www.anthropic.com/"
        },
        {
          "name": "Adept AI",
          "url": "https://adept.ai/"
        },
        {
          "name": "Liquid AI",
          "url": "https://www.liquid.ai/"
        }
      ]
    }'

  # Replay the exact original /enrich payload from secure application storage
  curl -X POST "https://api.parallel.ai/v1beta/findall/runs/findall_NEW_RUN_ID/enrich" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "processor": "core",
      "output_schema": {
        "type": "json",
        "json_schema": {
          "type": "object",
          "properties": {
            "funding_amount": {
              "type": "string",
              "description": "Total funding raised by the company in USD"
            }
          }
        }
      }
    }'
  ```

  ```python Python theme={"system"}
  from parallel import Parallel

  client = Parallel(api_key="YOUR_API_KEY")

  # Persist this exact payload securely when you first add the enrichment.
  # Load it from your database or secrets manager in production.
  SAVED_ENRICHMENT_REQUESTS = [
      {
          "processor": "core",
          "output_schema": {
              "type": "json",
              "json_schema": {
                  "type": "object",
                  "properties": {
                      "funding_amount": {
                          "type": "string",
                          "description": "Total funding raised by the company in USD"
                      }
                  }
              }
          }
      }
  ]

  schema = client.beta.findall.schema(
      findall_id="findall_40e0ab8c10754be0b7a16477abb38a2f"
  )

  findall_run = client.beta.findall.create(
      objective=schema.objective,
      entity_type=schema.entity_type,
      match_conditions=[condition.to_dict() for condition in schema.match_conditions],
      generator=schema.generator or "core",
      match_limit=schema.match_limit or 50,
      exclude_list=[
          {
              "name": "Anthropic",
              "url": "https://www.anthropic.com/"
          },
          {
              "name": "Adept AI",
              "url": "https://adept.ai/"
          },
          {
              "name": "Liquid AI",
              "url": "https://www.liquid.ai/"
          }
      ]
  )

  for enrichment_request in SAVED_ENRICHMENT_REQUESTS:
      client.beta.findall.enrich(
          findall_run.findall_id,
          **enrichment_request
      )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from 'parallel-web';

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY
  });

  // Persist these exact payloads securely when you first add the enrichments.
  // Load them from your database or secrets manager in production.
  const savedEnrichmentRequests = [
    {
      processor: "core",
      output_schema: {
        type: "json" as const,
        json_schema: {
          type: "object",
          properties: {
            funding_amount: {
              type: "string",
              description: "Total funding raised by the company in USD"
            }
          }
        }
      }
    }
  ];

  const schema = await client.beta.findall.schema("findall_40e0ab8c10754be0b7a16477abb38a2f");

  const run = await client.beta.findall.create({
    objective: schema.objective,
    entity_type: schema.entity_type,
    match_conditions: schema.match_conditions,
    generator: schema.generator,
    match_limit: schema.match_limit ?? 50,
    exclude_list: [
      {
        name: "Anthropic",
        url: "https://www.anthropic.com/"
      },
      {
        name: "Adept AI",
        url: "https://adept.ai/"
      },
      {
        name: "Liquid AI",
        url: "https://www.liquid.ai/"
      }
    ]
  });

  for (const enrichmentRequest of savedEnrichmentRequests) {
    await client.beta.findall.enrich(run.findall_id, enrichmentRequest);
  }
  ```
</CodeGroup>

### Exclude List Parameters

The `exclude_list` is an array of candidate objects to exclude. Each object contains:

| Parameter | Type | Required | Description |
| - | - | - | - |
| `name` | string | Yes | Name of the candidate to exclude |
| `url` | string | Yes | URL of the candidate to exclude |

**How exclusions work:**

* Candidates matching any entry in the `exclude_list` will be skipped during generation
* This prevents those entities from being returned or evaluated in the refreshed run
* FindAll uses both `name` and `url` to deduplicate and disambiguate exclusions; use the entity's official name and canonical URL for best results
* A request can contain at most 10,000 exclusion entries

## Building Your Exclude List

To construct the `exclude_list` from previous runs, retrieve candidates and extract their `name` and `url` fields:

```bash cURL theme={"system"}
curl -X GET "https://api.parallel.ai/v1beta/findall/runs/findall_40e0ab8c10754be0b7a16477abb38a2f/result" \
  -H "x-api-key: $PARALLEL_API_KEY"
```

The example below shows matched candidates:

```json theme={"system"}
{
  "run": {
    "findall_id": "findall_40e0ab8c10754be0b7a16477abb38a2f",
    "status": {
      "status": "completed",
      "is_active": false,
      "metrics": {
        "generated_candidates_count": 8,
        "matched_candidates_count": 2
      }
    },
    "generator": "core"
  },
  "candidates": [
    {
      "candidate_id": "candidate_abc123",
      "name": "Anthropic",
      "url": "https://www.anthropic.com/",
      "match_status": "matched"
    },
    {
      "candidate_id": "candidate_def456",
      "name": "Adept AI",
      "url": "https://adept.ai/",
      "match_status": "matched"
    }
  ],
  "last_event_id": "642c949cfbdcf"
}
```

Store these candidates and pass them as the `exclude_list` array in subsequent runs. Excluding only `matched` candidates prevents previous matches from being returned again while allowing earlier nonmatches to be reconsidered as web data changes. If you want the refreshed run to consider only entities that have never reached evaluation, save and exclude every candidate returned by `/result` instead.

Deduplicate the accumulated list before each request and fail explicitly if it exceeds 10,000 entries. Do not silently truncate it: truncation makes the refresh policy dependent on list order and can reintroduce older entities.

## Example: Weekly Scheduled Job

Here's a complete example showing how to set up a weekly FindAll job:

<CodeGroup>
  ```python Python theme={"system"}
  import json
  import os
  import time
  from datetime import datetime, timezone
  from pathlib import Path

  import requests

  PARALLEL_API_KEY = os.environ["PARALLEL_API_KEY"]
  BASE_URL = "https://api.parallel.ai/v1beta"
  HEADERS = {
      "x-api-key": PARALLEL_API_KEY,
      "Content-Type": "application/json"
  }
  ORIGINAL_FINDALL_ID = "findall_40e0ab8c10754be0b7a16477abb38a2f"
  STATE_FILE = Path("findall-refresh-state.json")
  MAX_EXCLUSIONS = 10_000

  # Save the exact original /enrich request payloads in secure configuration.
  # Include mcp_servers here if the original enrichment used them.
  SAVED_ENRICHMENT_REQUESTS = [
      {
          "processor": "core",
          "output_schema": {
              "type": "json",
              "json_schema": {
                  "type": "object",
                  "properties": {
                      "funding_amount": {
                          "type": "string",
                          "description": "Total funding raised by the company in USD"
                      }
                  }
              }
          }
      }
  ]

  def get_schema(findall_id):
      response = requests.get(
          f"{BASE_URL}/findall/runs/{findall_id}/schema",
          headers=HEADERS
      )
      response.raise_for_status()
      return response.json()

  def get_matched_candidates(findall_id):
      """Get all matched candidates from a run"""
      response = requests.get(
          f"{BASE_URL}/findall/runs/{findall_id}/result",
          headers=HEADERS
      )
      response.raise_for_status()
      candidates = response.json().get("candidates", [])
      return [c for c in candidates if c.get("match_status") == "matched"]

  def deduplicate_exclusions(candidates):
      unique = {}
      for candidate in candidates:
          name = candidate["name"].strip()
          url = candidate["url"].strip()
          unique[(name, url.rstrip("/"))] = {"name": name, "url": url}
      return list(unique.values())

  def load_exclusions():
      if not STATE_FILE.exists():
          return []
      state = json.loads(STATE_FILE.read_text())
      return deduplicate_exclusions(state.get("exclude_list", []))

  def save_exclusions(exclusions):
      # Replace the state file atomically so an interrupted write does not erase it.
      temporary_file = STATE_FILE.with_suffix(".tmp")
      temporary_file.write_text(json.dumps({"exclude_list": exclusions}, indent=2))
      temporary_file.replace(STATE_FILE)

  def create_scheduled_run(schema, exclusions):
      if len(exclusions) > MAX_EXCLUSIONS:
          raise RuntimeError(
              f"Saved exclusion set has {len(exclusions)} entries; "
              f"FindAll accepts at most {MAX_EXCLUSIONS}."
          )

      payload = {
          "objective": schema["objective"],
          "entity_type": schema["entity_type"],
          "match_conditions": schema["match_conditions"],
          "generator": schema.get("generator", "core"),
          "match_limit": schema.get("match_limit") or 50,
          "exclude_list": exclusions
      }

      response = requests.post(
          f"{BASE_URL}/findall/runs",
          headers=HEADERS,
          json=payload
      )
      response.raise_for_status()
      findall_id = response.json()["findall_id"]

      for enrichment_request in SAVED_ENRICHMENT_REQUESTS:
          enrich_response = requests.post(
              f"{BASE_URL}/findall/runs/{findall_id}/enrich",
              headers=HEADERS,
              json=enrichment_request
          )
          enrich_response.raise_for_status()

      return findall_id

  def wait_for_completion(findall_id):
      while True:
          response = requests.get(
              f"{BASE_URL}/findall/runs/{findall_id}",
              headers=HEADERS
          )
          response.raise_for_status()
          run_status = response.json()["status"]

          if not run_status["is_active"]:
              if run_status["status"] != "completed":
                  raise RuntimeError(
                      f"FindAll run stopped with status: {run_status['status']}"
                  )
              return

          time.sleep(30)

  def run_weekly_job():
      print(f"Starting scheduled job at {datetime.now(timezone.utc).isoformat()}")

      schema = get_schema(ORIGINAL_FINDALL_ID)
      exclusions = load_exclusions()
      if not exclusions:
          original_matches = get_matched_candidates(ORIGINAL_FINDALL_ID)
          exclusions = deduplicate_exclusions(original_matches)

      new_findall_id = create_scheduled_run(schema, exclusions)
      print(f"Created new run: {new_findall_id}")
      wait_for_completion(new_findall_id)

      new_candidates = get_matched_candidates(new_findall_id)
      print(f"Found {len(new_candidates)} new candidates")

      updated_exclusions = deduplicate_exclusions(exclusions + new_candidates)
      save_exclusions(updated_exclusions)
      if len(updated_exclusions) > MAX_EXCLUSIONS:
          print("The saved exclusion set now exceeds the API limit; choose a rotation policy before the next run.")

      return new_candidates

  if __name__ == "__main__":
      run_weekly_job()
  ```

  ```typescript TypeScript theme={"system"}
  import { readFile, rename, writeFile } from 'node:fs/promises';

  const PARALLEL_API_KEY = process.env.PARALLEL_API_KEY;
  if (!PARALLEL_API_KEY) throw new Error('PARALLEL_API_KEY is required');

  const BASE_URL = 'https://api.parallel.ai/v1beta';
  const HEADERS = {
    'x-api-key': PARALLEL_API_KEY,
    'Content-Type': 'application/json',
  };
  const ORIGINAL_FINDALL_ID = 'findall_40e0ab8c10754be0b7a16477abb38a2f';
  const STATE_FILE = 'findall-refresh-state.json';
  const MAX_EXCLUSIONS = 10_000;

  type Exclusion = { name: string; url: string };

  // Save the exact original /enrich request payloads in secure configuration.
  // Include mcp_servers here if the original enrichment used them.
  const savedEnrichmentRequests = [
    {
      processor: 'core',
      output_schema: {
        type: 'json',
        json_schema: {
          type: 'object',
          properties: {
            funding_amount: {
              type: 'string',
              description: 'Total funding raised by the company in USD',
            },
          },
        },
      },
    },
  ];

  async function requestJson(path: string, init: RequestInit = {}) {
    const response = await fetch(`${BASE_URL}${path}`, {
      ...init,
      headers: HEADERS,
    });
    if (!response.ok) {
      throw new Error(`Parallel API request failed: ${response.status} ${await response.text()}`);
    }
    return response.json();
  }

  async function getSchema(findallId: string) {
    return requestJson(`/findall/runs/${findallId}/schema`);
  }

  async function getMatchedCandidates(findallId: string) {
    const result: any = await requestJson(`/findall/runs/${findallId}/result`);
    return (result.candidates ?? []).filter((candidate: any) => candidate.match_status === 'matched');
  }

  function deduplicateExclusions(candidates: Exclusion[]) {
    const unique = new Map<string, Exclusion>();
    for (const candidate of candidates) {
      const name = candidate.name.trim();
      const url = candidate.url.trim();
      unique.set(JSON.stringify([name, url.replace(/\/$/, '')]), { name, url });
    }
    return [...unique.values()];
  }

  async function loadExclusions(): Promise<Exclusion[]> {
    try {
      const state = JSON.parse(await readFile(STATE_FILE, 'utf8'));
      return deduplicateExclusions(state.exclude_list ?? []);
    } catch (error: any) {
      if (error.code === 'ENOENT') return [];
      throw error;
    }
  }

  async function saveExclusions(exclusions: Exclusion[]) {
    const temporaryFile = `${STATE_FILE}.tmp`;
    await writeFile(temporaryFile, JSON.stringify({ exclude_list: exclusions }, null, 2));
    await rename(temporaryFile, STATE_FILE);
  }

  async function createScheduledRun(
    schema: any,
    exclusions: Exclusion[]
  ) {
    if (exclusions.length > MAX_EXCLUSIONS) {
      throw new Error(
        `Saved exclusion set has ${exclusions.length} entries; FindAll accepts at most ${MAX_EXCLUSIONS}.`
      );
    }

    const payload = {
      objective: schema.objective,
      entity_type: schema.entity_type,
      match_conditions: schema.match_conditions,
      generator: schema.generator ?? 'core',
      match_limit: schema.match_limit ?? 50,
      exclude_list: exclusions,
    };

    const run: any = await requestJson('/findall/runs', {
      method: 'POST',
      body: JSON.stringify(payload),
    });

    for (const enrichmentRequest of savedEnrichmentRequests) {
      await requestJson(`/findall/runs/${run.findall_id}/enrich`, {
        method: 'POST',
        body: JSON.stringify(enrichmentRequest),
      });
    }

    return run.findall_id;
  }

  async function waitForCompletion(findallId: string) {
    while (true) {
      const run: any = await requestJson(`/findall/runs/${findallId}`);
      if (!run.status.is_active) {
        if (run.status.status !== 'completed') {
          throw new Error(`FindAll run stopped with status: ${run.status.status}`);
        }
        return;
      }
      await new Promise(resolve => setTimeout(resolve, 30_000));
    }
  }

  async function runWeeklyJob() {
    console.log(`Starting scheduled job at ${new Date()}`);

    const schema = await getSchema(ORIGINAL_FINDALL_ID);
    let exclusions = await loadExclusions();
    if (exclusions.length === 0) {
      const originalCandidates = await getMatchedCandidates(ORIGINAL_FINDALL_ID);
      exclusions = deduplicateExclusions(originalCandidates.map((candidate: any) => ({
        name: candidate.name,
        url: candidate.url,
      })));
    }

    const newFindallId = await createScheduledRun(schema, exclusions);
    console.log(`Created new run: ${newFindallId}`);
    await waitForCompletion(newFindallId);

    const newCandidates = await getMatchedCandidates(newFindallId);
    console.log(`Found ${newCandidates.length} new candidates`);

    const updatedExclusions = deduplicateExclusions([
      ...exclusions,
      ...newCandidates.map((candidate: any) => ({
        name: candidate.name,
        url: candidate.url,
      })),
    ]);
    await saveExclusions(updatedExclusions);

    if (updatedExclusions.length > MAX_EXCLUSIONS) {
      console.warn('The saved exclusion set now exceeds the API limit; choose a rotation policy before the next run.');
    }

    return newCandidates;
  }

  await runWeeklyJob();
  ```
</CodeGroup>

## Best Practices

### Schema Modifications

While you should keep `match_conditions` consistent across runs, you can adjust:

* **`objective`**: Update to reflect the current time period (e.g., "founded in 2024" → "founded in 2025")
* **Enrichment requests**: Replay the original request payloads from secure application storage—or add new enrichments—through `/enrich` after creating the new run
* **`match_limit`**: Adjust based on expected growth rate
* **`generator`**: Change generators if needed (though this may affect result quality)

### Exclude List Management

* **Persist candidates**: Store discovered candidate objects (name and URL) in a database or file for long-term tracking
* **Deduplicate before sending**: Remove repeated name-and-URL pairs before building each request
* **Normalize URLs**: Ensure consistent URL formatting (trailing slashes, protocols, etc.) across runs
* **Periodic resets**: Consider occasionally running without exclusions to catch entities that may have changed
* **Respect the limit**: An exclude list can contain at most 10,000 candidates; define a rotation or reset policy before the saved set reaches that size

### Scheduling

* **Frequency**: Choose intervals based on your domain's update rate (daily, weekly, monthly)
* **Off-peak hours**: Schedule jobs during low-traffic periods if possible
* **Durable state**: Use transactional database storage rather than a local file when multiple scheduler instances may run concurrently
* **Webhooks**: Use [webhooks](/findall-api/features/findall-webhook) to get notified when jobs complete
* **Error handling**: Implement retry logic for failed runs

### Cost Optimization

* **Start small**: Use lower `match_limit` values initially, then [extend](/findall-api/features/findall-extend) if needed
* **Preview first**: Test schema changes with [preview](/findall-api/features/findall-preview) before running full jobs
* **Monitor metrics**: Track `generated_candidates_count` vs `matched_candidates_count` to optimize criteria

## Related Topics

* **[Preview](/findall-api/features/findall-preview)**: Test queries with 5–10 evaluated candidates before running full searches
* **[Generators and Pricing](/findall-api/core-concepts/findall-generator-pricing)**: Understand generator options and pricing
* **[Enrichments](/findall-api/features/findall-enrich)**: Extract additional structured data for matched candidates
* **[Extend Runs](/findall-api/features/findall-extend)**: Increase match limits without paying new fixed costs
* **[Webhooks](/findall-api/features/findall-webhook)**: Configure HTTP callbacks for run completion and matches
* **[Streaming Events](/findall-api/features/findall-sse)**: Receive real-time updates via Server-Sent Events
* **[Run Lifecycle](/findall-api/core-concepts/findall-lifecycle)**: Understand run statuses and how to cancel runs
* **[API Reference](/api-reference/findall/get-findall-run-schema)**: Complete endpoint documentation
