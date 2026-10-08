> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Create Response

> Create a response.

Generates an answer to the given input, grounded in live web research and
annotated with URL citations. Set `model` to `parallel`; `reasoning.effort`
(`low`/`medium`/`high`) controls how much research is performed, trading
response time for answer quality. Returns an OpenAI-format `Response` as
`application/json`, or a `text/event-stream` of OpenAI Responses SSE
events when `stream=true`.



## OpenAPI

````yaml /public-openapi.json post /v1/responses
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
  /v1/responses:
    post:
      tags:
        - Responses API
      summary: Create Response
      description: >-
        Create a response.


        Generates an answer to the given input, grounded in live web research
        and

        annotated with URL citations. Set `model` to `parallel`;
        `reasoning.effort`

        (`low`/`medium`/`high`) controls how much research is performed, trading

        response time for answer quality. Returns an OpenAI-format `Response` as

        `application/json`, or a `text/event-stream` of OpenAI Responses SSE

        events when `stream=true`.
      operationId: create_response_v1_responses_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ResponseCreateRequest'
        required: true
      responses:
        '200':
          description: >-
            Returns a Response object for non-streaming requests
            (application/json), or a stream of OpenAI Responses streaming events
            (text/event-stream) when `stream=true` is set in the request.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Response'
            text/event-stream:
              schema:
                $ref: '#/components/schemas/ResponseStreamEvent'
        '422':
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HTTPValidationError'
components:
  schemas:
    ResponseCreateRequest:
      properties:
        model:
          type: string
          title: Model
          description: >-
            The model to run. `parallel` is the only supported value (matched
            case-insensitively); any other value is rejected. To trade response
            time for answer quality, set `reasoning.effort` (low/medium/high)
            rather than changing the model name.
          examples:
            - parallel
        input:
          anyOf:
            - type: string
            - items:
                $ref: '#/components/schemas/ResponseInputMessage'
              type: array
          title: Input
          description: >-
            The input to generate a response for: a plain string, or a list of
            role/content messages that includes at least one `user` message.
            Must be non-empty, and only text content is supported. `input` and
            `instructions` together may total at most 20,000 characters.
          examples:
            - What are the latest developments in fusion energy?
        instructions:
          anyOf:
            - type: string
            - type: 'null'
          title: Instructions
          description: System instructions for the model.
        previous_response_id:
          anyOf:
            - type: string
            - type: 'null'
          title: Previous Response Id
          description: ID of a previous response to use as conversation context.
        stream:
          anyOf:
            - type: boolean
            - type: 'null'
          title: Stream
          description: Whether to stream the response.
        text:
          anyOf:
            - $ref: '#/components/schemas/ResponseTextConfig'
            - type: 'null'
          description: Configuration for text output, including structured output.
        metadata:
          anyOf:
            - additionalProperties:
                type: string
              type: object
              maxProperties: 16
            - type: 'null'
          title: Metadata
          description: >-
            Arbitrary key-value pairs, echoed back on the Response object.
            Useful for tagging requests. At most 16 keys; keys up to 64
            characters, values up to 512 characters.
        reasoning:
          anyOf:
            - $ref: '#/components/schemas/ResponseReasoningConfig'
            - type: 'null'
          description: >-
            Reasoning configuration. `effort` (low/medium/high) controls how
            much research is performed; defaults to `medium`.
        background:
          anyOf:
            - type: boolean
            - type: 'null'
          title: Background
          description: >-
            Background mode is not supported: requests with `background=true`
            are rejected with a 422 validation error. Use the Task API (POST
            /v1/tasks/runs) for long-running work.
        tools:
          anyOf:
            - items:
                anyOf:
                  - $ref: '#/components/schemas/ResponseWebSearchTool'
                  - $ref: '#/components/schemas/ResponseMcpTool'
                  - additionalProperties: true
                    type: object
              type: array
            - type: 'null'
          title: Tools
          description: >-
            OpenAI tools. Web grounding is always on, so no tool is needed to
            search; a `web_search` tool's `filters` restrict the domains
            searched and cited (`allowed_domains` maps to Parallel's
            `source_policy.include_domains`, `blocked_domains` to
            `exclude_domains`; set one, not both). An `mcp` tool gives the model
            a remote MCP server to call; at most 10 per request. Every other
            tool is accepted and ignored.
        data_sources:
          anyOf:
            - $ref: '#/components/schemas/TaskDataSources'
            - type: 'null'
          description: >-
            Data partners to enable for this request, in addition to web search:
            `pay_per_use` partners are billed per call, `free` partners are not.
            Same names and shape as the Task API's
            `advanced_settings.data_sources`; see the Data Sources
            documentation. Supported when `reasoning.effort` is `medium` (the
            default) or `high`. A partner name must not collide with an `mcp`
            tool's `server_label`.
      type: object
      required:
        - model
        - input
      title: ResponseCreateRequest
      description: >-
        Request body for the Responses API (`POST /v1/responses`).


        OpenAI-Responses-compatible: point a standard OpenAI client at

        `https://api.parallel.ai/v1` with your Parallel API key and set `model`
        to

        `parallel`. The fields below are the ones Parallel acts on. Web
        grounding

        is always on, so no tool is needed to search; a `web_search` tool only

        restricts the searched domains through its `filters`. An `mcp` tool
        gives

        the model a remote MCP server to call, and each call is executed and

        reported as an `mcp_call` output item. Every other tool is accepted and

        ignored. `data_sources` enables data partners in addition to web search.

        Other OpenAI request fields (`tool_choice`, `temperature`, `top_p`,

        `max_output_tokens`, `parallel_tool_calls`, `truncation`, `store`,
        `user`,

        `include`) are accepted for compatibility but have no effect.
    Response:
      additionalProperties: true
      description: |-
        A response from the `parallel` model. A completed response contains one
        `web_search_call` item per web search the model ran, `mcp_list_tools`
        discovery outcomes, and one `mcp_call` item per MCP tool call it made,
        followed by a single assistant message
        whose text is annotated with URL citations grounding the answer.
      properties:
        id:
          title: Id
          type: string
        created_at:
          title: Created At
          type: number
        error:
          anyOf:
            - $ref: '#/components/schemas/ResponseError'
            - type: 'null'
          default: null
        incomplete_details:
          anyOf:
            - $ref: '#/components/schemas/IncompleteDetails'
            - type: 'null'
          default: null
        instructions:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Instructions
        metadata:
          anyOf:
            - additionalProperties:
                type: string
              type: object
            - type: 'null'
          default: null
          title: Metadata
        model:
          const: parallel
          default: parallel
          title: Model
          type: string
        object:
          const: response
          title: Object
          type: string
        output:
          items:
            discriminator:
              mapping:
                mcp_call: '#/components/schemas/ResponseMcpCall'
                mcp_list_tools: '#/components/schemas/ResponseMcpListTools'
                message: '#/components/schemas/ResponseOutputMessage'
                web_search_call: '#/components/schemas/ResponseFunctionWebSearch'
              propertyName: type
            oneOf:
              - $ref: '#/components/schemas/ResponseOutputMessage'
              - $ref: '#/components/schemas/ResponseFunctionWebSearch'
              - $ref: '#/components/schemas/ResponseMcpCall'
              - $ref: '#/components/schemas/ResponseMcpListTools'
          title: Output
          type: array
        parallel_tool_calls:
          title: Parallel Tool Calls
          type: boolean
        temperature:
          anyOf:
            - type: number
            - type: 'null'
          default: null
          title: Temperature
        tool_choice:
          default: auto
          title: Tool Choice
          type: string
        tools:
          default: []
          items:
            anyOf:
              - $ref: '#/components/schemas/ResponseWebSearchTool'
              - $ref: '#/components/schemas/ResponseMcpTool'
          title: Tools
          type: array
        top_p:
          anyOf:
            - type: number
            - type: 'null'
          default: null
          title: Top P
        background:
          anyOf:
            - type: boolean
            - type: 'null'
          default: null
          title: Background
        completed_at:
          anyOf:
            - type: number
            - type: 'null'
          default: null
          title: Completed At
        conversation:
          default: null
          title: Conversation
          type: 'null'
        max_output_tokens:
          anyOf:
            - type: integer
            - type: 'null'
          default: null
          title: Max Output Tokens
        max_tool_calls:
          anyOf:
            - type: integer
            - type: 'null'
          default: null
          title: Max Tool Calls
        moderation:
          anyOf:
            - $ref: '#/components/schemas/Moderation'
            - type: 'null'
          default: null
        previous_response_id:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Previous Response Id
        prompt:
          default: null
          title: Prompt
          type: 'null'
        prompt_cache_key:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Prompt Cache Key
        prompt_cache_retention:
          anyOf:
            - enum:
                - in_memory
                - 24h
              type: string
            - type: 'null'
          default: null
          title: Prompt Cache Retention
        reasoning:
          anyOf:
            - $ref: '#/components/schemas/ResponseReasoningConfig'
            - type: 'null'
          default: null
        safety_identifier:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Safety Identifier
        service_tier:
          anyOf:
            - type: string
              enum:
                - auto
                - default
                - flex
                - scale
                - priority
            - type: 'null'
          default: null
          title: Service Tier
        status:
          anyOf:
            - enum:
                - completed
                - failed
                - in_progress
                - cancelled
                - queued
                - incomplete
              type: string
            - type: 'null'
          default: null
          title: Status
        text:
          anyOf:
            - $ref: '#/components/schemas/ResponseTextConfig'
            - type: 'null'
          default: null
        top_logprobs:
          anyOf:
            - type: integer
            - type: 'null'
          default: null
          title: Top Logprobs
        truncation:
          anyOf:
            - enum:
                - auto
                - disabled
              type: string
            - type: 'null'
          default: null
          title: Truncation
        usage:
          anyOf:
            - $ref: '#/components/schemas/ResponseUsage'
            - type: 'null'
          default: null
        user:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: User
      required:
        - id
        - created_at
        - object
        - output
        - parallel_tool_calls
      title: Response
      type: object
    ResponseStreamEvent:
      anyOf:
        - $ref: '#/components/schemas/ResponseCreatedEvent'
        - $ref: '#/components/schemas/ResponseInProgressEvent'
        - $ref: '#/components/schemas/ResponseOutputItemAddedEvent'
        - $ref: '#/components/schemas/ResponseWebSearchCallInProgressEvent'
        - $ref: '#/components/schemas/ResponseWebSearchCallSearchingEvent'
        - $ref: '#/components/schemas/ResponseWebSearchCallCompletedEvent'
        - $ref: '#/components/schemas/ResponseMcpCallCompletedEvent'
        - $ref: '#/components/schemas/ResponseMcpCallFailedEvent'
        - $ref: '#/components/schemas/ResponseMcpListToolsCompletedEvent'
        - $ref: '#/components/schemas/ResponseMcpListToolsFailedEvent'
        - $ref: '#/components/schemas/ResponseContentPartAddedEvent'
        - $ref: '#/components/schemas/ResponseTextDeltaEvent'
        - $ref: '#/components/schemas/ResponseTextDoneEvent'
        - $ref: '#/components/schemas/ResponseOutputTextAnnotationAddedEvent'
        - $ref: '#/components/schemas/ResponseContentPartDoneEvent'
        - $ref: '#/components/schemas/ResponseOutputItemDoneEvent'
        - $ref: '#/components/schemas/ResponseCompletedEvent'
        - $ref: '#/components/schemas/ResponseFailedEvent'
        - $ref: '#/components/schemas/ResponseIncompleteEvent'
      description: An event in the Responses API SSE stream; `type` identifies the event.
      title: ResponseStreamEvent
    HTTPValidationError:
      properties:
        detail:
          items:
            $ref: '#/components/schemas/ValidationError'
          type: array
          title: Detail
      type: object
      title: HTTPValidationError
    ResponseInputMessage:
      properties:
        role:
          type: string
          enum:
            - user
            - assistant
            - system
            - developer
          title: Role
          description: The role of the message author.
        content:
          anyOf:
            - type: string
            - items:
                $ref: '#/components/schemas/ResponseInputContentPart'
              type: array
          title: Content
          description: >-
            Text content of the message. Either a string or a list of content
            parts (`{text, type}` objects) for OpenAI SDK clients.
      type: object
      required:
        - role
        - content
      title: ResponseInputMessage
      description: |-
        A single input message for the Responses API.

        `content` accepts either a bare string (`"hi"`) or the canonical OpenAI
        list-of-parts (`[{"text": "hi", "type": "input_text"}]`).
    ResponseTextConfig:
      additionalProperties: true
      description: >-
        Text output configuration. By default the response is plain text; for

        structured output set `format` to

        `{"type": "json_schema", "name": ..., "schema": {...}}`. The
        `json_object`

        format is accepted for compatibility but produces plain text.
      properties:
        format:
          anyOf:
            - $ref: '#/components/schemas/ResponseFormatText'
            - $ref: '#/components/schemas/ResponseFormatTextJSONSchemaConfig'
            - $ref: '#/components/schemas/ResponseFormatJSONObject'
            - type: 'null'
          default: null
          title: Format
        verbosity:
          anyOf:
            - enum:
                - low
                - medium
                - high
              type: string
            - type: 'null'
          default: null
          title: Verbosity
      title: ResponseTextConfig
      type: object
    ResponseReasoningConfig:
      description: Reasoning configuration (OpenAI-compatible subset).
      properties:
        effort:
          anyOf:
            - enum:
                - low
                - medium
                - high
              type: string
            - type: 'null'
          default: null
          description: >-
            Controls how much research is performed, trading response time for
            answer quality. Defaults to `medium` when omitted.
          examples:
            - high
          title: Effort
      title: ResponseReasoningConfig
      type: object
    ResponseWebSearchTool:
      additionalProperties: true
      description: >-
        The OpenAI `web_search` tool. Web grounding is always on, so the tool

        itself changes nothing; its `filters` restrict which domains are
        searched.

        `search_context_size` and `user_location` are accepted and ignored.
      properties:
        type:
          enum:
            - web_search
            - web_search_2025_08_26
          title: Type
          type: string
        filters:
          anyOf:
            - $ref: '#/components/schemas/ResponseWebSearchToolFilters'
            - type: 'null'
          default: null
        search_context_size:
          anyOf:
            - enum:
                - low
                - medium
                - high
              type: string
            - type: 'null'
          default: null
          title: Search Context Size
        user_location:
          anyOf:
            - $ref: '#/components/schemas/UserLocation'
            - type: 'null'
          default: null
      required:
        - type
      title: ResponseWebSearchTool
      type: object
    ResponseMcpTool:
      additionalProperties: true
      description: >-
        The OpenAI `mcp` tool: a remote MCP server the `parallel` model may call

        while researching, alongside web search. Each call is reported as an

        `mcp_call` output item. `headers` and `authorization` are sent to the

        server and never echoed back. Parallel runs tool calls without an
        approval

        round trip, so `require_approval` must be set to `never` explicitly.

        `connector_id` and `tunnel_id` are not supported.
      properties:
        server_label:
          description: >-
            Name for this server. Appears as `server_label` on its `mcp_call`
            items and must not collide with a `data_sources` partner name.
          title: Server Label
          type: string
        type:
          const: mcp
          title: Type
          type: string
        allowed_tools:
          anyOf:
            - items:
                type: string
              type: array
            - type: 'null'
          default: null
          description: >-
            Tool names the model may call on this server. Omit to allow every
            tool; an empty list is rejected.
          title: Allowed Tools
        authorization:
          anyOf:
            - type: string
              format: password
              writeOnly: true
            - type: 'null'
          default: null
          description: 'OAuth access token, sent as `Authorization: Bearer <token>`.'
          title: Authorization
        headers:
          anyOf:
            - additionalProperties:
                type: string
                format: password
                writeOnly: true
              type: object
            - type: 'null'
          default: null
          description: >-
            HTTP headers sent on every request to the server, for authentication
            or other purposes.
          title: Headers
        require_approval:
          const: never
          description: 'Must be `never`: tool calls run without an approval step.'
          title: Require Approval
          type: string
        server_description:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Server Description
        server_url:
          description: URL of the MCP server.
          title: Server Url
          type: string
      required:
        - server_label
        - type
        - require_approval
        - server_url
      title: ResponseMcpTool
      type: object
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
    ResponseError:
      additionalProperties: true
      description: Details of a failed response.
      properties:
        code:
          const: server_error
          description: Error code; currently always "server_error".
          title: Code
          type: string
        message:
          title: Message
          type: string
      required:
        - code
        - message
      title: ResponseError
      type: object
    IncompleteDetails:
      additionalProperties: true
      description: Details about why the response is incomplete.
      properties:
        reason:
          anyOf:
            - enum:
                - max_output_tokens
                - content_filter
              type: string
            - type: 'null'
          default: null
          title: Reason
      title: IncompleteDetails
      type: object
    ResponseMcpCall:
      additionalProperties: true
      description: |-
        One tool call the `parallel` model made on an MCP server: a server from
        the request's `mcp` tools, or a partner enabled via `data_sources`.
        `arguments` is the JSON-encoded input; `output` is the text the tool
        returned, or `error` the failure when the call did not succeed.
      properties:
        id:
          title: Id
          type: string
        arguments:
          title: Arguments
          type: string
        name:
          type: string
          title: Name
        server_label:
          title: Server Label
          type: string
        type:
          const: mcp_call
          title: Type
          type: string
        error:
          anyOf:
            - discriminator:
                mapping:
                  http_error: '#/components/schemas/ResponseMcpHttpError'
                  mcp_protocol_error: '#/components/schemas/ResponseMcpProtocolError'
                  mcp_tool_execution_error: '#/components/schemas/ResponseMcpToolExecutionError'
                propertyName: type
              oneOf:
                - $ref: '#/components/schemas/ResponseMcpToolExecutionError'
                - $ref: '#/components/schemas/ResponseMcpProtocolError'
                - $ref: '#/components/schemas/ResponseMcpHttpError'
            - type: 'null'
          default: null
          title: Error
        output:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Output
        status:
          enum:
            - completed
            - failed
          title: Status
          type: string
      required:
        - id
        - arguments
        - name
        - server_label
        - type
        - status
      title: ResponseMcpCall
      type: object
    ResponseMcpListTools:
      additionalProperties: true
      description: >-
        Tools discovered on a customer MCP server, or a connection/listing
        error.

        Requested data partners report failures only, with an empty tools list.
      properties:
        id:
          title: Id
          type: string
        server_label:
          title: Server Label
          type: string
        tools:
          items:
            $ref: '#/components/schemas/McpListToolsTool'
          title: Tools
          type: array
        type:
          const: mcp_list_tools
          title: Type
          type: string
        error:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Error
      required:
        - id
        - server_label
        - tools
        - type
      title: ResponseMcpListTools
      type: object
    ResponseOutputMessage:
      additionalProperties: true
      description: An assistant message produced by the model.
      properties:
        id:
          title: Id
          type: string
        content:
          items:
            $ref: '#/components/schemas/ResponseOutputText'
          title: Content
          type: array
        role:
          const: assistant
          title: Role
          type: string
        status:
          enum:
            - in_progress
            - completed
            - incomplete
          title: Status
          type: string
        type:
          const: message
          title: Type
          type: string
        phase:
          anyOf:
            - enum:
                - commentary
                - final_answer
              type: string
            - type: 'null'
          default: null
          title: Phase
      required:
        - id
        - content
        - role
        - status
        - type
      title: ResponseOutputMessage
      type: object
    ResponseFunctionWebSearch:
      additionalProperties: true
      description: >-
        One web action the `parallel` model took while researching the answer:

        a `search` (`action.queries` are the search queries issued,
        `action.query`

        the first of them) or an `open_page` (`action.url` is the page it read).
      properties:
        id:
          title: Id
          type: string
        action:
          discriminator:
            mapping:
              open_page: '#/components/schemas/ActionOpenPage'
              search: '#/components/schemas/ActionSearch'
            propertyName: type
          oneOf:
            - $ref: '#/components/schemas/ActionSearch'
            - $ref: '#/components/schemas/ActionOpenPage'
          title: Action
        status:
          enum:
            - in_progress
            - searching
            - completed
            - failed
          title: Status
          type: string
        type:
          const: web_search_call
          title: Type
          type: string
      required:
        - id
        - action
        - status
        - type
      title: ResponseFunctionWebSearch
      type: object
    Moderation:
      additionalProperties: true
      description: >-
        Moderation results for the response input and output, if moderated
        completions were requested.
      properties:
        input:
          anyOf:
            - $ref: '#/components/schemas/ModerationInputModerationResult'
            - $ref: '#/components/schemas/ModerationInputError'
          title: Input
        output:
          anyOf:
            - $ref: '#/components/schemas/ModerationOutputModerationResult'
            - $ref: '#/components/schemas/ModerationOutputError'
          title: Output
      required:
        - input
        - output
      title: Moderation
      type: object
    ResponseUsage:
      additionalProperties: true
      description: |-
        Estimated token usage, populated for OpenAI SDK compatibility. Counts
        are approximate; Parallel bills per request, not per token.
      properties:
        input_tokens:
          title: Input Tokens
          type: integer
        input_tokens_details:
          $ref: '#/components/schemas/InputTokensDetails'
        output_tokens:
          title: Output Tokens
          type: integer
        output_tokens_details:
          $ref: '#/components/schemas/OutputTokensDetails'
        total_tokens:
          title: Total Tokens
          type: integer
      required:
        - input_tokens
        - input_tokens_details
        - output_tokens
        - output_tokens_details
        - total_tokens
      title: ResponseUsage
      type: object
    ResponseCreatedEvent:
      additionalProperties: true
      properties:
        response:
          $ref: '#/components/schemas/Response'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.created
          title: Type
          type: string
      required:
        - response
        - sequence_number
        - type
      title: ResponseCreatedEvent
      type: object
    ResponseInProgressEvent:
      additionalProperties: true
      properties:
        response:
          $ref: '#/components/schemas/Response'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.in_progress
          title: Type
          type: string
      required:
        - response
        - sequence_number
        - type
      title: ResponseInProgressEvent
      type: object
    ResponseOutputItemAddedEvent:
      additionalProperties: true
      properties:
        item:
          discriminator:
            mapping:
              mcp_call: '#/components/schemas/ResponseMcpCall'
              mcp_list_tools: '#/components/schemas/ResponseMcpListTools'
              message: '#/components/schemas/ResponseOutputMessage'
              web_search_call: '#/components/schemas/ResponseFunctionWebSearch'
            propertyName: type
          oneOf:
            - $ref: '#/components/schemas/ResponseOutputMessage'
            - $ref: '#/components/schemas/ResponseFunctionWebSearch'
            - $ref: '#/components/schemas/ResponseMcpCall'
            - $ref: '#/components/schemas/ResponseMcpListTools'
          title: Item
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.output_item.added
          title: Type
          type: string
      required:
        - item
        - output_index
        - sequence_number
        - type
      title: ResponseOutputItemAddedEvent
      type: object
    ResponseWebSearchCallInProgressEvent:
      additionalProperties: true
      description: Emitted when a web search call is initiated.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.web_search_call.in_progress
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseWebSearchCallInProgressEvent
      type: object
    ResponseWebSearchCallSearchingEvent:
      additionalProperties: true
      description: Emitted when a web search call is executing.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.web_search_call.searching
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseWebSearchCallSearchingEvent
      type: object
    ResponseWebSearchCallCompletedEvent:
      additionalProperties: true
      description: Emitted when a web search call is completed.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.web_search_call.completed
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseWebSearchCallCompletedEvent
      type: object
    ResponseMcpCallCompletedEvent:
      additionalProperties: true
      description: Emitted when an MCP  tool call has completed successfully.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.mcp_call.completed
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseMcpCallCompletedEvent
      type: object
    ResponseMcpCallFailedEvent:
      additionalProperties: true
      description: Emitted when an MCP  tool call has failed.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.mcp_call.failed
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseMcpCallFailedEvent
      type: object
    ResponseMcpListToolsCompletedEvent:
      additionalProperties: true
      description: >-
        Emitted when the list of available MCP tools has been successfully
        retrieved.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.mcp_list_tools.completed
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseMcpListToolsCompletedEvent
      type: object
    ResponseMcpListToolsFailedEvent:
      additionalProperties: true
      description: Emitted when the attempt to list available MCP tools has failed.
      properties:
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.mcp_list_tools.failed
          title: Type
          type: string
      required:
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseMcpListToolsFailedEvent
      type: object
    ResponseContentPartAddedEvent:
      additionalProperties: true
      properties:
        content_index:
          title: Content Index
          type: integer
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        part:
          $ref: '#/components/schemas/ResponseOutputText'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.content_part.added
          title: Type
          type: string
      required:
        - content_index
        - item_id
        - output_index
        - part
        - sequence_number
        - type
      title: ResponseContentPartAddedEvent
      type: object
    ResponseTextDeltaEvent:
      additionalProperties: true
      description: Emitted when there is an additional text delta.
      properties:
        content_index:
          title: Content Index
          type: integer
        delta:
          title: Delta
          type: string
        item_id:
          title: Item Id
          type: string
        logprobs:
          items:
            $ref: >-
              #/components/schemas/openai__types__responses__response_text_delta_event__Logprob
          title: Logprobs
          type: array
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.output_text.delta
          title: Type
          type: string
      required:
        - content_index
        - delta
        - item_id
        - logprobs
        - output_index
        - sequence_number
        - type
      title: ResponseTextDeltaEvent
      type: object
    ResponseTextDoneEvent:
      additionalProperties: true
      description: Emitted when text content is finalized.
      properties:
        content_index:
          title: Content Index
          type: integer
        item_id:
          title: Item Id
          type: string
        logprobs:
          items:
            $ref: >-
              #/components/schemas/openai__types__responses__response_text_done_event__Logprob
          title: Logprobs
          type: array
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        text:
          title: Text
          type: string
        type:
          const: response.output_text.done
          title: Type
          type: string
      required:
        - content_index
        - item_id
        - logprobs
        - output_index
        - sequence_number
        - text
        - type
      title: ResponseTextDoneEvent
      type: object
    ResponseOutputTextAnnotationAddedEvent:
      additionalProperties: true
      properties:
        annotation:
          $ref: '#/components/schemas/AnnotationURLCitation'
        annotation_index:
          title: Annotation Index
          type: integer
        content_index:
          title: Content Index
          type: integer
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.output_text.annotation.added
          title: Type
          type: string
      required:
        - annotation
        - annotation_index
        - content_index
        - item_id
        - output_index
        - sequence_number
        - type
      title: ResponseOutputTextAnnotationAddedEvent
      type: object
    ResponseContentPartDoneEvent:
      additionalProperties: true
      properties:
        content_index:
          title: Content Index
          type: integer
        item_id:
          title: Item Id
          type: string
        output_index:
          title: Output Index
          type: integer
        part:
          $ref: '#/components/schemas/ResponseOutputText'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.content_part.done
          title: Type
          type: string
      required:
        - content_index
        - item_id
        - output_index
        - part
        - sequence_number
        - type
      title: ResponseContentPartDoneEvent
      type: object
    ResponseOutputItemDoneEvent:
      additionalProperties: true
      properties:
        item:
          discriminator:
            mapping:
              mcp_call: '#/components/schemas/ResponseMcpCall'
              mcp_list_tools: '#/components/schemas/ResponseMcpListTools'
              message: '#/components/schemas/ResponseOutputMessage'
              web_search_call: '#/components/schemas/ResponseFunctionWebSearch'
            propertyName: type
          oneOf:
            - $ref: '#/components/schemas/ResponseOutputMessage'
            - $ref: '#/components/schemas/ResponseFunctionWebSearch'
            - $ref: '#/components/schemas/ResponseMcpCall'
            - $ref: '#/components/schemas/ResponseMcpListTools'
          title: Item
        output_index:
          title: Output Index
          type: integer
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.output_item.done
          title: Type
          type: string
      required:
        - item
        - output_index
        - sequence_number
        - type
      title: ResponseOutputItemDoneEvent
      type: object
    ResponseCompletedEvent:
      additionalProperties: true
      properties:
        response:
          $ref: '#/components/schemas/Response'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.completed
          title: Type
          type: string
      required:
        - response
        - sequence_number
        - type
      title: ResponseCompletedEvent
      type: object
    ResponseFailedEvent:
      additionalProperties: true
      properties:
        response:
          $ref: '#/components/schemas/Response'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.failed
          title: Type
          type: string
      required:
        - response
        - sequence_number
        - type
      title: ResponseFailedEvent
      type: object
    ResponseIncompleteEvent:
      additionalProperties: true
      properties:
        response:
          $ref: '#/components/schemas/Response'
        sequence_number:
          title: Sequence Number
          type: integer
        type:
          const: response.incomplete
          title: Type
          type: string
      required:
        - response
        - sequence_number
        - type
      title: ResponseIncompleteEvent
      type: object
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
    ResponseInputContentPart:
      properties:
        type:
          anyOf:
            - type: string
            - type: 'null'
          title: Type
          description: >-
            The content part type. Supported: `input_text`, `output_text`, and
            `text`. Multimodal types (`input_image`, `input_audio`,
            `input_file`) are rejected with a 422 error.
        text:
          anyOf:
            - type: string
            - type: 'null'
          title: Text
          description: The text payload, when the part carries one.
      type: object
      title: ResponseInputContentPart
      description: |-
        A single content part of a message, e.g.
        `{"text": "hi", "type": "input_text"}`. Only text parts are supported;
        requests containing image, audio, or file parts fail with a 422 error.
    ResponseFormatText:
      additionalProperties: true
      description: Default response format. Used to generate text responses.
      properties:
        type:
          const: text
          title: Type
          type: string
      required:
        - type
      title: ResponseFormatText
      type: object
    ResponseFormatTextJSONSchemaConfig:
      additionalProperties: true
      description: >-
        JSON Schema output format: the response's output text conforms to
        `schema`.
      properties:
        name:
          type: string
          title: Name
        schema:
          additionalProperties: true
          title: Schema
          type: object
        type:
          type: string
          const: json_schema
          title: Type
        description:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Description
        strict:
          anyOf:
            - type: boolean
            - type: 'null'
          default: null
          title: Strict
      required:
        - name
        - schema
        - type
      title: ResponseFormatTextJSONSchemaConfig
      type: object
    ResponseFormatJSONObject:
      additionalProperties: true
      description: >-
        JSON object response format.


        An older method of generating JSON responses.

        Using `json_schema` is recommended for models that support it. Note that
        the

        model will not generate JSON without a system or user message
        instructing it

        to do so.
      properties:
        type:
          const: json_object
          title: Type
          type: string
      required:
        - type
      title: ResponseFormatJSONObject
      type: object
    ResponseWebSearchToolFilters:
      additionalProperties: false
      description: >-
        Domain filters on the OpenAI `web_search` tool, mapped onto Parallel's

        source policy: `allowed_domains` -> `include_domains`, `blocked_domains`
        ->

        `exclude_domains`. Entries follow the Source Policy rules (plain domains

        match their subdomains; no schemes, ports, query strings or fragments;
        at

        most 200 entries in total). Setting both lists is accepted, but only the

        allow list applies today.
      properties:
        allowed_domains:
          anyOf:
            - items:
                type: string
              type: array
            - type: 'null'
          default: null
          description: >-
            Only sources on these domains (and their subdomains) are used. Maps
            to `source_policy.include_domains`.
          title: Allowed Domains
        blocked_domains:
          anyOf:
            - items:
                type: string
              type: array
            - type: 'null'
          default: null
          description: >-
            Sources on these domains (and their subdomains) are excluded. Maps
            to `source_policy.exclude_domains`. When `allowed_domains` is also
            set, the allow list is what applies.
          title: Blocked Domains
      title: ResponseWebSearchToolFilters
      type: object
    UserLocation:
      additionalProperties: true
      description: The approximate location of the user.
      properties:
        city:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: City
        country:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Country
        region:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Region
        timezone:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Timezone
        type:
          anyOf:
            - const: approximate
              type: string
            - type: 'null'
          default: null
          title: Type
      title: UserLocation
      type: object
    ResponseMcpHttpError:
      properties:
        type:
          const: http_error
          default: http_error
          title: Type
          type: string
        code:
          title: Code
          type: integer
        message:
          title: Message
          type: string
      required:
        - code
        - message
      title: ResponseMcpHttpError
      type: object
    ResponseMcpProtocolError:
      properties:
        type:
          const: mcp_protocol_error
          default: mcp_protocol_error
          title: Type
          type: string
        code:
          title: Code
          type: integer
        message:
          title: Message
          type: string
      required:
        - code
        - message
      title: ResponseMcpProtocolError
      type: object
    ResponseMcpToolExecutionError:
      properties:
        type:
          const: mcp_tool_execution_error
          default: mcp_tool_execution_error
          title: Type
          type: string
        content:
          title: Content
      required:
        - content
      title: ResponseMcpToolExecutionError
      type: object
    McpListToolsTool:
      additionalProperties: true
      description: A tool available on an MCP server.
      properties:
        input_schema:
          title: Input Schema
        name:
          type: string
          title: Name
        annotations:
          anyOf:
            - {}
            - type: 'null'
          default: null
          title: Annotations
        description:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Description
      required:
        - input_schema
        - name
      title: McpListToolsTool
      type: object
    ResponseOutputText:
      additionalProperties: true
      description: |-
        A text content part of an output message. `annotations` carries the URL
        citations grounding the answer.
      properties:
        annotations:
          items:
            $ref: '#/components/schemas/AnnotationURLCitation'
          title: Annotations
          type: array
        text:
          title: Text
          type: string
        type:
          const: output_text
          title: Type
          type: string
        logprobs:
          anyOf:
            - items:
                $ref: >-
                  #/components/schemas/openai__types__responses__response_output_text__Logprob
              type: array
            - type: 'null'
          default: null
          title: Logprobs
      required:
        - annotations
        - text
        - type
      title: ResponseOutputText
      type: object
    ActionOpenPage:
      additionalProperties: true
      description: Action type "open_page" - Opens a specific URL from search results.
      properties:
        type:
          const: open_page
          title: Type
          type: string
        url:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Url
      required:
        - type
      title: ActionOpenPage
      type: object
    ActionSearch:
      additionalProperties: true
      description: Action type "search" - Performs a web search query.
      properties:
        type:
          const: search
          title: Type
          type: string
        queries:
          anyOf:
            - items:
                type: string
              type: array
            - type: 'null'
          default: null
          title: Queries
        query:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Query
        sources:
          anyOf:
            - items:
                $ref: '#/components/schemas/ActionSearchSource'
              type: array
            - type: 'null'
          default: null
          title: Sources
      required:
        - type
      title: ActionSearch
      type: object
    ModerationInputModerationResult:
      additionalProperties: true
      description: A moderation result produced for the response input or output.
      properties:
        categories:
          additionalProperties:
            type: boolean
          title: Categories
          type: object
        category_applied_input_types:
          additionalProperties:
            items:
              enum:
                - text
                - image
              type: string
            type: array
          title: Category Applied Input Types
          type: object
        category_scores:
          additionalProperties:
            type: number
          title: Category Scores
          type: object
        flagged:
          title: Flagged
          type: boolean
        model:
          type: string
          title: Model
        type:
          const: moderation_result
          title: Type
          type: string
      required:
        - categories
        - category_applied_input_types
        - category_scores
        - flagged
        - model
        - type
      title: ModerationInputModerationResult
      type: object
    ModerationInputError:
      additionalProperties: true
      description: >-
        An error produced while attempting moderation for the response input or
        output.
      properties:
        code:
          type: string
          title: Code
        message:
          title: Message
          type: string
        type:
          type: string
          const: error
          title: Type
      required:
        - code
        - message
        - type
      title: ModerationInputError
      type: object
    ModerationOutputModerationResult:
      additionalProperties: true
      description: A moderation result produced for the response input or output.
      properties:
        categories:
          additionalProperties:
            type: boolean
          title: Categories
          type: object
        category_applied_input_types:
          additionalProperties:
            items:
              enum:
                - text
                - image
              type: string
            type: array
          title: Category Applied Input Types
          type: object
        category_scores:
          additionalProperties:
            type: number
          title: Category Scores
          type: object
        flagged:
          title: Flagged
          type: boolean
        model:
          type: string
          title: Model
        type:
          const: moderation_result
          title: Type
          type: string
      required:
        - categories
        - category_applied_input_types
        - category_scores
        - flagged
        - model
        - type
      title: ModerationOutputModerationResult
      type: object
    ModerationOutputError:
      additionalProperties: true
      description: >-
        An error produced while attempting moderation for the response input or
        output.
      properties:
        code:
          type: string
          title: Code
        message:
          title: Message
          type: string
        type:
          type: string
          const: error
          title: Type
      required:
        - code
        - message
        - type
      title: ModerationOutputError
      type: object
    InputTokensDetails:
      additionalProperties: true
      description: A detailed breakdown of the input tokens.
      properties:
        cached_tokens:
          title: Cached Tokens
          type: integer
      required:
        - cached_tokens
      title: InputTokensDetails
      type: object
    OutputTokensDetails:
      additionalProperties: true
      description: A detailed breakdown of the output tokens.
      properties:
        reasoning_tokens:
          title: Reasoning Tokens
          type: integer
      required:
        - reasoning_tokens
      title: OutputTokensDetails
      type: object
    openai__types__responses__response_text_delta_event__Logprob:
      additionalProperties: true
      description: >-
        A logprob is the logarithmic probability that the model assigns to
        producing

        a particular token at a given position in the sequence. Less-negative
        (higher)

        logprob values indicate greater model confidence in that token choice.
      properties:
        token:
          title: Token
          type: string
        logprob:
          title: Logprob
          type: number
        top_logprobs:
          anyOf:
            - items:
                $ref: >-
                  #/components/schemas/openai__types__responses__response_text_delta_event__LogprobTopLogprob
              type: array
            - type: 'null'
          default: null
          title: Top Logprobs
      required:
        - token
        - logprob
      title: Logprob
      type: object
    openai__types__responses__response_text_done_event__Logprob:
      additionalProperties: true
      description: >-
        A logprob is the logarithmic probability that the model assigns to
        producing

        a particular token at a given position in the sequence. Less-negative
        (higher)

        logprob values indicate greater model confidence in that token choice.
      properties:
        token:
          title: Token
          type: string
        logprob:
          title: Logprob
          type: number
        top_logprobs:
          anyOf:
            - items:
                $ref: >-
                  #/components/schemas/openai__types__responses__response_text_done_event__LogprobTopLogprob
              type: array
            - type: 'null'
          default: null
          title: Top Logprobs
      required:
        - token
        - logprob
      title: Logprob
      type: object
    AnnotationURLCitation:
      additionalProperties: true
      description: A citation for a web resource used to generate a model response.
      properties:
        end_index:
          title: End Index
          type: integer
        start_index:
          title: Start Index
          type: integer
        title:
          title: Title
          type: string
        type:
          const: url_citation
          title: Type
          type: string
        url:
          title: Url
          type: string
      required:
        - end_index
        - start_index
        - title
        - type
        - url
      title: AnnotationURLCitation
      type: object
    openai__types__responses__response_output_text__Logprob:
      additionalProperties: true
      description: The log probability of a token.
      properties:
        token:
          title: Token
          type: string
        bytes:
          items:
            type: integer
          title: Bytes
          type: array
        logprob:
          title: Logprob
          type: number
        top_logprobs:
          items:
            $ref: >-
              #/components/schemas/openai__types__responses__response_output_text__LogprobTopLogprob
          title: Top Logprobs
          type: array
      required:
        - token
        - bytes
        - logprob
        - top_logprobs
      title: Logprob
      type: object
    ActionSearchSource:
      additionalProperties: true
      description: A source used in the search.
      properties:
        type:
          const: url
          title: Type
          type: string
        url:
          title: Url
          type: string
      required:
        - type
        - url
      title: ActionSearchSource
      type: object
    openai__types__responses__response_text_delta_event__LogprobTopLogprob:
      additionalProperties: true
      properties:
        token:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Token
        logprob:
          anyOf:
            - type: number
            - type: 'null'
          default: null
          title: Logprob
      title: LogprobTopLogprob
      type: object
    openai__types__responses__response_text_done_event__LogprobTopLogprob:
      additionalProperties: true
      properties:
        token:
          anyOf:
            - type: string
            - type: 'null'
          default: null
          title: Token
        logprob:
          anyOf:
            - type: number
            - type: 'null'
          default: null
          title: Logprob
      title: LogprobTopLogprob
      type: object
    openai__types__responses__response_output_text__LogprobTopLogprob:
      additionalProperties: true
      description: The top log probability of a token.
      properties:
        token:
          title: Token
          type: string
        bytes:
          items:
            type: integer
          title: Bytes
          type: array
        logprob:
          title: Logprob
          type: number
      required:
        - token
        - bytes
        - logprob
      title: LogprobTopLogprob
      type: object
  securitySchemes:
    ApiKeyAuth:
      type: apiKey
      in: header
      name: x-api-key

````