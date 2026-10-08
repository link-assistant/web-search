"""Build exhaustive source/operation coverage and the issue dependency inventory."""
import importlib.util
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
CASE=ROOT/'docs/case-studies/issue-29'
spec=importlib.util.spec_from_file_location('filing',Path(__file__).with_name('file-issues.py'))
filing=importlib.util.module_from_spec(spec); spec.loader.exec_module(filing)
drafts=filing.load_drafts(CASE/'issues'); order=filing.ordered_ids(drafts)
manifest=json.loads((CASE/'data/parallel-docs/manifest.json').read_text())

EXACT={
'getting-started/pricing':['WS-34'], 'getting-started/rate-limits':['WS-17','WC-11'],
'getting-started/overview':['WS-31','WS-40'], 'getting-started/choose-an-api':['WS-40'], 'getting-started/glossary':['WS-40'],
'search/search-quickstart':['WS-01','WS-10'], 'search/best-practices':['WS-04','WS-41'],
'search/evaluating-search':['WS-41'], 'search/modes':['WS-06'], 'search/migrate-to-parallel':['WS-40'],
'search/indexed-content-for-agents':['WS-07','WC-04'], 'search/advanced-search-settings':['WS-02','WS-03','WS-04','WS-07'],
'search/source-policy':['WS-02'], 'search/search-mcp':['WS-11'], 'search/search-migration-guide':['WS-40'],
'extract/extract-quickstart':['WC-02','WC-06','WC-11'], 'extract/best-practices':['WC-12'],
'extract/advanced-extract-settings':['WC-01','WC-04','WC-11'], 'extract/extract-migration-guide':['WC-11','WS-40'],
'image-search/best-practices':['WS-08','WS-41'], 'image-search/modes':['WS-08','WS-06'],
'task-api/guides/interactions':['WS-24','WS-42'], 'task-api/guides/specify-a-task':['WS-24'],
 'task-api/guides/choose-a-processor':['WS-24'], 'task-api/guides/access-research-basis':['WS-14'],
 'task-api/guides/execute-task-run':['WS-13','WS-14'], 'task-api/group-api':['WS-21'],
 'task-api/ingest-api':['WS-24'], 'task-api/task-sse':['WS-19'], 'task-api/webhooks':['WS-20'],
 'task-api/mcp-tool-call':['WS-25'], 'task-api/data-connectors':['WS-26'], 'task-api/task-mcp':['WS-30'],
 'task-api/source-policy':['WS-02'],
 'responses-api/features/mcp-tools':['WS-22','WS-25'], 'responses-api/features/data-connectors':['WS-22','WS-26'],
 'responses-api/features/web-search-tool':['WS-22','WS-02'],
 'findall-api/entity-search':['WS-09'], 'findall-api/findall-migration-guide':['WS-40'],
 'findall-api/features/findall-enrich':['WS-27'], 'findall-api/features/findall-sse':['WS-19'],
 'findall-api/features/findall-webhook':['WS-20'], 'findall-api/features/findall-preview':['WS-28'],
 'findall-api/features/findall-extend':['WS-28'], 'findall-api/features/findall-cancel':['WS-28'],
 'findall-api/features/findall-refresh':['WS-28'], 'findall-api/core-concepts/findall-lifecycle':['WS-28'],
 'monitor-api/quickstart-snapshot':['WS-29'], 'monitor-api/monitor-task':['WS-29'],
 'monitor-api/monitor-webhooks':['WS-20'], 'monitor-api/monitor-migration-guide':['WS-40'],
 'integrations/developer-quickstart':['WS-31','WS-37'], 'integrations/account-api':['WS-32','WS-33'],
 'integrations/cli':['WS-12','WS-44'], 'integrations/oauth-provider':['WS-32'],
 'integrations/agentic-payments':['WS-35'], 'integrations/browseruse':['WC-09','WS-25'],
 'integrations/aws-marketplace':['WS-43'], 'integrations/google-cloud-marketplace':['WS-43'],
 'integrations/mcp/search-mcp':['WS-11','WC-08'], 'integrations/mcp/task-mcp':['WS-30'],
 'integrations/mcp/quickstart':['WS-11','WS-30'], 'integrations/mcp/programmatic-use':['WS-11','WS-30'],
 'resources/data-connectors':['WS-26'], 'resources/memory':['WS-18'], 'resources/source-policy':['WS-02'],
 'resources/organization-roles-and-permissions':['WS-33','WS-34'], 'resources/warnings-and-errors':['WS-10','WC-11'],
 'resources/webhook-setup':['WS-20'], 'resources/crawler':['WC-05'], 'resources/status':['WS-42'],
 'resources/faqs':['WS-42','WS-34','WC-09'], 'resources/changelog':['WS-40'],
}
PLUGINS={'agent-skills','claude-code-marketplace','cursor-marketplace','opencode-plugin','pi-extension','clawhub'}
WORKFLOWS={'n8n','zapier','gsuite','render','vercel','superhuman'}
FRAMEWORKS={'langchain','litellm','openrouter','ollama-tool-calling','google-gemini-enterprise','anthropic-tool-calling','openai-tool-calling'}

def operation(path):
 if path.startswith('/service/v1/balance'): return ['WS-34']
 if path.startswith('/service/v1/'): return ['WS-33']
 if path.endswith('/search') and 'images' in path: return ['WS-08','WS-10']
 if path in ['/v1/search','/v1beta/search']: return ['WS-01','WS-10','WS-40']
 if path in ['/v1/extract','/v1beta/extract']: return ['WC-02','WC-11','WS-10','WS-40']
 if '/tasks/groups' in path: return ['WS-21','WS-19'] if path.endswith('/events') else ['WS-21']
 if '/tasks/runs' in path: return ['WS-19'] if path.endswith('/events') else ['WS-13','WS-14','WS-24']
 if path.endswith('/entity-search'): return ['WS-09']
 if path.endswith('/ingest'): return ['WS-24']
 if '/findall/' in path:
  if path.endswith('/events'): return ['WS-19']
  if path.endswith('/enrich'): return ['WS-27']
  if path.endswith(('/extend','/cancel','/candidates')): return ['WS-28','WS-40']
  return ['WS-16']
 if '/monitors' in path: return ['WS-15','WS-29','WS-40']
 if '/memory/' in path: return ['WS-18']
 if path.endswith('/chat/completions'): return ['WS-23']
 if path.endswith('/responses'): return ['WS-22']
 raise ValueError('Unmapped operation '+path)

api_docs={}
operations=[]
for filename,kind in [('public-openapi.json','current-product'),('account-openapi.json','current-account'),('docs-legacy-openapi.json','legacy')]:
 api=json.loads((CASE/'data/parallel-docs'/filename).read_text())
 for path,methods in api['paths'].items():
  for method,info in methods.items():
   if method not in ['get','post','patch','delete','put']: continue
   ids=operation(path)
   operations.append({'spec':filename,'kind':kind,'method':method.upper(),'path':path,'issues':ids})
   if 'operationId' in info:
    api_docs[info['operationId']]=ids

def page_issues(page):
 if page in EXACT: return EXACT[page]
 if page.startswith('integrations/'):
  leaf=page.split('/')[-1]
  if leaf in PLUGINS: return ['WS-37']
  if leaf in WORKFLOWS: return ['WS-39']
  if leaf in FRAMEWORKS: return ['WS-36']
 if page.startswith('image-search/'): return ['WS-08']
 if page.startswith('task-api/examples/'): return ['WS-14','WS-24','WS-21']
 if page in ['task-api/task-quickstart','task-api/best-practices']: return ['WS-14','WS-24']
 if page.startswith('responses-api/'): return ['WS-22','WS-25','WS-26'] if 'data-connectors' in page else ['WS-22']
 if page.startswith('findall-api/'): return ['WS-16','WS-28']
 if page.startswith('monitor-api/'): return ['WS-15','WS-29']
 if page.startswith('data-integrations/'): return ['WS-38']
 if page.startswith('api-reference/search/'): return ['WS-01','WS-10']
 if page.startswith('api-reference/image-search/'): return ['WS-08']
 if page.startswith('api-reference/extract/'): return ['WC-02','WC-11']
 if page.startswith('api-reference/tasks/'):
  if 'group' in page: return ['WS-21','WS-19'] if 'stream' in page else ['WS-21']
  return ['WS-19'] if 'stream' in page else ['WS-13','WS-14','WS-24']
 if page.startswith('api-reference/findall/'):
  if 'entity' in page: return ['WS-09']
  if 'ingest' in page: return ['WS-24']
  if 'enrichment' in page: return ['WS-27']
  if 'stream' in page: return ['WS-19']
  if 'extend' in page or 'cancel' in page: return ['WS-28']
  return ['WS-16']
 if page.startswith('api-reference/monitor/'): return ['WS-15','WS-29']
 if page.startswith('api-reference/memory/'): return ['WS-18']
 if page.startswith('api-reference/chat-api-beta/'): return ['WS-23']
 if page.startswith('api-reference/responses-api/'): return ['WS-22']
 if page.startswith('service-api/balance/'): return ['WS-34']
 if page.startswith(('service-api/apps/','service-api/keys/')): return ['WS-33']
 raise ValueError('Unmapped documentation page '+page)

sources=[]
for source in manifest['sources']:
 url=source['url']
 if url.startswith('https://docs.parallel.ai/') and url.endswith('.md'):
  page=url.removeprefix('https://docs.parallel.ai/').removesuffix('.md')
  sources.append({**source,'issues':page_issues(page)})
for item in sources+operations:
 for id_ in item['issues']:
  if id_ not in drafts: raise ValueError('Unknown coverage id '+id_)
coverage={'retrieved_at':manifest['retrieved_at'],'indexed_pages':len(sources),'issues':len(drafts),
          'current_operations':sum(i['kind']!='legacy' for i in operations),'sources':sources,'operations':operations}
(CASE/'coverage.json').write_text(json.dumps(coverage,indent=2)+'\n')
mapping_path=CASE/'created-issues.json'
mapping=json.loads(mapping_path.read_text()) if mapping_path.exists() else {}

def link(id_):
 return f"[{id_}]({mapping[id_]['url']})" if id_ in mapping else f"[{id_}](issues/{drafts[id_]['file']})"

lines=['# Issue inventory and dependencies','',
       'Generated from draft front matter and the durable GitHub mapping. Dependencies are prerequisites, with arrows from prerequisite to dependent. No cycles are permitted.','',
       '| ID | Repository | Work item | Depends on |','| --- | --- | --- | --- |']
for id_ in sorted(drafts):
 d=drafts[id_]
 lines.append(f"| {link(id_)} | {d['repo'].split('/')[-1]} | {d['title']} | {', '.join(link(dep) for dep in d['deps']) or 'None'} |")
lines+=['','## Full dependency graph','','```mermaid','graph TD']
for id_ in sorted(drafts): lines.append(f'  {id_.replace("-","")}["{id_}"]')
for id_ in sorted(drafts):
 for dep in drafts[id_]['deps']: lines.append(f'  {dep.replace("-","")} --> {id_.replace("-","")}')
lines+=['```','','## Dependency-valid delivery order','',', '.join(link(id_) for id_ in order), '',
        'Within this order, items with completed prerequisites can run independently. No phase supersedes the per-issue prerequisites above.','',
        '## Endpoint coverage','', '| Spec | Method | Path | Work items |','| --- | --- | --- | --- |']
for op in operations: lines.append(f"| {op['kind']} | {op['method']} | `{op['path']}` | {', '.join(link(id_) for id_ in op['issues'])} |")
(CASE/'issue-inventory.md').write_text('\n'.join(lines)+'\n')
print(f"Covered {len(sources)} indexed pages, {coverage['current_operations']} current operations, {len(operations)-coverage['current_operations']} legacy operations, and {len(drafts)} acyclic issues.")
