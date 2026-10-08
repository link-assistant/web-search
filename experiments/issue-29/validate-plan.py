"""Read-only validation of issue coverage, source integrity, and filed mappings."""
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
CASE=ROOT/'docs/case-studies/issue-29'
spec=importlib.util.spec_from_file_location('filing',Path(__file__).with_name('file-issues.py'))
filing=importlib.util.module_from_spec(spec); spec.loader.exec_module(filing)
drafts=filing.load_drafts(CASE/'issues'); filing.ordered_ids(drafts)
assert len(drafts)==56, len(drafts)
assert sum(d['repo'].endswith('web-search') for d in drafts.values())==44
coverage=json.loads((CASE/'coverage.json').read_text())
manifest=json.loads((CASE/'data/parallel-docs/manifest.json').read_text())
assert not manifest['errors'], manifest['errors']
indexed={s['url'] for s in manifest['sources'] if s['url'].startswith('https://docs.parallel.ai/') and s['url'].endswith('.md')}
assert indexed=={s['url'] for s in coverage['sources']}
for source in manifest['sources']:
 data=(CASE/'data/parallel-docs'/source['file']).read_bytes()
 assert hashlib.sha256(data).hexdigest()==source['sha256'], source['file']
for entry in coverage['sources']+coverage['operations']:
 assert entry['issues'] and set(entry['issues'])<=drafts.keys(), entry
for filename in ['public-openapi.json','account-openapi.json','docs-legacy-openapi.json']:
 api=json.loads((CASE/'data/parallel-docs'/filename).read_text())
 actual={(method.upper(),path) for path,methods in api['paths'].items() for method in methods if method in ['get','post','patch','delete','put']}
 planned={(o['method'],o['path']) for o in coverage['operations'] if o['spec']==filename}
 assert actual==planned, (filename,actual-planned)
for id_,draft in drafts.items():
 assert '## References' in draft['body'] or '## Planning references' in draft['body'],id_
 assert 'implementation' in draft['body'].lower(),id_
 assert 'http' in draft['body'],id_
 path=CASE/'created-issues.json'
 if path.exists():
  mapping=json.loads(path.read_text())
  assert mapping.keys()==drafts.keys(), 'Incomplete filed mapping'
  record=mapping[id_]
  assert record['repo']==draft['repo'] and record['title']==draft['title'],id_
  assert record['url']==f"https://github.com/{draft['repo']}/issues/{record['number']}",id_
print(f"Validated {len(drafts)} drafts, {len(indexed)} indexed sources, all current/legacy operations, source checksums, and dependencies.")
