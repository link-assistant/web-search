> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud Marketplace

> Subscribe to Parallel via Google Cloud Marketplace

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Parallel is available on the Google Cloud Marketplace through three listings. Subscribing
through any of them lets you consolidate billing into your Google Cloud account.

| | Parallel Web Search for Grounding | Parallel Web Search for Grounding - ZDR | Parallel Web Systems |
| - | - | - | - |
| **Listing** | [Subscribe](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems) | [Subscribe](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr) | [Subscribe](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-all) |
| **Includes** | Search API for grounding in [Google Gemini Enterprise](/integrations/google-gemini-enterprise) | The same grounding integration, with zero data retention | Search, Extract, Task, FindAll, and Monitor APIs |
| **Gemini grounding** | Yes | Yes | No |
| **Zero data retention** | No | Yes | Available with an enterprise agreement |

## Parallel Web Search for Grounding

The [Parallel Web Search for Grounding listing](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems)
provides the Parallel Search API as an external grounding provider in the Google Gemini
Enterprise Agent Platform. Requests run through the Gemini Enterprise Agent Platform
harness. You can use it either via the API or in Agent Studio.

For sensitive workloads, subscribe to the separate
[Parallel Web Search for Grounding - ZDR listing](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-zdr)
instead, which adds zero data retention. Using it also requires setting
`enable_zero_data_retention` in your requests.

See [Google Gemini Enterprise](/integrations/google-gemini-enterprise) for setup instructions.

## Parallel Web Systems

The [Parallel Web Systems listing](https://console.cloud.google.com/marketplace/product/parallel-web-systems-public/parallel-web-systems-all)
provides access to the Search, Extract, Task, FindAll, and Monitor APIs with billing
consolidated through your Google Cloud account. Zero data retention is available with an
enterprise agreement — contact `sales@parallel.ai`.

<Note>
  This listing does not include Gemini Enterprise grounding. To ground Gemini responses,
  subscribe to one of the Parallel Web Search for Grounding listings above.
</Note>

Once subscribed, see the [Overview](/getting-started/overview) to start building with the
Parallel APIs.
