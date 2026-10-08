> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Processors

> Select the right Task API processor (lite, base, core, pro, ultra) based on task complexity and latency requirements

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Processors are the engines that execute Task Runs. The choice of Processor determines the performance profile and reasoning behavior used. Any Task Run can be executed on any Processor.

<Tip> Choose a processor based on **task complexity**. Use `lite` or `base` for simple enrichments, `core` for reliable accuracy on up to 10 output fields, and `pro` or `ultra` when reasoning depth is critical. For latency-sensitive workflows—including agentic research where an agent is waiting on results—use the [Responses API](/responses-api/responses-quickstart) instead. </Tip>

Each processor varies in performance characteristics and supported features. Use the table below to compare processors.

<Note>
  **About "Max Fields" (the `~` symbol):** The max fields column shows approximate limits because actual capacity depends on field complexity. Simple fields like dates or booleans use less capacity than complex fields requiring extensive research. A task with 5 complex analytical fields may require more processing than one with 15 simple lookup fields. Use these numbers as guidelines. If you're near the limit and seeing quality issues, try a higher-tier processor.
</Note>

| Processor | Latency | Strengths | Max Fields |
| - | - | - | - |
| `lite` | 10s - 2min | Basic metadata, fallback, low latency | \~2 fields |
| `base` | 15s - 3min | Reliable standard enrichments | \~5 fields |
| `core` | 60s - 5min | Cross-referenced, moderately complex outputs | \~10 fields |
| `core2x` | 60s - 10min | High complexity cross referenced outputs | \~10 fields |
| `pro` | 2min - 10min | Exploratory web research | \~20 fields |
| `ultra` | 3min - 25min | Advanced multi-source deep research | \~20 fields |
| `ultra2x` | 5min - 50min | Difficult deep research | \~25 fields |
| `ultra4x` | 5min - 90min | Very difficult deep research | \~25 fields |
| `ultra8x` | 5min - 2hr | The most difficult deep research | \~25 fields |

<Note>
  See [Pricing](/getting-started/pricing) for processor costs and all API rates.
</Note>

## Observed Latency

The table below shows median (p50) and 90th percentile (p90) **execution times** measured in production for standard processors across all customer workloads. Your results will vary with task complexity, output schema size, and input difficulty.

| Processor | p50 | p90 |
| - | - | - |
| `lite` | 45s | 1.5min |
| `base` | 50s | 2min |
| `core` | 1.5min | 3min |
| `core2x` | 3.5min | 6.5min |
| `pro` | 3.5min | 7.5min |
| `ultra` | 4min | 10min |
| `ultra2x` | 6min | 9.5min |
| `ultra4x` | 8min | 10.5min |
| `ultra8x` | 6.5min | 11min |

### Execution Time vs Queue Time

A Task Run moves through three states: **queued → running → completed** on success, or transitions to **failed** from either **queued** or **running**. The latencies above measure only the `running` phase—the time a processor actively spends executing your task. Time spent in the queue **is not included**.

Queue time varies with your workload. Runs execute concurrently, but processing capacity is finite: when a large burst of runs is submitted at once, runs beyond the available capacity wait in the queue until capacity frees up. End-to-end time (creation to completion) can therefore exceed the execution time ranges above.

<Tip>
  For latency-sensitive workflows, use the [Responses API](/responses-api/responses-quickstart).
</Tip>

## Latency-Sensitive Workflows

The Task API is designed for asynchronous, accuracy-critical research: runs are queued, executed, and retrieved when complete. If your use case is latency-sensitive—an interactive application, a subagent call, or any agentic research workflow where a caller is actively waiting on results—use the [Responses API](/responses-api/responses-quickstart) instead. It is purpose-built for low-latency requests.

## Examples

Processors can be used flexibly depending on the scope and structure of your task. The examples below show how to:

* Use a single processor (like `lite`, `base`, `core`, `pro`, or `ultra`) to handle specific types of input and reasoning depth.
* Chain processors together to combine fast lookups with deeper synthesis.

This structure enables flexibility across a variety of tasks—whether you're extracting metadata, enriching structured records, or generating analytical reports.

### Sample Task for each Processor

<CodeGroup>
  ```python lite theme={"system"}
  task_run = client.task_run.create(
      input="Parallel Web Systems (parallel.ai)",
      task_spec={"output_schema":"The founding date of the company in the format MM-YYYY"},
      processor="lite"
  )
  print(f"Run ID: {task_run.run_id}")

  run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
  print(run_result.output)
  ```

  ```python base theme={"system"}
  task_run = client.task_run.create(
      input="Parallel Web Systems (parallel.ai)",
      task_spec={"output_schema":"The founding date and most recent product launch of the company"},
      processor="base"
  )
  print(f"Run ID: {task_run.run_id}")

  run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
  print(run_result.output)
  ```

  ```python core theme={"system"}
  task_run = client.task_run.create(
      input="Parallel Web Systems (parallel.ai)",
      task_spec={"output_schema":"The founding date, founders, and most recent product launch of the company"},
      processor="core"
  )
  print(f"Run ID: {task_run.run_id}")

  run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
  print(run_result.output)
  ```

  ```python pro theme={"system"}
  task_run = client.task_run.create(
      input="Parallel Web Systems (parallel.ai)",
      task_spec={"output_schema":"The founding date, founders, mission, benchmarked competitors and most recent product launch of the company"},
      processor="pro"
  )
  print(f"Run ID: {task_run.run_id}")

  run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
  print(run_result.output)
  ```

  ```python ultra theme={"system"}
  task_run = client.task_run.create(
      input="Parallel Web Systems (parallel.ai)",
      task_spec={"output_schema":"A comprehensive analysis of the industry of the company, including growth factors and major competitors."},
      processor="ultra"
  )
  print(f"Run ID: {task_run.run_id}")

  run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
  print(run_result.output)
  ```
</CodeGroup>

### Multi-Processor Workflows

You can combine processors in sequence to support more advanced workflows.

Start by retrieving basic information with `base`:

```python theme={"system"}
task_run_base = client.task_run.create(
    input="Pfizer",
    task_spec={"output_schema":"Who are the current executive leaders at Pfizer? Include their full name and title. Ensure that you retrieve this information from a reliable source, such as major news outlets or the company website."},
    processor="base"
)
print(f"Run ID: {task_run_base.run_id}")

base_result = client.task_run.result(task_run_base.run_id, api_timeout=3600)
print(base_result.output)
```

Then use the result as input to `core` to generate detailed background information:

```python theme={"system"}
import json

task_run = client.task_run.create(
    input=json.dumps(base_result.output.content),
    task_spec={"output_schema":"For the executive provided, find their professional background tenure at their current company, and notable strategic responsibilities."},
    processor="pro"
)
print(f"Run ID: {task_run.run_id}")

run_result = client.task_run.result(task_run.run_id, api_timeout=3600)
print(run_result.output)
```

This lets you use a lower compute processor for initial retrieval, then switch to a more capable one for analysis and context-building.
