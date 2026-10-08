> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Zapier

> Use Parallel in Zapier workflows

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Integrate Parallel's AI-powered web research into your Zapier workflows with our official app.

## Installation

Search for "Parallel Web Systems" when adding a step to your Zap,
or use [this link](https://zapier.com/apps/parallel-web-systems/integrations) to get started.

<Note>Version 1.1.0 and later supports OAuth.</Note>

## Available Actions

| Name | Key | Description | Use Cases |
| - | - | - | - |
| **Create Async Web Enrichment** | `async_web_enrichment` | Create an asynchronous Task Run. | Lead enrichment, competitive analysis, content research |
| **Fetch Result for Async Runs** | `process_async_completion` | Retrieve results for an async Task Run. | Complex multi-source research, deep competitive intelligence |

## Common Use Cases

* **Sales**: Lead scoring, account research, contact discovery
* **Marketing**: Content research, trend analysis, competitor monitoring
* **Operations**: Vendor research, risk assessment, due diligence
* **Support**: Real-time information lookup, documentation generation

For detailed configuration and advanced features, see the [Task API quickstart](../task-api/task-quickstart).

## Best Practices

* **Use webhooks**: Let your workflow continue automatically when results are ready, without continuously polling.
* **Choose processors appropriately**: Use the right processors for your workflow to ensure the best results.
  For more information on choosing processors, see our [guide](/task-api/guides/choose-a-processor).

## Migration Guide

If you're already using Parallel version 1.0.3 or earlier with Zapier, you can easily migrate to the latest version.

1. Open any existing Zap and click `Edit Zap`.
2. In `Setup`, click `Change` under `Account`, then reconnect or create a new account.
3. Update the account connection for all Zaps that use Parallel.
