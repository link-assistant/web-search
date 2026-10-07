#!/usr/bin/env bash
# Download the public parallel.ai documentation pages (Mintlify serves a
# markdown rendition of every page at <page>.md) into the issue-29 case-study
# data folder so the analysis is reproducible and offline-reviewable.
#
# Usage: bash experiments/issue-29/download-parallel-docs.sh [out-dir]
set -u
OUT="${1:-docs/case-studies/issue-29/data/parallel-docs}"
mkdir -p "$OUT"
BASE="https://docs.parallel.ai"
PAGES=(
  getting-started/overview getting-started/choose-an-api getting-started/pricing
  getting-started/rate-limits getting-started/glossary
  search/search-quickstart search/best-practices search/evaluating-search search/modes
  search/migrate-to-parallel search/indexed-content-for-agents
  search/advanced-search-settings search/source-policy search/search-mcp
  search/search-migration-guide
  image-search/image-search-quickstart image-search/best-practices image-search/modes
  extract/extract-quickstart extract/best-practices extract/advanced-extract-settings
  extract/extract-migration-guide
  task-api/task-quickstart task-api/best-practices task-api/guides/specify-a-task
  task-api/guides/choose-a-processor task-api/guides/execute-task-run
  task-api/guides/access-research-basis task-api/guides/interactions
  task-api/examples/interactive-research task-api/examples/task-deep-research
  task-api/examples/task-enrichment task-api/webhooks task-api/group-api
  task-api/ingest-api task-api/task-sse task-api/mcp-tool-call task-api/data-connectors
  task-api/task-mcp task-api/source-policy
  responses-api/responses-quickstart responses-api/features/statefulness
  responses-api/features/structured-outputs responses-api/features/streaming-events
  responses-api/features/citations responses-api/features/web-search-tool
  responses-api/features/mcp-tools responses-api/features/data-connectors
  responses-api/examples/direct-requests responses-api/examples/research-subagent
  responses-api/openai-compatibility
  findall-api/findall-quickstart findall-api/entity-search
  findall-api/core-concepts/findall-generator-pricing findall-api/core-concepts/findall-candidates
  findall-api/core-concepts/findall-lifecycle findall-api/features/findall-preview
  findall-api/features/findall-enrich findall-api/features/findall-sse
  findall-api/features/findall-webhook findall-api/features/findall-extend
  findall-api/features/findall-cancel findall-api/features/findall-refresh
  findall-api/findall-migration-guide
  monitor-api/monitor-quickstart monitor-api/monitor-events monitor-api/monitor-webhooks
  monitor-api/quickstart-snapshot monitor-api/monitor-task monitor-api/monitor-migration-guide
  integrations/developer-quickstart integrations/account-api integrations/agent-skills
  integrations/anthropic-tool-calling integrations/claude-code-marketplace integrations/cli
  integrations/langchain integrations/mcp/quickstart integrations/mcp/programmatic-use
  integrations/mcp/search-mcp integrations/mcp/task-mcp integrations/oauth-provider
  integrations/openai-tool-calling integrations/agentic-payments integrations/browseruse
  integrations/litellm integrations/openrouter integrations/gsuite
  data-integrations/overview data-integrations/duckdb data-integrations/polars
  resources/data-connectors resources/memory resources/source-policy
  resources/warnings-and-errors resources/webhook-setup resources/crawler resources/faqs
  resources/changelog
  api-reference/search/search api-reference/image-search/image-search
  api-reference/extract/extract api-reference/tasks/create-task-run
  api-reference/tasks/retrieve-task-run-result api-reference/tasks/create-task-group
  api-reference/findall/fast-entity-search api-reference/findall/create-findall-run
  api-reference/monitor/create-monitor api-reference/memory/retrieve-memory
  api-reference/chat-api-beta/chat-completions api-reference/responses-api/create-response
)
ok=0; fail=0
for p in "${PAGES[@]}"; do
  f="$OUT/$(echo "$p" | tr '/' '__').md"
  code=$(curl -sL --max-time 30 -A "web-search-case-study/1.0" -o "$f" -w '%{http_code}' "$BASE/$p.md")
  if [ "$code" = "200" ] && [ -s "$f" ]; then ok=$((ok+1)); else fail=$((fail+1)); echo "FAIL $code $p"; rm -f "$f"; fi
done
curl -sL --max-time 60 -A "web-search-case-study/1.0" -o "$OUT/public-openapi.json" "$BASE/public-openapi.json" || true
curl -sL --max-time 30 -A "web-search-case-study/1.0" -o "$OUT/llms.txt" "$BASE/llms.txt" || true
curl -sL --max-time 30 -A "web-search-case-study/1.0" -o "$OUT/parallel-agents.md" "https://parallel.ai/agents.md" || true
echo "downloaded=$ok failed=$fail"
