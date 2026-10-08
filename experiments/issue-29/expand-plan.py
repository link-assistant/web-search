"""Reproduce the additional drafts identified by the review coverage audit."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DRAFTS = ROOT / 'docs/case-studies/issue-29/issues'
CASE = 'https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md'

# id, slug, title, dependencies, scope, tests, design/component alternatives, sources
ITEMS = [
('WS-19','sse-events','Replayable SSE for task runs, groups, FindAll, and Responses','WS-13',
 'Persist ordered resource events with stable IDs/types/payloads and expose the documented product-specific event adapters;Bound subscriber buffers, send heartbeats, release subscriptions on disconnect, and handle completed/failed/cancelled streams;Support Last-Event-ID where the product contract permits it and report retention gaps explicitly;Keep Monitor events as paginated JSON history plus webhooks: its /events route is not documented as SSE',
 'Reconnect after event 2 preserves event order without missed terminal updates;Slow/disconnected subscribers cannot grow buffers or keep heartbeat timers alive;JS/Rust fixtures match framing and payloads for success/failure/cancellation',
 'Compare ephemeral pub/sub with a durable event log and select the latter. Reuse native JS streams or better-sse and axum::response::sse with WS-13 storage.',
 'task-api/task-sse task-api/group-api findall-api/features/findall-sse responses-api/features/streaming-events monitor-api/monitor-events'),
('WS-20','webhook-delivery','Durable Standard Webhooks delivery, retries, replay, and secret rotation','WS-13',
 'Implement documented webhook-id/timestamp/signature headers and HMAC-SHA256 of exact raw bytes using versioned secrets;Persist delivery attempts in an outbox with finite retry horizon, jitter, rate-limit handling, operator replay, and dead-letter diagnostics;Provide per-product event subscriptions, stable delivery IDs, and documented at-least-once semantics;Validate outbound destinations through configured transport policy and redact credentials, including secret-rotation overlap',
 'A reference verifier accepts emitted requests and rejects altered bytes/wrong secret/expired timestamps;A local receiver returning 500 then 429 then 200 receives one stable event ID;Worker restart, dead-letter replay, and rotation preserve pending delivery without real notifications',
 'Use official standardwebhooks libraries instead of inventing signing. Compare a hosted queue with a storage-backed outbox and select the outbox for self-hosting.',
 'resources/webhook-setup task-api/webhooks findall-api/features/findall-webhook monitor-api/monitor-webhooks'),
('WS-21','task-groups','Task groups: bulk submission, aggregate status, pagination, and partial failures','WS-13 WS-14 WS-19',
 'Implement create/retrieve group, add/list runs, retrieve group run, and group events from current OpenAPI;Allow more runs while a group is active and preserve per-run text/JSON inputs and specification overrides;Define atomic admission or explicit per-item rejection, stable counters/cursors, and deterministic input association;Bound per-group/organization concurrency and retain successful results when another run fails',
 'Add a second batch during execution and verify totals after mixed success/failure;Pagination returns each run once and a foreign group cannot access it;Restart and finite large batches respect limits and retain structured inputs exactly',
 'Compose WS-13 runs instead of building a second batch engine. Use storage transactions and indexed group membership with shared queue limits.',
 'task-api/group-api api-reference/tasks/add-runs-to-task-group api-reference/tasks/fetch-task-group-runs'),
('WS-22','responses-api','Responses API statefulness, citations, structured output, and stream compatibility','WS-14 WS-19 WS-24',
 'Mount POST /v1/responses with documented input forms, Bearer auth, model alias, reasoning.effort, previous_response_id, text.format, web_search tools, and stream;Map research traces to message/web_search_call items and url_citation annotations with correct Unicode offsets;Persist caller-scoped ancestry and handle missing/deleted context explicitly;Validate JSON-schema output and documented unsupported fields, with extension hooks for WS-25 MCP and WS-26 data_sources',
 'Current request fixtures work against a local compatible client in streaming and nonstreaming modes;Unicode citation offsets reference evidence actually used and schema errors are typed;Follow-ups reuse context only within caller scope and missing ancestry returns an explicit error',
 'Select a format adapter over WS-14/24 rather than another agent loop. Use ajv/jsonschema and WS-19 event adapters.',
 'responses-api/responses-quickstart responses-api/openai-compatibility responses-api/features/statefulness responses-api/features/citations responses-api/features/structured-outputs'),
('WS-23','chat-api','Chat Completions compatibility over grounded research','WS-14 WS-19',
 'Mount POST /v1beta/chat/completions with documented messages/models/stream/configuration;Map message history to grounded research and results to choices/usage/finish reasons;Emit ordered Chat-specific SSE chunks and terminal sentinel independently of Responses event names;Publish supported/unsupported field matrix and configurable base URL examples',
 'A compatible client obtains text and streaming chunks using local retrieval and a stub LLM;Empty messages, unsupported options, cancellation, and backend failure have explicit outcomes;Multi-turn history stays scoped and receipts contain no credentials',
 'Select a thin adapter over the shared model/research interface, reusing SSE framing while retaining Chat-specific payloads.',
 'api-reference/chat-api-beta/chat-completions getting-started/choose-an-api'),
('WS-24','task-spec-interactions','Task schemas, processor budgets, Ingest, and interaction chains','WS-14',
 'Support task_spec input_schema/output_schema with text/JSON validation and field-level research basis;Resolve lite/base/core/core2x/pro/ultra/ultra2x/ultra4x/ultra8x into configurable finite research budgets and report measured capabilities;Implement documented POST /v1beta/findall/ingest objective-to-spec inference and GET run/schema retrieval rather than inventing POST /findall/schema;Return interaction_id and accept previous_interaction_id for cross-processor follow-ups with caller isolation and retention controls',
 'A stub model infers schemas and produces valid cited fields while invalid outputs fail explicitly;Every processor has finite-budget and exhaustion tests;Interaction chains use interaction IDs rather than previous_run_id, reject cross-caller ancestry, and honor retention-disabled mode',
 'Support caller schemas plus injectable model inference through ajv/jsonschema. Reuse WS-14 backend and WS-13 ancestry storage.',
 'task-api/guides/specify-a-task task-api/guides/choose-a-processor task-api/guides/interactions task-api/ingest-api'),
('WS-25','remote-mcp','Remote MCP tools and authenticated private sources in research runs','WS-14',
 'Accept named Task mcp_servers and Responses-style mcp tool configuration with supported authentication;Discover/validate schemas, restrict allowed tools, bound calls/result sizes, and close connections on cancellation;Record mcp_tool_calls and attributable basis with typed failures and redacted credentials;Provide a Browser Use/private-web capture adapter owned by WC-09 without blocking independent public-web research',
 'A local authenticated MCP server exposes tools and a run calls only permitted ones;Name collisions, malformed schema, unavailable server, oversized output, and cancellation are tested;No credential appears in logs/results/history and separate callers cannot share private source state',
 'Use official @modelcontextprotocol/sdk and rmcp clients rather than manual JSON-RPC. Prefer operator-managed secret references for hosted instances.',
 'task-api/mcp-tool-call responses-api/features/mcp-tools integrations/browseruse resources/data-connectors'),
('WS-26','data-connectors','Research connector registry for free, paid, licensed, and BYOL data','WS-25 WS-17',
 'Extend descriptors with access model, schemas, credential/license requirements, processor support, citations, and call accounting;Add reference adapters for pubmed, clinical_trials, chembl, biorxiv/medrxiv, npi_registry, and cms_coverage using injectable transports;Validate Task advanced_settings.data_sources versus Responses top-level data_sources, free/paid list membership, unsupported processors, and MCP name collisions;Support operator-installed paid/licensed sources including a Carbon Arc adapter contract, treating proprietary access as a license prerequisite',
 'All six free connectors have mocked response/citation/error contracts;No selected connector is invoked unnecessarily and failures remain attributable;Unavailable paid connectors fail without charges, and a fake paid adapter counts successful calls only',
 'Select a plugin registry instead of hard-coded proprietary datasets. Reuse existing REST transports and WS-25 MCP for BYOL.',
 'resources/data-connectors task-api/data-connectors responses-api/features/data-connectors'),
('WS-27','findall-enrichment','FindAll per-candidate enrichment with schemas, citations, and partial progress','WS-16 WS-19 WS-24 WS-25',
 'Implement POST run/enrich with saved request specifications, schema validation, processor configuration, and optional MCP sources;Keep matching and enrichment basis separate and preserve match conditions/status;Expose per-field progress/failure through results and events, retaining successful fields;Persist exact enrichment payloads for deliberate reuse on refresh rather than assuming GET /schema includes them',
 'Ten fixture candidates with one failing source retain successful cited fields;Enrichment never changes matched/unmatched counts and repeat requests have defined merge semantics;Restart resumes pending fields and schema retrieval remains separate from saved request payloads',
 'Reuse Task execution and durable candidate work items rather than a second enrichment engine.',
 'findall-api/features/findall-enrich findall-api/features/findall-refresh api-reference/findall/add-enrichment-to-findall-run'),
('WS-28','findall-lifecycle','FindAll preview, exclusions, extend, cancel, and refresh lifecycle','WS-16 WS-19 WS-27',
 'Implement preview/base/core/pro budgets, terminal reasons, and applicable action_required/cancelling states;Extend without repeating completed matches and cancel generation/enrichment while retaining partial results;Refresh creates a new run from the original query with stable deduplicated exclude_list history;Retain every historical matched entity across reordered pages and explicitly reapply saved enrichment payloads',
 'Preview/extend retains candidate IDs and adds only new matches;Two refresh cycles cannot reintroduce old matched entities or silently exclude unmatched ones;Cancellation races, restart, and budget exhaustion preserve partial results and correct status events',
 'Select documented new-run refresh over mutation of a completed run. Use canonical entity IDs and storage transactions.',
 'findall-api/features/findall-preview findall-api/features/findall-extend findall-api/features/findall-cancel findall-api/features/findall-refresh findall-api/core-concepts/findall-lifecycle'),
('WS-29','monitor-snapshot','Snapshot monitors and follow-up Task enrichment for detected events','WS-15 WS-24 WS-20',
 'Create snapshot monitors from task_run_id with retained specification/processor/source policy;Compare normalized structured outputs and emit partial changed_output plus full previous_output with basis;Represent event_stream/snapshot/completion/error as mutually exclusive execution outcomes with event_group_id and paginated history;Provide event-to-Task helper with bounded fan-out, provenance, and deduplication',
 'Fake-clock first/no-change/changed/failure runs produce correct outcomes and one-field diffs;Duplicate event/webhook starts one follow-up Task without losing original events;Restart/update/cancel avoid overlapping runs and include_completions pagination works',
 'Use structured semantic diffs instead of raw text hashes for snapshots. Compose the Task engine, existing scheduler, and WS-20 delivery.',
 'monitor-api/quickstart-snapshot monitor-api/monitor-task monitor-api/monitor-events'),
('WS-30','task-mcp','Task MCP tools for research, enrichment, and async retrieval','WS-11 WS-21 WS-24',
 'Publish research/create/status/result and batch-enrichment tool schemas from documented Task MCP inventory;Reuse WS-11 stdio/HTTP hosting with caller auth forwarded to tasks/groups;Return run IDs, polling guidance, structured output, and field-level basis;Publish Task skills and agent configuration with finite polling and sanitized errors',
 'An MCP SDK client creates a fixture Task and retrieves cited results without one long-held call;Tool argument/error schemas validate and caller isolation is tested;Group enrichment returns partial failures without discarding good rows',
 'Reuse inbound MCP hosting and group execution. Prefer async run-ID tools over blocking long research calls.',
 'integrations/mcp/task-mcp task-api/task-mcp integrations/mcp/programmatic-use'),
('WS-31','http-client-sdks','Typed HTTP clients for Python, TypeScript, and Rust with polling and streams','WS-10 WS-18 WS-19 WS-21 WS-22 WS-23 WS-28 WS-29',
 'Generate or maintain typed service clients for all product routes with a configurable base URL and auth;Provide cancellation, finite polling, pagination, Retry-After retries, SSE, injectable transport, and typed errors;Keep existing native JS/Rust search libraries independent and add installed Python/client-package examples;Define schema/client drift checks and client release/version/runtime support',
 'Installed clients run local search/extract/task/stream quickstarts;429, partial failure, pagination, malformed response, and cancellation have fixture tests;Generated drift fails CI and no client requires a repository checkout',
 'Evaluate OpenAPI Generator/openapi-typescript and select generation where streams are supported, with tested thin polling helpers. Native libraries are not replaced by HTTP SDKs.',
 'getting-started/overview integrations/developer-quickstart integrations/mcp/programmatic-use'),
('WS-32','oauth-device-flow','Device OAuth login, refresh, and revoke for CLI and agent clients','WS-10',
 'Support RFC 8628 client registration, device/user codes, verification URLs, polling, refresh, and revoke with configurable identity provider;Handle authorization_pending/slow_down/access_denied/expired_token and finite cancellation-aware polling;Add CLI login/logout with secure storage and Bearer access distinct from research API keys;Keep identity integration optional for static-key self-hosted instances',
 'A fake auth server covers every polling outcome, rotation, expiry, logout, and cancellation;Installed CLI sign-in stores tokens securely without diagnostics leaks;Static-key service works without an OAuth dependency',
 'Prefer established providers such as Keycloak plus openid-client/oauth2 adapters instead of building an identity provider. Use RFC 8628 fixtures.',
 'integrations/account-api integrations/oauth-provider integrations/cli'),
('WS-33','apps-keys-organizations','Account apps, API key lifecycle, organization roles, and resource isolation','WS-32',
 'Implement account /service/v1/apps and app/key create/delete contracts from account OpenAPI;Scope runs/groups/monitors/memory/connectors/secrets to organization/app/key and expose key material only at creation;Enforce Member/Admin permissions for key ownership/history/billing/webhook-secret reveal and rotation;Integrate invitation/role administration with operator identity provider and audit retention/deletion rules',
 'Create app/key, call search, revoke key, and verify subsequent auth failure;A role matrix rejects cross-key history and admin-only changes for members;Deletion follows documented history rules and audit records contain no keys',
 'Select scoped storage/auth context over global key checks. Reuse established identity and key-verifier implementations.',
 'resources/organization-roles-and-permissions service-api/apps/list-apps service-api/apps/create-app service-api/keys/create-key service-api/keys/delete-key'),
('WS-34','hosted-billing','Optional hosted usage ledger, credit balance, spend limits, and reload','WS-17 WS-33',
 'Implement account balance GET/add POST and immutable request/run/connector ledger entries;Support configurable prices, granted credits, app/organization monthly caps, payment references, history, and controlled auto-reload;Reserve/refund async costs and settle retries exactly once, separating free polling/connectors;Provide test-mode payment adapter and billing-disabled self-hosted operation',
 'Concurrent fixture requests cannot exceed spend caps or double charge;Failed/cancelled work settles by documented rules and free calls never debit;Fake provider tests cover add/reload retries and billing-disabled operation',
 'Select a transactional ledger and optional payment-provider adapter rather than a bespoke billing platform. Use decimal/integer minor units.',
 'getting-started/pricing service-api/balance/get-balance service-api/balance/add-to-balance resources/organization-roles-and-permissions resources/faqs'),
('WS-35','agentic-payments','Optional MPP and x402 payment gateway for machine clients','WS-34 WS-24',
 'Expose paid Search/Extract/Task creation and free polling through an optional gateway;Handle HTTP 402 challenge/proof verification/replay protection/idempotency using provider adapters;Publish prices/currencies/spend limits and test-mode Stripe/Tempo/Base settlement contracts;Provide mppx/purl examples and a skill without adding live settlement to default tests',
 'Fake settlement tests challenge/accepted/expired/wrong/reused proof;Retried paid requests produce one task and one ledger debit;Free polling and disabled gateway leave ordinary API behavior intact',
 'Reuse maintained MPP/x402 reference SDKs and settlement adapters. Do not implement a blockchain or store user wallets in core search.',
 'integrations/agentic-payments'),
('WS-36','llm-framework-adapters','LLM framework and gateway adapters for grounded search','WS-10 WS-11 WS-22 WS-26 WS-31',
 'Publish shared Search/Extract tool adapters for OpenAI-style/Anthropic-style/Gemini-style calling;Add LangChain wrappers, LiteLLM gateway configuration, Ollama examples, and OpenRouter search adapter contract;Provide Gemini Enterprise grounding mapping with source attribution and explicit external registration prerequisites;Preserve policy/budgets/errors/cancellation/usage through all adapters',
 'Stub tool-call fixtures round-trip arguments/results for each format without live model keys;Installed examples run locally and cancellation/errors remain typed;Integration matrix records tested versions and registration state without claiming unverified listings',
 'Select framework-native thin wrappers over shared clients/schemas. Evaluate maintained SDKs before adding new tool loops.',
 'integrations/langchain integrations/litellm integrations/openrouter integrations/ollama-tool-calling integrations/google-gemini-enterprise integrations/anthropic-tool-calling integrations/openai-tool-calling'),
('WS-37','agent-plugins','Agent skills and editor plugins for search, extraction, and research','WS-11 WS-30 WS-32',
 'Publish versioned shared skills and stdio/HTTP MCP configuration with uninstall steps;Add native manifests for Claude Code/Cursor/OpenCode/Pi/OpenClaw-ClawHub and compatibility recipes for other agents;Support self-hosted URLs, static keys/device login, finite polling, and JSON;Prepare distribution artifacts and track external marketplace approvals separately',
 'Schema lint and installation smoke tests use package artifacts and mocked local service;Each supported agent has search/extract/research-result examples with citations;Upgrade/uninstall preserves unrelated settings and secret storage',
 'Select shared skill content with thin native wrappers and official installation mechanisms rather than bespoke installers.',
 'integrations/agent-skills integrations/claude-code-marketplace integrations/cursor-marketplace integrations/opencode-plugin integrations/pi-extension integrations/clawhub'),
('WS-38','dataframe-sql','DataFrame and SQL enrichment: DuckDB, Polars, Spark, BigQuery, Snowflake, Supabase','WS-21 WS-31',
 'Define stable row-ID batching/schema/retry/checkpoint contract with bounded work;Publish DuckDB/Polars adapters and Spark UDF recipes preserving row alignment/types/null/error columns;Provide BigQuery remote-function and Snowflake UDTF deployment templates with secret setup and batch limits;Provide Supabase Edge Function integration and explicit cloud-account prerequisites',
 'Finite fixture tables with duplicate IDs/nulls/mixed outcomes preserve row count/order;DuckDB/Polars run locally and cloud adapters have mocked wire tests;Restart/cancellation resumes checkpoints without repeating paid jobs',
 'Select shared task-group client plus lightweight platform-native wrappers. Reuse current DataFrame APIs and remote-function templates.',
 'data-integrations/overview data-integrations/duckdb data-integrations/polars data-integrations/spark data-integrations/bigquery data-integrations/snowflake data-integrations/supabase'),
('WS-39','workflow-integrations','Workflow integrations: n8n, Zapier, Sheets, Render, Vercel, and Superhuman','WS-10 WS-20 WS-31',
 'Publish common search/extract/task/result actions and signed webhook triggers;Provide n8n node and Zapier action templates plus Google Sheets function/batch examples;Add Render Workflows/Vercel deployment and AI-tool templates and Superhuman recipe using available extension contract;Document secrets, endpoint reachability, retry/idempotency, and third-party approval requirements',
 'Each template has mocked actions and sample cited output, and invalid webhook signatures fail;Duplicate trigger cannot repeat Task enrichment and Sheets preserves row alignment;Deployment examples have finite execution limits and record unavailable external prerequisites',
 'Use platform-native thin actions over HTTP clients and webhook delivery. Avoid building another automation engine.',
 'integrations/n8n integrations/zapier integrations/gsuite integrations/render integrations/vercel integrations/superhuman'),
('WS-40','migration-conformance','Migration guides and conformance fixtures for Parallel, Tavily, Exa, and SERP','WS-10 WS-22 WS-23 WS-28 WS-29 WS-33 WS-34',
 'Map shared endpoints and competitor fields to search/source-policy/extract options with concrete examples;Test GA nested advanced_settings and explicitly separate beta compatibility adapters;Cover current beta FindAll/Ingest/Memory/Chat and GA Monitor plus selected legacy candidates/alpha-monitor/task-group aliases;Publish exact compatibility/extensions/unsupported fields and third-party prerequisite matrix',
 'Fixtures cover every current product/account operation and selected legacy aliases;Quickstart bodies run locally against JS/Rust for the claimed subset;No guide asserts equal latency/model quality/licensed data/marketplace availability without measurements',
 'Select versioned fixtures and explicit adapters instead of replacing every beta prefix with v1. Reuse shared OpenAPI and schema validation.',
 'search/migrate-to-parallel search/search-migration-guide extract/extract-migration-guide findall-api/findall-migration-guide monitor-api/monitor-migration-guide responses-api/openai-compatibility'),
('WS-41','quality-evaluation','Reproducible search/research evaluation for relevance, latency, citations, and cost','WS-06 WS-14 WC-12',
 'Create versioned objective/query sets for locale/policy/freshness/images/entities/research-schema tasks;Measure recall/nDCG/citation correctness/excerpt efficiency/p50-p95 latency/errors/provider-model cost;Run fixed offline fixtures in CI and opt-in same-budget comparisons with Parallel/Tavily/Exa;Publish raw results, versions, uncertainty, and thresholds separating self-hosting advantages from measured quality',
 'Seeded finite benchmark works without API keys and never fabricates live scores;Only the search adapter changes between live comparisons while synthesis harness stays fixed;Fixtures catch broken citations/irrelevant excerpts/missed results with bounded input/concurrency/memory',
 'Compare promptfoo with a minimal fixture runner and select the smallest reproducible harness. Use standard ranking metrics and WC-12 extraction corpus.',
 'search/evaluating-search search/best-practices task-api/best-practices image-search/best-practices'),
('WS-42','operations-retention','Production operations, status, retention controls, and failure diagnostics','WS-13 WS-17 WS-20',
 'Expose correlated provider/run timings, queue/outbox health, limit counters, and optional redacted tracing/metrics;Document readiness/liveness/graceful shutdown/storage migrations/backups/recovery and status-page integration;Configure expiry/deletion for inputs/outputs/events/memory/secrets and retention-disabled operation;Publish limits/overload/data handling/deployment examples without claiming unaudited certifications',
 'Controlled failures/restarts yield correlated actionable diagnostics without credentials/content leaks;Expiry removes scoped history/context and disabled retention does not persist interactions;Shutdown drains/requeues work safely and finite load fixtures obey queue bounds',
 'Reuse OpenTelemetry/tracing and storage policies with tracing off by default. Keep a hosted observability vendor optional.',
 'resources/status resources/faqs resources/warnings-and-errors getting-started/rate-limits task-api/guides/interactions'),
('WS-43','marketplace-distribution','AWS and Google Cloud marketplace deployment and subscription plans','WS-33 WS-34 WS-42',
 'Prepare reproducible container/cloud infrastructure templates with local build validation;Define entitlement/provisioning/organization mapping/metering/cancellation adapters;Prepare listing assets/support information and explicit partner account/approval prerequisites;Track submission/activation as external milestones requiring public listing and subscription smoke evidence',
 'Local templates build and mocked entitlement callbacks provision/revoke idempotently;Metering agrees with the ledger and retry does not duplicate usage;Issue records review-ready artifacts and each external blocker without claiming a live listing',
 'Compare SaaS fulfillment with deploy-your-own containers against operator capacity and official cloud contracts. Reuse billing/operating infrastructure.',
 'integrations/aws-marketplace integrations/google-cloud-marketplace'),
('WS-44','async-cli','CLI for Tasks, groups, Responses, FindAll, Monitor, Memory, and accounts','WS-12 WS-21 WS-22 WS-23 WS-28 WS-29 WS-18 WS-33 WS-34',
 'Add create/status/result/stream research and group enrichment commands plus Responses/Chat;Add FindAll preview/extend/enrich/cancel/refresh, Monitor events/update/trigger/cancel, and Memory retrieve/evict/clear;Add app/key/balance commands with login/profiles/custom base URLs and explicit secret output;Support stdin/file JSON, stable exit codes, finite wait/cancellation, and shared JS/Rust help/command parity',
 'Installed binaries agree on mocked command JSON/exit codes for each family;SIGINT stops polling/streams and invalid inputs do not create work;Legacy positional search works and secrets never appear in ordinary diagnostics',
 'Extend existing parsers and typed service interfaces instead of writing a second shell HTTP client. Share command metadata where useful.',
 'integrations/cli integrations/developer-quickstart integrations/account-api'),
('WC-09','authenticated-capture','Authenticated browser capture with scoped sessions and Browser Use integration','WC-06 WC-10',
 'Accept caller-managed cookie/header/browser storage-state references with isolated caller/session contexts;Provide an adapter for remote MCP private-web research without exposing credential paths or values;Maintain session continuity through bounded batches and safe provenance;Support expiry/logout/deletion and private cache partitioning',
 'Local login fixtures prove authenticated content never appears in anonymous shared cache;Two users stay isolated and redirects cannot forward credentials across unrelated hosts;Cancellation/expiry releases browser contexts and redacts receipt/error secrets',
 'Reuse browser-commander/browser storage-state mechanisms instead of implementing site-specific login automation.',
 'integrations/browseruse task-api/mcp-tool-call resources/faqs'),
('WC-10','outbound-policy','Capture target/redirect policy and bounded document processing','',
 'Define injectable target policy for schemes/hosts/networks/redirects/DNS changes and credential forwarding;Hosted defaults block unintended private-network access with explicit intranet opt-in for authorized connectors;Bound bytes/decompression/redirects/browser concurrency/PDF conversion using existing transport limits;Return typed per-URL target/size errors with secret-safe rejected-destination diagnostics',
 'Local DNS/transport fixtures cover blocked schemes/networks/redirects/rebinding/credential stripping;Finite oversized/compressed/malformed PDF probes obey process/memory limits;Authorized intranet policy works while anonymous hosted capture remains restricted',
 'Select one transport policy over scattered route checks. Reuse URL parsing/resource limits and evaluate isolated converter workers.',
 'extract/extract-quickstart resources/faqs'),
('WC-11','extract-contract','Standalone /v1/extract contract, OpenAPI, hints, errors, and usage','WC-02',
 'Mount POST /v1/extract backed by WC-02 and generate OpenAPI/tool schemas from shared types;Validate nested GA settings and reject beta-only top-level fields with typed 422 errors;Support extract_id/session_id/client_model/warnings/usage/partial errors/auth/configurable Retry-After limits;Keep legacy routes and explicit adapters, with excerpt max_chars_total independent of full_content limits',
 'GA quickstart bodies work in JS/Rust while beta top-level fields fail on GA and translate through explicit legacy route;Empty/invalid inputs/Unicode/budgets/mixed errors/session hints have shared fixtures;A generated client calls standalone web-capture without needing web-search',
 'Select shared batch schemas behind versioned adapters. Reuse existing server/schema libraries and avoid duplicating WC-02.',
 'api-reference/extract/extract extract/extract-migration-guide extract/advanced-extract-settings resources/warnings-and-errors getting-started/rate-limits'),
('WC-12','extraction-evaluation','Extraction quality/parity fixtures for HTML, multilingual pages, JS, and PDF','WC-01 WC-03 WC-06',
 'Add local article/table/code/multilingual/SPA/PDF fixtures with expected markdown/metadata/relevant passages;Measure boilerplate removal/content retention/link-table preservation/date attribution/budget correctness;Compare existing converters with Readability/trafilatura/Kreuzberg candidates using the same corpus and licenses;Publish offline evaluation and opt-in Parallel comparisons with path receipts and format limits',
 'JS/Rust equivalent normalized outputs have documented converter differences;Tables/code/links/Unicode survive within budgets without invalid truncation;Finite corpus and bounded converter/browser workers record corpus/runtime versions and quality evidence',
 'Add a common fixture suite before choosing converter replacements and make only evidence-driven targeted improvements.',
 'extract/extract-quickstart extract/best-practices search/evaluating-search'),
]

for id_,slug,title,deps,scope,tests,design,refs in ITEMS:
    repo='web-search' if id_.startswith('WS') else 'web-capture'
    def bullets(value): return '\n'.join('- '+part.strip()+'.' for part in value.split(';'))
    text=f'''---
id: {id_}
repo: link-assistant/{repo}
title: {title}
depends_on: [{', '.join(deps.split())}]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **{title}**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

{bullets(scope)}

## Solution alternatives and implementation plan

{design}

1. Inspect the existing {'search registry, transport, server, CLI, and shared JS/Rust types' if repo=='web-search' else 'capture transport, conversion modules, server, CLI, and capture receipts'}. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

{bullets(tests)}
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

'''
    text+='\n'.join('- https://docs.parallel.ai/'+ref for ref in refs.split())
    text+=f'\n- [Case study and source snapshot]({CASE}).\n- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.\n'
    (DRAFTS/f'{repo}-{id_[-2:]}-{slug}.md').write_text(text)
print(f'Added {len(ITEMS)} independent work items.')
