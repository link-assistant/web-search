"""Validate, file, and resume the cross-repository issue plan using gh."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_DRAFTS = ROOT / "docs/case-studies/issue-29/issues"
DEFAULT_MAPPING = DEFAULT_DRAFTS.parent / "created-issues.json"
REPOS = {"WS": "link-assistant/web-search", "WC": "link-assistant/web-capture"}


def load_drafts(directory):
    drafts = {}
    for path in sorted(directory.glob("*.md")):
        text = path.read_text()
        match = re.match(r"\A---\n(.*?)\n---\n(.*)", text, re.S)
        if not match:
            raise ValueError(f"Missing front matter: {path}")
        metadata = dict(line.split(": ", 1) for line in match[1].splitlines())
        id_ = metadata["id"]
        if not re.fullmatch(r"(?:WS|WC)-\d{2}", id_) or id_ in drafts:
            raise ValueError(f"Invalid or duplicate id: {id_}")
        if metadata["repo"] != REPOS[id_[:2]]:
            raise ValueError(f"Wrong repository for {id_}")
        deps = metadata["depends_on"]
        if not deps.startswith("[") or not deps.endswith("]"):
            raise ValueError(f"Invalid depends_on for {id_}")
        metadata["deps"] = re.findall(r"(?:WS|WC)-\d{2}", deps)
        metadata["body"] = match[2].strip()
        metadata["file"] = path.name
        for section in ("Summary", "Scope", "Acceptance criteria"):
            if f"## {section}" not in metadata["body"]:
                raise ValueError(f"Missing {section} in {id_}")
        drafts[id_] = metadata
    if not drafts:
        raise ValueError("No drafts found")
    return drafts


def ordered_ids(drafts):
    order, visiting, visited = [], set(), set()

    def visit(id_):
        if id_ not in drafts:
            raise ValueError(f"Unknown dependency: {id_}")
        if id_ in visiting:
            raise ValueError(f"Dependency cycle at {id_}")
        if id_ in visited:
            return
        visiting.add(id_)
        for dependency in drafts[id_]["deps"]:
            visit(dependency)
        visiting.remove(id_)
        visited.add(id_)
        order.append(id_)

    for id_ in sorted(drafts):
        visit(id_)
    return order


def gh(*args):
    return subprocess.run(["gh", *args], check=True, capture_output=True, text=True).stdout.strip()


def marker(id_):
    return f"<!-- parallel-parity:issue-29:{id_} -->"


def save_mapping(path, mapping):
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(".tmp")
    temp.write_text(json.dumps(dict(sorted(mapping.items())), indent=2) + "\n")
    temp.replace(path)


def render_body(id_, draft, mapping):
    body = draft["body"]
    # Forward references are linked in the final pass after every issue exists.
    body = re.sub(r"\b(?:WS|WC)-\d{2}\b",
                  lambda m: f"[{m[0]}]({mapping[m[0]]['url']})" if m[0] in mapping else m[0], body)
    dependencies = "\n".join(f"- [{dep}]({mapping[dep]['url']})" for dep in draft["deps"])
    return (marker(id_) + "\n\n" + body + "\n\n## Dependencies\n\n" +
            (dependencies or "None; this work can start independently.") +
            "\n\nPlanned from [web-search issue #29](https://github.com/link-assistant/web-search/issues/29) "
            "and [PR #30](https://github.com/link-assistant/web-search/pull/30).\n")


def file_issues(drafts, order, mapping_path):
    # Read the repositories even when a local mapping exists: recover the
    # create-success/save-failure window and detect duplicate remote markers.
    mapping = {}
    for repo in sorted({draft["repo"] for draft in drafts.values()}):
        pages = json.loads(gh("api", f"repos/{repo}/issues", "--paginate", "--slurp", "-X", "GET",
                              "-f", "state=all", "-f", "per_page=100"))
        remote = [issue for page in pages for issue in page]
        for issue in remote:
            if "pull_request" in issue:
                continue
            for id_, draft in drafts.items():
                if draft["repo"] == repo and marker(id_) in (issue.get("body") or ""):
                    if id_ in mapping:
                        raise ValueError(f"Multiple GitHub issues carry the marker for {id_}")
                    mapping[id_] = {"repo": repo, "number": issue["number"],
                                    "url": issue["html_url"], "title": draft["title"]}
    save_mapping(mapping_path, mapping)
    with tempfile.TemporaryDirectory(prefix="issue-29-bodies-") as temp:
        body_path = Path(temp) / "body.md"
        for id_ in order:
            draft = drafts[id_]
            if id_ not in mapping:
                body_path.write_text(render_body(id_, draft, mapping))
                labels = ",".join(label.strip() for label in draft["labels"].split(","))
                url = gh("issue", "create", "--repo", draft["repo"], "--title", draft["title"],
                         "--label", labels, "--body-file", str(body_path)).splitlines()[-1]
                expected = f"https://github.com/{draft['repo']}/issues/"
                if not url.startswith(expected) or not url.removeprefix(expected).isdigit():
                    raise ValueError(f"Unexpected issue URL: {url}")
                mapping[id_] = {"repo": draft["repo"], "number": int(url.rsplit("/", 1)[1]),
                                "url": url, "title": draft["title"]}
                save_mapping(mapping_path, mapping)
                print(f"created {id_}: {url}", flush=True)
            else:
                print(f"reusing {id_}: {mapping[id_]['url']}", flush=True)
        # Link all dependencies and forward references in issue bodies, avoiding
        # repeated dependency comments on resumed runs.
        for id_ in order:
            draft = drafts[id_]
            body_path.write_text(render_body(id_, draft, mapping))
            gh("issue", "edit", mapping[id_]["url"], "--title", draft["title"],
               "--body-file", str(body_path))
            print(f"linked {id_}", flush=True)
    return mapping


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--drafts", type=Path, default=Path(os.environ.get("DRAFTS", DEFAULT_DRAFTS)))
    parser.add_argument("--mapping", type=Path, default=DEFAULT_MAPPING)
    args = parser.parse_args()
    drafts = load_drafts(args.drafts)
    order = ordered_ids(drafts)
    if not args.apply:
        for id_ in order:
            draft = drafts[id_]
            print(f"[dry-run] {id_} {draft['repo']}: {draft['title']} (depends on {draft['deps']})")
        print(f"Validated {len(order)} issues; dependency graph is acyclic. No GitHub writes.")
    else:
        mapping = file_issues(drafts, order, args.mapping)
        print(f"Filed and linked {len(mapping)} issues; mapping: {args.mapping}")


if __name__ == "__main__":
    main()
