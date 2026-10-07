---
id: WC-08
repo: link-assistant/web-capture
title: Provide a web_fetch tool definition, CLI JSON output, and optional authenticated capture
depends_on: [WC-02]
labels: enhancement
---

## Summary

parallel.ai's Search MCP exposes `web_fetch(urls, objective, search_queries)`
next to `web_search` (`integrations_mcp_search-mcp.md`), and the CLI has
`parallel extract --objective --full-content --json` (`integrations_cli.md`).
The Task API gained authenticated page access in Jan 2026
(`resources_changelog.md`), which parallel.ai's FAQ says the public APIs do not
offer; web-capture can do better here.

## Scope

- Tool definition JSON (MCP tool schema plus OpenAI/Anthropic function
  formats) for `web_fetch` backed by WC-02, published in the package so
  web-search WS-11 can mount it.
- CLI: `web-capture extract ... --json` prints the WC-02 response.
- Optional: `auth` option accepting a cookie jar or Playwright storage state
  file for capturing pages behind a login the user controls.

## Acceptance criteria

- Tool schema validates against the MCP SDK types; CLI JSON snapshot test.
- Authenticated capture test against a local fixture login page.

## References

- https://docs.parallel.ai/integrations/mcp/search-mcp
- Consumer: web-search WS-11
