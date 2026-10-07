> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Citations

> Source annotations on Responses API answers

Every Responses API answer is grounded in live web research, and the sources are returned as
OpenAI-style `url_citation` annotations on the output text. Because this is the stock
wire format, existing OpenAI SDK code that reads `content[].annotations` works unchanged.

## Annotation format

Citations appear on the `message` output item's `output_text` content part. Each annotation
carries the source `url` and `title`, plus `start_index`/`end_index` marking the span of
answer text it supports:

```json theme={"system"}
{
  "type": "message",
  "content": [
    {
      "type": "output_text",
      "text": "Jensen Huang has been the CEO of Nvidia since April 1993.",
      "annotations": [
        {
          "type": "url_citation",
          "url": "https://nvidianews.nvidia.com/bios/jensen-huang",
          "title": "Jensen Huang | NVIDIA Newsroom",
          "start_index": 0,
          "end_index": 56
        },
        {
          "type": "url_citation",
          "url": "https://simplywall.st/stocks/de/semiconductors/etr-nvd/nvidia-shares/management",
          "title": "NVIDIA Corporation (NVD) Leadership & Management Team Analysis - Simply Wall St",
          "start_index": 0,
          "end_index": 56
        }
      ]
    }
  ]
}
```

A span may carry multiple citations when several sources support the same claim.

## Reading citations

<CodeGroup>
  ```python Python theme={"system"}
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  response = client.responses.create(
      model="parallel",
      input="Who is the current CEO of Nvidia and when did he take the role?",
      reasoning={"effort": "low"},
  )

  for item in response.output:
      if item.type == "message":
          for part in item.content:
              if part.type == "output_text":
                  print(part.text)
                  for a in part.annotations:
                      print(f"  [{a.start_index}:{a.end_index}] {a.url}")
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "Who is the current CEO of Nvidia and when did he take the role?",
    reasoning: { effort: "low" },
  });

  for (const item of response.output) {
    if (item.type === "message") {
      for (const part of item.content) {
        if (part.type === "output_text") {
          console.log(part.text);
          for (const a of part.annotations) {
            console.log(`  [${a.start_index}:${a.end_index}] ${a.url}`);
          }
        }
      }
    }
  }
  ```
</CodeGroup>

## Citations while streaming

With [streaming](/responses-api/features/streaming-events) enabled, each citation arrives as
a standard `response.output_text.annotation.added` event after the text delta:

```
event: response.output_text.annotation.added
data: {"annotation": {"type": "url_citation", "url": "https://…", "title": "…",
       "start_index": 0, "end_index": 42}, "annotation_index": 0, …}
```

The final `response.completed` event carries the complete annotations array on the message
content, so non-incremental consumers can also read citations from there.

## Richer evidence

Citations tell you which sources support the answer. The searches and page reads behind
them are returned as `web_search_call` items (see
[Web Search Tool](/responses-api/features/web-search-tool)). If you need per-field evidence with
excerpts, reasoning, and confidence ratings, use the [Task API](/task-api/task-quickstart)'s
[Research Basis](/task-api/guides/access-research-basis).
