> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# OpenClaw Skills for Web Search and Research

> What OpenClaw skills are, how to install them from ClawHub, and how to give OpenClaw web search, content extraction, deep research, and data enrichment with Parallel.

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

[ClawHub](https://clawhub.ai) is the public skill registry for [OpenClaw](https://openclaw.ai), an open-source AI agent that runs locally on your machine. Parallel publishes four official skills on ClawHub that give OpenClaw access to web search, content extraction, deep research, and data enrichment via the [Parallel CLI](/integrations/cli).

## What are OpenClaw skills?

Skills are installable packages that extend what OpenClaw can do. Each one teaches the agent a new capability — searching the web, reading a URL, running a research report — without you writing any integration code. You install a skill once with the `clawhub` CLI, and OpenClaw invokes it on its own when a request calls for it.

Because OpenClaw runs locally, skills are how it reaches services beyond your machine. The four Parallel skills below cover the web-data side: live search, content extraction, deep research, and bulk enrichment.

## Installing skills safely

ClawHub is an open registry, so treat skills the way you would any dependency:

* **Check the source before installing.** A skill runs with the same permissions as your agent.
* **Bind the gateway to loopback** (as shown in [Installation](#installation)) so it isn't reachable from your network.
* **Prefer skills you can inspect.** Each Parallel skill wraps documented [Parallel CLI](/integrations/cli) commands, so you can run the same calls yourself to see exactly what it does.

<Note>ClawHub does not currently support verified or "official" publisher accounts. The Parallel skills are published under [@NormallyGaussian](https://clawhub.ai/u/normallygaussian).</Note>

## Available Skills

| Skill | Description |
| - | - |
| [parallel-search](https://clawhub.ai/normallygaussian/parallel-search) | AI-powered web search with domain filtering and date ranges |
| [parallel-extract](https://clawhub.ai/normallygaussian/parallel-extract) | Extract clean markdown from URLs, PDFs, and JS-heavy sites |
| [parallel-deep-research](https://clawhub.ai/normallygaussian/parallel-deep-research) | Multi-source deep research with configurable processor tiers |
| [parallel-enrichment](https://clawhub.ai/normallygaussian/parallel-enrichment) | Bulk data enrichment for companies, people, or products |

## Prerequisites

<Steps>
  <Step title="Install the Parallel CLI">
    Install the [Parallel CLI](/integrations/cli) via `pipx`:

    ```bash theme={"system"}
    pipx install "parallel-web-tools[cli]" && pipx ensurepath
    ```

    See the [CLI docs](/integrations/cli) for `uv`, Homebrew, npm, and other installation methods.
  </Step>

  <Step title="Authenticate">
    ```bash theme={"system"}
    parallel-cli login
    # or
    export PARALLEL_API_KEY="your_api_key"
    ```

    See the [CLI docs](/integrations/cli#authentication) for other authentication methods.
  </Step>
</Steps>

## Installation

First, install OpenClaw and ClawHub if you haven't already:

```bash theme={"system"}
# Install OpenClaw (recommended: run locally for security)
npm install -g openclaw@latest
openclaw gateway --port 18789 --bind loopback
openclaw dashboard  # start the chat interface

# Install ClawHub CLI
npm install -g clawhub
```

Then install the Parallel skills:

```bash theme={"system"}
clawhub install parallel-search
clawhub install parallel-extract
clawhub install parallel-deep-research
clawhub install parallel-enrichment
```

## Usage

Once installed, the skills are automatically available to OpenClaw. The agent will invoke them based on your messages:

* **Search** — "Search the web for the latest AI funding news"
* **Extract** — "Read this URL and summarize it: `<paste-url-here>`"
* **Deep Research** — "Do a thorough investigation of the EV battery market"
* **Enrichment** — "Find the CEO and annual revenue for each company in this list"

All skills use the [Parallel CLI](/integrations/cli) under the hood and support `--json` output for structured results.

## Learn More

* [Parallel CLI documentation](/integrations/cli) for full command reference
* [Agent Skills](/integrations/agent-skills) for Cursor, Cline, Copilot, and other coding agents
* [Claude Code](/integrations/claude-code-marketplace) for the Claude Code plugin
