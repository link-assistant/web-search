> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Data Connectors

> Connect Task Runs to sources beyond the public web: Index Partners included by default, free and pay-per-use connectors you select per run, or a license you already hold

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Data Connectors let a Task Run or a Responses API request combine web research with specialized data from third-party providers. Browse what is available in the [Data Connectors directory](https://parallel.ai/data-connectors). The right access model depends on how the data is licensed and made available to your organization.

| Access model | Best for | How you use it |
| - | - | - |
| **Index Partners** | Data Parallel has licensed for every eligible run | Ask your question normally. Included by default, with nothing to configure. |
| **Pay-Per-Use** | A connector you want Parallel to query for a specific run | Name the connector in `advanced_settings.data_sources`, under `free` or `pay_per_use`. |
| **Bring Your Own License (BYOL)** | Data your organization already licenses or hosts | Add the provider's remote MCP server and your credentials to `mcp_servers`. |

All three complement Parallel's default web research. Pay-Per-Use connectors and BYOL servers are added to the run; they do not replace web search and planning.

The field names above are the Task API's. The Responses API takes the same connector names in a top-level `data_sources` field and BYOL servers as OpenAI `mcp` tools; see [Responses API](#responses-api).

## Index Partners

Index Partners are licensed by Parallel through [Index](https://index.parallel.ai/) and are included by default on every eligible run. Use the Task API normally, and the processor draws on them when they are relevant. There is nothing to configure, and nothing to select: naming an Index Partner in `data_sources` is rejected.

Availability varies by partner and by processor.

## Pay-Per-Use

Pay-Per-Use connectors are invoked explicitly, per run. Select them with the `advanced_settings.data_sources` field when creating a Task Run. The processor queries them the same way it queries the web: it decides when a connector is useful for the task, calls it, and cites what it used in the `basis`. You do not host anything or manage credentials.

Free connectors, such as PubMed and ClinicalTrials.gov, use the same explicit invocation model and are never billed. Paid connectors are billed per successful call.

| Parameter | Type | Description |
| - | - | - |
| `free` | `array[string]` | Free connectors to enable for this run. Never billed. |
| `pay_per_use` | `array[string]` | Pay-per-use connectors to enable for this run. Each successful call the processor makes is billed. |

Both lists are optional and empty by default.

### Sample request

<CodeGroup>
  ```bash cURL theme={"system"}
  curl -X POST "https://api.parallel.ai/v1/tasks/runs" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    --data '{
    "input": "Summarize the registered phase 3 trials for semaglutide in adolescents.",
    "processor": "core",
    "advanced_settings": {
      "data_sources": {
        "free": ["clinical_trials", "pubmed"]
      }
    }
  }'
  ```

  ```python Python theme={"system"}
  import os
  from parallel import Parallel

  client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

  task_run = client.task_run.create(
      input="Summarize the registered phase 3 trials for semaglutide in adolescents.",
      processor="core",
      advanced_settings={"data_sources": {"free": ["clinical_trials", "pubmed"]}},
  )
  ```

  ```typescript TypeScript theme={"system"}
  import Parallel from "parallel-web";

  const client = new Parallel({
    apiKey: process.env.PARALLEL_API_KEY,
  });

  const taskRun = await client.taskRun.create({
    input: "Summarize the registered phase 3 trials for semaglutide in adolescents.",
    processor: "core",
    advanced_settings: { data_sources: { free: ["clinical_trials", "pubmed"] } },
  });
  ```
</CodeGroup>

### Free connectors

| Name | Covers |
| - | - |
| `pubmed` | PubMed: biomedical and life-sciences literature, including article metadata, related articles, and citation lookup. |
| `clinical_trials` | ClinicalTrials.gov: registered clinical studies, their status, conditions, interventions, and sponsors. |
| `chembl` | ChEMBL: bioactive molecules and drug-like compounds, with targets, assays, and mechanism data. |
| `biorxiv` | bioRxiv and medRxiv: biology and health-sciences preprints. |
| `npi_registry` | NPI Registry (NPPES): US healthcare providers and organizations by National Provider Identifier. |
| `cms_coverage` | CMS Coverage: Medicare national and local coverage determinations. |

### Pay-per-use connectors

| Name | Covers |
| - | - |
| `carbonarc` | Carbon Arc: alternative data on companies and markets, including consumer spend, foot traffic, app and web activity, and events. |

### Billing

Pay-per-use connectors are billed through Parallel at a price per call set by the provider. Charges are based on successful connector tool calls and are separate from the Task processor charge. Free connectors are never billed.

### How Pay-Per-Use connectors are used

When a run starts, Parallel attaches the selected connectors as tools. The processor calls a connector only when it judges it relevant to the task, so a selected connector may be called several times or not at all.

Every call appears in the run output's `mcp_tool_calls` list with `server_name` set to the connector name, using the same shape as [MCP tool calls](/task-api/mcp-tool-call#response-content). Facts drawn from a connector are cited in the `basis` like any other source.

### Restrictions

* Pay-Per-Use connectors are supported on the Task API [processors](/task-api/guides/choose-a-processor) (`lite`, `base`, `core`, `core2x`, `pro`, `ultra`, `ultra2x`, `ultra4x`, `ultra8x`). Selecting one on an unsupported processor is rejected.
* Each connector belongs to one list. Naming a free connector under `pay_per_use`, or a paid connector under `free`, is rejected.
* A connector name may not match the `name` of a server in `mcp_servers` on the same request.
* Connectors are being rolled out per organization. If a selected connector is not yet available to your organization, the request is rejected with a `422` whose message lists the connectors you can use. Contact [support@parallel.ai](mailto:support@parallel.ai) to request access.

## Bring Your Own License (BYOL)

Use BYOL when your organization already holds a license to a provider or operates its own private data service. Pass the provider's remote MCP server in `mcp_servers`, including any authentication headers required by that server. The licenses you already hold carry straight into your Task Runs.

```json Task API theme={"system"}
{
  "input": "Summarize the renewal risks in our account notes.",
  "processor": "pro",
  "mcp_servers": [
    {
      "type": "url",
      "name": "my_internal_kb",
      "url": "https://kb.example.com/mcp",
      "headers": {
        "Authorization": "Bearer YOUR_PROVIDER_TOKEN"
      }
    }
  ]
}
```

Parallel discovers the server's available tools and lets the processor call them when they are useful for the task. See [MCP Tool Calling](/task-api/mcp-tool-call) for authentication, configuration fields, restrictions, and complete request and response examples.

## Combine BYOL and Pay-Per-Use connectors

You can use your own MCP server and Pay-Per-Use connectors in the same Task API request. Names must be unique across `mcp_servers` and `data_sources`.

```json Task API theme={"system"}
{
  "input": "How has foot traffic trended this year at the retailers named in our account notes?",
  "processor": "pro",
  "mcp_servers": [
    {
      "type": "url",
      "name": "my_internal_kb",
      "url": "https://kb.example.com/mcp",
      "headers": {
        "Authorization": "Bearer YOUR_PROVIDER_TOKEN"
      }
    }
  ],
  "advanced_settings": {
    "data_sources": {
      "pay_per_use": ["carbonarc"]
    }
  }
}
```

## Responses API

The [Responses API](/responses-api/responses-quickstart) selects connectors with a top-level `data_sources` field that takes the same `free` and `pay_per_use` lists and connector names as the Task API. The same billing applies.

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "Summarize the registered phase 3 trials for semaglutide in adolescents.",
      "reasoning": {"effort": "medium"},
      "data_sources": {"free": ["clinical_trials", "pubmed"]}
    }'
  ```

  ```python Python theme={"system"}
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  response = client.responses.create(
      model="parallel",
      input="Summarize the registered phase 3 trials for semaglutide in adolescents.",
      reasoning={"effort": "medium"},
      extra_body={"data_sources": {"free": ["clinical_trials", "pubmed"]}},
  )
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "Summarize the registered phase 3 trials for semaglutide in adolescents.",
    reasoning: { effort: "medium" },
    // @ts-expect-error data_sources is a Parallel extension to the OpenAI request
    data_sources: { free: ["clinical_trials", "pubmed"] },
  });
  ```
</CodeGroup>

`data_sources` is a Parallel extension, so the OpenAI Python SDK sends it through `extra_body`. The TypeScript SDK sends unknown fields as they are.

* Connectors in `data_sources` require `reasoning.effort` `medium` (the default) or `high`. Selecting one with `low` is rejected.
* Every call to a connector in `data_sources` is returned as an `mcp_call` output item with `server_label` set to the connector name, the same item your own MCP tools produce. Calls to Index Partners are not reported.
* For BYOL, pass the provider's server as an OpenAI `mcp` tool. See [MCP Tools](/responses-api/features/mcp-tools) for the fields, restrictions, and output. A connector name may not match the `server_label` of an `mcp` tool on the same request.
* Rejected requests return a `400` in the OpenAI error envelope, not a `422`. The unavailable-connector message is the same and lists the connectors you can use.
* The earlier `additional_data_providers` field is retired, and a request that sends it is rejected with a message pointing to `data_sources`.

## Next steps

<CardGroup cols={3}>
  <Card title="Data Connectors directory" icon="database" href="https://parallel.ai/data-connectors">
    Browse every available connector, or request a source that is not listed yet.
  </Card>

  <Card title="MCP Tool Calling" icon="plug" href="/task-api/mcp-tool-call">
    Learn how Parallel discovers and invokes tools from a remote MCP server.
  </Card>

  <Card title="Pricing" icon="dollar-sign" href="/getting-started/pricing">
    Review Task processor pricing and billing guidance.
  </Card>
</CardGroup>
