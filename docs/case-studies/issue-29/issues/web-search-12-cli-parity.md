---
id: WS-12
repo: link-assistant/web-search
title: CLI parity with parallel-cli: search and extract subcommands with JSON output and policy flags
depends_on: [WS-10]
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
