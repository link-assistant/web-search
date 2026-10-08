> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Vercel

> Use Parallel with Vercel

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Use Parallel through Vercel AI Gateway, the Vercel AI SDK, or the Vercel Marketplace.

## Vercel AI Gateway

Parallel Search is available as a built-in tool in [Vercel AI Gateway](https://vercel.com/docs/ai-gateway). AI Gateway provides a unified API to access hundreds of models through a single endpoint, with built-in web search capabilities.

The `parallelSearch` tool can be used with any model regardless of the model provider. When the model needs current information, it calls the tool and AI Gateway routes the request to Parallel's Search API.

For local development, install the AI SDK and configure an AI Gateway API key:

```bash theme={"system"}
npm install ai
export AI_GATEWAY_API_KEY="your-ai-gateway-api-key"
```

Vercel deployments can use [OIDC authentication](https://vercel.com/docs/ai-gateway/authentication-and-byok/oidc) instead of an API key.

<CodeGroup>
  ```typescript streamText theme={"system"}
  import { gateway, streamText } from 'ai';

  export async function POST(request: Request) {
    const { prompt } = await request.json();

    const result = streamText({
      model: 'anthropic/claude-sonnet-5', // Works with any model
      prompt,
      tools: {
        parallel_search: gateway.tools.parallelSearch(),
      },
    });

    return result.toUIMessageStreamResponse();
  }
  ```

  ```typescript generateText theme={"system"}
  import { gateway, generateText } from 'ai';

  export async function POST(request: Request) {
    const { prompt } = await request.json();

    const { text } = await generateText({
      model: 'anthropic/claude-sonnet-5', // Works with any model
      prompt,
      tools: {
        parallel_search: gateway.tools.parallelSearch(),
      },
    });

    return Response.json({ text });
  }
  ```
</CodeGroup>

You can configure Gateway options such as `maxResults`, `sourcePolicy`, `excerpts`, and `fetchPolicy`. Gateway search modes differ from the direct [Parallel Search API modes](/search/modes). See the [Parallel parameters documentation](https://vercel.com/docs/ai-gateway/models-and-providers/web-search#parallel-parameters) for all Gateway options.

## Vercel AI SDK

Easily drop in Parallel Search API or Extract API with any Vercel AI SDK compatible model provider.

* **Search API**: Given the required `search_queries` and optional `objective`, Parallel returns ranked URLs with compressed excerpts
* **Extract API**: Given a URL and an optional objective, Parallel returns compressed excerpts or full page contents

Install AI SDK v6, the OpenAI provider, and version 1 of Parallel's tool package:

```bash theme={"system"}
npm install ai@^6 @ai-sdk/openai@^3 @parallel-web/ai-sdk-tools@^1
export PARALLEL_API_KEY="your-parallel-api-key"
export OPENAI_API_KEY="your-openai-api-key"
```

The direct Search API supports the `turbo`, `fast`, `basic`, and `advanced` [search modes](/search/modes). In `@parallel-web/ai-sdk-tools@^1`, `searchTool` supports `basic` and `advanced` and defaults to `advanced`. The `agentic` and `one-shot` tool modes belong to older package versions.

**Links:**

* [NPM Package](https://www.npmjs.com/package/@parallel-web/ai-sdk-tools)
* [Vercel AI SDK Toolkit](https://ai-sdk.dev/docs/foundations/tools#ready-to-use-tool-packages)
* [Vercel AI SDK Web Search Agent Cookbook](https://ai-sdk.dev/cookbook/node/web-search-agent#parallel-web)

### Sample Code

Parallel search and extract tools can be used with any Vercel AI SDK compatible model provider.

<CodeGroup>
  ```typescript Search theme={"system"}
  import { openai } from '@ai-sdk/openai';
  import { stepCountIs, streamText } from 'ai';
  import { searchTool } from '@parallel-web/ai-sdk-tools';

  export async function POST() {
    const result = streamText({
      model: openai('gpt-6-astra'),
      prompt: 'What are the latest developments in AI?',
      tools: {
        'web-search': searchTool,
      },
      stopWhen: stepCountIs(3),
    });

    return result.toUIMessageStreamResponse();
  }
  ```

  ```typescript Extract theme={"system"}
  import { openai } from '@ai-sdk/openai';
  import { stepCountIs, streamText } from 'ai';
  import { extractTool } from '@parallel-web/ai-sdk-tools';

  export async function POST() {
    const result = streamText({
      model: openai('gpt-6-astra'),
      prompt: 'How should tools be used in the Vercel AI SDK based on https://vercel.com/docs/ai-sdk',
      tools: {
        'web-extract': extractTool,
      },
      stopWhen: stepCountIs(3),
    });

    return result.toUIMessageStreamResponse();
  }
  ```
</CodeGroup>

## Vercel Marketplace

Parallel is available on the [Vercel Marketplace](https://vercel.com/marketplace/parallel). Install the integration to get a Parallel API key that you can use directly in your Vercel apps, with billing managed through Vercel.

### Getting started

1. Install the [Parallel integration](https://vercel.com/marketplace/parallel) from the Vercel Marketplace
2. Once installed, you'll receive a Parallel API key automatically provisioned for your account
3. Use the API key in your Vercel applications to access Parallel Search and Extract APIs

### Example application

See the [Parallel Vercel Template](https://parallel-vercel-template-cookbook.vercel.app) for a working example, with source code available in the [cookbook repository](https://github.com/parallel-web/parallel-cookbook/tree/main/typescript-recipes/parallel-vercel-template).
