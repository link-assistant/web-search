> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# LiteLLM

> Use Parallel Search with the LiteLLM Python SDK and AI Gateway

[LiteLLM](https://www.litellm.ai) supports two separate ways to use Parallel: its native Search provider or a connection to the [Parallel Search MCP](/integrations/mcp/search-mcp) through the LiteLLM MCP Gateway. The native Search provider does not require MCP.

## Native LiteLLM Search provider

Install LiteLLM and set your Parallel API key:

```bash theme={"system"}
pip install litellm
export PARALLEL_API_KEY="your-parallel-api-key"
```

### Python SDK

Use `search_provider="parallel_ai"` to call Parallel Search through the LiteLLM Python SDK:

```python theme={"system"}
from litellm import search

response = search(
    query="latest AI developments",
    search_provider="parallel_ai",
    max_results=5,
)

for result in response.results:
    print(result.title, result.url, result.snippet)
```

For more information, see LiteLLM's [Parallel Search provider documentation](https://docs.litellm.ai/docs/search/parallel_ai).

### LiteLLM AI Gateway

Install the proxy dependencies and set a separate master key to authenticate requests to your local LiteLLM gateway:

```bash theme={"system"}
pip install 'litellm[proxy]'
export LITELLM_MASTER_KEY="sk-your-local-proxy-key"
```

Create a `config.yaml` file that explicitly registers `parallel-search`:

```yaml theme={"system"}
search_tools:
  - search_tool_name: parallel-search
    litellm_params:
      search_provider: parallel_ai
      api_key: os.environ/PARALLEL_API_KEY

general_settings:
  master_key: os.environ/LITELLM_MASTER_KEY
```

Start the gateway:

```bash theme={"system"}
litellm --config config.yaml --host 127.0.0.1
```

In another terminal, set the same `LITELLM_MASTER_KEY` and verify the tool is registered without making a Parallel Search request:

```bash theme={"system"}
export LITELLM_MASTER_KEY="sk-your-local-proxy-key"

curl http://localhost:4000/v1/search/tools \
  -H "Authorization: Bearer $LITELLM_MASTER_KEY"
```

To run a real search, call the explicitly named `parallel-search` endpoint:

```bash theme={"system"}
curl http://localhost:4000/v1/search/parallel-search \
  -H "Authorization: Bearer $LITELLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{"query":"latest AI developments","max_results":5}'
```

## Parallel Search MCP through LiteLLM MCP Gateway

The LiteLLM MCP Gateway can also connect to Parallel Search MCP. This is separate from the native Search provider and exposes the `web_search` and `web_fetch` MCP tools.

Add the server to your existing LiteLLM gateway configuration with HTTP transport:

```yaml theme={"system"}
mcp_servers:
  parallel_search:
    url: https://search.parallel.ai/mcp
    transport: http
    # Optional, for higher rate limits:
    # auth_type: bearer_token
    # auth_value: os.environ/PARALLEL_API_KEY
```

The connection is anonymous by default. To enable optional Bearer authentication for higher rate limits, set `PARALLEL_API_KEY` and uncomment both authentication settings.

For more information, see LiteLLM's [MCP Gateway documentation](https://docs.litellm.ai/docs/mcp) and the [Parallel Search MCP guide](/integrations/mcp/search-mcp).
