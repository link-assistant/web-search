# Issue 29: Parallel feature coverage and cross-repository delivery plan

[Issue #29](https://github.com/link-assistant/web-search/issues/29) requests a
complete analysis of Parallel's services, solutions for every requirement,
and issues with clear dependencies in both repositories. Search and research
orchestration belong in `web-search`; page fetching, conversion, metadata,
and extraction belong in `web-capture`.

The [review request](https://github.com/link-assistant/web-search/pull/30#issuecomment-6047229877)
requires improving the original drafts and **creating all issues**. The plan
now contains **56 issues: 44 for web-search and 12 for web-capture**, expanded
from 26. [The issue inventory](issue-inventory.md) contains every work item,
its prerequisites, the full dependency graph, delivery order, and endpoint
coverage. [created-issues.json](created-issues.json) records the filed issue URLs.

This PR delivers research and an actionable backlog. The product features in
that backlog are future implementation work; they are not implemented by this PR.

## Requirements and completion evidence

| Requirement                                                                    | Selected solution and evidence                                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1. Support everything expected from Parallel and improve the service.         | Inventory every indexed public documentation page and API operation, assign each to implementation work, and define measured quality improvements in WS-41/WC-12. Alternatives were API-only copying or a subset limited to public search; the selected plan includes research, integrations, and optional hosted services. |
| R2. Create search issues in web-search.                                        | WS-01–WS-44 cover search, research, service contracts, and integrations. Each issue has scope, a solution plan, acceptance tests, references, and dependency URLs.                                                                                                                                                          |
| R3. Create pure fetching issues in web-capture.                                | WC-01–WC-12 own extraction, freshness, metadata, browser/PDF routing, authenticated sessions, target policy, fetch tools, and quality fixtures. Search references these interfaces rather than reimplementing capture.                                                                                                      |
| R4. Analyze and plan all issues with clear dependencies.                       | The inventory graph is generated from issue front matter and checked for unknown dependencies and cycles. Bundled products were split into independent protocol and execution work.                                                                                                                                         |
| R5. Collect data here and search online for additional facts.                  | `data/` contains issue/PR feedback, both repositories' issue histories, the capture README, the live Parallel documentation/specification snapshot, and five primary component READMEs with checksums.                                                                                                                      |
| R6. List requirements, alternatives, component candidates, and solution plans. | The capability/solution tables below and all 56 issue bodies describe reuse, alternatives, implementation boundaries, and offline verification.                                                                                                                                                                             |
| Review. Make issues more detailed, add missing work, and actually file them.   | Every original draft was refined, 30 work items were added, and the durable mapping plus inventory records all filed issues. The resumable script updates issue bodies with real dependency links.                                                                                                                          |

## Research data and method

The documentation index was retrieved on **2026-10-08**. The prior snapshot
contained a hard-coded selection of 111 pages; it did not cover the whole
public index. The revised downloader discovers the index rather than using
that selection.

| Evidence                                                                   | Contents                                                                                                                                                                                 |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `data/issue-29*.json`                                                      | Original issue and its comments.                                                                                                                                                         |
| `data/pr-30-before-2026-10-08.json`, `data/pr-30-comments-2026-10-08.json` | PR description and all conversation feedback before this revision. Inline review comments and reviews were also checked; both were empty.                                                |
| `data/*-issues-before-filing-2026-10-08.json`                              | Paginated issue/PR histories in both repositories before filing. Only search #29 and capture #160 were open implementation/issues; #160 concerns an unrelated storage dependency.        |
| `data/web-capture-README.md`                                               | Capture capabilities and existing transport/conversion interfaces.                                                                                                                       |
| `data/parallel-docs/manifest.json`                                         | **162 indexed pages**, retrieval URLs/final URLs, media types, byte counts, and SHA-256 checksums. Two indexed links redirect to HTML, preserved as HTML instead of mislabeled markdown. |
| `data/parallel-docs/public-openapi.json` and `docs-latest-openapi.json`    | Current product specification: **37 operations** across 35 paths.                                                                                                                        |
| `data/parallel-docs/account-openapi.json`                                  | Account service specification: **7 operations** across 6 paths.                                                                                                                          |
| `data/parallel-docs/docs-legacy-openapi.json`                              | **44 legacy operations**, used to plan explicit compatibility adapters.                                                                                                                  |
| `data/parallel-docs/parallel-agents.md`                                    | Public agent integration guidance.                                                                                                                                                       |
| `data/components/`                                                         | Official Transformers.js, FastEmbed, Standard Webhooks, and TypeScript/Rust MCP SDK READMEs with retrieval manifest.                                                                     |
| `data/filed-issues-verification-2026-10-08.json`                           | Read-back verification of all 56 open GitHub issues: exact draft bodies, labels, titles, and dependency URLs.                                                                            |
| [coverage.json](coverage.json)                                             | Every indexed page and all **44 current + 44 legacy operations**, mapped to work items. This is the machine-readable traceability record.                                                |

Reproduce the snapshot and plan validation from the repository root:

```bash
bash experiments/issue-29/download-parallel-docs.sh
python3 experiments/issue-29/build-coverage.py
python3 experiments/issue-29/validate-plan.py
python3 experiments/issue-29/test_file_issues.py
```

The downloaded data is kept in its source format and excluded from formatting.
`manifest.json` is authoritative for current snapshot membership; historical
extra files from the earlier selection remain available in Git history/data.

## Verified capability inventory and ownership

Endpoint versions below follow the published specs, not a blanket replacement
of `v1beta` with `v1`. Local evidence uses the corresponding slash-to-underscore
filename in `data/parallel-docs/`; the coverage record identifies the exact file.

| Capability                  | Contract and relevant behavior                                                                                                                                            | Planned work                     | Primary source                                                                            |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- | ----------------------------------------------------------------------------------------- |
| Web Search                  | `/v1/search`: objective and/or queries, bounded excerpts, location, modes, source policy, warnings/usage/session envelope.                                                | WS-01–07, WS-10, WS-17           | [Search reference](https://docs.parallel.ai/api-reference/search/search)                  |
| Source policy               | Domain/subdomain/path-prefix normalization, include/exclude precedence, suffixes, and publication-date filtering. Provider operators are complemented by a final matcher. | WS-02, WC-03                     | [Source policy](https://docs.parallel.ai/resources/source-policy)                         |
| Image Search                | `/v1/images/search`: keyword/objective input, image/source page URLs and dimensions. Preserve available attribution and bound optional probes.                            | WS-08, WC-07                     | [Image Search](https://docs.parallel.ai/image-search/image-search-quickstart)             |
| Batch Extract               | `/v1/extract`: up to 20 URLs, per-URL errors, markdown excerpts/full content, JS/PDF conversion, and nested advanced settings.                                            | WC-01–04, WC-06, WC-11, WS-04/10 | [Extract settings](https://docs.parallel.ai/extract/advanced-extract-settings)            |
| Freshness                   | GA `max_age_seconds` minimum 600, live timeout, and `disable_cache_fallback`; stale fallback is distinct from force-live/cache bypass.                                    | WC-04, WS-07                     | [Fetch policy](https://docs.parallel.ai/extract/advanced-extract-settings#fetch-policy)   |
| Task research               | `/v1/tasks/runs`: text/JSON specifications, processor budgets, field-level basis/citations, status/input/result, and interactions.                                        | WS-13/14/24                      | [Task specification](https://docs.parallel.ai/task-api/guides/specify-a-task)             |
| Task groups                 | Add runs while active, aggregate progress, individual input/result retrieval, pagination, group events.                                                                   | WS-21                            | [Group API](https://docs.parallel.ai/task-api/group-api)                                  |
| SSE and webhooks            | Task/group/FindAll streams, Responses stream events, signed Standard Webhooks, durable retry and replay.                                                                  | WS-19/20                         | [Webhook setup](https://docs.parallel.ai/resources/webhook-setup)                         |
| Responses                   | `/v1/responses`: stateful follow-ups, reasoning effort, JSON schema, citations with offsets, search/open-page items, MCP, data sources, SSE.                              | WS-22/24/25/26                   | [Compatibility reference](https://docs.parallel.ai/responses-api/openai-compatibility)    |
| Chat                        | `/v1beta/chat/completions`: messages, model/options, usage/finish reasons, chat-specific streaming.                                                                       | WS-23                            | [Chat reference](https://docs.parallel.ai/api-reference/chat-api-beta/chat-completions)   |
| Entity Search               | `/v1beta/findall/entity-search`: people/companies objective, matching limits, ranked entity records.                                                                      | WS-09                            | [Entity Search](https://docs.parallel.ai/findall-api/entity-search)                       |
| FindAll                     | Beta runs/ingest/schema, candidate conditions and basis, preview/generator tiers, enrichment, exclusions, extend/cancel/refresh, events.                                  | WS-16/24/27/28                   | [FindAll lifecycle](https://docs.parallel.ai/findall-api/core-concepts/findall-lifecycle) |
| Monitor                     | GA control routes, schedules, event-stream detections, JSON event history, no-change completion/error, snapshots, follow-up Tasks, webhooks.                              | WS-15/20/29                      | [Monitor events](https://docs.parallel.ai/monitor-api/monitor-events)                     |
| Memory                      | `/v1beta/memory/{retrieve,evict,clear}` and scoped reuse of past Task/Monitor/FindAll research; removing memory does not delete source resources.                         | WS-18                            | [Memory](https://docs.parallel.ai/resources/memory)                                       |
| External/private tools      | Task MCP clients, Responses MCP tools, authenticated Browser Use/private sources, tool-call records.                                                                      | WS-25, WC-09/10                  | [MCP tool calling](https://docs.parallel.ai/task-api/mcp-tool-call)                       |
| Data connectors             | Free, pay-per-use, licensed index partners, BYOL. Six public reference adapters and a pluggable licensed/paid contract are planned.                                       | WS-26                            | [Data connectors](https://docs.parallel.ai/resources/data-connectors)                     |
| Developer tools             | Search/Task MCP, function schemas, agent skills/plugins, native and service SDKs, full CLI product coverage.                                                              | WS-11/12/30/31/36/37/44, WC-08   | [Developer tools](https://docs.parallel.ai/integrations/developer-quickstart)             |
| Data/workflow integrations  | DuckDB/Polars/Spark/BigQuery/Snowflake/Supabase; n8n/Zapier/Sheets/Render/Vercel/Superhuman; framework and gateway adapters.                                              | WS-36/38/39                      | [Data integrations](https://docs.parallel.ai/data-integrations/overview)                  |
| Account services            | Device OAuth, refresh/revoke, apps/keys, Member/Admin capabilities and scoped histories/secrets.                                                                          | WS-32/33                         | [Account API](https://docs.parallel.ai/integrations/account-api)                          |
| Hosted services             | Usage/credits/balance/reload/spend caps, MPP/x402 gateway, AWS/GCP deployment and marketplace subscription plans. Optional for self-hosting, explicitly tracked.          | WS-34/35/43                      | [Agentic payments](https://docs.parallel.ai/integrations/agentic-payments)                |
| Crawler/operations          | Crawler identity/robots/politeness, status/diagnostics, retention controls, operating limits, migration/version conformance.                                              | WC-05/10, WS-40/42               | [Crawler](https://docs.parallel.ai/resources/crawler)                                     |
| Demonstrably better quality | Fixed-harness relevance, freshness, latency, citation, cost, and extraction measurements; results and uncertainty retained.                                               | WS-41, WC-12                     | [Evaluation guide](https://docs.parallel.ai/search/evaluating-search)                     |

Parallel publishes pricing/quotas and latency guidance in
[pricing](https://docs.parallel.ai/getting-started/pricing),
[rate limits](https://docs.parallel.ai/getting-started/rate-limits), and product
pages. These are dated competitor observations, not performance guarantees
for this stack. WS-17 supports configurable quotas, WS-34 supports operator
prices, and WS-41 measures the actual comparison.

## Current repository capabilities and gaps

`web-search` has 40 descriptor-driven providers in search/knowledge/papers/code,
RRF/weighted/interleave merging, canonical URL deduplication, detailed provider
outcomes, and caller-owned transport. JS and Rust library, CLI, and HTTP entry
points are maintained with parity checks. Relevant existing modules are
`js/src/search.js`, `js/src/merger.js`, `js/src/providers/registry.js`,
`js/src/server.js`, and the corresponding Rust modules.

Its current request is a query with provider/limit/language/region/merge options,
and its results are title/URL/snippet/source/rank records. Objective-directed
excerpts, GA envelopes/policy, image/entity categories, agent research,
async protocols, MCP, and account/hosted services are gaps covered above.
Existing weighted/RRF reranking is retained; optional learned relevance ranking
in WS-05 supplements it.

`web-capture` already owns fetch/HTML/text/markdown/image/archive/PDF/DOCX and
other specialized converters, browser capture, receipt-producing injectable
transport, and a content-addressed CaptureStore/CachedTransport with replay.
Its small structured provider capture surface is reused for capture but does
not replace the search registry. Batch extraction, passage ranking, verified
publication metadata, public freshness controls, standalone GA schema,
scoped authenticated capture, and quality fixtures belong there.

Closed capture #130/#135/#156 provide relevant transport, contract, and cache
history. The refreshed issue lists were checked before filing so this plan
adds missing work rather than duplicating the unrelated open capture #160.

## Solutions, alternatives, and component candidates

Each individual issue contains implementation steps and specific tests. This
table explains the shared design choices and reusable components. Components
are candidates to validate for runtime compatibility and license during their
implementation issues, not new dependencies introduced by this documentation PR.

| Area                   | Alternatives and selected plan                                                                                                                                              | Reusable components                                                                                                                                                           |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Request/merge          | Adding flags to the single query versus an explicit structured request: select a structured overload and GA adapter, retaining legacy wrappers and current merger/receipts. | Existing engines/mergers, `serde`, schema validation.                                                                                                                         |
| Policy/locale/metadata | Provider-only filtering versus final filtering: select both. Separate country from language and confirmed publication from modification/capture timestamps.                 | Existing descriptors, URL parsing, `tldts`/public suffix candidates, metadata parsers/`chrono`.                                                                               |
| Excerpts/conversion    | Lead text versus BM25 versus embeddings: select deterministic BM25 first, optional embeddings/reranker. Reuse converters before replacing them.                             | MiniSearch/wink-BM25 candidates, existing browser/PDF conversion, Readability/trafilatura/Kreuzberg comparisons.                                                              |
| Learned ranking        | Hosted model versus optional local inference: select injected local/remote adapters with lazy model loading and stable fallback.                                            | [Transformers.js](https://github.com/huggingface/transformers.js), [FastEmbed](https://github.com/Anush008/fastembed-rs).                                                     |
| Freshness              | New cache versus exposing existing CaptureStore policy: select reuse, scoped keys, typed stale fallback, and explicitly named local extensions.                             | Existing CaptureStore/CachedTransport and receipt contract.                                                                                                                   |
| Image/entity sources   | One proprietary source versus a descriptor/plugin catalog: select open sources first, with explicit credential/license prerequisites for restricted sources.                | Wikimedia/Openverse/Wikidata, existing REST/HTML provider adapters, bounded dimension probes.                                                                                 |
| Async execution        | Separate runtime per product versus shared workers/store: select shared durable runs with independent protocol adapters and finite budgets.                                 | SQLite/`sqlx`, native JS queues/Tokio, existing transport injection.                                                                                                          |
| Streaming/webhooks     | Ephemeral events/best-effort POST versus durable events/outbox: select durable storage and bounded clients with reference-compatible signing.                               | [Standard Webhooks](https://github.com/standard-webhooks/standard-webhooks), native streams/axum SSE.                                                                         |
| Research/answers       | Bundled hosted model versus caller-owned models: select bounded planning/retrieval/synthesis with actual evidence, then independent Task/Responses/Chat adapters.           | Existing search/extract, backend SDK candidates, `ajv`/`jsonschema`.                                                                                                          |
| MCP/connectors         | Manual JSON-RPC/proprietary datasets versus SDKs and plugins: select official clients/servers and explicit access models.                                                   | [TypeScript MCP SDK](https://github.com/modelcontextprotocol/typescript-sdk), [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk), existing registry/transports. |
| Monitor/FindAll/Memory | Separate products versus composition: select shared runs, candidates, scoped memory, scheduler/checkpoints, and field-level basis.                                          | Existing capture hashes, cron scheduler candidates, storage transactions and passage ranking.                                                                                 |
| SDKs/integrations      | Handwritten parallel clients versus shared OpenAPI/types: select generated clients where supported and thin tested framework/platform adapters.                             | OpenAPI Generator/openapi-typescript candidates, native framework APIs and deployment templates.                                                                              |
| Accounts/payments      | Mandatory hosted business stack versus optional operator adapters: select optional standards-based auth, scoped accounts, transactional ledger, and protocol gateway.       | Established OAuth providers, RFC 8628 clients, payment-provider/MPP/x402 SDK candidates.                                                                                      |
| Quality/operations     | Unmeasured parity promises versus reproducible evidence: select a fixed fixture corpus plus opt-in live comparisons, retention policies, and redacted diagnostics.          | Existing test frameworks, small ranking/eval runner or promptfoo, OpenTelemetry/tracing candidates.                                                                           |

## Review corrections and finer delivery boundaries

The initial 26 drafts were too broad in several places and excluded documented
commercial/integration services. Every original draft now adds concrete
implementation follow-through and edge-case verification. Thirty new work items
split protocols/products and cover previously excluded capabilities.

- WS-13 now owns durable runtime only; WS-19/20/21 own SSE/webhooks/groups.
- WS-14 owns shared grounded Task execution; WS-22/23/24 own Responses, Chat,
  and task specs/processor budgets/interactions.
- WS-16 owns core discovery; WS-27/28 own enrichment and lifecycle/refresh.
- WS-15 owns GA event-stream Monitor; WS-29 owns snapshots/follow-up Tasks.
- WS-18 owns Memory; WS-40 owns migration/conformance.
- WC-08 owns tool schemas/CLI; WC-09 owns required authenticated capture,
  WC-10 owns target/resource policy, and WC-11 owns standalone GA Extract.
- Accounts, credits, payments, marketplace plans, six public connectors,
  client SDKs, agent plugins, SQL/DataFrame and workflow integrations,
  full CLI coverage, evaluation, and operations all have dedicated issues.

Specific stale assumptions were corrected against the current snapshot:

1. GA fetch policy uses `disable_cache_fallback`, not `disable_cached_content`,
   and has minimum `max_age_seconds: 600`. Zero-age force-live is a local
   extension, not claimed Parallel compatibility.
2. Task follow-ups use `interaction_id`/`previous_interaction_id`, not
   `previous_run_id`.
3. FindAll inference is `POST /v1beta/findall/ingest`; run schema retrieval is
   `GET .../schema`. No invented `POST /schema` route is planned.
4. Monitor history uses paginated JSON and webhooks, not an SSE endpoint.
5. `max_chars_total` budgets excerpts and does not cap full content.
6. Modification timestamps and URL date guesses cannot be silently presented
   as verified publication dates.
7. Current optional JS inference candidates use `@huggingface/transformers`.
8. Marketplace activation and proprietary dataset access are tracked external
   prerequisites rather than omitted requirements or promised availability.

## Delivery and filing

The [full inventory](issue-inventory.md) is the authoritative delivery order.
Start foundation work in each repository independently, then batch capture and
search contracts, then retrieval quality/public surfaces, then shared runtime,
research/protocol adapters, then composed products and integrations. Accounts,
operations, and optional hosted distribution have their own prerequisites.
Do not use phase labels to override any dependency in the inventory.

The script defaults to a validating dry run:

```bash
bash experiments/issue-29/file-issues.sh
bash experiments/issue-29/file-issues.sh --apply
```

`--apply` creates issues in topological order and stores each URL atomically.
Each issue has a stable marker. On restart, the script rediscovers markers
through paginated GitHub APIs before creation, recovering even if the local
mapping was lost after a successful create. It refuses duplicate markers,
unknown dependencies, and cycles. A final pass embeds real dependency and
forward-reference links in issue bodies without posting duplicate comments.

Verify the published issues without modifying GitHub:

```bash
python3 experiments/issue-29/verify-filed-issues.py
```

## Assumptions and limits

- Hosted processor labels are configurable budgets, not a promise to reproduce
  Parallel's proprietary models/index or measured latency. A caller-owned model
  is needed for synthesis; basic retrieval remains independently usable.
- Commercial services are optional in a self-hosted deployment, but their
  documented capabilities have implementation plans and issues. Proprietary
  datasets and external marketplace approvals require operator access.
- Self-hosting, provider diversity, injected transports/models, and JS/Rust native
  libraries are architectural advantages. Better answer/search quality requires
  WS-41/WC-12 measurements before making performance claims.
- This task plans and files the implementation backlog. No core library behavior
  or package version changes, release changeset, or Rust changelog are required.

## Validation

The downloader completed with `downloaded=168 pages=162 failed=0`. The coverage
validator checks all indexed pages and all 44 current plus 44 legacy operations,
source hashes, 56 draft IDs, and an acyclic dependency graph. Offline filing
regression tests reproduce and prevent invalid dependency writes, cycles,
filename-order mistakes, lost mappings, and duplicates after partial failure.
CI runs the plan validator and filing regression tests alongside repository
parity checks. The separate GitHub read-back verified every published body,
title, label, and linked prerequisite. Local checks and latest-commit CI
results are recorded in PR #30.
