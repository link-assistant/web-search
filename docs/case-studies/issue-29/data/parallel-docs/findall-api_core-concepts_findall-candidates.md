> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Candidates

> Understanding FindAll candidates, their structure, states, and how to exclude specific entities

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## Overview

A **candidate** is an entity that FindAll discovers during the generation phase of a run. Each candidate represents a potential match that is evaluated against your match conditions.

### Candidate States

Candidates progress through these states during evaluation:

* **Generated**: Discovered from web data, queued for evaluation
* **Matched**: Successfully satisfied all match conditions
* **Unmatched**: Failed to satisfy one or more match conditions
* **Discarded**: Removed from further evaluation because the candidate was invalid, irrelevant, or duplicated

<Note>
  **Post-Match Events**: When using [Streaming Events](/findall-api/features/findall-sse) or [Webhooks](/findall-api/features/findall-webhook), you may receive **`enriched`** events for matched candidates. These are event types (not `match_status` values) that indicate when additional data has been extracted via enrichments after a candidate has already matched.
</Note>

## Candidate Object Structure

Candidates in FindAll results, SSE events, and webhook payloads use this model. Webhook payloads also include `findall_id`. Optional fields are omitted when they are not available.

| Property | Type | Description |
| - | - | - |
| `candidate_id` | string | Unique identifier for the candidate |
| `name` | string | Name of the entity |
| `url` | string | Primary URL for the entity |
| `description` | string, optional | Brief description of the entity, when available |
| `match_status` | enum | One of `generated`, `matched`, `unmatched`, and `discarded` |
| `output` | object, optional | Key-value pairs showing evaluation results for each match condition and enrichment; omitted before evaluation (see section below for more details) |
| `basis` | array\[FieldBasis], optional | Citations, reasoning, and confidence scores for evaluated fields. See [FieldBasis](/task-api/guides/access-research-basis#the-fieldbasis-object) for more details. |

### Understanding the `output` Field

The `output` field contains evaluation results where each key corresponds to a field name. Match conditions include an `is_matched` boolean, while enrichments do not:

```json theme={"system"}
{
  "founded_after_2020_check": {
    "value": "2021",
    "type": "match_condition",
    "is_matched": true
  },
  "ceo_name": {
    "value": "Ramin Hasani",
    "type": "enrichment"
  }
}
```

### Understanding the `basis` Field

The `basis` field provides citations, reasoning, and confidence scores for each field in `output`.

<Note>
  **For complete details on basis structure and usage**, see [Access Research Basis](/task-api/guides/access-research-basis).
</Note>

## Excluding Candidates

**Use case**: Excluding candidates is useful when you already know certain entities match your criteria (such as results from previous runs or entities you've already identified), allowing you to focus the run on discovering new matches.

<Note>
  FindAll uses intelligence to deduplicate and disambiguate candidates you provide in the exclude list, which handles aliases and entities with slightly different names or URL variations. However, using the most official and disambiguated name and URL is recommended for best results.
</Note>

Provide an `exclude_list` with up to 10,000 entries to prevent specific entities from being generated or evaluated. Excluded entities do not appear in results or events.

**Exclude list structure:** Array of objects with `name` (string) and `url` (string) fields.

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -X POST "https://api.parallel.ai/v1beta/findall/runs" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "objective": "FindAll portfolio companies of Khosla Ventures",
      "entity_type": "companies",
      "match_conditions": [
        {
          "name": "khosla_ventures_portfolio_check",
          "description": "Company must be a portfolio company of Khosla Ventures."
        }
      ],
      "generator": "core",
      "match_limit": 20,
      "exclude_list": [
        {"name": "Figure AI", "url": "https://www.figure.ai"},
        {"name": "Anthropic", "url": "https://www.anthropic.com"}
      ]
    }'
  ```

  ```python Python theme={"system"}
  from parallel import Parallel

  client = Parallel(api_key="YOUR_API_KEY")

  findall_run = client.beta.findall.create(
      objective="FindAll portfolio companies of Khosla Ventures",
      entity_type="companies",
      match_conditions=[
          {
              "name": "khosla_ventures_portfolio_check",
              "description": "Company must be a portfolio company of Khosla Ventures."
          }
      ],
      generator="core",
      match_limit=20,
      exclude_list=[
          {"name": "Figure AI", "url": "https://www.figure.ai"},
          {"name": "Anthropic", "url": "https://www.anthropic.com"}
      ]
  )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from 'parallel-web';

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY
  });

  const run = await client.beta.findall.create({
    objective: "FindAll portfolio companies of Khosla Ventures",
    entity_type: "companies",
    match_conditions: [
      {
        name: "khosla_ventures_portfolio_check",
        description: "Company must be a portfolio company of Khosla Ventures."
      }
    ],
    generator: "core",
    match_limit: 20,
    exclude_list: [
      { name: "Figure AI", url: "https://www.figure.ai" },
      { name: "Anthropic", url: "https://www.anthropic.com" }
    ]
  });
  ```
</CodeGroup>

## Retrieving Candidates

Candidates can be accessed through multiple methods:

* **[`/result` endpoint](/findall-api/findall-quickstart#step-4-get-results)**: Retrieve the current candidate snapshot while a run is active or after it stops. Active snapshots can also include `generated` candidates.
* **[Streaming Events](/findall-api/features/findall-sse)**: Stream candidates in real-time as they're generated and evaluated
* **[Webhooks](/findall-api/features/findall-webhook)**: Receive HTTP callbacks for the candidate event types emitted by the webhook runtime

## Related Topics

* **[FindAll Quickstart](/findall-api/findall-quickstart)**: Get started with FindAll API
* **[Generators and Pricing](/findall-api/core-concepts/findall-generator-pricing)**: Understand generator options and pricing
* **[Run Lifecycle](/findall-api/core-concepts/findall-lifecycle)**: Learn about run statuses and metrics
* **[Enrichments](/findall-api/features/findall-enrich)**: Extract additional data from matched candidates
* **[Streaming Events](/findall-api/features/findall-sse)**: Monitor candidates in real-time
* **[Webhooks](/findall-api/features/findall-webhook)**: Set up notifications for candidate events
* **[Access Research Basis](/task-api/guides/access-research-basis)**: Deep dive into citation and reasoning structure
