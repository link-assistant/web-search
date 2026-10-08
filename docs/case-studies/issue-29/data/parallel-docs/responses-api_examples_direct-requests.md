> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Using the Responses API Directly

> Call the Responses API directly: single questions, deep research, dataset enrichment, and follow-ups

The simplest way to use the Responses API is to call it directly: your application asks a
question, the API researches the live web and returns a synthesized, cited answer. This page
covers the common direct-call patterns.

## Ask a question

Use `low` effort for simple fact retrieval — it returns in a few seconds:

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "Who is the current CEO of the largest cloud provider by revenue?",
      "reasoning": {"effort": "low"}
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
      input="Who is the current CEO of the largest cloud provider by revenue?",
      reasoning={"effort": "low"},
  )

  print(response.output_text)
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "Who is the current CEO of the largest cloud provider by revenue?",
    reasoning: { effort: "low" },
  });

  console.log(response.output_text);
  ```
</CodeGroup>

## Ask a deep-research question

The `high` tier runs extensive multi-step searching and synthesis and returns in roughly
30–60 seconds. Give your client timeout headroom, and consider
[streaming](/responses-api/features/streaming-events) for a connection acknowledgment while
research runs:

<CodeGroup>
  ```python Python theme={"system"}
  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
      timeout=120.0,  # allow headroom for high-effort research
  )

  response = client.responses.create(
      model="parallel",
      input="Which universities did the 2018 Fields Medalists get their PhDs from?",
      reasoning={"effort": "high"},
  )

  print(response.output_text)
  ```

  ```typescript TypeScript theme={"system"}
  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
    timeout: 120_000, // allow headroom for high-effort research
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "Which universities did the 2018 Fields Medalists get their PhDs from?",
    reasoning: { effort: "high" },
  });

  console.log(response.output_text);
  ```
</CodeGroup>

The Responses API is synchronous and designed for research that fits an interactive latency
budget. For long-running deep research (minutes, not seconds) with per-field evidence and
webhooks, use the [Task API Deep Research](/task-api/examples/task-deep-research) flow.

## Enrich a dataset

Each row becomes one request: pass the entity as the question, use
[Structured Outputs](/responses-api/features/structured-outputs) to shape the answer, and
run rows concurrently — wall-clock time stays close to a single request instead of the sum.

<CodeGroup>
  ```python Python theme={"system"}
  import asyncio
  import json
  import os
  from openai import AsyncOpenAI

  client = AsyncOpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  SCHEMA = {
      "type": "json_schema",
      "name": "company_facts",
      "schema": {
          "type": "object",
          "properties": {
              "founding_year": {
                  "type": "integer",
                  "description": "Year the company was founded, as a four-digit number",
              },
              "headquarters_city": {
                  "type": "string",
                  "description": "City of the company's current headquarters",
              },
              "ceo": {
                  "type": "string",
                  "description": "Full name of the current CEO",
              },
          },
          "required": ["founding_year", "headquarters_city", "ceo"],
          "additionalProperties": False,
      },
  }

  async def enrich(company: str) -> dict:
      response = await client.responses.create(
          model="parallel",
          input=f"Find the founding year, headquarters city, and current CEO of {company}.",
          reasoning={"effort": "low"},
          text={"format": SCHEMA},
          metadata={"company": company},
      )
      return {"company": company, **json.loads(response.output_text)}

  async def main():
      companies = ["Anthropic", "Vercel", "Datadog"]
      rows = await asyncio.gather(*(enrich(c) for c in companies))
      for row in rows:
          print(row)

  asyncio.run(main())
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const SCHEMA = {
    type: "json_schema" as const,
    name: "company_facts",
    schema: {
      type: "object",
      properties: {
        founding_year: {
          type: "integer",
          description: "Year the company was founded, as a four-digit number",
        },
        headquarters_city: {
          type: "string",
          description: "City of the company's current headquarters",
        },
        ceo: { type: "string", description: "Full name of the current CEO" },
      },
      required: ["founding_year", "headquarters_city", "ceo"],
      additionalProperties: false,
    },
  };

  async function enrich(company: string) {
    const response = await client.responses.create({
      model: "parallel",
      input: `Find the founding year, headquarters city, and current CEO of ${company}.`,
      reasoning: { effort: "low" },
      text: { format: SCHEMA },
      metadata: { company },
    });
    return { company, ...JSON.parse(response.output_text) };
  }

  const companies = ["Anthropic", "Vercel", "Datadog"];
  const rows = await Promise.all(companies.map(enrich));
  console.log(rows);
  ```
</CodeGroup>

Tips for enrichment at scale:

* **Put the complete question in `input`**, including qualifiers ("current", units, date
  ranges) — each request is self-contained.
* **Use `metadata`** to tag each request with your row id; it is echoed back on the response.
* **Back off and retry on `429` responses.**
* For large batches with per-field citations and confidence ratings, consider the
  [Task API Enrichment](/task-api/examples/task-enrichment) flow, which is purpose-built for
  that workload.

## Ask follow-up questions

Chain turns with [Statefulness](/responses-api/features/statefulness): pass the previous
response's `id` and follow-ups inherit the conversation context. Tiers can vary across
turns — open with a `high`-effort research question and ask cheap `low`-effort follow-ups.

```python Python theme={"system"}
first = client.responses.create(
    model="parallel",
    input="What are the leading commercial fusion energy startups?",
    reasoning={"effort": "high"},
)

follow_up = client.responses.create(
    model="parallel",
    input="Which of them has raised the most funding?",
    reasoning={"effort": "low"},
    previous_response_id=first.id,
)
```

## Next steps

* [Using It as a Research Subagent](/responses-api/examples/research-subagent) — let another
  LLM call the Responses API as a tool
* [Citations](/responses-api/features/citations) — read the sources behind each answer
