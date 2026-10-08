> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Supabase

> Enrich your Supabase data with live web intelligence using Edge Functions and Parallel

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Enrich your Supabase data with live web intelligence using [Supabase Edge Functions](https://supabase.com/docs/guides/functions) and Parallel's Task API.

<Tip>
  Check out the [Parallel integration on Supabase](https://supabase.com/partners/integrations/parallel-company-enrichment) for more information.
</Tip>

## Getting Started

We provide a complete cookbook with Supabase Edge Functions, a Next.js frontend, and step-by-step setup instructions.

<Card title="Supabase + Parallel Cookbook" icon="github" href="https://github.com/parallel-web/parallel-cookbook/tree/main/typescript-recipes/parallel-supabase-enrichment">
  Complete working example showing how to build a data enrichment pipeline with Supabase and Parallel.
</Card>

The cookbook includes:

* **Supabase Edge Functions** that call Parallel's Task API
* **Next.js frontend** with live updates via Supabase Realtime
* **SQL schemas** for storing enrichment data
* **Polling pattern** for handling long-running enrichments

## Example Usage

The Edge Function uses the `parallel-web` SDK to call Parallel's Task API:

```typescript theme={"system"}
import Parallel from "npm:parallel-web@1.0.1";

const parallel = new Parallel({ apiKey: Deno.env.get("PARALLEL_API_KEY") });

const taskRun = await parallel.taskRun.create({
  input: {
    company_name: "Stripe",
    website: "stripe.com",
  },
  processor: "base",
  task_spec: {
    output_schema: {
      type: "json",
      json_schema: {
        type: "object",
        properties: {
          industry: { type: "string" },
          employee_count: { type: "string" },
          headquarters: { type: "string" },
          description: { type: "string" },
        },
      },
    },
  },
});

const result = await parallel.taskRun.result(taskRun.run_id, { timeout: 30 });
```

For detailed configuration and advanced features, see the [Task API Quickstart](/task-api/task-quickstart).

**Links:**

* [Supabase + Parallel Cookbook](https://github.com/parallel-web/parallel-cookbook/tree/main/typescript-recipes/parallel-supabase-enrichment)
* [Parallel on Supabase Integrations](https://supabase.com/partners/integrations/parallel-company-enrichment)
* [parallel-web npm package](https://www.npmjs.com/package/parallel-web)
