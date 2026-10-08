> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Evict from Memory

> Removes a task run, monitor, or FindAll run from the selected memory without deleting the original resource.



## OpenAPI

````yaml /public-openapi.json post /v1beta/memory/evict
openapi: 3.1.0
info:
  title: Parallel API
  description: Parallel API
  contact:
    name: Parallel Support
    url: https://parallel.ai
    email: support@parallel.ai
  version: 0.1.2
servers:
  - url: https://api.parallel.ai
    description: Parallel API
security:
  - ApiKeyAuth: []
tags:
  - name: Search
    description: >-
      Search returns ranked URLs with extended excerpts suitable for LLM
      consumption. Inputs are a natural-language objective and optional keyword
      queries. Source policies allow including or excluding specific domains and
      have configurable output sizes. The returned extended snippets contain
      dense, relevant information from relevant pages.

      - Result: ranked list with URL, title, and long text excerpts
  - name: Image Search
    description: >-
      Image Search returns ranked images from the web. Inputs are keyword
      queries and an optional natural-language objective. `fast`: lowest latency
      and cost. `advanced` (default): slower, with higher-quality results.

      - Result: title, direct image URL, source page URL, and dimensions
  - name: Extract
    description: >-
      Extract returns excerpts or full content from one or more URLs. Inputs are
      a list of URLs and an optional search objective and keyword queries. The
      returned excerpts or full content is formatted as markdown and suitable
      for LLM consumption.

      - Result: excerpts or full content from the URL formatted as markdown
  - name: Tasks
    description: >-
      The Task API executes web research and extraction tasks. Clients submit a
      natural-language objective with an optional input schema; the service
      plans retrieval, fetches relevant URLs, and returns outputs that conform
      to a provided or inferred JSON schema. Supports deep research style
      queries and can return rich structured JSON outputs. Processors trade-off
      between cost, latency, and quality. Each processor supports calibrated
      confidences.

      - Output metadata: citations, excerpts, reasoning, and confidence per
      field


      Task Groups enable batch execution of many independent Task runs with
      group-level monitoring and failure handling.

      - Submit hundreds or thousands of Tasks as a single group

      - Observe group progress and receive results as they complete

      - Real-time updates via Server-Sent Events (SSE)

      - Add tasks to an existing group while it is running

      - Group-level retry and error aggregation
  - name: FindAll
    description: >-
      The FindAll API discovers and evaluates entities that match complex
      criteria from natural language objectives. Submit a high-level goal and
      the service automatically generates structured match conditions, discovers
      relevant candidates, and evaluates each against the criteria. Returns
      comprehensive results with detailed reasoning, citations, and confidence
      scores for each match decision. Streaming events and webhooks are
      supported.
  - name: Monitor
    description: >-
      The Monitor API watches the web for material changes on a fixed frequency.
      Each monitor runs once on creation and then on its configured schedule,
      emitting events when meaningful changes are detected.

      - `event_stream` monitors track a search query and emit an event for each
      new material change.

      - `snapshot` monitors track a specific task run's output and emit an event
      when the output changes.


      Results can be polled via the events endpoint or delivered via webhooks.
  - name: Memory
    description: >-
      The Memory API retrieves and manages memories created by Tasks, Monitors,
      and FindAll runs. Memories can be personal or isolated with a
      `memory_scope_key`.
  - name: Chat API (Beta)
    description: >-
      The Chat API provides a programmatic chat-style text generation interface.
      It accepts a sequence of messages and returns model responses. Intended
      for assistant-like interactions and evaluation. Streaming responses are
      supported.
  - name: Responses API
    description: >-
      An OpenAI-Responses-compatible interface for answers grounded in live web
      research, with URL citations. Point any Responses-API client — the OpenAI
      Python SDK, OpenAI TypeScript SDK, the Agents SDK, or raw HTTP — at
      `https://api.parallel.ai` with your Parallel API key, set `model` to
      `parallel`, and call `/v1/responses`.

      - `input` accepts a plain string or an array of role/content messages
      (canonical OpenAI shape; text content only).

      - `reasoning.effort` (`low`/`medium`/`high`) controls how much research is
      performed, trading response time for answer quality.

      - Multi-turn via `previous_response_id`.

      - Structured outputs via `text.format = {"type": "json_schema", "name":
      ..., "schema": {...}}`.

      - Streaming (`stream=true`) emits the standard OpenAI Responses SSE
      lifecycle: `response.created` and `response.in_progress`, then output item
      / content part / text delta events with URL-citation annotations, the
      matching `*.done` events, and a terminal `response.completed` — or
      `response.failed` if the request fails mid-stream.
paths:
  /v1beta/memory/evict:
    post:
      tags:
        - Memory
      summary: Evict from Memory
      description: >-
        Removes a task run, monitor, or FindAll run from the selected memory
        without deleting the original resource.
      operationId: evict_memory_source_v1beta_memory_evict_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/MemoryEvictRequest'
        required: true
      responses:
        '204':
          description: Source removed from memory.
        '422':
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HTTPValidationError'
components:
  schemas:
    MemoryEvictRequest:
      properties:
        memory_scope_key:
          anyOf:
            - type: string
              maxLength: 128
              minLength: 1
              pattern: ^[a-zA-Z0-9_-]+$
            - type: 'null'
          title: Memory Scope Key
          description: >-
            User-provided key identifying the memory scope to use. Omit to use
            personal memory, if available.
        kind:
          type: string
          enum:
            - task
            - monitor
            - findall
          title: Kind
          description: 'Kind of source to evict: `task`, `monitor`, or `findall`.'
        id:
          type: string
          maxLength: 128
          minLength: 1
          pattern: ^[a-zA-Z0-9_-]+$
          title: Id
          description: ID of the task run, monitor, or FindAll run to evict.
      type: object
      required:
        - kind
        - id
      title: MemoryEvictRequest
    HTTPValidationError:
      properties:
        detail:
          items:
            $ref: '#/components/schemas/ValidationError'
          type: array
          title: Detail
      type: object
      title: HTTPValidationError
    ValidationError:
      properties:
        loc:
          items:
            anyOf:
              - type: string
              - type: integer
          type: array
          title: Location
        msg:
          title: Message
          type: string
        type:
          type: string
          title: Error Type
      type: object
      required:
        - loc
        - msg
        - type
      title: ValidationError
  securitySchemes:
    ApiKeyAuth:
      type: apiKey
      in: header
      name: x-api-key

````