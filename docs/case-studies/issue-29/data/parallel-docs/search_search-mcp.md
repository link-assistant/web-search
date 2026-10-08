> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Search MCP

> Add real-time web search and content extraction to AI agents with the Parallel Search MCP Server

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

The Parallel Search MCP Server provides drop-in web search and content extraction capabilities for any MCP-aware model. The tools invoke the [Search API](/search/search-quickstart) and [Extract API](/extract/extract-quickstart) endpoints but present a simpler interface to ensure effective use by agents. Anonymous free-tier searches run in [`fast` mode](/search/modes) by default, providing high-quality, sub-second search for agent loops. Total excerpts per tool call are capped to roughly 25,000 characters to stay within typical MCP client output limits. Authenticated clients can [configure the search behavior](#configure-search-behavior).

<Tip>
  **The Search MCP is free to use** — no API key required, great for exploration and light use. For production use cases or higher rate limits, [create a Parallel account](https://platform.parallel.ai) and pass your API key as a Bearer token in the `Authorization` header.
</Tip>

The Search MCP comprises two tools:

* **`web_search`** — General-purpose web search inside an agent's reasoning loop. Use when the agent needs current information or diverse sources across multiple angles.
* **`web_fetch`** — Pulls token-efficient markdown from specific URLs. Use after `web_search` narrows down candidates, or when the agent already has a URL it needs to read in depth.

## One-Click Install

Install in any of the following clients with a single click — no API key required.

<CardGroup cols={2}>
  <Card title="Install in Cursor" icon="arrow-pointer" href="https://cursor.com/en/install-mcp?name=Parallel%20Search%20MCP&config=eyJ1cmwiOiJodHRwczovL3NlYXJjaC5wYXJhbGxlbC5haS9tY3AifQ==">
    One-click install for Cursor.
  </Card>

  <Card title="Install in VS Code" icon="code" href="https://insiders.vscode.dev/redirect/mcp/install?name=Parallel%20Search%20MCP&config=%7B%22type%22%3A%22http%22%2C%22url%22%3A%22https%3A%2F%2Fsearch.parallel.ai%2Fmcp%22%7D">
    One-click install for VS Code.
  </Card>

  <Card title="Install in LM Studio" icon="microchip" href="lmstudio://add_mcp?name=Parallel%20Search%20MCP&config=eyJ1cmwiOiJodHRwczovL3NlYXJjaC5wYXJhbGxlbC5haS9tY3AifQ%3D%3D">
    One-click install for LM Studio.
  </Card>

  <Card title="Install in Goose" icon="feather" href="goose://extension?url=https%3A%2F%2Fsearch.parallel.ai%2Fmcp&type=streamable_http&timeout=300&id=parallel-search-mcp&name=Parallel%20Search%20MCP&description=Parallel%20Search%20MCP%20server">
    One-click install for Goose.
  </Card>
</CardGroup>

For Claude Code, Codex CLI, Claude Desktop, Windsurf, Zed, Gemini CLI, Warp, Kiro, fx (Vercel), and other clients, see the [Installation](#installation) section below — those clients use a CLI command or a JSON config file rather than deep-link URLs.

## Use Cases

The Search MCP is suited for any application where real-world information is needed as part of an AI agent's reasoning loop. Common use cases include:

* Real-time fact checking and verification during conversations
* Gathering current information to answer user questions
* Researching topics that require recent or live data
* Retrieving content from specific URLs to analyze or summarize
* Competitive intelligence and market research

## Configure search behavior

The Search MCP has two configuration layers: model-visible arguments selected for each tool call, and authenticated connection-level overrides that apply to every `web_search` call.

### Per-call tool parameters

These arguments are returned by the server's `tools/list` response and supplied on individual tool calls.

#### `web_search`

| Parameter | Type | Required | Description |
| - | - | - | - |
| `objective` | string | Yes | A natural-language description of the information to find. Keep it focused on one goal; it can include preferred sources or freshness requirements. |
| `search_queries` | string\[] | Yes | Concise, related keyword queries of 3–6 words each. At least one is required; 2–3 diverse queries are recommended. Search operators are supported. |
| `session_id` | string | No | A stable identifier for the current conversation, up to 100 characters. Generate a UUID or 32+ character hex string once and reuse it across related `web_search` and `web_fetch` calls. It is used for free-tier rate limiting and log correlation, and is ignored for rate limiting on paid-tier keys. |
| `model_name` | string | No | The exact model identifier, up to 100 characters, read from trusted runtime or client configuration. It is used only for product analytics and does not affect search behavior. |

#### `web_fetch`

| Parameter | Type | Required | Default | Description |
| - | - | - | - | - |
| `urls` | string\[] | Yes | — | Valid HTTP or HTTPS URLs to extract, up to 20 per request. |
| `objective` | string | No | `null` | A natural-language description of the information to find in the pages, limited to 200 characters. |
| `search_queries` | string\[] | No | `null` | Keyword queries of 3–6 words each that help focus the excerpts. When fetching results from `web_search`, pass the queries used to find those URLs. |
| `full_content` | boolean | No | `false` | Return complete page markdown in addition to excerpts. Prefer excerpt mode unless the complete document is required; full content can be tens of thousands of tokens and may exceed the MCP client's output limit. |
| `session_id` | string | No | — | A stable identifier for the current conversation, up to 100 characters. Reuse the same value across related `web_search` and `web_fetch` calls. It is used for free-tier rate limiting and log correlation, and is ignored for rate limiting on paid-tier keys. |
| `model_name` | string | No | — | The exact model identifier, up to 100 characters, read from trusted runtime or client configuration. It is used only for product analytics and does not affect extraction behavior. |

<Note>
  Use a new `session_id` for each conversation. Within a conversation, changing it between calls can split free-tier rate-limit accounting and trace correlation.
</Note>

### Connection-level search overrides

Anonymous free-tier requests use `fast` mode with server-managed search settings. When your MCP request is authenticated with a Parallel API key or OAuth, you can pin Search API settings, including the mode, for every `web_search` call made through that MCP connection.

<Warning>
  Search overrides are ignored for anonymous free-tier requests. Authenticate the `/mcp` endpoint with a Bearer API key, or use `/mcp-oauth`, before adding overrides.
</Warning>

The `objective` and `search_queries` remain arguments selected by the model for each tool call. Overrides configure how those searches run; they cannot replace what the model searches for. They also do not affect `web_fetch`.

#### URL query parameters

Append fields to the MCP server URL. Use the full dotted path for nested fields:

```text theme={"system"}
https://search.parallel.ai/mcp?mode=fast&advanced_settings.location=gb&advanced_settings.source_policy.include_domains=wikipedia.org,.gov
```

For list fields such as `include_domains` and `exclude_domains`, use comma-separated values, repeat the parameter, or combine both forms. For example:

```text theme={"system"}
?advanced_settings.source_policy.include_domains=wikipedia.org,.gov&advanced_settings.source_policy.include_domains=usa.gov
```

#### Configuration header

If your MCP client supports custom headers, send the settings as a JSON object matching the Search API request shape:

```http theme={"system"}
Authorization: Bearer <PARALLEL_API_KEY>
x-parallel-search-config: {"mode":"fast","advanced_settings":{"location":"gb","max_results":5}}
```

Use query parameters for short, easily shareable configurations and the header for larger nested configurations. If both set the same field, the URL query parameter wins. Unrelated header fields are preserved. For example, if the header sets `{"mode":"turbo","advanced_settings":{"max_results":5}}` and the URL contains `?mode=fast&advanced_settings.location=gb`, the effective settings use `fast` mode, return up to five results, and target Great Britain.

#### Supported override settings

The Search MCP supports the same configurable parameters as the v1 [Search API](/search/search-quickstart), except `objective` and `search_queries`, which remain per-call tool inputs. See [Search modes](/search/modes), [Source policy](/resources/source-policy), and [Advanced Search Settings](/search/advanced-search-settings) for the available fields and constraints.

Domain/path prefixes in `include_domains` and `exclude_domains` require `fast`, `basic`, or
`advanced` mode; they are not supported in `turbo` mode.

The MCP validates these settings against the Search API request schema when the client connects. Unknown fields, invalid values, malformed header JSON, attempts to pin `objective` or `search_queries`, and unsupported mode values return an HTTP `400` error during the MCP handshake. Blank query-parameter values are ignored.

## Installation

The Search MCP can be installed in any MCP client. Two endpoints are available:

* **`https://search.parallel.ai/mcp`** — default. Free to use anonymously at lower rate limits. Pass a Parallel API key as a Bearer token (`Authorization: Bearer <key>`) to unlock higher limits. **OAuth is not advertised on this endpoint**, so clients that support OAuth sign-in (Claude Desktop, Claude.ai custom connectors, Codex, etc.) will not prompt you to log in here — use `/mcp-oauth` below if you prefer OAuth.
* **`https://search.parallel.ai/mcp-oauth`** — the OAuth-capable endpoint. Requires authentication: Bearer API key OR the OAuth flow. Anonymous requests return `401`. Use this when you want OAuth instead of managing a Bearer key, or when you need to guarantee every request is attributed to a Parallel account — for example, organization-wide deployments, Zero Data Retention (ZDR) setups, or any context where unauthenticated traffic is not acceptable.

Swap `/mcp` for `/mcp-oauth` in any of the config snippets below if you want OAuth or enforced authentication. The Search MCP can also be [used programmatically](/integrations/mcp/programmatic-use) by providing your Parallel API key in the Authorization header as a Bearer token.

### Cursor

Add to `~/.cursor/mcp.json` or `.cursor/mcp.json` (project-specific):

```json theme={"system"}
{
  "mcpServers": {
    "Parallel Search MCP": {
      "url": "https://search.parallel.ai/mcp"
    }
  }
}
```

For more details, see the [Cursor MCP documentation](https://cursor.com/docs/context/mcp).

***

### VS Code

The quickest path is the guided flow — no file editing required. Run **MCP: Add Server** from the Command Palette (⇧⌘P / Ctrl+Shift+P), pick **HTTP**, and paste the server URL:

```text theme={"system"}
https://search.parallel.ai/mcp
```

VS Code then prompts for a server name (`Parallel Search MCP`) and whether to add it as **Global** (available in every workspace) or **Workspace** (this project only), and writes the configuration for you. Confirm the trust prompt the first time the server starts.

To configure it by hand instead, add the following to `.vscode/mcp.json` in your workspace, or to the user-level `mcp.json` that the **MCP: Open User Configuration** command opens:

```json theme={"system"}
{
  "servers": {
    "Parallel Search MCP": {
      "type": "http",
      "url": "https://search.parallel.ai/mcp"
    }
  }
}
```

<Warning>
  The user-level `mcp.json` lives inside your VS Code profile folder, **not** at `~/.vscode/mcp.json`. Open it with **MCP: Open User Configuration** rather than typing the path — a file you create at `~/.vscode/mcp.json` is ignored, with no error to tell you why.
</Warning>

For more details, see the [VS Code MCP documentation](https://code.visualstudio.com/docs/copilot/customization/mcp-servers).

***

### Claude Desktop / Claude.ai

Go to Settings → Connectors → Add Custom Connector, and fill in:

```text theme={"system"}
Name: Parallel Search MCP
URL: https://search.parallel.ai/mcp
```

This URL connects anonymously. If you'd rather sign in with OAuth (so Claude manages the token for you), use `https://search.parallel.ai/mcp-oauth` instead — it triggers Claude's OAuth sign-in flow.

If you are part of an organization, you may not have access to custom connectors. Contact your organization administrator for assistance.

<Tip>
  **Org admins (Team / Enterprise plans):** to route Claude's web queries through the Parallel Search MCP instead of Claude's built-in web search, disable the native web search tool at [**Admin settings → Capabilities**](https://claude.ai/admin-settings/capabilities). Once off at the workspace level, members cannot re-enable it individually. See the [Claude web search docs](https://support.claude.com/en/articles/10684626-enabling-and-using-web-search).

  For the org-wide deployment, also point the connector at the [`/mcp-oauth` endpoint](#installation) (not `/mcp`) so every request requires a Parallel API key or OAuth — useful when you need every call attributed to the organization.
</Tip>

If you are not an admin, go to Settings → Developer → Edit Config and use the following JSON. If your config already has an `mcpServers` object, add the `Parallel Search MCP` entry inside it without replacing your other servers.

```json theme={"system"}
{
  "mcpServers": {
    "Parallel Search MCP": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-remote",
        "https://search.parallel.ai/mcp"
      ]
    }
  }
}
```

No API key is required for the Search MCP. To unlock higher rate limits, grab a key from [Platform](https://platform.parallel.ai) and append `"--header", "authorization: Bearer YOUR-PARALLEL-API-KEY"` to the `args` array.

For more details, see the [Claude remote MCP documentation](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

***

### Claude Code

Run this command in your terminal:

```bash theme={"system"}
claude mcp add --transport http "Parallel-Search-MCP" https://search.parallel.ai/mcp
```

In Claude code, use the command:

```
/mcp
```

Then follow the steps in your browser to login.

For more details, see the [Claude Code MCP documentation](https://code.claude.com/docs/en/mcp#authenticate-with-remote-mcp-servers).

***

### Codex CLI

Run one of the following, depending on how you want to authenticate:

```bash Free, anonymous theme={"system"}
codex mcp add parallel-search --url https://search.parallel.ai/mcp
```

```bash Higher rate limits via API key theme={"system"}
# set PARALLEL_API_KEY in your environment first (key from platform.parallel.ai)
codex mcp add parallel-search --url https://search.parallel.ai/mcp \
  --bearer-token-env-var PARALLEL_API_KEY
```

```bash Higher rate limits via OAuth theme={"system"}
codex mcp add parallel-search --url https://search.parallel.ai/mcp-oauth
# complete the browser authorization when prompted
```

Each command writes the equivalent `[mcp_servers.parallel-search]` entry to `~/.codex/config.toml` — you can also hand-edit it if you prefer.

Restart Codex after adding the server. For more details, see the [Codex MCP documentation](https://developers.openai.com/codex/mcp/).

***

### fx (Vercel)

[fx](https://github.com/vercel-labs/fx) connects to the Search MCP through `mcp-remote`. This setup requires Node.js and `npx`. Run this command inside fx:

```text theme={"system"}
/mcp add parallel-search npx -y mcp-remote https://search.parallel.ai/mcp
```

fx saves the server to `~/.fx/mcp.json` without removing your existing MCP servers and reloads the configuration. Run `/mcp list` to confirm `parallel-search` shows `state=ready`.

For higher rate limits, get an API key from [platform.parallel.ai](https://platform.parallel.ai) and export `PARALLEL_API_KEY` before launching fx. Add `"--header"` and `"Authorization: Bearer ${PARALLEL_API_KEY}"` to the `parallel-search` command array in `~/.fx/mcp.json`, then run `/mcp reload`.

For more details, see the [fx MCP documentation](https://fx.sh/docs/capabilities/mcp).

***

### Other Clients

<AccordionGroup>
  <Accordion title="Windsurf">
    Add to `~/.codeium/windsurf/mcp_config.json`:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "serverUrl": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    For more details, see the [Windsurf MCP documentation](https://docs.windsurf.com/windsurf/cascade/mcp).
  </Accordion>

  <Accordion title="Cline">
    Go to MCP Servers → Remote Servers → Edit Configuration:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "url": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    For more details, see the [Cline MCP documentation](https://docs.cline.bot/mcp/adding-and-configuring-servers).
  </Accordion>

  <Accordion title="Gemini CLI">
    Add to `~/.gemini/settings.json`:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "httpUrl": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    For more details, see the [Gemini CLI MCP documentation](https://geminicli.com/docs/tools/mcp-server/).
  </Accordion>

  <Accordion title="ChatGPT">
    **Warning:** [Developer Mode](https://platform.openai.com/docs/guides/developer-mode) must be enabled, and this feature may not be available to everyone. MCPs in ChatGPT are experimental and may not work reliably.

    First, go to Settings → Connectors → Advanced Settings, and turn on Developer Mode.

    Then, in connector settings, click Create and fill in:

    ```text theme={"system"}
    Name: Parallel Search MCP
    URL: https://search.parallel.ai/mcp
    Authentication: OAuth
    ```

    In a new chat, ensure Developer Mode is turned on with the connector(s) selected.

    For more details, see the [ChatGPT Developer Mode documentation](https://help.openai.com/en/articles/12584461-developer-mode-apps-and-full-mcp-connectors-in-chatgpt-beta).
  </Accordion>

  <Accordion title="Amp">
    Run this command in your terminal:

    ```bash theme={"system"}
    amp mcp add "Parallel-Search-MCP" https://search.parallel.ai/mcp
    ```

    The OAuth flow will start when you start Amp.

    For more details, see the [Amp MCP documentation](https://ampcode.com/manual#mcp-oauth).
  </Accordion>

  <Accordion title="Kiro">
    Add to `.kiro/settings/mcp.json` (workspace) or `~/.kiro/settings/mcp.json` (global):

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "url": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    For more details, see the [Kiro MCP documentation](https://kiro.dev/docs/mcp/configuration/).
  </Accordion>

  <Accordion title="Google Antigravity">
    In the Antigravity Agent pane, click the menu (⋮) → MCP Servers → Manage MCP Servers → View raw config, then add:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel-Search-MCP": {
          "serverUrl": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    For higher rate limits, add a `headers` block with `"Authorization": "Bearer YOUR_API_KEY"` and your key from [platform.parallel.ai](https://platform.parallel.ai).

    For more details, see the [Antigravity MCP documentation](https://antigravity.google/docs/mcp).
  </Accordion>

  <Accordion title="OpenCode">
    Add to `opencode.json` (project) or `~/.config/opencode/opencode.json` (global):

    ```json theme={"system"}
    {
      "mcp": {
        "parallel-search": {
          "type": "remote",
          "url": "https://search.parallel.ai/mcp",
          "enabled": true
        }
      }
    }
    ```

    For more details, see the [OpenCode MCP documentation](https://opencode.ai/docs/mcp-servers/).
  </Accordion>

  <Accordion title="Roo Code">
    Add to `.roo/mcp.json` in your workspace (or edit the global `mcp_settings.json` from the Roo Code MCP settings view):

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "type": "streamable-http",
          "url": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    Project config takes precedence over global. For more details, see the [Roo Code MCP documentation](https://docs.roocode.com/features/mcp/using-mcp-in-roo).
  </Accordion>

  <Accordion title="OpenHands">
    Run this command in your terminal:

    ```bash theme={"system"}
    openhands mcp add parallel-search --transport http https://search.parallel.ai/mcp
    ```

    Or add manually to `~/.openhands/mcp.json`:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "url": "https://search.parallel.ai/mcp"
        }
      }
    }
    ```

    Inside an OpenHands conversation, use `/mcp` to verify the server is active. For more details, see the [OpenHands MCP documentation](https://docs.openhands.dev/openhands/usage/cli/mcp-servers).
  </Accordion>

  <Accordion title="Factory CLI (droid)">
    Run this command in your terminal:

    ```bash theme={"system"}
    droid mcp add parallel-search https://search.parallel.ai/mcp --type http
    ```

    Inside droid, type `/mcp` to open the interactive manager and verify the server is connected. For more details, see the [Factory MCP documentation](https://docs.factory.ai/cli/configuration/mcp).
  </Accordion>

  <Accordion title="Pi">
    Pi ships without built-in MCP support, so install the [`pi-mcp-adapter`](https://github.com/nicobailon/pi-mcp-adapter) package first:

    ```bash theme={"system"}
    pi install npm:pi-mcp-adapter
    ```

    Restart Pi, then add the Search MCP to `.mcp.json` in your project (or `~/.config/mcp/mcp.json` for a user-global config):

    ```json theme={"system"}
    {
      "mcpServers": {
        "parallel-search": {
          "url": "https://search.parallel.ai/mcp",
          "directTools": true
        }
      }
    }
    ```

    `directTools: true` registers `web_search` and `web_fetch` alongside Pi's built-in tools instead of hiding them behind the adapter's `mcp` proxy. Run `/mcp` inside Pi to verify the server is detected. For more details, see the [pi-mcp-adapter README](https://github.com/nicobailon/pi-mcp-adapter).

    Alternatively, skip MCP entirely and use the [Parallel CLI](/integrations/cli) as a Pi skill — that's the same pattern as our [ClawHub / OpenClaw](/integrations/clawhub) integration.
  </Accordion>

  <Accordion title="OpenClaw">
    OpenClaw stores MCP server definitions in `~/.openclaw/openclaw.json` under `mcp.servers`. Save the Search MCP via the `openclaw mcp set` CLI:

    ```bash theme={"system"}
    openclaw mcp set parallel-search '{"url":"https://search.parallel.ai/mcp","transport":"streamable-http"}'
    ```

    Or edit `~/.openclaw/openclaw.json` directly:

    ```json theme={"system"}
    {
      "mcp": {
        "servers": {
          "parallel-search": {
            "url": "https://search.parallel.ai/mcp",
            "transport": "streamable-http"
          }
        }
      }
    }
    ```

    The `transport` field is required — OpenClaw defaults to SSE when it's omitted, but the Search MCP uses Streamable HTTP. No API key is required; for higher rate limits, add an optional `headers` map with `"Authorization": "Bearer YOUR-PARALLEL-API-KEY"` (key from [platform.parallel.ai](https://platform.parallel.ai)).

    `openclaw mcp set` only writes to config — it doesn't connect to the server or reload running agents. Start a new OpenClaw agent session (or restart your current one) for the tools to show up. Verify the saved definition with `openclaw mcp list` / `openclaw mcp show parallel-search`. For the full MCP CLI reference, see the [OpenClaw MCP docs](https://docs.openclaw.ai/cli/mcp).

    You can also skip MCP entirely and use the [Parallel CLI](/integrations/cli) as an OpenClaw skill — see the [ClawHub integration](/integrations/clawhub).
  </Accordion>

  <Accordion title="Hermes Agent">
    Add to `~/.hermes/config.yaml`:

    ```yaml theme={"system"}
    mcp_servers:
      parallel_search:
        url: "https://search.parallel.ai/mcp"
    ```

    For more details, see the [Hermes Agent MCP documentation](https://hermes-agent.nousresearch.com/docs/user-guide/features/mcp).
  </Accordion>

  <Accordion title="Continue.dev">
    Add a file at `.continue/mcpServers/parallel-search.yaml` in your workspace:

    ```yaml theme={"system"}
    name: Parallel Search MCP
    version: 0.0.1
    schema: v1
    mcpServers:
      - name: Parallel Search MCP
        type: streamable-http
        url: https://search.parallel.ai/mcp
    ```

    For more details, see the [Continue.dev MCP documentation](https://docs.continue.dev/customize/deep-dives/mcp).
  </Accordion>

  <Accordion title="Zed, Warp, Raycast (stdio-only clients)">
    These clients currently support only stdio-transport MCP servers — they can't connect directly to remote HTTP endpoints. Wrap the Search MCP with [`mcp-remote`](https://www.npmjs.com/package/mcp-remote) to proxy it through a local stdio process:

    ```json theme={"system"}
    {
      "mcpServers": {
        "Parallel Search MCP": {
          "command": "npx",
          "args": ["-y", "mcp-remote", "https://search.parallel.ai/mcp"]
        }
      }
    }
    ```

    Add an `--header "Authorization: Bearer YOUR-PARALLEL-API-KEY"` argument if you're hitting `/mcp-oauth` or want higher rate limits.

    Placement differs per client — see the [Zed MCP docs](https://zed.dev/docs/ai/mcp), [Warp MCP docs](https://docs.warp.dev/university/how-warp-uses-warp/using-mcp-servers-with-warp), or [Raycast MCP docs](https://manual.raycast.com/ai/model-context-protocol) for the exact file location and wrapper format.
  </Accordion>
</AccordionGroup>

If your client isn't listed, most MCP clients accept the `mcpServers` format shown in the Cursor section. For clients that use a different wrapper (e.g., VS Code's top-level `servers`, Windsurf's `serverUrl`), check the client's documentation for the correct field names.

## Best practices

### Filtering by date, domain, or path

For flexible date, domain, or path preferences, include the constraints directly in the search query or objective. This gives the model room to find relevant sources without unnecessarily excluding the rest of the web. For example:

* "Latest AI research papers from 2026"
* "News about climate change from nytimes.com"
* "Product announcements from apple.com in the last month"

If your authenticated deployment requires a hard constraint, pin `advanced_settings.source_policy.after_date`, `advanced_settings.source_policy.include_domains`, or `advanced_settings.source_policy.exclude_domains` using [Search MCP overrides](#configure-search-behavior). Because the configuration applies to every `web_search` call on that connection, reserve hard filters for requirements that should apply throughout the session.

<Warning>
  Source-policy fields are hard filters and can significantly reduce result quality. Prefer natural-language guidance unless your application must include or exclude specific domains or paths.
</Warning>

## Troubleshooting

### Common Installation Issues

<Accordion title="Cline: 'Authorization Error redirect_uri must be https'">
  This error occurs when Cline attempts OAuth authentication but the redirect URI isn't using HTTPS. The Search MCP at `/mcp` is free and doesn't require OAuth in the first place — wrap it in `mcp-remote` so Cline treats it as a plain stdio server:

  ```json theme={"system"}
  {
    "mcpServers": {
      "Parallel Search MCP": {
        "command": "npx",
        "args": [
          "-y",
          "mcp-remote",
          "https://search.parallel.ai/mcp"
        ]
      }
    }
  }
  ```

  If you want higher rate limits (or you're specifically targeting `/mcp-oauth`), append `"--header", "authorization: Bearer YOUR-PARALLEL-API-KEY"` to the `args` array with a key from [platform.parallel.ai](https://platform.parallel.ai).
</Accordion>

<Accordion title="Gemini CLI: Where to provide API key">
  Gemini CLI uses HTTP MCPs and can authenticate via OAuth. The Search MCP at `/mcp` doesn't require an API key or OAuth, so the simplest setup has no auth at all:

  ```json theme={"system"}
  {
    "mcpServers": {
      "Parallel Search MCP": {
        "httpUrl": "https://search.parallel.ai/mcp"
      }
    }
  }
  ```

  Add this to `~/.gemini/settings.json`. If you want higher rate limits (or to target `/mcp-oauth`), swap in the `mcp-remote` wrapper and pass a key from [platform.parallel.ai](https://platform.parallel.ai) as a Bearer header:

  ```json theme={"system"}
  {
    "mcpServers": {
      "Parallel Search MCP": {
        "command": "npx",
        "args": [
          "-y",
          "mcp-remote",
          "https://search.parallel.ai/mcp",
          "--header",
          "authorization: Bearer YOUR-PARALLEL-API-KEY"
        ]
      }
    }
  }
  ```
</Accordion>

<Accordion title="VS Code: Server not detected (wrong file location)">
  VS Code reads two `mcp.json` locations: `.vscode/mcp.json` inside the workspace, and a user-level `mcp.json` in your VS Code profile folder, which the **MCP: Open User Configuration** command opens. `~/.vscode/mcp.json` is **not** one of them — that directory holds `argv.json` and your installed extensions, so a server defined there never loads and VS Code reports no error.

  If a hand-written config isn't picked up, run **MCP: Open User Configuration** and check whether the file that opens is the one you edited. Run **MCP: List Servers** to confirm VS Code sees the server, and use **MCP: Add Server** to let VS Code write the entry to the correct file.
</Accordion>

<Accordion title="VS Code: Incorrect configuration format">
  VS Code's `mcp.json` uses a different structure than Cursor's `mcp.json`. Common mistake: copying a Cursor-style config into VS Code.

  **Incorrect (Cursor format):**

  ```json theme={"system"}
  {
    "mcpServers": {
      "parallel-search": {
        "url": "https://search.parallel.ai/mcp"
      }
    }
  }
  ```

  **Correct (VS Code format, `.vscode/mcp.json` or user-level `mcp.json`):**

  ```json theme={"system"}
  {
    "servers": {
      "Parallel Search MCP": {
        "type": "http",
        "url": "https://search.parallel.ai/mcp"
      }
    }
  }
  ```

  Note: VS Code uses a top-level `servers` object (not `mcpServers`) and includes `type: "http"` for remote HTTP servers.
</Accordion>

<Accordion title="Windsurf: Configuration location and format">
  Windsurf uses a different configuration format than Cursor.

  **Correct Windsurf configuration:**

  ```json theme={"system"}
  {
    "mcpServers": {
      "Parallel Search MCP": {
        "serverUrl": "https://search.parallel.ai/mcp"
      }
    }
  }
  ```

  Note: Windsurf uses `serverUrl` instead of `url`. Add this to your Windsurf MCP configuration file.
</Accordion>

<Accordion title="Connection timeout or 'server unavailable' errors">
  If you're getting connection errors:

  1. **Check your network:** Ensure you can reach `https://search.parallel.ai`
  2. **Verify API key:** Make sure your key is valid at [platform.parallel.ai](https://platform.parallel.ai)
  3. **Check balance:** A 402 error means insufficient credits—add funds to your account
  4. **Restart your IDE:** Some clients cache MCP connections
</Accordion>

<Accordion title="Tools not appearing in the IDE">
  If the MCP installs but tools don't show up:

  1. **Restart your IDE** completely (not just reload)
  2. **Check configuration syntax:** Ensure valid JSON with no trailing commas
  3. **Verify the server URL:** Must be exactly `https://search.parallel.ai/mcp`
  4. **Check IDE logs:** Look for MCP-related errors in your IDE's output/debug panel
</Accordion>
