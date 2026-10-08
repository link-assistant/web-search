"""Offline regression tests for bulk filing; never contact GitHub."""

import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("file-issues.sh").resolve()
MOCK = '''#!/usr/bin/env python3
import json, os, pathlib, sys
state_path = pathlib.Path(os.environ["MOCK_STATE"])
state = json.loads(state_path.read_text()) if state_path.exists() else {"issues": [], "calls": []}
args = sys.argv[1:]
state["calls"].append(args)
def save():
    state_path.write_text(json.dumps(state))
def option(key):
    return args[args.index(key)+1]
if args[0] == "api":
    repo = args[1].removeprefix("repos/").removesuffix("/issues")
    issues = [i for i in state["issues"] if i["repo"] == repo]
    # Deliberately use tiny pages to cover multiple-page marker recovery.
    pages = [issues[i:i+1] for i in range(len(issues))] or [[]]
    print(json.dumps(pages) if "--slurp" in args else json.dumps(issues))
elif args[:2] == ["issue", "create"]:
    if os.environ.get("MOCK_FAIL_AFTER") == str(len(state["issues"])):
        save()
        sys.exit(1)
    repo = option("--repo")
    number = len(state["issues"]) + 1
    body = pathlib.Path(option("--body-file")).read_text() if "--body-file" in args else option("--body")
    issue = {"repo": repo, "number": number, "title": option("--title"), "body": body,
             "html_url": f"https://github.com/{repo}/issues/{number}"}
    state["issues"].append(issue)
    print(issue["html_url"])
elif args[:2] == ["issue", "edit"]:
    issue = next(i for i in state["issues"] if i["html_url"] == args[2])
    issue["body"] = pathlib.Path(option("--body-file")).read_text()
    issue["title"] = option("--title")
elif args[:2] == ["issue", "comment"]:
    pass
else:
    sys.exit("Unexpected mocked gh command: " + str(args))
save()
'''


class FilingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.drafts = self.root / "drafts"
        self.drafts.mkdir()
        mock = self.root / "gh"
        mock.write_text(MOCK)
        mock.chmod(0o755)
        self.state = self.root / "state.json"
        self.mapping = self.root / "mapping.json"
        self.env = dict(os.environ, DRAFTS=str(self.drafts), MOCK_STATE=str(self.state),
                        PATH=str(self.root) + os.pathsep + os.environ["PATH"])
        self.draft("WC-01", "web-capture", "[]")
        self.draft("WS-01", "web-search", "[WC-01]")

    def tearDown(self):
        self.temp.cleanup()

    def draft(self, id_, repo, deps):
        name = f"{repo}-{id_[-2:]}-test.md"
        (self.drafts / name).write_text(f"---\nid: {id_}\nrepo: link-assistant/{repo}\n"
            f"title: Implement {id_}\ndepends_on: {deps}\nlabels: enhancement\n---\n"
            "\n## Summary\n\nTest draft.\n\n## Scope\n\nImplement it.\n"
            "\n## Acceptance criteria\n\nVerify it.\n")

    def run_script(self, apply=True, **env):
        return subprocess.run(["bash", str(SCRIPT), *( ["--apply"] if apply else []),
                               "--mapping", str(self.mapping)], cwd=self.root,
                              env=dict(self.env, **env), capture_output=True, text=True)

    def issues(self):
        return json.loads(self.state.read_text())["issues"]

    def test_dry_run_has_no_github_or_mapping_writes(self):
        result = self.run_script(False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.state.exists())
        self.assertFalse(self.mapping.exists())

    def test_invalid_dependency_is_rejected_before_github_calls(self):
        self.draft("WS-01", "web-search", "[WC-99]")
        result = self.run_script()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(self.state.exists())

    def test_creates_once_and_recovers_mapping_from_markers(self):
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(self.mapping.exists())
        self.mapping.unlink()
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(self.issues()), 2)

    def test_dependency_links_are_in_body(self):
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(self.issues()[0]["html_url"], self.issues()[1]["body"])

    def test_topological_order_is_not_filename_order(self):
        self.draft("WC-01", "web-capture", "[WC-02]")
        self.draft("WC-02", "web-capture", "[]")
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.issues()[0]["title"], "Implement WC-02")

    def test_partial_failure_resumes_without_duplicates(self):
        result = self.run_script(MOCK_FAIL_AFTER="1")
        self.assertNotEqual(result.returncode, 0)
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(self.issues()), 2)

    def test_cycle_is_rejected_before_github_calls(self):
        self.draft("WC-01", "web-capture", "[WS-01]")
        result = self.run_script()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(self.state.exists())

    def test_duplicate_remote_markers_stop_before_creation(self):
        marker = "<!-- parallel-parity:issue-29:WC-01 -->"
        issues = [{"repo": "link-assistant/web-capture", "number": n, "title": "Duplicate",
                   "body": marker, "html_url": f"https://github.com/link-assistant/web-capture/issues/{n}"}
                  for n in (1, 2)]
        self.state.write_text(json.dumps({"issues": issues, "calls": []}))
        result = self.run_script()
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(self.issues()), 2)
        calls = json.loads(self.state.read_text())["calls"]
        self.assertTrue(all(call[0] == "api" for call in calls))


if __name__ == "__main__":
    unittest.main()
