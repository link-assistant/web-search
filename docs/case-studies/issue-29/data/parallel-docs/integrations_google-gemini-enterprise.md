> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Gemini Enterprise

> Use Parallel as a grounding provider in Google Gemini Enterprise Agent Platform

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

The Parallel Search API is available in the Google Gemini Enterprise Agent Platform as an external grounding provider. Use it to ground Gemini model responses with up-to-date context from the public web.

There are two ways to get started:

| | Google Cloud Marketplace | Bring Your Own Key (BYOK) |
| - | - | - |
| **Setup** | Subscribe via Google Cloud Marketplace | Get an API key from [Parallel Platform](https://platform.parallel.ai) |
| **Authentication** | Automatic — no API key needed | API key passed in each request |
| **Billing** | Consolidated through Google Cloud | Billed through Parallel |
| **Zero Data Retention** | Available via the [ZDR offering](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr) | Not available |

Both paths have the same 200 prompts per minute quota. If a request carries both a Marketplace subscription and an API key, the API key takes precedence.

Google classifies Grounding with Parallel as a Separate Offering under your Google Cloud Agreement, so Parallel's terms govern the search itself. See Google's [Grounding with Parallel documentation](https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/grounding/grounding-with-parallel) for their side of the integration.

## Use cases

* Using web data for information completion or enrichment.
* Multi-hop agents that require deeper web searches for complex questions.
* Building APIs that integrate web search data.
* Employee-facing assistants for up-to-date analysis and reporting.
* Consumer apps (retail, travel) supporting informed purchase decisions.
* Automated agents (e.g., news analysis, KYC checks).
* Vertical agents (sales, coding, finance) fetching the latest context from the web.

## Example

Who won the 2025 Las Vegas F1 Grand Prix?

| Without Grounding | With Grounding |
| - | - |
| The 2025 Las Vegas Grand Prix has not happened yet. The race is scheduled to take place on the weekend of November 20-22, 2025. Therefore, the winner is currently unknown. | The winner of the 2025 Las Vegas F1 Grand Prix was Max Verstappen of Red Bull Racing. The race took place on November 22, 2025. Sources: domain1.com, domain2.com, ... |

## Supported models

The following models support Grounding with Parallel web search:

* Gemini 2.5 Flash (`gemini-2.5-flash`)
* Gemini 2.5 Flash-Lite (`gemini-2.5-flash-lite`)
* Gemini 2.5 Pro (`gemini-2.5-pro`)
* Gemini 3.1 Pro (`gemini-3.1-pro-preview`)
* Gemini 3.1 Flash-Lite (`gemini-3.1-flash-lite`)
* Gemini 3.5 Flash (`gemini-3.5-flash`)

## Setup

<Tabs>
  <Tab title="Google Cloud Marketplace (Recommended)">
    The fastest way to get started is through the Google Cloud Marketplace. This approach requires no API key — authentication is handled automatically through your Google Cloud project.

    1. Go to the [Parallel Web Search listing](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems) on Google Cloud Marketplace.
    2. Click **Subscribe**.
    3. Review the pricing, accept the terms of service, and confirm your subscription.
    4. Ensure the subscription is active in the Google Cloud project you plan to use with Gemini Enterprise.

    Once subscribed, you can start making grounded requests immediately.

    <Note>
      For sensitive workloads, a separate [Zero Data Retention (ZDR) offering](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr) is available on Google Cloud Marketplace. To use ZDR, you must be subscribed to the ZDR offering **and** set `enable_zero_data_retention: true` in your requests (see [Make a grounded request](#make-a-grounded-request)). ZDR is only available through Google Cloud Marketplace.
    </Note>
  </Tab>

  <Tab title="Bring Your Own Key">
    1. Sign up at [Parallel Platform](https://platform.parallel.ai).
    2. Create an API key from your dashboard.
    3. Include the API key in your Gemini Enterprise requests.
  </Tab>
</Tabs>

<Note>
  With either option, Google Cloud sends data such as queries derived and rewritten from the original user prompt to Parallel Web Search for processing. Use of the service is governed by the Google Cloud Marketplace terms for Parallel Web Search if you subscribe through Marketplace, or by Parallel's [terms of use](https://parallel.ai/customer-terms) and [acceptable use policy](https://parallel.ai/acceptable-use-policy) if you bring your own key.
</Note>

## Agent Studio

You can also use Parallel as a grounding source directly in [Agent Studio](https://console.cloud.google.com/agent-platform/studio/multimodal;mode=prompt) — no code required. This requires an active Google Cloud Marketplace subscription.

<video autoPlay muted loop playsInline className="w-full aspect-video rounded-xl" src="https://mintcdn.com/parallel-6fabab31-mtje7p526we/WIFxCwqMrt1Ku5Fv/images/Parallel_AgentPlatformStudio.mp4?fit=max&auto=format&n=WIFxCwqMrt1Ku5Fv&q=85&s=30c3ffbe9c0bd53708a262ebc86d7360" data-path="images/Parallel_AgentPlatformStudio.mp4" />

1. Open [Agent Studio](https://console.cloud.google.com/agent-platform/studio/multimodal;mode=prompt) in the Google Cloud Console.
2. In the side panel under **Model settings**, find **Grounding** and turn on **Partners** (**Search results from grounding partners**).
3. Select **Parallel Web Search**, then click **Apply**.
4. Enter your prompt and send it. The model response will be grounded with web results from Parallel.

<Tip>
  Agent Studio is a great way to experiment with grounded responses before integrating via the API.
</Tip>

## Make a grounded request

Request grounded responses from Gemini by using the Google Gen AI SDK or the REST API. Before running the examples, complete one of the [setup](#setup) options.

Replace `MODEL_ID` with a [supported model](#supported-models), such as `gemini-3.5-flash`.

Both setup paths make the same call. Marketplace subscribers send no key; Bring Your Own Key users set the API key on the `parallelAiSearch` tool, shown commented out in each example.

### Google Gen AI SDK

Authenticate with Google Cloud, then set the environment variables the SDK clients read:

```bash theme={"system"}
gcloud auth application-default login

export GOOGLE_CLOUD_PROJECT=PROJECT_ID
export GOOGLE_CLOUD_LOCATION=global
export GOOGLE_GENAI_USE_ENTERPRISE=True
```

`GOOGLE_GENAI_USE_ENTERPRISE` routes the client to the Gemini Enterprise Agent Platform rather than the Gemini Developer API. Use `global` for the location unless you have a data residency requirement, in which case set a region such as `us-central1`.

<Tabs>
  <Tab title="Python">
    Install the [Google Gen AI SDK for Python](https://googleapis.github.io/python-genai/):

    ```bash theme={"system"}
    pip install --upgrade google-genai
    ```

    ```python theme={"system"}
    from google import genai
    from google.genai import types

    client = genai.Client()

    response = client.models.generate_content(
        model="MODEL_ID",
        contents="Who won the 2025 Las Vegas F1 Grand Prix?",
        config=types.GenerateContentConfig(
            tools=[
                types.Tool(
                    parallel_ai_search=types.ToolParallelAiSearch(
                        # Omit api_key when using a Marketplace subscription.
                        # Otherwise, provide your Parallel API key.
                        # api_key="API_KEY",
                        # Keys in custom_configs use the Search API's
                        # snake_case field names. All fields are optional.
                        custom_configs={
                            "mode": "basic",
                            "max_results": 10,
                            "source_policy": {
                                "include_domains": ["wikipedia.org"]
                            },
                        },
                    )
                )
            ],
        ),
    )

    print(response.text)
    print(response.candidates[0].grounding_metadata.grounding_chunks)
    ```
  </Tab>

  <Tab title="Java">
    [Install or update the Google Gen AI SDK for Java](https://cloud.google.com/vertex-ai/generative-ai/docs/sdks/overview). See the [SDK reference](https://central.sonatype.com/artifact/com.google.genai/google-genai) for package details.

    ```java theme={"system"}
    import com.google.genai.Client;
    import com.google.genai.types.GenerateContentConfig;
    import com.google.genai.types.GenerateContentResponse;
    import com.google.genai.types.Tool;
    import com.google.genai.types.ToolParallelAiSearch;
    import java.util.List;
    import java.util.Map;

    public class ParallelGroundingSample {
      public static void main(String[] args) {
        try (Client client = Client.builder().build()) {
          GenerateContentConfig config =
              GenerateContentConfig.builder()
                  .tools(
                      Tool.builder()
                          .parallelAiSearch(
                              ToolParallelAiSearch.builder()
                                  // Omit apiKey when using a Marketplace
                                  // subscription. Otherwise, provide your
                                  // Parallel API key.
                                  // .apiKey("API_KEY")
                                  // Keys in customConfigs use the Search API's
                                  // snake_case field names. All fields are optional.
                                  .customConfigs(
                                      Map.of(
                                          "mode", "basic",
                                          "max_results", 10,
                                          "source_policy",
                                          Map.of(
                                              "include_domains",
                                              List.of("wikipedia.org"))))
                                  .build())
                          .build())
                  .build();

          GenerateContentResponse response =
              client.models.generateContent(
                  "MODEL_ID",
                  "Who won the 2025 Las Vegas F1 Grand Prix?",
                  config);

          System.out.println(response.text());
          response.candidates().get().get(0).groundingMetadata()
              .ifPresent(metadata -> System.out.println(metadata.groundingChunks()));
        }
      }
    }
    ```
  </Tab>

  <Tab title="TypeScript">
    Install the [Google Gen AI SDK for JavaScript](https://googleapis.github.io/js-genai/):

    ```bash theme={"system"}
    npm install @google/genai
    ```

    ```typescript theme={"system"}
    import {GoogleGenAI} from '@google/genai';

    const ai = new GoogleGenAI({});

    const response = await ai.models.generateContent({
      model: 'MODEL_ID',
      contents: 'Who won the 2025 Las Vegas F1 Grand Prix?',
      config: {
        tools: [
          {
            parallelAiSearch: {
              // Omit apiKey when using a Marketplace subscription.
              // Otherwise, provide your Parallel API key.
              // apiKey: 'API_KEY',
              // Keys in customConfigs use the Search API's snake_case
              // field names. All fields are optional.
              customConfigs: {
                mode: 'basic',
                max_results: 10,
                source_policy: {include_domains: ['wikipedia.org']},
              },
            },
          },
        ],
      },
    });

    console.log(response.text);
    console.log(response.candidates[0].groundingMetadata);
    ```
  </Tab>
</Tabs>

Configure the search with the tool's `custom_configs` field (`customConfigs` in Java and TypeScript). It accepts the optional parameters in [Configuration options](#configuration-options), including `source_policy`, `excerpts`, `max_results`, and `mode`.

<Note>
  `enable_zero_data_retention` is not exposed by the Gen AI SDKs — `ToolParallelAiSearch` accepts only the API key and custom configs. To use the [ZDR offering](#setup), call the [REST API](#rest-api) directly.
</Note>

### REST API

Send requests to the regional endpoint:

```
POST https://LOCATION-aiplatform.googleapis.com/v1/projects/PROJECT_ID/locations/LOCATION/publishers/google/models/MODEL_ID:generateContent
```

* `PROJECT_ID`: Your Google Cloud project ID.
* `LOCATION`: The Google Cloud region that processes the request. Regional availability varies by model — check the model's page in Google's docs before picking one. To use the global endpoint, drop the region prefix from the hostname (`https://aiplatform.googleapis.com`) and use `global` in place of `LOCATION` in the path. This is unrelated to the `location` search parameter below.
* `MODEL_ID`: The Gemini model to use (for example, `gemini-3.5-flash`).

<Tabs>
  <Tab title="Google Cloud Marketplace">
    No `api_key` field is needed when using the Marketplace subscription. Set `enable_zero_data_retention` to `true` to use the [ZDR offering](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr) for the request (requires an active ZDR subscription); if unspecified, the standard offering is used:

    ```json theme={"system"}
    {
      "contents": [{
        "role": "user",
        "parts": [{
          "text": "MODEL_PROMPT_TEXT"
        }]
      }],
      "tools": [{
        "parallelAiSearch": {
          "enable_zero_data_retention": ENABLE_ZERO_DATA_RETENTION,
          "customConfigs": {
            "mode": "MODE",
            "location": "SEARCH_LOCATION",
            "max_results": MAX_RESULTS,
            "source_policy": {
              "exclude_domains": ["EXCLUDE_DOMAINS"],
              "include_domains": ["INCLUDE_DOMAINS"]
            },
            "excerpts": {
              "max_chars_per_result": MAX_CHARS_PER_RESULT,
              "max_chars_total": MAX_CHARS_TOTAL
            }
          }
        }
      }],
      "model": "projects/PROJECT_ID/locations/LOCATION/publishers/google/models/MODEL_ID"
    }
    ```
  </Tab>

  <Tab title="Bring Your Own Key">
    Include your API key in the `parallelAiSearch` object:

    ```json theme={"system"}
    {
      "contents": [{
        "role": "user",
        "parts": [{
          "text": "MODEL_PROMPT_TEXT"
        }]
      }],
      "tools": [{
        "parallelAiSearch": {
          "api_key": "PARALLEL_API_KEY",
          "customConfigs": {
            "mode": "MODE",
            "location": "SEARCH_LOCATION",
            "max_results": MAX_RESULTS,
            "source_policy": {
              "exclude_domains": ["EXCLUDE_DOMAINS"],
              "include_domains": ["INCLUDE_DOMAINS"]
            },
            "excerpts": {
              "max_chars_per_result": MAX_CHARS_PER_RESULT,
              "max_chars_total": MAX_CHARS_TOTAL
            }
          }
        }
      }],
      "model": "projects/PROJECT_ID/locations/LOCATION/publishers/google/models/MODEL_ID"
    }
    ```
  </Tab>
</Tabs>

Execute the request:

```bash theme={"system"}
curl -X POST \
     -H "Authorization: Bearer $(gcloud auth print-access-token)" \
     -H "Content-Type: application/json; charset=utf-8" \
     -d @request.json \
     "https://LOCATION-aiplatform.googleapis.com/v1/projects/PROJECT_ID/locations/LOCATION/publishers/google/models/MODEL_ID:generateContent"
```

For error responses, see [Warnings and errors](/resources/warnings-and-errors).

## Read the grounded response

Grounded responses include a `groundingMetadata` object alongside the generated text. Use it to render citations and to see which search queries Gemini sent to Parallel.

Gemini decides whether to search. If it answers without grounding, `groundingMetadata` is absent — check for it before rendering citations.

```json theme={"system"}
{
  "candidates": [{
    "content": {
      "role": "model",
      "parts": [{ "text": "The winner of the 2025 Las Vegas F1 Grand Prix was Max Verstappen of Red Bull Racing." }]
    },
    "finishReason": "STOP",
    "groundingMetadata": {
      "webSearchQueries": ["2025 Las Vegas F1 Grand Prix winner"],
      "groundingChunks": [
        {
          "web": {
            "uri": "https://...",
            "title": "2025 Las Vegas Grand Prix",
            "domain": "domain.com"
          }
        }
      ],
      "groundingSupports": [
        {
          "segment": {
            "startIndex": 0,
            "endIndex": 85,
            "text": "The winner of the 2025 Las Vegas F1 Grand Prix was Max Verstappen of Red Bull Racing."
          },
          "groundingChunkIndices": [0]
        }
      ]
    }
  }],
  "usageMetadata": {
    "promptTokenCount": 33,
    "candidatesTokenCount": 106,
    "totalTokenCount": 284,
    "trafficType": "ON_DEMAND",
    "toolUsePromptTokensDetails": [{ "modality": "TEXT", "tokenCount": 39 }]
  }
}
```

| Field | Description |
| - | - |
| `groundingMetadata.webSearchQueries` | The queries Gemini derived from the prompt and sent to Parallel. Gemini may issue several searches for one prompt, or none. |
| `groundingMetadata.groundingChunks[].web` | Each source used for grounding, with its `uri`, `title`, and `domain` |
| `groundingMetadata.groundingSupports[]` | Maps a `segment` of the answer text to the `groundingChunkIndices` that support it |
| `usageMetadata.toolUsePromptTokensDetails` | Tokens consumed by grounding, on top of your prompt and output tokens |

<Note>
  The REST API returns these fields in camelCase. The Gen AI SDKs expose the same fields in snake\_case — REST's `groundingMetadata.groundingSupports` is `grounding_metadata.grounding_supports` in Python.
</Note>

### Render inline citations

Each `groundingSupports` entry carries a `segment` that locates the cited claim in the answer. Two details matter:

* `startIndex` and `endIndex` are **byte** offsets, not character positions. Slicing a Python string directly by them misplaces markers in any answer containing non-ASCII text. Splice in byte space instead.
* The offsets are relative to a single `Part`, identified by `partIndex` — not to the whole concatenated answer.

Walk each part's supports in reverse so that inserting a marker doesn't shift the offsets you haven't used yet:

```python theme={"system"}
candidate = response.candidates[0]
metadata = candidate.grounding_metadata
supports = (metadata.grounding_supports or []) if metadata else []

rendered = []
for part_index, part in enumerate(candidate.content.parts):
    data = part.text.encode("utf-8")
    part_supports = [s for s in supports if (s.segment.part_index or 0) == part_index]

    for support in sorted(part_supports, key=lambda s: s.segment.end_index, reverse=True):
        markers = "".join(f"[{i + 1}]" for i in support.grounding_chunk_indices)
        end = support.segment.end_index
        data = data[:end] + markers.encode("utf-8") + data[end:]

    rendered.append(data.decode("utf-8"))

print("".join(rendered))

if metadata and metadata.grounding_chunks:
    for i, chunk in enumerate(metadata.grounding_chunks, start=1):
        print(f"[{i}] {chunk.web.title} — {chunk.web.uri}")
```

## Configuration options

All `customConfigs` fields are optional. For best performance, use defaults unless you have specific requirements.

In addition, the `parallelAiSearch` object accepts an optional `enable_zero_data_retention` field (a sibling of `customConfigs`, not nested inside it). Set it to `true` to route the request through the [ZDR offering](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr) for sensitive workloads — this requires an active ZDR subscription on Google Cloud Marketplace and is not available with Bring Your Own Key.

| Parameter | Default | Range | Description |
| - | - | - | - |
| `max_results` | 10 | 1–20 | Maximum number of search results used for grounding |
| `excerpts.max_chars_per_result` | 30,000 | 1,000–100,000 | Maximum characters per excerpt |
| `excerpts.max_chars_total` | 100,000 | 1,000–1,000,000 | Maximum total excerpt characters |
| `source_policy.include_domains` | — | See below | Only return results matching these domains or domain/path prefixes |
| `source_policy.exclude_domains` | — | See below | Exclude results matching these domains or domain/path prefixes |
| `location` | — | — | ISO 3166-1 alpha-2 country code for geo-targeted search results (for example, `us`). Unrelated to the Google Cloud `LOCATION` region in the request URL. |
| `mode` | `basic` | `basic` \| `advanced` | [Search mode](/search/modes). Use `advanced` for more thorough results at the expense of higher latency. |

Both [`source_policy`](/search/source-policy) lists accept a full domain (`www.example.com`), a domain/path prefix (`docs.example.com/api`), or a domain extension beginning with a period (`.gov`, `.edu`, `.co.uk`). The leading period is required for extensions. Path matching is case-sensitive and occurs at segment boundaries. The combined number of entries across `include_domains` and `exclude_domains` cannot exceed 200.

The defaults suit most workloads. Reach for `source_policy` when you need to restrict sources, `mode: advanced` when answer quality matters more than latency, and `excerpts` when you are managing context length. For guidance on writing search queries, see [Search API Best Practices](/search/best-practices).

<Tip>
  For a complete working example, see the [Gemini demo](https://github.com/parallel-web/parallel-cookbook/tree/main/python-recipes/gemini_ai_demo) in the Parallel Cookbook.
</Tip>

## Quota

The default quota is 200 prompts per minute. To request higher rate limits, share your use case and requirements with:

* **Google Cloud Marketplace**: your Google account team.
* **Bring Your Own Key**: `support@parallel.ai` and your Google account team.

## Billing

Using Gemini with Parallel can incur charges for:

* **Gemini token consumption** — prompt, thinking, and output tokens.
* **Gemini grounding requests** — Google charges per grounded prompt in addition to tokens.
* **Parallel Web Search API usage**.

See [Google's Gemini Enterprise Agent Platform pricing](https://cloud.google.com/gemini-enterprise-agent-platform/generative-ai/pricing) for the Google charges. Search API charges depend on your setup path:

* **Google Cloud Marketplace**: consolidated into your Google Cloud billing.
* **Bring Your Own Key**: billed through [Parallel's pricing](/getting-started/pricing).
