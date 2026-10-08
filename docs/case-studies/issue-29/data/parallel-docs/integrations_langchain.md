> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# LangChain

> LangChain integrations for Parallel, enabling real-time web research and AI capabilities

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Add Parallel's search and extract tools to your LangChain applications.

<Tip> View the complete repository for this integration [here](https://github.com/parallel-web/langchain-parallel) </Tip>

## Features

* **Search Tools**: `ParallelWebSearchTool` - Direct access to Parallel's Search API
* **Extract Tools**: `ParallelExtractTool` - Clean content extraction from web pages
* **Async/Await**: Full asynchronous operation support
* **LangChain Integration**: Seamless integration with LangChain ecosystem

## Installation

```bash theme={"system"}
pip install langchain-parallel
```

## Setup

1. Get your API key from [Parallel](https://platform.parallel.ai)
2. Set your API key as an environment variable:

```bash theme={"system"}
export PARALLEL_API_KEY="your-api-key-here"
```

## Search API

The Search API provides direct access to Parallel's web search capabilities, returning structured, compressed excerpts optimized for LLM consumption.

### ParallelWebSearchTool

The search tool provides direct access to Parallel's Search API:

```python theme={"system"}
from langchain_parallel import ParallelWebSearchTool

# Initialize the search tool
search_tool = ParallelWebSearchTool()

# Search with queries and an objective
result = search_tool.invoke({
    "search_queries": [
        "renewable energy technology developments 2026",
        "renewable energy policy updates 2026"
    ],
    "objective": "What are the latest developments in renewable energy?",
    "mode": "advanced"
})

print(result)
# {
#     "search_id": "search_123...",
#     "results": [
#         {
#             "url": "https://example.com/renewable-energy",
#             "title": "Latest Renewable Energy Developments",
#             "excerpts": [
#                 "Solar energy has seen remarkable growth...",
#                 "Wind power capacity increased by 15%..."
#             ]
#         }
#     ]
# }
```

#### Search API Configuration

| Parameter | Type | Default | Description |
| - | - | - | - |
| `objective` | Optional\[str] | None | Natural-language description of research goal |
| `search_queries` | List\[str] | Required | Specific search queries (1-5 queries, 200 chars each) |
| `max_results` | int | 10 | Maximum requested results. The Search API caps results at 20 and reduces higher values with a warning |
| `excerpts` | Optional\[dict] | None | Excerpt settings (e.g., `{'max_chars_per_result': 1500}`) |
| `mode` | Optional\[str] | None | Search mode: 'turbo' for the lowest latency and cost, 'fast' for high-quality search in around 700ms (recommended for most agents), 'basic' for extended snippets per result, 'advanced' (default) for the highest-quality results |
| `fetch_policy` | Optional\[dict] | None | Policy for cached vs live content (e.g., `{'max_age_seconds': 86400, 'timeout_seconds': 60}`) |
| `api_key` | Optional\[SecretStr] | None | API key (uses env var if not provided) |
| `base_url` | str | `"https://api.parallel.ai"` | API base URL |

<Note>
  `turbo` mode requires `langchain-parallel` v0.5.0 or later.
</Note>

#### Search with Specific Queries

You can provide specific search queries instead of an objective:

```python theme={"system"}
# Search with specific queries
result = search_tool.invoke({
    "search_queries": [
        "renewable energy 2024",
        "solar power developments",
        "wind energy statistics"
    ],
    "mode": "advanced"
})
```

#### Tool Usage in Agents

The search tool works seamlessly with LangChain agents. Pair it with any LangChain-compatible chat model:

```python theme={"system"}
from langchain.agents import create_openai_functions_agent, AgentExecutor
from langchain_core.prompts import ChatPromptTemplate
from langchain_openai import ChatOpenAI

# Bring your own LLM
llm = ChatOpenAI(model="gpt-6-astra")

# Create agent with search capabilities
tools = [search_tool]

prompt = ChatPromptTemplate.from_messages([
    ("system", "You are a research assistant. Use the search tool to find current information."),
    ("human", "{input}"),
    ("placeholder", "{agent_scratchpad}"),
])

agent = create_openai_functions_agent(llm, tools, prompt)
agent_executor = AgentExecutor(agent=agent, tools=tools)

# Run the agent
result = agent_executor.invoke({
    "input": "What are the latest developments in artificial intelligence?"
})
print(result["output"])
```

## Extract API

The Extract API provides clean content extraction from web pages, returning structured markdown-formatted content optimized for LLM consumption.

### ParallelExtractTool

The extract tool extracts clean, structured content from web pages:

```python theme={"system"}
from langchain_parallel import ParallelExtractTool

# Initialize the extract tool
extract_tool = ParallelExtractTool()

# Extract from a single URL
result = extract_tool.invoke({
    "urls": ["https://en.wikipedia.org/wiki/Artificial_intelligence"]
})

print(result)
# [
#     {
#         "url": "https://en.wikipedia.org/wiki/Artificial_intelligence",
#         "title": "Artificial intelligence - Wikipedia",
#         "content": "# Artificial intelligence\n\nMain content in markdown...",
#         "publish_date": "2024-01-15"  # Optional
#     }
# ]
```

#### Extract with Search Objective and Advanced Options

Focus extraction on specific topics using search objectives, with control over excerpts and fetch policy:

```python theme={"system"}
# Extract content focused on a specific objective with excerpt settings
result = extract_tool.invoke({
    "urls": ["https://en.wikipedia.org/wiki/Artificial_intelligence"],
    "search_objective": "What are the main applications and ethical concerns of AI?",
    "excerpts": {"max_chars_per_result": 2000},
    "full_content": False,
    "fetch_policy": {
        "max_age_seconds": 86400,
        "timeout_seconds": 60,
        "disable_cache_fallback": False
    }
})

# Returns relevant excerpts focused on the objective
print(result[0]["excerpts"])  # List of relevant text excerpts
```

#### Extract with Search Queries

Extract content relevant to specific search queries:

```python theme={"system"}
# Extract content focused on specific queries
result = extract_tool.invoke({
    "urls": [
        "https://en.wikipedia.org/wiki/Machine_learning",
        "https://en.wikipedia.org/wiki/Deep_learning"
    ],
    "search_queries": ["neural networks", "training algorithms", "applications"],
    "excerpts": True
})

for item in result:
    print(f"Title: {item['title']}")
    print(f"Relevant excerpts: {len(item['excerpts'])}")
    print()
```

#### Content Length Control

```python theme={"system"}
# Control full content length per extraction
result = extract_tool.invoke({
    "urls": ["https://en.wikipedia.org/wiki/Quantum_computing"],
    "full_content": {"max_chars_per_result": 3000}
})

print(f"Content length: {len(result[0]['content'])} characters")
```

#### Extract API Configuration

| Parameter | Type | Default | Description |
| - | - | - | - |
| `urls` | List\[str] | Required | List of URLs to extract content from |
| `search_objective` | Optional\[str] | None | Natural language objective to focus extraction |
| `search_queries` | Optional\[List\[str]] | None | Specific keyword queries to focus extraction |
| `excerpts` | Union\[bool, ExcerptSettings] | True | Include relevant excerpts (focused on objective/queries if provided) |
| `full_content` | Union\[bool, FullContentSettings] | False | Include full page content |
| `fetch_policy` | Optional\[FetchPolicy] | None | Cache vs live content policy |
| `max_chars_per_extract` | Optional\[int] | None | Maximum characters per extraction (tool-level setting) |
| `api_key` | Optional\[SecretStr] | None | API key (uses env var if not provided) |
| `base_url` | str | `"https://api.parallel.ai"` | API base URL |

#### Extract Error Handling

The extract tool gracefully handles failed extractions:

```python theme={"system"}
# Mix of valid and invalid URLs
result = extract_tool.invoke({
    "urls": [
        "https://en.wikipedia.org/wiki/Python_(programming_language)",
        "https://this-domain-does-not-exist-12345.com/"
    ]
})

for item in result:
    if "error_type" in item:
        print(f"Failed: {item['url']} - {item['content']}")
    else:
        print(f"Success: {item['url']} - {len(item['content'])} chars")
```

#### Async Extract

```python theme={"system"}
import asyncio

async def extract_async():
    result = await extract_tool.ainvoke({
        "urls": ["https://en.wikipedia.org/wiki/Artificial_intelligence"]
    })
    return result

# Run async extraction
result = asyncio.run(extract_async())
```

## Error Handling

```python theme={"system"}
try:
    result = search_tool.invoke({
        "search_queries": ["renewable energy 2026"],
        "mode": "advanced",
    })
    print(result)
except ValueError as e:
    if "API key not found" in str(e):
        print("Please set your PARALLEL_API_KEY environment variable")
    else:
        print(f"API Error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

## Examples

See the `docs/` directory for complete working examples:

* `docs/search_tool.ipynb` - Search tool examples and tutorials
* `docs/extract_tool.ipynb` - Extract tool examples and tutorials

Examples include:

* Basic synchronous usage
* Async operations
* Tool usage in agents
