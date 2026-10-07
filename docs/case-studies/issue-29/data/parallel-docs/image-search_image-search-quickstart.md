> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Image Search API Quickstart

> Find relevant images on the web from keyword queries and a natural language objective with the Parallel Image Search API

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

The **Image Search API** takes keyword queries and an optional natural language objective
and returns relevant images from across the web. Each result includes a direct image URL,
the URL of the page the image appears on, a title, and the image's dimensions.

Use Image Search when an agent or application needs pictures rather than text: illustrating
an answer, finding product or place photos, or collecting visual references for a topic. To
retrieve pages and text excerpts, use the [Search API](/search/search-quickstart).

## 1. Set Up Prerequisites

Generate your API key on [Platform](https://platform.parallel.ai), then export it:

```bash theme={"system"}
export PARALLEL_API_KEY="PARALLEL_API_KEY"
```

## 2. Execute Your First Image Search

### Sample Request

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/images/search \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "objective": "Photos of the Golden Gate Bridge at sunset, showing the full span",
      "search_queries": [
        "Golden Gate Bridge sunset",
        "Golden Gate Bridge full span photo"
      ]
    }'
  ```

  ```python Python theme={"system"}
  import os

  import requests

  response = requests.post(
      "https://api.parallel.ai/v1/images/search",
      headers={"x-api-key": os.environ["PARALLEL_API_KEY"]},
      json={
          "objective": "Photos of the Golden Gate Bridge at sunset, showing the full span",
          "search_queries": [
              "Golden Gate Bridge sunset",
              "Golden Gate Bridge full span photo",
          ],
      },
      timeout=60,
  )
  response.raise_for_status()

  for image in response.json()["results"]:
      print(f"{image['title']}: {image['image_url']}")
      print(f"  from {image['source_page_url']}")
  ```

  ```typescript TypeScript theme={"system"}
  async function main() {
      const response = await fetch("https://api.parallel.ai/v1/images/search", {
          method: "POST",
          headers: {
              "Content-Type": "application/json",
              "x-api-key": process.env.PARALLEL_API_KEY!,
          },
          body: JSON.stringify({
              objective: "Photos of the Golden Gate Bridge at sunset, showing the full span",
              search_queries: [
                  "Golden Gate Bridge sunset",
                  "Golden Gate Bridge full span photo",
              ],
          }),
      });
      if (!response.ok) {
          throw new Error(`Image Search failed: ${response.status} ${await response.text()}`);
      }

      const { results } = await response.json();
      for (const image of results) {
          console.log(`${image.title}: ${image.image_url}`);
          console.log(`  from ${image.source_page_url}`);
      }
  }

  main().catch(console.error);
  ```
</CodeGroup>

<Tip>
  This request uses the default `advanced` mode. Add `"mode": "fast"` for the lowest latency
  and cost. See [Image Search Modes](/image-search/modes).
</Tip>

### Sample Response

The API returns a JSON response with the following structure. The `results` array below is
shortened for readability.

```json theme={"system"}
{
  "search_id": "search_fcb2b4f3c75e418687bccaa1a8381331",
  "session_id": "session_fcb2b4f3c75e418687bccaa1a8381331",
  "results": [
    {
      "title": "Golden Gate Bridge at sunset",
      "image_url": "https://www.example.com/images/golden-gate-sunset.jpg",
      "source_page_url": "https://www.example.com/golden-gate-bridge",
      "width": 1920,
      "height": 1080
    },
    {
      "title": "The Golden Gate Bridge seen from the Marin Headlands",
      "image_url": "https://www.example.org/photos/golden-gate-marin.jpg",
      "source_page_url": "https://www.example.org/travel/san-francisco",
      "width": 1600,
      "height": 1067
    }
  ]
}
```

## Next Steps

* **[Best Practices](/image-search/best-practices)**: request fields and how to write objectives and queries
* **[Image Search Modes](/image-search/modes)**: choose between `fast` and `advanced`
* **[Pricing](/getting-started/pricing#image-search-api)**: per-request cost for each mode
* **[Rate Limits](/getting-started/rate-limits)**: default quotas per product
* **[API Reference](/api-reference/image-search/image-search)**: full parameter specifications, constraints, and response schema
