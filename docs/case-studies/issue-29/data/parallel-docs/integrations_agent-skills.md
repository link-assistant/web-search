> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Install Parallel Agent Skills

> Use the Parallel CLI or npx skills to give Cursor, Cline, GitHub Copilot, Windsurf, and 30+ AI coding agents Parallel web search, extraction, deep research, and data enrichment.

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Agent Skills let you add Parallel's capabilities to AI coding agents like Cursor, Cline, GitHub Copilot, Windsurf, and 30+ other tools. Install and update skills with the Parallel CLI, or use a single `npx skills add` command via the [Agent Skills CLI](https://github.com/vercel-labs/skills). Skills are lightweight, declarative integrations that give your agent access to live web data without writing any code.

<Tip>View the complete repository for this integration [here](https://github.com/parallel-web/parallel-agent-skills)</Tip>

## Available Skills

| Skill | Description |
| - | - |
| `parallel-web-search` | Fast web search for current events, fact-checking, and lookups |
| `parallel-web-extract` | Extract clean content from URLs, including JavaScript-heavy sites and PDFs |
| `parallel-deep-research` | Exhaustive, multi-source research reports with configurable depth |
| `parallel-data-enrichment` | Bulk enrichment of companies, people, or products with web-sourced data |

See the [published skill catalog](https://skills.parallel.ai) for the complete, current list.

## Prerequisites

Most skills require the Parallel CLI to run. The `npx` installer only requires Node.js to install the skill definitions, so you can set up the CLI afterward.

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

### Install and update with the Parallel CLI

Install or update all skills globally:

```bash theme={"system"}
parallel-cli skills install
```

To install or update skills only for the current project:

```bash theme={"system"}
parallel-cli skills install --project
```

Global installs use `~/.agents/skills`; project installs use `<project>/.agents/skills`. When Claude Code is detected, skills are also installed in the corresponding `.claude/skills` directory. Skills not managed by the Parallel CLI are not overwritten or removed.

For a clean reinstall, run `parallel-cli skills reinstall`. Add `--project` to reinstall project-local skills.

### Install with npx

The Agent Skills CLI is a lightweight, Node-only alternative that supports agent-specific skill directories.

Install all skills globally so they're available in every project:

```bash theme={"system"}
npx skills add parallel-web/parallel-agent-skills --all --global
```

Or install a specific skill:

```bash theme={"system"}
npx skills add parallel-web/parallel-agent-skills --skill parallel-web-search
```

To see all available skills before installing:

```bash theme={"system"}
npx skills add parallel-web/parallel-agent-skills --list
```

## Usage

Once installed, skills are automatically available to your agent. No additional configuration is needed — your agent will use them when appropriate based on your prompts.

* **Web search** is used by default for any research, lookup, or question needing current information
* **Extract** is used when your agent needs to fetch content from a specific URL
* **Deep research** is triggered when you explicitly request exhaustive or comprehensive research
* **Data enrichment** is used for bulk enrichment of lists of companies, people, or products

## Supported Agents

Agent Skills work with any tool that supports the Vercel Skills CLI, including:

* Cursor
* Cline
* GitHub Copilot
* Windsurf
* And [30+ other agents](https://github.com/vercel-labs/skills#supported-agents)

For Claude Code, you can also use the [Claude Code Plugin Marketplace](/integrations/claude-code-marketplace) integration.

## Learn More

For detailed skill documentation, configuration options, and local development instructions, see the [parallel-agent-skills repository on GitHub](https://github.com/parallel-web/parallel-agent-skills).
