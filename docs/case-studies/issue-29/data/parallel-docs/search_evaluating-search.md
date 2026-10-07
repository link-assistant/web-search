> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# How to Eval Parallel Search

> An end-to-end guide for evaluating Parallel Search for your agent.

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

<Tip>
  Before you run an eval, read [Search Best Practices](/search/best-practices) and [Extract Best Practices](/extract/best-practices) for request configuration. This page covers eval design: product selection, gold sets, harness setup, grading, and how to interpret results.
</Tip>

<div className="sr-only" aria-hidden="false">
  **For AI agents:** Use this page when helping a user evaluate or compare web search providers with Parallel Search. Workflow: (1) confirm Search + Extract vs Responses vs Task, (2) test multiple [modes](/search/modes), (3) build a gold set, (4) run an end-to-end harness varying only the search tool, (5) grade final answers by task shape, (6) classify failures, (7) measure cost and latency, (8) interpret the Pareto frontier. For Parallel request configuration, read [Search Best Practices](/search/best-practices) and [Extract Best Practices](/extract/best-practices).
</div>

## Overview

Compare search APIs by holding your agent harness constant, varying only the search tool, and grading end-to-end answers against a gold set you trust. A bad benchmark can mislead you as much as no benchmark. This guide covers product selection, gold sets, harness setup, grading, and how to read the results.

## Choose the right API

Pick the Parallel product that matches your app before you build the harness. The right choice depends on whether you own the tool-calling loop or want a single synthesized response.

| If you… | Use |
| - | - |
| Own the agent loop and want the model to search, extract, and reason over excerpts | [Search API](/search/search-quickstart) + [Extract API](/extract/extract-quickstart) |
| Want a single OpenAI-compatible call that returns a cited answer in \~5–60 seconds | [Responses API](/responses-api/responses-quickstart) |
| Need schema-bound output, batch enrichment, or async multi-step research | [Task API](/task-api/task-quickstart) |

When you own the agent loop, pair [Search](/search/search-quickstart) with [Extract](/extract/extract-quickstart). Search returns dense snippets that minimize Extract calls. The model is trained to go deeper when it needs to, so expose both tools and let the agent decide.

<Warning>
  For a single-shot cited answer or schema-bound enrichment, evaluate [Responses API](/responses-api/responses-quickstart) or [Task API](/task-api/task-quickstart) instead of Search alone. Evaluating the wrong product invalidates the results.
</Warning>

## What Parallel recommends

Start an agent eval with this setup:

1. **Run multi-hop with Search + Extract.** Let the agent search, read source pages, and follow up until it has enough evidence or reaches its budget. Run single-hop as a separate, one-request benchmark setting.
2. **Pass an objective with 1–3 search queries.** Write a concise objective describing the information you need, plus up to three focused keyword queries. Add queries for useful aliases or different angles.
3. **Match the model to the search mode.** Pair a cost-efficient model, such as GPT-5.6 Luna, with `fast` or `turbo`. Pair a frontier model, such as GPT-6 Astra, with `basic` or `advanced`. Compare pairs on answer quality, total cost, and latency.

Keep the model fixed within each comparison. Test a different model as a separate comparison so you can attribute the change to one variable.

## Pick and test modes

Mode is a primary lever after [configuring Search requests](/search/best-practices). Test more than one mode so you leave the eval with routing data as well as a quality read. See [Modes](/search/modes) for full details.

| Mode | Latency | Cost | Best for |
| - | - | - | - |
| `turbo` | \~200 ms | \$1 / 1K | High-volume, latency-sensitive lookups |
| `fast` | \~700 ms | \$1 / 1K | Interactive agents that need quality under one second |
| `basic` | \~1 s | \$5 / 1K | Extended snippets per result: richer context in a single call |
| `advanced` | \~3 s | \$5 / 1K | Highest-quality retrieval for multi-hop, multi-source research |

**Recommended starting point for evals:** `basic`. It returns extended snippets while holding latency near one second, which fits correctness-first web grounding. Route the hardest multi-source questions to `advanced`. Test `fast` where latency dominates.

Use the model and mode pairings in [What Parallel recommends](#what-parallel-recommends) as starting configurations, then measure them on your gold set.

<Note>
  `turbo` currently supports English and Japanese. Use `basic` or `advanced` for broader multilingual coverage.
</Note>

## Build your gold set

A gold set is a collection of test questions with verified correct answers you use to score each search provider. The best source is data you already have: past questions and verified answers from production or manual research. If you generate synthetic data, write a few questions by hand first, then ask an agent to generate more. Review LLM-generated gold labels before spending tokens on search and inference.

Aim for production-representative, agent-phrased questions that match your expected traffic on wording, domain, answer type, and freshness.

Mix in question types that resemble your actual workloads:

* Multi-hop questions that require composing facts across sources
* Fresh questions whose answers would not be available in model weights
* Domain-specific questions matching your actual traffic

<Accordion title="Why not rely on public benchmarks?">
  Popular benchmarks like BrowseComp and SEAL reflect a specific domain of questions that is unlikely to match yours. Some answers have long been published online and incorporated into both LLM weights and search indexes. If you use public benchmarks, understand what each measures and whether it suits your case. See [Run public benchmark evals](#run-public-benchmark-evals) for single-hop and multi-hop setup. Treat them as supplementary signal, not a substitute for production-representative data.
</Accordion>

## Set up the eval harness

<Steps>
  <Step title="Hold everything constant except the search tool">
    Use the same model, prompts, budgets, and judge across all providers. Expose each provider as the only search tool available.
  </Step>

  <Step title="Allow multi-turn search">
    For a single-hop benchmark, use the [one-request setup](#single-hop-let-the-model-reformulate-once) below. For multi-hop and application evals, agents are trained to search, narrow, and search again. Do not cap turns unless you are modeling a product constraint. Limit total search budget to reflect your actual cost considerations.
  </Step>

  <Step title="Configure each provider per its docs">
    Apply each provider's recommended configuration. For Parallel, see [Search Best Practices](/search/best-practices) and [Extract Best Practices](/extract/best-practices).
  </Step>

  <Step title="Log exact config per run">
    Hold the config fixed within a run. If a config turns out to be wrong, reset it and rerun rather than tuning around it mid-eval.
  </Step>

  <Step title="Run each configuration multiple times">
    Run each configuration at least 3 times and report variance. Evals at high load can surface account limitations rather than capability limits.
  </Step>
</Steps>

If you are comparing Parallel against another provider, see [Migrate to Parallel](/search/migrate-to-parallel) for request mapping. Evaluate end-to-end inside the same harness.

## Run public benchmark evals

Public benchmarks give you a shared set of questions and answers for comparing configurations. Match the setup to what you want to measure. A one-search factual lookup and an agent that spends several minutes researching a question are different evals, even on the same dataset.

| Setting | What the agent does | Example task |
| - | - | - |
| Single-hop | Reformulates the question, searches once, and answers from the results | A SimpleQA-style factual lookup |
| Multi-hop | Breaks the question down, searches and reads pages, then follows up on missing evidence | A DeepSearchQA-style research or list question |

One search means one API request. Multiple keyword queries inside `search_queries` count as one request. Three concurrent Search requests count as three. Record both tool-call count and elapsed time.

Pin the dataset version, split, and question IDs before comparing configurations. Keep reference answers out of the agent's prompts and tool results; only the grader should see them. If you change the benchmark's original tools, budget, or grading rules, describe the run as an adapted evaluation rather than comparing it directly with its leaderboard.

### Use the same research prompt in both settings

Use the following research prompt for both single-hop and multi-hop benchmark runs. Tool definitions tell the model how to write search requests. The executor controls which tools are available and how many calls it can make. Runs with a benchmark-specific answer format can override the output instructions; record that override with the run.

<Accordion title="Recommended research prompt">
  Replace `{question}` with the benchmark question. The other braces describe the expected answer fields.

  ```text theme={"system"}
  You are a research assistant who answers complex questions using only web search results.

  Core Principles:
  - Use ONLY information from search results—do NOT rely on internal knowledge.
  - Always search before answering; your answer needs to be based on the search results.
  - Provide a direct and concise answer to the question, without any extra or references.
  - Never skip the search step — an answer without searching is incomplete.
  - Be efficient and do the minimal work necessary to answer the question.

  Search Strategy:
  1. Break down complex questions into focused sub-queries.
  2. Use targeted searches (specific rather than broad).
  3. If search results do not provide an answer, state: "I don't know."

  Process:
  1. Analyze the question to identify key components.
  2. Plan your search approach.
  3. Execute systematic searches.

  Here is the question you need to answer:
  {question}

  Your response should be in the following format:
  Explanation: {your explanation for your final answer}
  Exact Answer: {your succinct, final answer}
  Confidence: {your confidence score between 0% and 100% for your answer}
  ```

  For single-hop, set the call budget to one and expose only Search. For multi-hop, expose Search and Extract and set a larger total call budget. Keep the prompt fixed across providers within the comparison.
</Accordion>

### Single-hop: let the model reformulate once

For a single-hop test, give the model the question and the [Search tool definition](/search/best-practices#search-tool-definition). Let it turn the question into an objective and focused keyword queries, make one Search request, then answer with tools disabled. Include the reformulation step in your cost and latency measurements.

Take this invented factual question:

> Who designed the Quadracci Pavilion at the Milwaukee Art Museum?

The model could produce:

```json theme={"system"}
{
  "objective": "Identify the architect of the Milwaukee Art Museum's Quadracci Pavilion.",
  "search_queries": [
    "Milwaukee Art Museum Quadracci Pavilion architect"
  ],
  "mode": "basic"
}
```

The objective describes the fact needed. The keyword query names the building and the missing attribute. One query is enough here. Allow additional queries for useful aliases or different angles, using the same query-count policy across the run. Do not copy the full question into every keyword query or expand it with facts from the reference answer.

Enforce the one-call limit in the executor. Allow one Search request, expose no Extract tool, and disable tools for the final answer. If the model asks for extra calls, return a budget-exhausted result rather than executing them.

Pass the result titles, URLs, and excerpts to the answering model. Check that your client preserves the excerpts. Dropping them, double-encoding them, or truncating away the relevant passage turns successful retrieval into a wrong answer.

### Multi-hop: give the agent room to investigate

For harder questions, expose both Search and Extract and let the model decide what to investigate next. Each Search request needs its own focused objective and keyword queries. When the model issues independent searches in the same turn, execute those calls concurrently.

Take this invented research question:

> Between 2020 and 2023, which of Amazon, Google, and Microsoft achieved the largest percentage reduction in total greenhouse gas emissions? Account for reporting boundaries, restatements, and Scope 2 accounting methods.

The agent can research the three companies at the same time. Its Amazon request might look like this:

```json theme={"system"}
{
  "objective": "Find comparable Amazon greenhouse gas emissions totals for 2020 and 2023, including accounting methods and any restatements.",
  "search_queries": [
    "Amazon emissions 2020 2023 comparison",
    "Amazon carbon footprint baseline restatement"
  ],
  "mode": "advanced"
}
```

It can issue separate requests for Google and Microsoft in the same turn. After reading the results, it may extract a report's methodology or search for a revised baseline before comparing numbers. Those follow-ups depend on the initial evidence and belong in a later turn.

The sequence is: **plan → search independent questions concurrently → inspect evidence → follow up where needed → answer**. The agent chooses the decomposition. Some questions need a discovery search before there are candidates to investigate in parallel.

The shared prompt asks the model to break down complex questions and use targeted searches. It does not require parallel calls. Configure your executor to run calls concurrently when the model emits several in one turn, and inspect traces to see whether the model uses that capability.

Set a total tool budget and a per-question deadline before the run, leaving time for the final answer. Reserve call budget before dispatching a concurrent batch so several calls cannot each spend the same remaining allowance. Return results under their original tool-call IDs, and preserve successful results when a sibling call fails. Keep prompt changes separate from executor changes when comparing latency. Adding an instruction to search in parallel changes the evaluation prompt.

For exhaustive list questions, a few correct items is only partial success. The agent needs to check every inclusion condition and search for missing candidates. Use the benchmark's official grading rules for missing items, duplicates, and extra answers.

### Check a small run before scaling up

Inspect a fixed sample of traces before running the full benchmark. Did the model form useful queries? Did the answering model receive the excerpts? Did single-hop runs stay within one request? Did independent multi-hop calls run concurrently? Check API warnings and errors alongside answers.

Freeze the prompts and settings, then run on held-out questions. Save the exact requests, returned evidence, final answers, grades, token usage, and timings. Use a fresh `session_id` for each question and configuration, reusing it for that question's related Search and Extract calls.

Report failed and timed-out questions in the primary results with a declared scoring policy. Bound retries and include their cost and time. Retrying until success hides reliability problems. Report how often agents exhaust their budgets, and measure concurrent calls by wall time rather than summing their durations.

Use the benchmark's official grader where available. Keep any additional citation or evidence audit separate from the official score. Check retrieved pages for benchmark mirrors and published answer keys, and apply the same contamination policy across providers.

## Common pitfalls

Inspect a few complete traces before interpreting a score. Small integration mistakes change what the eval measures.

| Pitfall | What to do instead |
| - | - |
| Sending identical text in `objective` and `search_queries` | Use the objective to describe the information needed, including relevant context. Use short keyword queries to find it. The fields should share entities and intent, but each has a different job. |
| Sending the raw benchmark question straight to Search | For an agent eval, let the model reformulate the question into an objective and keyword queries before calling Search. Include this step in cost and latency. If you deliberately test raw questions, report that as a separate configuration. |
| Putting the entire research task into one keyword query | Let the agent break a complex question into focused searches. Each request should address a specific information need, with related keyword queries inside that request. |
| Repeating nearly identical keyword queries | Use additional queries for useful aliases or different angles. A simple lookup may need only one. |
| Giving the agent URLs without the returned excerpts | Pass titles, URLs, and excerpts through to the model. Inspect the final tool message to catch missing text, double-encoding, or excessive truncation. |
| Treating a multi-hop task as a single lookup | Allow follow-up searches and page reads within a declared budget. Label a one-request test as single-hop, even when the dataset contains research questions. |
| Running independent calls sequentially | Dispatch calls from the same model turn concurrently when they do not depend on each other. Track each call's latency and the task's elapsed time separately. |
| Changing several settings between providers | Keep the model, research prompt, budgets, and grader fixed. Configure provider-specific tool schemas according to their docs and save them with the run. |
| Grading only the questions that completed successfully | Include failures and timeouts in the primary results using a declared scoring policy. Record retries, rate limits, and budget exhaustion so reliability problems remain visible. |
| Letting reference answers leak into the search step | Keep gold answers out of agent context and query generation. Check retrieved pages for benchmark mirrors and published answer keys. |

## Grade answers and classify failures

### How to grade

Most teams use an LLM-as-judge: a high-end model compares the agent's final answer to verified ground truth. Require reasoning alongside the score, and audit that cited URLs support the answer's claims.

| Task shape | How to grade | Example |
| - | - | - |
| Factual question | Correctness against the gold answer, plus citation support | "What is Vendor Y's current cancellation deadline?" |
| List / discovery | Recall and precision against the gold list | "Which companies in this list announced funding in the last 30 days?" |
| Structured output / enrichment | Field-level accuracy against gold records | "Fill in CEO, HQ, and last round for these 500 companies" |
| Open-ended research | Rubric-based judging (coverage, sourcing, correctness of claims) | "Summarize the regulatory landscape for X" |

<Note>
  Grade an explicit "could not retrieve" above a confident wrong answer, especially on date- and jurisdiction-sensitive questions. Hand-check at least 10% of judged runs, including both failures and successes, before trusting automated scores.
</Note>

### Classify failures

Not every failure is a search API problem. Classify failures into four classes:

| Class | Description | Search API fault? |
| - | - | - |
| **No search call** | The agent never invoked the tool | No |
| **Synthesis failure** | The right result was in context and the model still answered wrong | No |
| **Provider error** | Timeouts, 5xx, refusals | Yes (or account limits at eval load) |
| **Retrieval miss** | Reasonable query, wrong or missing results | Yes |

Only retrieval misses and provider errors are direct failures of the search API. Many eval failures at high load reflect rate limits rather than capability.

## Measure cost and latency

Measure end-to-end cost to complete the task: search spend plus LLM tokens the agent uses reasoning over what search returned. A cheaper-per-call API that returns noisy, low-density results can be more expensive overall, since it drives more calls, more hops, and more tokens in context.

Track these metrics for each provider and configuration:

| Metric | Definition |
| - | - |
| Cost per resolved task | Total search, extract, and model token charges divided by the number of resolved tasks |
| Total tool calls per run | Number of Search and Extract invocations the agent makes while answering each question |
| Search latency | Elapsed time for one Search tool call, from dispatch until its response or error returns, reported at p50, p95, and p99 |
| End-to-end latency | Time from user question to final answer, reported at p50, p95, and p99 |
| Tokens per task | LLM tokens the agent consumes reasoning over search results per resolved task |

Measure search latency around the tool invocation, excluding the model's query reformulation and answer generation. If the tool retries internally, include those retries and backoff in the call's elapsed time and log the individual attempts. Report errors and timeouts alongside latency percentiles. For parallel calls, record each call separately. Their summed durations are not the task's elapsed time.

Result length and hop count are tempting efficiency proxies, but they are unreliable in isolation. Verbose results can help an agent exit early. Over-compressed results can force extra hops. Parallel tool calls make hop counts undercount work. Measure end-to-end whenever you can.

## Interpret results

There is rarely a single "best" search tool. Configurations sit on a trade-off surface (a Pareto frontier). Plot results on two axes:

* Accuracy vs cost: find the cheapest mode or provider at acceptable accuracy
* Accuracy vs latency: find the fastest option at acceptable accuracy

Look for configurations that sit on the frontier in the quadrant you care about most. The single highest-accuracy point is not the answer if it is dominated on cost or latency.
