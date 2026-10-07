---
id: WS-11
repo: link-assistant/web-search
title: MCP server with web_search and web_fetch tools, LLM tool definitions, and an agent skill
depends_on: [WS-10, web-capture WC-08]
labels: enhancement
---

## Summary

parallel.ai ships a hosted Search MCP (`web_search`, `web_fetch`, streamable
HTTP, header/query overrides, ~25k character cap), tool-definition snippets for
OpenAI/Anthropic/Gemini, an agent skill, and marketplace listings
(`integrations_mcp_search-mcp.md`, `integrations_developer-quickstart.md`,
`parallel-agents.md`).

## Scope

- `web-search mcp` (stdio) and `POST /mcp` (streamable HTTP) exposing
  `web_search(objective, search_queries, session_id, mode, source_policy)` and
  `web_fetch` delegating to WC-08; `@modelcontextprotocol/sdk` (JS), `rmcp` (Rust).
- Per-request overrides via headers/query (`mode`, `max_results`, `max_chars_total`).
- `tools/` directory with OpenAI, Anthropic, Gemini function definitions and
  a `SKILL.md` for Claude Code/Cursor; README section "Use with agents".

## Acceptance criteria

- MCP inspector/SDK client test listing tools and calling `web_search` with mocked providers.
- Tool definition JSON validated in CI.

## References

- https://docs.parallel.ai/integrations/mcp/search-mcp
- https://modelcontextprotocol.io/specification
