> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Retrieve Task Run Input

> Retrieves the input of a run by run_id.



## OpenAPI

````yaml /public-openapi.json get /v1/tasks/runs/{run_id}/input
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
  /v1/tasks/runs/{run_id}/input:
    get:
      tags:
        - Tasks
      summary: Retrieve Task Run Input
      description: Retrieves the input of a run by run_id.
      operationId: tasks_runs_input_get_v1_tasks_runs__run_id__input_get
      parameters:
        - name: run_id
          in: path
          required: true
          schema:
            type: string
            title: Run Id
      responses:
        '200':
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/TaskRunInput'
              example:
                processor: core
                metadata:
                  my_key: my_value
                task_spec:
                  output_schema:
                    json_schema:
                      type: object
                      properties:
                        gdp:
                          type: string
                          description: >-
                            GDP in USD for the year, formatted like '$3.1
                            trillion (2023)'
                      required:
                        - gdp
                      additionalProperties: false
                    type: json
                  input_schema:
                    json_schema:
                      type: object
                      properties:
                        country:
                          type: string
                        year:
                          type: integer
                      required:
                        - country
                        - year
                      additionalProperties: false
                    type: json
                input:
                  country: France
                  year: 2023
        '401':
          description: 'Unauthorized: invalid or missing credentials'
          content:
            application/json:
              example:
                type: error
                error:
                  ref_id: fcb2b4f3-c75e-4186-87bc-caa1a8381331
                  message: 'Unauthorized: invalid or missing credentials'
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '404':
          description: Run id not found
          content:
            application/json:
              example:
                type: error
                error:
                  ref_id: fcb2b4f3-c75e-4186-87bc-caa1a8381331
                  message: Run id not found
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '422':
          description: Request validation error
          content:
            application/json:
              example:
                type: error
                error:
                  ref_id: fcb2b4f3-c75e-4186-87bc-caa1a8381331
                  message: Request validation error
              schema:
                $ref: '#/components/schemas/ErrorResponse'
components:
  schemas:
    TaskRunInput:
      properties:
        processor:
          type: string
          title: Processor
          description: Processor to use for the task.
          examples:
            - base
        metadata:
          anyOf:
            - additionalProperties:
                anyOf:
                  - type: string
                  - type: integer
                  - type: number
                  - type: boolean
              type: object
            - type: 'null'
          title: Metadata
          description: >-
            User-provided metadata stored with the run. Keys and values must be
            strings with a maximum length of 16 and 512 characters respectively.
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
        source_policy:
          anyOf:
            - $ref: '#/components/schemas/SourcePolicy'
            - type: 'null'
          description: >-
            Optional source policy governing included and excluded domains and
            domain/path prefixes in web search results.
        advanced_settings:
          anyOf:
            - $ref: '#/components/schemas/TaskAdvancedSettings'
            - type: 'null'
          description: Advanced search configuration for the task run.
        task_spec:
          anyOf:
            - $ref: '#/components/schemas/TaskSpec'
            - type: 'null'
          description: Task specification. If unspecified, defaults to auto output schema.
        input:
          anyOf:
            - type: string
            - additionalProperties: true
              type: object
          title: Input
          description: Input to the task, either text or a JSON object.
          examples:
            - What was the GDP of France in 2023?
            - '{"country": "France", "year": 2023}'
        previous_interaction_id:
          anyOf:
            - type: string
            - type: 'null'
          title: Previous Interaction Id
          description: Interaction ID to use as context for this request.
        mcp_servers:
          anyOf:
            - items:
                $ref: '#/components/schemas/McpServer'
              type: array
            - type: 'null'
          title: Mcp Servers
          description: Optional list of MCP servers to use for the run.
        enable_events:
          anyOf:
            - type: boolean
            - type: 'null'
          title: Enable Events
          description: >-
            Controls tracking of task run execution progress. When set to true,
            progress events are recorded and can be accessed via the [Task Run
            events](https://docs.parallel.ai/api-reference) endpoint. When
            false, no progress events are tracked. Note that progress tracking
            cannot be enabled after a run has been created. The flag is set to
            true by default for premium processors (pro and above).
        webhook:
          anyOf:
            - $ref: '#/components/schemas/Webhook'
            - type: 'null'
          description: >-
            Callback URL (webhook endpoint) that will receive an HTTP POST when
            the run completes. 

            This feature is not available via the Python SDK.
      type: object
      required:
        - processor
        - input
      title: TaskRunInput
      description: Request to run a task.
    ErrorResponse:
      properties:
        type:
          type: string
          const: error
          title: Type
          description: Always 'error'.
        error:
          $ref: '#/components/schemas/Error'
          description: Error.
      type: object
      required:
        - type
        - error
      title: ErrorResponse
      description: Response object used for non-200 status codes.
    SourcePolicy:
      properties:
        include_domains:
          items:
            type: string
          type: array
          title: Include Domains
          description: >-
            List of domains or domain/path prefixes to restrict results to. If
            specified, only matching sources will be included and
            exclude_domains will be ignored. Accepts plain domains (e.g.,
            wikipedia.org), domain/path prefixes (e.g., docs.python.org/3), or
            bare domain extensions (e.g., .gov, .edu, .co.uk). The combined
            number of entries in include_domains and exclude_domains cannot
            exceed 200.
          examples:
            - - wikipedia.org
              - docs.python.org/3
              - .edu
        exclude_domains:
          items:
            type: string
          type: array
          title: Exclude Domains
          description: >-
            List of domains or domain/path prefixes to exclude from results.
            Applied only when include_domains is empty. If specified, matching
            sources will be excluded. Accepts plain domains (e.g., reddit.com),
            domain/path prefixes (e.g., youtube.com/shorts), or bare domain
            extensions (e.g., .gov, .edu, .co.uk). The combined number of
            entries in include_domains and exclude_domains cannot exceed 200.
          examples:
            - - reddit.com
              - youtube.com/shorts
              - .ai
        after_date:
          anyOf:
            - type: string
              format: date
            - type: 'null'
          title: After Date
          description: >-
            Optional start date for filtering search results. Results will be
            limited to content published on or after this date. Provided as an
            RFC 3339 date string (YYYY-MM-DD).
          examples:
            - '2024-01-01'
      type: object
      title: SourcePolicy
      description: >-
        Source policy for web search results.


        Plain domains match that domain and its subdomains. Domain/path entries
        use

        case-sensitive path matching at segment boundaries; trailing slashes are
        ignored,

        dot segments are normalized, and other percent-encoded path spelling is
        preserved.

        Entries omit schemes, ports, query strings, and fragments. When
        include_domains is

        non-empty, it defines the complete allowlist and exclude_domains is
        ignored.
    TaskAdvancedSettings:
      properties:
        location:
          anyOf:
            - type: string
            - type: 'null'
          title: Location
          description: ISO 3166-1 alpha-2 country code for geo-targeted search results.
          examples:
            - us
            - gb
            - de
            - jp
        data_sources:
          anyOf:
            - $ref: '#/components/schemas/TaskDataSources'
            - type: 'null'
          description: >-
            Optional partner data sources to enable for this task run. Supported
            on standard processors only; a selected partner name must not
            collide with an `mcp_servers` entry.
      type: object
      title: TaskAdvancedSettings
      description: Advanced search configuration for a task run.
    TaskSpec:
      properties:
        output_schema:
          anyOf:
            - $ref: '#/components/schemas/JsonSchema'
            - $ref: '#/components/schemas/TextSchema'
            - $ref: '#/components/schemas/AutoSchema'
            - type: string
          title: Output Schema
          description: >-
            JSON schema or text fully describing the desired output from the
            task. Descriptions of output fields will determine the form and
            content of the response. A bare string is equivalent to a text
            schema with the same description.
        input_schema:
          anyOf:
            - type: string
            - $ref: '#/components/schemas/JsonSchema'
            - $ref: '#/components/schemas/TextSchema'
            - type: 'null'
          title: Input Schema
          description: >-
            Optional JSON schema or text description of expected input to the
            task. A bare string is equivalent to a text schema with the same
            description.
      type: object
      required:
        - output_schema
      title: TaskSpec
      description: >-
        Specification for a task.


        Auto output schemas can be specified by setting
        `output_schema={"type":"auto"}`. Not

        specifying a TaskSpec is the same as setting an auto output schema.


        For convenience bare strings are also accepted as input or output
        schemas.
    McpServer:
      properties:
        type:
          type: string
          const: url
          title: Type
          description: Type of MCP server being configured. Always `url`.
          default: url
        url:
          type: string
          title: Url
          description: URL of the MCP server.
        headers:
          anyOf:
            - additionalProperties:
                type: string
                format: password
                writeOnly: true
              type: object
            - type: 'null'
          title: Headers
          description: Headers for the MCP server.
        name:
          type: string
          title: Name
          description: Name of the MCP server.
        allowed_tools:
          anyOf:
            - items:
                type: string
              type: array
            - type: 'null'
          title: Allowed Tools
          description: List of allowed tools for the MCP server.
      type: object
      required:
        - url
        - name
      title: McpServer
      description: MCP server configuration.
    Webhook:
      properties:
        url:
          type: string
          title: Url
          description: URL for the webhook.
        event_types:
          items:
            type: string
            enum:
              - task_run.status
          type: array
          title: Event Types
          description: Event types to send the webhook notifications for.
          default: []
      type: object
      required:
        - url
      title: Webhook
      description: Webhooks for Task Runs.
    Error:
      properties:
        ref_id:
          type: string
          title: Reference ID
          description: Reference ID for the error.
        message:
          type: string
          title: Message
          description: Human-readable message.
        detail:
          anyOf:
            - additionalProperties: true
              type: object
            - type: 'null'
          title: Detail
          description: Optional detail supporting the error.
      type: object
      required:
        - ref_id
        - message
      title: Error
      description: An error message.
    TaskDataSources:
      properties:
        pay_per_use:
          items:
            type: string
          type: array
          title: Pay Per Use
          description: >-
            Pay-per-use data partners to enable for this task, in addition to
            sources included with the processor. See the Data Sources
            documentation for the available names.
        free:
          items:
            type: string
          type: array
          title: Free
          description: >-
            Free data partners to enable for this task, in addition to sources
            included with the processor. Never billed. See the Data Sources
            documentation for the available names.
      additionalProperties: false
      type: object
      title: TaskDataSources
    JsonSchema:
      properties:
        json_schema:
          additionalProperties: true
          type: object
          title: Json Schema
          description: A JSON Schema object. Only a subset of JSON Schema is supported.
          examples:
            - additionalProperties: false
              properties:
                gdp:
                  description: >-
                    GDP in USD for the year, formatted like '$3.1 trillion
                    (2023)'
                  type: string
              required:
                - gdp
              type: object
        type:
          type: string
          const: json
          title: Type
          description: The type of schema being defined. Always `json`.
          default: json
      type: object
      required:
        - json_schema
      title: JsonSchema
      description: JSON schema for a task input or output.
    TextSchema:
      properties:
        description:
          anyOf:
            - type: string
            - type: 'null'
          title: Description
          description: A text description of the desired output from the task.
          examples:
            - GDP in USD for the year, formatted like '$3.1 trillion (2023)'
        type:
          type: string
          const: text
          title: Type
          description: The type of schema being defined. Always `text`.
          default: text
      type: object
      title: TextSchema
      description: Text description for a task input or output.
    AutoSchema:
      properties:
        type:
          type: string
          const: auto
          title: Type
          description: The type of schema being defined. Always `auto`.
          default: auto
      type: object
      title: AutoSchema
      description: Auto schema for a task input or output.
  securitySchemes:
    ApiKeyAuth:
      type: apiKey
      in: header
      name: x-api-key

````