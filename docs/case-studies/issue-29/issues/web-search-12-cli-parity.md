---
id: WS-12
repo: link-assistant/web-search
title: CLI parity with parallel-cli: search and extract subcommands with JSON output and policy flags
depends_on: [WS-09, WS-10, WS-11]
labels: enhancement
---

## Summary

`parallel search "<query>" --json --after-date --include-domains --mode` and
`parallel extract <url> --objective --full-content` (`integrations_cli.md`)
cover the agent-facing CLI surface. web-search has `--providers --limit
--strategy --format` only.

## Scope

- `web-search search <query...>` with `--objective`, `--mode`, `--include-domains`,
  `--exclude-domains`, `--after-date`, `--location`, `--max-results`,
  `--max-chars`, `--json` (envelope), `--urls`.
- `web-search extract <url...> --objective --full-content --json` forwarding to web-capture.
- `web-search images`, `web-search entities`, `web-search mcp`, `web-search serve`.
- Keep existing flags as aliases; update `--help` and README; same surface in `clap`.

## Acceptance criteria

- CLI snapshot tests for each subcommand with mocked transport.
- Parity script extended to compare CLI flags between JS and Rust.

## References

- https://docs.parallel.ai/integrations/cli

## Implementation follow-through

1. Extend current installed CLI/parser tests with direct typed library/service delegation.
2. Keep async and account command families in WS-44 and mount MCP subcommands after WS-11.

## Additional verification

- Legacy positional search, repeated queries, stdin, invalid combinations, JSON stdout/diagnostics stderr, and exit codes.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
