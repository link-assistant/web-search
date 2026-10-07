> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Image Search API Best Practices

> Request fields and query guidance for the Parallel Image Search API

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Send the Image Search API one or more keyword queries and, optionally, a natural language
objective. The API returns the most relevant images from across the web.

## Request Fields

Send requests to `POST https://api.parallel.ai/v1/images/search` with your API key in the
`x-api-key` header. `search_queries` is required; all other fields are optional. The API
rejects unknown fields. See the [API Reference](/api-reference/image-search/image-search) for the
full parameter definitions.

| Field | Type | Notes | Example |
| - | - | - | - |
| search\_queries | string\[] | One to three keyword queries, each no more than 200 characters. At least one query must be non-empty. | `["Golden Gate Bridge sunset", "Golden Gate Bridge full span photo"]` |
| objective | string | A natural language description of the images you want. Maximum 5000 characters. | `"Photos of the Golden Gate Bridge at sunset, showing the full span"` |
| mode | string | `fast` or `advanced`. Defaults to `advanced`. See [Modes](/image-search/modes). | `"fast"` |
| client\_model | string | The model that creates the request and consumes the results. | `"claude-opus-5"`, `"gpt-6-astra"`, `"gemini-3.8-flash"` |
| session\_id | string | A string that groups related calls for one task. Up to 1000 characters. The API returns the value you provide, or generates one when you omit it. | `"session_<uuid>"` |
| advanced\_settings.location | string | ISO 3166-1 alpha-2 country code for geo-targeted results. Uses the same codes as the Search API; see [Supported locations](/search/advanced-search-settings#supported-locations). Unrecognized codes are ignored with a warning. | `"us"`, `"gb"`, `"jp"` |
| advanced\_settings.max\_results | int | Maximum number of results to return, from 1 to 20. Defaults to 10. Values above 20 are reduced to 20 with a warning. | `20` |

### Example

```json theme={"system"}
{
  "mode": "advanced",
  "objective": "Photos of the Tokyo skyline at night that include Tokyo Tower",
  "search_queries": ["Tokyo skyline night", "Tokyo Tower night view"],
  "advanced_settings": {
    "location": "jp",
    "max_results": 20
  }
}
```

## Writing Objectives and Queries

* **Describe what should be in the picture.** Name the subject and the details that
  distinguish the image you want, such as the angle, setting, time of day, or style: "Product
  photos of the Eames lounge chair on a white background" rather than "Eames chair".
* **Keep queries short and specific.** Use a few keywords per query, the way you would type
  them into an image search box. Use two or three queries when different phrasings or angles
  would surface different images.
* **Put requirements in the objective.** Constraints that are hard to express as keywords,
  such as "showing the full span" or "no people in frame", belong in the objective.
* **Use `location` for regional results.** Set `advanced_settings.location` when the right
  images depend on the country, such as local landmarks, storefronts, or regional products.
