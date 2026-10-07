> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Image search

> Searches the web for images.



## OpenAPI

````yaml /public-openapi.json post /v1/images/search
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
  /v1/images/search:
    post:
      tags:
        - Image Search
      summary: Image search
      description: Searches the web for images.
      operationId: image_search_v1_images_search_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ImageSearchRequest'
        required: true
      responses:
        '200':
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ImageSearchResponse'
              example:
                search_id: search_fcb2b4f3c75e418687bccaa1a8381331
                session_id: session_fcb2b4f3c75e418687bccaa1a8381331
                results:
                  - title: Golden Gate Bridge at sunset
                    image_url: https://www.example.com/images/golden-gate.jpg
                    source_page_url: https://www.example.com/golden-gate-bridge
                    width: 1920
                    height: 1080
        '422':
          description: Request validation error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
              example:
                type: error
                error:
                  ref_id: search_fcb2b4f3c75e418687bccaa1a8381331
                  message: Request validation error
components:
  schemas:
    ImageSearchRequest:
      properties:
        objective:
          anyOf:
            - type: string
            - type: 'null'
          title: Objective
          description: >-
            Natural-language description of the images wanted. Used together
            with search_queries to focus results on the most relevant images.
        search_queries:
          items:
            type: string
            maxLength: 200
          type: array
          maxItems: 5
          title: Search Queries
          description: >-
            Concise keyword image search queries. At least one query is
            required, and at most 5, each at most 200 characters.
        mode:
          anyOf:
            - type: string
              enum:
                - fast
                - advanced
            - type: 'null'
          title: Mode
          description: >-
            `fast`: lowest latency and cost. `advanced` (default): slower, with
            higher-quality results.
        session_id:
          anyOf:
            - type: string
              maxLength: 1000
            - type: 'null'
          title: Session Id
          description: >-
            Session identifier that groups related calls made as part of a
            larger task. Echoed back in the response; generated by the server if
            omitted.
        client_model:
          anyOf:
            - type: string
            - type: 'null'
          title: Client Model
          description: The model generating this request and consuming the results.
          examples:
            - claude-opus-4-7
            - gpt-5.4
            - gemini-3.1-pro
        advanced_settings:
          anyOf:
            - $ref: '#/components/schemas/AdvancedImageSearchSettings'
            - type: 'null'
          description: Advanced configuration for location and result count.
      additionalProperties: false
      type: object
      required:
        - search_queries
      title: ImageSearchRequest
      description: Image search request.
    ImageSearchResponse:
      properties:
        search_id:
          type: string
          title: Search Id
          description: 'Search ID. Example: `search_cad0a6d2dec046bd95ae900527d880e7`'
        session_id:
          type: string
          title: Session Id
          description: >-
            Session identifier, echoed back from the request if provided,
            otherwise generated by the server. Pass it to later calls made as
            part of the same task.
          examples:
            - session_8a911eb27c7a4afaa20d0d9dc98d07c0
        results:
          items:
            $ref: '#/components/schemas/ImageSearchResult'
          type: array
          title: Results
          description: A list of image results, ordered by decreasing relevance.
        warnings:
          anyOf:
            - items:
                $ref: '#/components/schemas/Warning'
              type: array
            - type: 'null'
          title: Warnings
          description: Warnings for the search request, if any.
        usage:
          anyOf:
            - items:
                $ref: '#/components/schemas/UsageItem'
              type: array
            - type: 'null'
          title: Usage
          description: Usage metrics for the search request.
      type: object
      required:
        - search_id
        - session_id
        - results
      title: ImageSearchResponse
      description: Image search response.
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
    AdvancedImageSearchSettings:
      properties:
        location:
          anyOf:
            - type: string
            - type: 'null'
          title: Location
          description: ISO 3166-1 alpha-2 country code for geo-targeted image results.
          examples:
            - us
            - gb
            - de
            - jp
        max_results:
          anyOf:
            - type: integer
              minimum: 1
            - type: 'null'
          title: Max Results
          description: >-
            Upper bound on the number of results to return. Defaults to 10 if
            not provided. Values above the mode's maximum are reduced to it,
            with a warning.
      additionalProperties: false
      type: object
      title: AdvancedImageSearchSettings
      description: Advanced image search configuration.
    ImageSearchResult:
      properties:
        title:
          type: string
          title: Title
          description: Image title, alt text, or caption.
        image_url:
          type: string
          title: Image Url
          description: Direct URL of the full-size image.
        source_page_url:
          type: string
          title: Source Page Url
          description: URL of the page the image appears on, for attribution and context.
        width:
          anyOf:
            - type: integer
            - type: 'null'
          title: Width
          description: Image width in pixels, if known.
        height:
          anyOf:
            - type: integer
            - type: 'null'
          title: Height
          description: Image height in pixels, if known.
      type: object
      required:
        - title
        - image_url
        - source_page_url
      title: ImageSearchResult
      description: A single image search result.
    Warning:
      properties:
        type:
          type: string
          enum:
            - spec_validation_warning
            - input_validation_warning
            - warning
          title: Type
          description: >-
            Type of warning. Note that adding new warning types is considered a
            backward-compatible change.
          examples:
            - spec_validation_warning
            - input_validation_warning
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
          description: Optional detail supporting the warning.
      type: object
      required:
        - type
        - message
      title: Warning
      description: Human-readable message for a task.
    UsageItem:
      properties:
        name:
          type: string
          title: Name
          description: Name of the SKU.
          examples:
            - sku_search
            - sku_extract_excerpts
        count:
          type: integer
          title: Count
          description: Count of the SKU.
          examples:
            - 1
      type: object
      required:
        - name
        - count
      title: UsageItem
      description: Usage item for a single operation.
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
  securitySchemes:
    ApiKeyAuth:
      type: apiKey
      in: header
      name: x-api-key

````