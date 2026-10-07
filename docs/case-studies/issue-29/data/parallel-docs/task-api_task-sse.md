> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Task API Streaming Events

> SSE for Task Runs

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## Overview

Task Runs support Server-Sent Events (SSE) at the run level, allowing you to receive real-time
updates on ongoing research conducted by our processors during execution.

For streaming events related to Task Groups, see the [streaming endpoints on the Task Group API](./group-api#5-retrieve-results).
Task Group streams provide aggregate group status updates and non-active run state changes. Task Run streams provide detailed events for individual task runs.
For a more comprehensive list of differences, [see here.](#differences-between-task-group-events-and-task-run-events)

### Enabling Events Streaming

To enable periodic event publishing for a task run, set the `enable_events` flag to `true`
when creating the task run. If not specified, events may still be available, but frequent updates are not guaranteed.

Create a Task Run with events aggregation enabled explicitly:

<CodeGroup>
  ```bash Task API theme={"system"}
  curl -X POST "https://api.parallel.ai/v1/tasks/runs" \
    -H "x-api-key: ${PARALLEL_API_KEY}" \
    -H "Content-Type: application/json" \
    --data '{
    "input": "What is the latest in AI research?",
    "processor": "lite",
    "enable_events": true
  }'
  ```
</CodeGroup>

To access the event stream for a specific run, use the `/v1/tasks/runs/{run_id}/events` endpoint:

<CodeGroup>
  ```bash Access event stream theme={"system"}
  curl -X GET "https://api.parallel.ai/v1/tasks/runs/trun_6eb64c73e4324b15af2a351bef6d0190/events" \
    -H "x-api-key: ${PARALLEL_API_KEY}" \
    -H "Accept: text/event-stream"
  ```
</CodeGroup>

This is what a sample stream looks like:

<CodeGroup>
  ```bash Event stream theme={"system"}

  event: task_run.state
  data: {"type":"task_run.state","event_id":null,"input":null,"run":{"run_id":"trun_aa9c7a780c9d4d4b9aa0ca064f61a6f7","interaction_id":"trun_aa9c7a780c9d4d4b9aa0ca064f61a6f7","status":"running","is_active":true,"warnings":null,"error":null,"processor":"pro","metadata":{},"taskgroup_id":null,"created_at":"2025-08-06T00:52:58.619503Z","modified_at":"2025-08-06T00:52:59.495063Z"},"output":null}

  event: task_run.progress_msg.exec_status
  data: {"type":"task_run.progress_msg.exec_status","message":"Starting research","timestamp":"2025-08-06T00:52:59.786126Z"}

  event: task_run.progress_msg.plan
  data: {"type":"task_run.progress_msg.plan","message":"I'm working on gathering information about Google's hiring in 2024, including where most jobs were created and any official announcements. I'll review recent news, reports, and Google's own statements to provide a comprehensive answer.","timestamp":"2025-08-06T00:53:19.281306Z"}

  event: task_run.progress_msg.tool_call
  data: {"type":"task_run.progress_msg.tool_call","message":"I've looked into Google's hiring activity in 2024, focusing on locations and official statements. I'll compile the findings and share a clear update with you shortly.","timestamp":"2025-08-06T00:53:28.282905Z"}

  event: task_run.progress_msg.search
  data: {"type":"task_run.progress_msg.search","message":"Objective: Find where Google created the most jobs in 2024","timestamp":"2025-08-06T00:53:30.114920Z"}

  event: task_run.progress_msg.search
  data: {"type":"task_run.progress_msg.search","message":"Query: Google hiring 2024","timestamp":"2025-08-06T00:53:30.114981Z"}

  event: task_run.progress_msg.search
  data: {"type":"task_run.progress_msg.search","message":"Query: Google 2024 job openings by location","timestamp":"2025-08-06T00:53:30.115044Z"}

  event: task_run.progress_stats
  data: {"type":"task_run.progress_stats","source_stats":{"num_sources_considered":223,"num_sources_read":22,"sources_read_sample":["http://stcloudlive.com/business/19-layoffs-coming-in-mid-march-at-st-cloud-arctic-cat-facility-company-says","http://snowgoer.com/snowmobiles/arctic-cat-sleds/putting-the-arctic-cat-layoffs-production-stop-in-context/32826","http://25newsnow.com/2024/07/26/cat-deere-cyclical-layoff-mode-say-industry-experts","http://citizen.org/article/big-tech-lobbying-update","http://businessalabama.com/women-in-tech-23-for-23","http://itif.org/publications/2019/10/28/policymakers-guide-techlash","http://distributech.com/","http://newyorker.com/magazine/2019/09/30/four-years-in-startups"]},"progress_meter":50.0}

  ...

  ```
</CodeGroup>

**Notes:**

* All [Task API processors](/task-api/guides/choose-a-processor) starting from `pro` and above have event streaming enabled by default.
* Event streams remain open for up to 570 seconds and close earlier when the run becomes non-active.

## Stream Behavior

While a run is active, starting or reconnecting to its stream replays the reasoning messages recorded so far before continuing with new updates. This lets clients reconnect without persisting every reasoning message themselves. Non-active runs behave differently, as described below.

### For Running Tasks

When connecting to a stream for a task that is still running:

* **Complete reasoning trace:** You receive all reasoning messages (`task_run.progress_msg.*`) from the beginning of the task execution, regardless of when you connect to the stream
* **Latest progress stats:** You receive only the current aggregate state via `task_run.progress_stats` events, not historical progress snapshots
* **Real-time updates:** As the task continues, you'll receive new reasoning messages and updated progress statistics
* **Closing status:** Before the connection closes—because the run became non-active or the 570-second window ended—the stream emits a `task_run.state` event. If the run completed successfully, the event includes the complete output

### For Non-Active Tasks

When connecting to a run that is already `completed`, `failed`, `cancelled`, or `action_required`:

* **Current status only:** You receive one `task_run.state` event with the run's current status
* **Immediate result:** If the run completed successfully, that status event includes the complete task output in the `output` field, so you don't also need to use the result endpoint
* **No trace replay:** Reasoning messages and progress statistics are not replayed after the run becomes non-active

### Reconnection Behavior

* Event streams are **not resumable** - there are no sequence numbers or cursors to resume from a specific point

* If you disconnect and reconnect to the same task:

  * **Running tasks:** You get the reasoning trace recorded so far plus current progress stats
  * **Non-active tasks:** You get the current status event only

* Every connection starts with a `task_run.state` event indicating the current status

### Supported Events

Currently, four types of events are supported:

* **Run Status Events (`task_run.state`):** Indicate the current status of the run. These are sent at the beginning of every stream and when the run transitions to a non-active status.

* **Progress Statistics Events (`task_run.progress_stats`):** Provide point-in-time updates on the number of sources considered and other aggregate statistics. Only the current state is provided, not historical snapshots.

* **Message Events (`task_run.progress_msg.*`):** Communicate progress at various stages of task run execution. While progress data is available, the sequence from the beginning of execution is provided. Subtypes include `.plan` (planning), `.tool_call` (tool use), `.result` (intermediate findings), `.exec_status` (execution status), and `.search` (search activity).

  * **Search events (`task_run.progress_msg.search`)** record what the run searched for. Each `message` is prefixed `Objective:` (the goal being researched at that step) or `Query:` (a web search query that was run). A search step typically produces one `Objective:` line followed by one or more `Query:` lines. Reasoning messages such as `.plan` / `.tool_call` may be limited on `lite`; search events are emitted on `base` and above.

* **Error Events (`error`):** Report errors that occur during execution.

<Note>
  Surfaced search queries reflect what the engine searched for at each step, for auditability and observability. When a step issues no explicit query, only the `Objective:` line appears.
</Note>

**Additional Notes:**

* Event streams always start and end with a status event; for an already non-active run, one event serves as both
* The final status event for completed tasks always includes the complete output in the `output` field
* Events within the reasoning trace maintain their original timestamps, allowing you to understand the execution timeline
* After a run becomes non-active, reconnecting to the event stream returns its current status but does not replay the reasoning trace.

For the full specification of each event, see the examples above.

### Differences Between Task Group Events and Task Run Events

Task Group events are not a strict superset of Task Run events. See the differences below:

| | Task Run Events | Task Group Events |
| - | - | - |
| **Purpose** | Events for a single Task Run. | Events for an entire Task Group. |
| **Run-level events** | Progress updates, messages, status changes. | Only non-active run status changes. |
| **Resumable streams** | No | Yes, using `event_id`. |
| **Events supported** | Progress updates, messages, status changes, and errors for an individual run. | Group status, non-active run state changes, and stream errors. |
| **Reasoning trace** | Available while the run is active; not replayed after it becomes non-active. | Not available. |
| **Final results** | Included in the status event when the run completes successfully. | Available through separate API. |
