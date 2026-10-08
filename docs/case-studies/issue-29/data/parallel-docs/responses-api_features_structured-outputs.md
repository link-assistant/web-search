> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Responses API Structured Outputs

> Get JSON answers conforming to your schema from the Responses API

The Responses API supports structured output through the standard OpenAI `text.format`
parameter. Provide a JSON Schema and the researched answer is returned as a JSON object
conforming to it, JSON-encoded in the output text.

## Usage

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "What is the capital and population of Iceland?",
      "reasoning": {"effort": "low"},
      "text": {
        "format": {
          "type": "json_schema",
          "name": "capital_info",
          "schema": {
            "type": "object",
            "properties": {
              "capital": {"type": "string"},
              "population": {"type": "integer"}
            },
            "required": ["capital", "population"],
            "additionalProperties": false
          }
        }
      }
    }'
  ```

  ```python Python theme={"system"}
  import json
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  response = client.responses.create(
      model="parallel",
      input="What is the capital and population of Iceland?",
      reasoning={"effort": "low"},
      text={
          "format": {
              "type": "json_schema",
              "name": "capital_info",
              "schema": {
                  "type": "object",
                  "properties": {
                      "capital": {"type": "string"},
                      "population": {"type": "integer"},
                  },
                  "required": ["capital", "population"],
                  "additionalProperties": False,
              },
          }
      },
  )

  answer = json.loads(response.output_text)
  print(answer)  # {'capital': 'Reykjavik', 'population': 386506}
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "What is the capital and population of Iceland?",
    reasoning: { effort: "low" },
    text: {
      format: {
        type: "json_schema",
        name: "capital_info",
        schema: {
          type: "object",
          properties: {
            capital: { type: "string" },
            population: { type: "integer" },
          },
          required: ["capital", "population"],
          additionalProperties: false,
        },
      },
    },
  });

  const answer = JSON.parse(response.output_text);
  console.log(answer); // { capital: "Reykjavik", population: 386506 }
  ```
</CodeGroup>

The structured answer is returned JSON-encoded in the output text — parse `output_text` to
get the object, and handle parse errors as you would for any network payload.
[Citations](/responses-api/features/citations) still work: annotation
`start_index`/`end_index` values point into the JSON-encoded string, anchored to the
individual field values (e.g. a citation covering `"Reykjavik"` for the `capital` field).

## Schema guidance

* Follow OpenAI structured-output schema conventions: object root, `required` listing every
  property, and `additionalProperties: false`.
* Prefer flat schemas with descriptive field names and descriptions — field descriptions
  guide the research, so state formats and units explicitly (e.g. "Founding year as a
  four-digit number").

## Next steps

* [Direct Requests example](/responses-api/examples/direct-requests) — structured outputs
  applied to dataset enrichment, one entity per request
