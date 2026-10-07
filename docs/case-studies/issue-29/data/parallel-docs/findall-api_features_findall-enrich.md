> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Enrichments

> Add non-boolean enrichment data to FindAll candidates without affecting match conditions

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

<Note>
  **Built on Task API**: FindAll enrichments use Task API [processors](/task-api/guides/choose-a-processor), [JSON output schemas](/task-api/guides/specify-a-task#output-schema), and pricing. FindAll supplies the candidate input and handles orchestration automatically, running a task on each matched candidate.
</Note>

## Overview

FindAll enrichments allow you to extract additional non-boolean information about candidates that should not be used as filters for matches. For example, if you're finding companies, you might want to extract the CEO name as pure enrichment data—something you want to know about each match, but not something that should affect whether a candidate matches your criteria.

## Match Conditions vs. Enrichments

Understanding the distinction between match conditions and enrichments is fundamental to using FindAll effectively.

| | **Match Conditions** | **Enrichments** |
| - | - | - |
| **Purpose** | Required criteria that determine whether a candidate is a match | Additional data fields extracted only for matched candidates |
| **When Executed** | During FindAll generation and evaluation process | **Only on matched candidates** using the Task API |
| **Output format** | Boolean match decision + extracted value | Structured JSON values defined by `output_schema` |
| **Type of Criteria** | Must be boolean/filterable (yes/no questions) | Can be any type of data extraction |
| **Affects Matching?** | ✅ Yes - determines which candidates reach `matched` status | ❌ No - does not affect which candidates match |
| **When to Add** | Must be defined when creating the run | Add after the run is created through `/enrich`; the endpoint can be called multiple times |
| **Example Questions** | • "Is the company founded after 2020?"<br />• "Has the company raised Series A funding?"<br />• "Is the company in the healthcare industry?" | • "What is the CEO's name?"<br />• "What is the company's revenue?"<br />• "What products does the company offer?" |

### Why This Separation Matters

This two-stage approach is efficient and cost-effective:

1. **Filter first**: Match conditions quickly narrow down candidates to relevant matches
2. **Enrich selectively**: Extract detailed data only from the matches that matter

This means you don't pay to enrich hundreds of candidates that won't match your criteria.

## Adding Enrichments

Enrichments can be added anytime after a FindAll run is created, even for completed runs. Once added:

* Enrichments will run on **all matches** (both ones that exist when the request is made and all future matches)
* If enrichments are present, **extend** will also perform the same set of enrichments on all extended matches

<Note>Adding an enrichment to a terminal run requeues the run while FindAll schedules and processes the enrichment.</Note>

## Creating Enrichments

<Info>
  **Task API Concepts Apply Here**: Enrichments use the same [task spec](/task-api/guides/specify-a-task) structure as Task API runs. You'll define:

  * **[Processors](/task-api/guides/choose-a-processor)**: Choose any supported Task API processor
  * **[Output Schema](/task-api/guides/specify-a-task#output-schema)**: Define structured JSON output (same format as Task API)
  * **[Pricing](/task-api/guides/execute-task-run#pricing)**: Charged according to Task API processor pricing

  You don't define `input_schema`. FindAll supplies `entity_name`, `entity_url`, and `entity_description` for each matched candidate; `entity_name` is always present.
</Info>

### Quick Example

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -X POST "https://api.parallel.ai/v1beta/findall/runs/findall_40e0ab8c10754be0b7a16477abb38a2f/enrich" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "processor": "core",
      "output_schema": {
        "type": "json",
        "json_schema": {
          "type": "object",
          "properties": {
            "ceo_name": {
              "type": "string",
              "description": "Name of the CEO"
            },
            "founding_year": {
              "type": "string",
              "description": "Year the company was founded"
            }
          },
          "required": ["ceo_name", "founding_year"],
          "additionalProperties": false
        }
      }
    }'
  ```

  ```python Python theme={"system"}
  from parallel import Parallel
  from pydantic import BaseModel, Field

  client = Parallel(api_key="YOUR_API_KEY")

  class CompanyEnrichment(BaseModel):
      ceo_name: str = Field(
          description="Name of the CEO"
      )
      founding_year: str = Field(
          description="Year the company was founded"
      )

  client.beta.findall.enrich(
      findall_id="findall_40e0ab8c10754be0b7a16477abb38a2f",
      processor="core",
      output_schema={
          "type": "json",
          "json_schema": CompanyEnrichment.model_json_schema()
      }
  )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from 'parallel-web';

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY
  });

  await client.beta.findall.enrich(
    "findall_40e0ab8c10754be0b7a16477abb38a2f",
    {
      processor: "core",
      output_schema: {
        type: "json",
        json_schema: {
          type: "object",
          properties: {
            ceo_name: {
              type: "string",
              description: "Name of the CEO"
            },
            founding_year: {
              type: "string",
              description: "Year the company was founded"
            }
          },
          required: ["ceo_name", "founding_year"],
          additionalProperties: false
        }
      }
    }
  );
  ```
</CodeGroup>

## Retrieving Enrichment Results

You can access enrichment results through multiple methods:

* **[Streaming Events](/findall-api/features/findall-sse)** (`/events`): Enrichment results stream in real-time as they complete
* **[Webhooks](/findall-api/features/findall-webhook)**: Subscribe to `findall.candidate.enriched` events to receive enrichment results via HTTP callbacks
* **Result endpoint** (`/result`): Enrichment data is included in candidate snapshots once it is available; the endpoint works while a run is active and after it stops

<Note>
  Enrichment data is added to the candidate's `output` object with `type: "enrichment"`. See [Candidates](/findall-api/core-concepts/findall-candidates) for details on how enrichments appear in the candidate structure.
</Note>

## Related Topics

### Task API Foundation

Enrichments are built on Task API, so these guides will help you understand how they work:

* **[Task API Quickstart](/task-api/task-quickstart)**: Learn the Task API that powers enrichments
* **[Specify a Task](/task-api/guides/specify-a-task)**: Master task\_spec structure and best practices
* **[Choose a Task Processor](/task-api/guides/choose-a-processor)**: Understand Task API processor options
* **[Execute Task Runs](/task-api/guides/execute-task-run)**: Learn about pricing and execution patterns

### FindAll Features

* **[Preview](/findall-api/features/findall-preview)**: Test queries with 5–10 evaluated candidates before running full searches
* **[Extend Runs](/findall-api/features/findall-extend)**: Increase match limits without paying new fixed costs
* **[Streaming Events](/findall-api/features/findall-sse)**: Receive real-time updates via Server-Sent Events
* **[Webhooks](/findall-api/features/findall-webhook)**: Configure HTTP callbacks for run completion and matches
* **[Run Lifecycle](/findall-api/core-concepts/findall-lifecycle)**: Understand run statuses and how to cancel runs
* **[API Reference](/api-reference/findall/add-enrichment-to-findall-run)**: Complete endpoint documentation
