> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# MCP Tool Calling

> Using MCP servers for tool calls in Tasks

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## Overview

The Parallel API allows you to specify remote MCP servers for Task API execution. This enables the model to access tools hosted on remote MCP servers without needing a separate MCP client.

<Note>
  This page covers MCP servers you supply, including providers your organization already licenses. For Data Connectors that Parallel manages, see [Data Connectors](/resources/data-connectors).
</Note>

### Specifying MCP Servers

MCP servers are specified using the `mcp_servers` field in the Task API call. Each request can include up to 10 MCP servers.

| Parameter | Type | Description |
| - | - | - |
| `type` | `string` | Always `url`. |
| `url` | `string` | The URL of the MCP server. |
| `name` | `string` | A name for the MCP server. |
| `headers` | `dict[string, string]` | Headers for authenticating with the MCP server. |
| `allowed_tools` | `array[string]` or `null` | List of tools to allow, or null for all. |

#### Sample Request

<CodeGroup>
  ```bash Task API theme={"system"}
  curl -X POST "https://api.parallel.ai/v1/tasks/runs" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    --data '{
    "input": "What is the latest in AI research?",
    "processor": "lite",
    "mcp_servers": [
      {
          "type": "url",
          "url": "https://dummy_mcp_server",
          "name": "dummy_mcp_server",
          "headers": {"x-api-key": "API_KEY"}
      }
    ]
  }'
  ```
</CodeGroup>

#### Restrictions

* Only MCP servers with Streamable HTTP transport are currently supported.
* From the [MCP specification](https://modelcontextprotocol.io/specification/2025-03-26), only tools are supported.
* For [MCP servers using OAuth](https://modelcontextprotocol.io/specification/draft/basic/authorization), you must generate the authorization token separately and include it as a bearer token in the headers.
* You can specify up to 10 MCP servers per request, but using fewer is recommended for optimal result quality.

## Using MCP Servers in the Task API

When you make a Task API request, the API first fetches the available tools from the specified MCP servers.
The processor will invoke tools from these servers if it determines they are useful for the task. The number of tool calls depends
on the [processor](/task-api/guides/choose-a-processor):

* For `lite` and `core`, at most one tool is invoked.
* For all other processors, multiple tool calls may be made.

## Response Content

The Task API response includes a list of tool calls made during execution. Each tool call entry contains:

| Parameter | Type | Description |
| - | - | - |
| `tool_call_id` | `string` | Unique identifier for the tool call. |
| `server_name` | `string` | Name of the MCP server, as provided in the input. |
| `tool_name` | `string` | Name of the tool invoked. |
| `arguments` | `string` | JSON-encoded string of the arguments used for the tool call. |
| `content` | `string` | Response from the MCP server. |
| `error` | `string` | Error message if the tool call failed. Either `content` or `error` will always be populated. |

If there is an authentication issue with any MCP server, the top-level `warning` field in the Task Run output
will be populated.

<CodeGroup>
  ```bash Success theme={"system"}
  {
    "run": {
      "run_id": "trun_0cb15e174bc44a019a58b8a02ebefe54",
      "interaction_id": "trun_0cb15e174bc44a019a58b8a02ebefe54",
      "status": "completed",
      "is_active": false,
      "processor": "lite",
      "metadata": {},
      "created_at": "2026-09-30T18:37:31.805687Z",
      "modified_at": "2026-09-30T18:38:49.550141Z"
    },
    "output": {
      "basis": [
        {
          "field": "output",
          "citations": [
            {
              "title": "Reimagining research papers as interactive and reliable AI agents - PubMed",
              "url": "https://pubmed.ncbi.nlm.nih.gov/42749808",
              "excerpts": [
                "Paper2Agent transforms research output from passive artefacts into active systems that accelerate use and discovery. ... Paper2Agent addresses this challenge by converting a paper into an AI agent ..."
              ]
            }
          ],
          "reasoning": "The PubMed abstract describes Paper2Agent as turning papers into active AI agents, including access to research materials and the ability to answer natural-language questions while invoking paper workflows. Stanford Medicine’s September 16 report describes ...",
          "confidence": "medium"
        }
      ],
      "type": "json",
      "mcp_tool_calls": [
        {
          "tool_call_id": "call_rrlBJ4VpZYl7VkYqosVkGKjD",
          "server_name": "parallel_web_search",
          "tool_name": "web_search",
          "arguments": "{\"objective\": \"Identify notable, very recent AI research findings and publications available as of September 30, 2026.\", \"search_queries\": [\"AI research breakthroughs September 2026\", \"latest AI research papers September 2026 arXiv\", \"AI lab research announcements September 2026\"]}",
          "content": "{\n  \"search_id\": \"search_cc65ce8e28c50dff38b30b57e79e1230\",\n  \"results\": ...}",
          "error": ""
        },
        {
          "tool_call_id": "call_8XO1rg0mLd1J8TntcykQJoa9",
          "server_name": "parallel_web_search",
          "tool_name": "web_search",
          "arguments": "{\"objective\": \"Find primary-source AI research papers or lab research announcements published in the last week of September 2026.\", \"search_queries\": [\"AI research paper September 29 2026\", \"AI research announcement September 2026 lab\", \"AI science breakthrough September 2026\"]}",
          "content": "{\n  \"search_id\": \"search_f27ffd70902ddbf1922b28fe9442c224\",\n  \"results\": ...}",
          "error": ""
        }
      ],
      "content": {
        "output": "Notable recent AI-research developments in September 2026:\n- **Paper2Agent:** A framework that turns scientific papers into interactive AI agents, with access ..."
      },
      "output_schema": {}
    }
  }
  ```

  ```bash Failure authenticating to MCP server theme={"system"}
  {
    "run": {
      "run_id": "trun_0cb15e174bc44a01a523e494f20f0a3a",
      "interaction_id": "trun_0cb15e174bc44a01a523e494f20f0a3a",
      "status": "completed",
      "is_active": false,
      "warnings": [
        {
          "type": "warning",
          "message": "Error listing tools from MCP server dummy_mcp_server: 502 Bad gateway. Server details: Name: dummy_mcp_server, URL: https://dummy_mcp_server, Headers: {\"x-api-key\": \"REDACTED\"}. Reference ID: 5e87a738-16e4-43a1-ba35-3787d0a75d1d",
          "detail": {}
        }
      ],
      "processor": "lite",
      "metadata": {},
      "created_at": "2026-09-30T18:36:02.302876Z",
      "modified_at": "2026-09-30T18:36:37.756581Z"
    },
    "output": {
      "basis": [
        {
          "field": "output",
          "citations": [
            {
              "title": "AI system helps lab devices 'talk' with each other — streamlining research | Nature",
              "url": "https://www.nature.com/articles/d41586-026-02990-8",
              "excerpts": [
                "nature news article NEWS 24 September 2026 AI system helps lab devices 'talk' with each other — streamlining research Platform allows disparate machines to communicate and to be controlled by an ..."
              ]
            }
          ],
          "reasoning": "The cited Nature passage identifies the September 24 lab-device platform and its AI-agent functions. The Paper2Agent passage describes making research papers and their materials usable through agents and workflows. The scientific-ideas study reports both the ...",
          "confidence": "medium"
        }
      ],
      "type": "json",
      "content": {
        "output": "As of September 30, 2026, notable recent directions in AI research include:\n\n- **AI-enabled lab automation:** A September 24 report described a system ..."
      },
      "output_schema": {}
    }
  }
  ```
</CodeGroup>
