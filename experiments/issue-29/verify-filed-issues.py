"""Read GitHub and verify every filed issue matches its draft and dependency links."""

from datetime import datetime, timezone
import importlib.util
import json
from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[2]
CASE = ROOT / "docs/case-studies/issue-29"
spec = importlib.util.spec_from_file_location("filing", Path(__file__).with_name("file-issues.py"))
filing = importlib.util.module_from_spec(spec)
spec.loader.exec_module(filing)
drafts = filing.load_drafts(CASE / "issues")
filing.ordered_ids(drafts)
mapping = json.loads((CASE / "created-issues.json").read_text())
assert mapping.keys() == drafts.keys(), "Incomplete local mapping"

found = {}
for repo in sorted({draft["repo"] for draft in drafts.values()}):
    pages = json.loads(filing.gh("api", f"repos/{repo}/issues", "--paginate", "--slurp",
                                 "-X", "GET", "-f", "state=all", "-f", "per_page=100"))
    for issue in (issue for page in pages for issue in page):
        if "pull_request" in issue:
            continue
        for id_, draft in drafts.items():
            if draft["repo"] != repo or filing.marker(id_) not in (issue.get("body") or ""):
                continue
            assert id_ not in found, f"Duplicate remote marker: {id_}"
            assert issue["state"] == "open", f"Closed planning issue: {id_}"
            assert issue["title"] == draft["title"], f"Title mismatch: {id_}"
            assert issue["html_url"] == mapping[id_]["url"], f"URL mismatch: {id_}"
            expected = filing.render_body(id_, draft, mapping)
            assert (issue.get("body") or "").strip() == expected.strip(), f"Body mismatch: {id_}"
            # The marker contains an ID; all body references must be actual links.
            body = issue["body"].replace(filing.marker(id_), "")
            assert not re.search(r"(?<!\[)\b(?:WS|WC)-\d{2}\b", body), f"Unlinked reference: {id_}"
            labels = {label["name"] for label in issue["labels"]}
            assert {label.strip() for label in draft["labels"].split(",")} <= labels, id_
            found[id_] = {
                **mapping[id_], "state": issue["state"], "updated_at": issue["updated_at"],
                "dependencies": [mapping[dep]["url"] for dep in draft["deps"]],
                "body_matches_draft": True,
            }

assert found.keys() == drafts.keys(), f"Missing remote issues: {drafts.keys() - found.keys()}"
report = {
    "verified_at": datetime.now(timezone.utc).isoformat(),
    "count": len(found), "issues": dict(sorted(found.items())),
}
target = CASE / "data/filed-issues-verification-2026-10-08.json"
target.write_text(json.dumps(report, indent=2) + "\n")
print(f"Verified all {len(found)} open GitHub issues, complete bodies, labels, and dependency links.")
