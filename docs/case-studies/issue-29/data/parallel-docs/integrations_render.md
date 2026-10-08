> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Render

> Deploy a web research agent with Parallel and Render Workflows

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Give the agent a question, and it breaks it into smaller questions, researches them with Parallel, and brings the findings together in a report with source links. [Render Workflows](https://render.com/docs/workflows) runs the tasks and retries them when needed.

Start with the [Parallel Research Agent template](https://github.com/render-examples/parallel-research-agent). Here’s how to deploy it and get your first report.

## How it works

1. You send a question to the gateway, a small FastAPI app, and get back a run ID.
2. The agent breaks your question into smaller questions it can research separately.
3. Each research branch uses Parallel [Search](/search/search-quickstart) and [Extract](/extract/extract-quickstart) to find information. Search uses `fast` mode by default. Extract returns focused excerpts within a set content budget.
4. A final task brings the findings together in a report with source links.
5. You use the run ID to check progress and fetch the report.

Each branch has its own retries. If one still fails, the final task receives the unanswered question so it can explain what’s missing. If all branches fail, the main task raises an error and can retry too. The run reports a failure if its retries are exhausted.

## What you’ll need

* A [Render account](https://dashboard.render.com) and [Render API key](https://render.com/docs/api-keys) with access to the workspace where you will create the Workflow.
* A [Parallel API key](https://platform.parallel.ai).
* An [Anthropic API key](https://console.anthropic.com) to use the template’s defaults. You can also use another provider through [LiteLLM](https://docs.litellm.ai/docs/providers).
* A terminal with `curl` to submit a question and retrieve the result.

The template defaults to Anthropic. If you use another provider, leave the gateway’s `ANTHROPIC_API_KEY` field blank and configure your provider’s credentials and model settings on the Workflow as described below. The gateway only uses the Render key to dispatch tasks; you don’t need to fork the repository just to switch providers.

The gateway runs on a paid Starter web service. You’ll also pay for Workflow usage and API calls, so check the [costs](#costs) before you deploy.

## Deploy the application

### 1. Deploy the gateway

[Deploy to Render](https://render.com/deploy?repo=https://github.com/render-examples/parallel-research-agent).

Render reads the template’s Blueprint file and sets up a Python web service called `parallel-research-gateway`. Enter `PARALLEL_API_KEY` and `RENDER_API_KEY` when prompted. If you use Anthropic, also enter `ANTHROPIC_API_KEY`; otherwise leave that field blank. Leave `RENDER_WORKFLOW_SLUG` blank for now.

Render also creates an `API_SECRET`. You’ll use it to submit questions, so keep it set on the hosted gateway.

<Note>
  There are two parts to deploy: the gateway and the Workflow. The button sets up the gateway. You’ll add the Workflow next.
</Note>

Want to change the code? Fork the repository first, replace the deploy link’s `repo` parameter with your fork’s URL, and use that same fork for the Workflow.

### 2. Create the Workflow service

In the [Render Dashboard](https://dashboard.render.com), choose **New → Workflow**, then **My own workflow**. Connect the same repository used by the gateway. To use the public repository without connecting a Git provider, choose **Public Git Repository** and enter:

```text theme={"system"}
https://github.com/render-examples/parallel-research-agent
```

Use these settings for the Python service:

* **Build command:** `pip install -r requirements.txt`
* **Start command:** `python -m workflow.main`
* **Environment variables:** `PARALLEL_API_KEY` and `ANTHROPIC_API_KEY`

Using another model provider? Add its API credentials to the Workflow and set both `LLM_MODEL` and `PLANNER_MODEL` to that provider’s LiteLLM model identifiers. The template’s [configuration instructions](https://github.com/render-examples/parallel-research-agent#configuration) list the current defaults. Choose a model that supports tool calling for `LLM_MODEL`.

Deploy the Workflow, then check that its tasks appear in the dashboard. They use the default Flex compute plan.

### 3. Connect the gateway

Open the `research_agent` task in Render and find its task slug, which has the form `your-workflow/research_agent`. Copy just the Workflow portion, `your-workflow`. Open the gateway’s Environment page and paste it into `RENDER_WORKFLOW_SLUG`. If that variable isn’t listed because you left it blank earlier, add it now.

Save the change and wait for the gateway to redeploy. Copy its public URL for the next step.

## Submit a question

Copy `API_SECRET` from the gateway’s Environment page, then fill in your gateway URL and secret below:

```bash theme={"system"}
export GATEWAY_URL="https://your-gateway.onrender.com"
export API_SECRET="your-generated-secret"

curl --fail-with-body --silent --show-error \
  -X POST "$GATEWAY_URL/research" \
  -H "Authorization: Bearer $API_SECRET" \
  -H "Content-Type: application/json" \
  -d '{"query":"What are the leading open-source alternatives to Elasticsearch?"}'
```

You’ll get back a `run_id` and `status: "dispatched"`. Save the run ID and use it to check the result below. Send the POST once: each new submission starts another billable research job.

For this hosted setup, use the request above. The gateway’s browser form is for local development.

## Get your report

Paste the run ID you just received:

```bash theme={"system"}
export RUN_ID="your-returned-run-id"
curl --fail-with-body --silent --show-error \
  "$GATEWAY_URL/research/$RUN_ID"
```

Run this GET again from time to time until `status` is `completed` or `failed`. While the agent is working, `result` is `null`. When `status` is `completed`, you’ll find the report in `result.report` and its source URLs in `result.sources`. Check `result.branches_completed`, `result.branches_failed`, and `result.failed_questions` to see how much of the research finished.

A `failed` run has no report: `result` stays `null`. If something fails, check the task logs in Render before starting another run. A `401` when submitting means the secret is missing or incorrect. A `503` about an unset Workflow slug means you still need to connect the gateway. A `429` means you’ve reached the gateway’s submission limit.

<Note>
  Submitting a question requires your secret. Reading results or progress does not. Stick to non-sensitive questions until you’ve added access controls for private research.
</Note>

Save the report if you want to keep it. Render retains task inputs and results for 30 days.

### Check the findings

Read through the report before relying on it. Even a completed run can include incorrect claims, so check important figures, units, dates, and the passages it cites. Look at `failed_questions` for gaps. The sources array is pulled from the generated report, not a verified log of pages the agent read. It holds up to 20 URLs; that count doesn’t tell you whether the research is complete or accurate.

## Costs

The cost of a report depends on how much research it takes. There are four parts to keep track of:

* **Gateway hosting:** the Starter web service costs money while it’s running, including between questions. See [Render pricing](https://render.com/pricing).
* **Workflow usage:** Render charges for task compute and temporarily storing task inputs and results. See [Workflow limits and pricing](https://render.com/docs/workflows-limits).
* **Parallel usage:** Search pricing depends on the mode and requested results. Extract is billed per URL, and one call can include several URLs. See [Parallel pricing](/getting-started/pricing).
* **Model usage:** your provider charges for the model calls used to plan, research, and write the report. The model, token usage, and retries all affect the cost.

Use your providers’ usage records to check actual charges. The template’s `tool_calls_made` count combines Search and Extract calls from successful branches and leaves out failed attempts, so it won’t give you the full bill. Limiting submissions also won’t set a dollar cap on spending.

## Customize or run locally

Want to try it locally? Follow the repository’s [local development instructions](https://github.com/render-examples/parallel-research-agent#local-development) to start the Workflow server and gateway.

To make the agent your own, start with [workflow/tasks.py](https://github.com/render-examples/parallel-research-agent/blob/main/workflow/tasks.py). You can change the prompts, number of branches, and how many research turns each branch gets. On the Workflow, set `PARALLEL_SEARCH_MODE` to choose a [Search mode](/search/modes), or `PARALLEL_EXTRACT_MAX_CHARS` to adjust the Extract content budget.

## Clean up a test deployment

Finished testing? Save any reports you want to keep, disconnect the test Blueprint, and delete both the gateway and Workflow services. Disconnecting the Blueprint alone leaves those services running. Revoke any API keys you created just for the test, but keep keys that other apps still use.
