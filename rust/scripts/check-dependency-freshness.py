#!/usr/bin/env python3
"""Fail if any resolved direct Cargo dependency is behind its latest stable release.

Optional/server and dev dependencies are included. A stale dependency is allowed
only when its manifest line cites an open GitHub issue explaining the blocker.
Uses the sparse registry directly, including releases outside the declared range.
"""
import argparse
import concurrent.futures
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib
import urllib.request

ISSUE = r"https://github\.com/([\w.-]+)/([\w.-]+)/issues/(\d+)"


def read_json(url):
    headers = {"User-Agent": "web-search-dependency-freshness"}
    if url.startswith("https://api.github.com/") and os.environ.get("GH_TOKEN"):
        headers["Authorization"] = "Bearer " + os.environ["GH_TOKEN"]
    with urllib.request.urlopen(urllib.request.Request(url, headers=headers)) as response:
        return json.load(response)


def latest_stable(versions):
    stable = [v["vers"] for v in versions if not v["yanked"] and "-" not in v["vers"].split("+", 1)[0]]
    return max(stable, key=lambda v: tuple(map(int, v.split("+", 1)[0].split("."))))


def latest_version(name):
    path = name if len(name) == 1 else f"2/{name}" if len(name) == 2 else f"3/{name[0]}/{name}" if len(name) == 3 else f"{name[:2]}/{name[2:4]}/{name}"
    request = urllib.request.Request(f"https://index.crates.io/{path}",
                                    headers={"User-Agent": "web-search-dependency-freshness"})
    with urllib.request.urlopen(request) as response:
        return latest_stable([json.loads(line) for line in response])


def blocker_for(manifest, name):
    # Require the issue on this dependency's line, not a nearby/global waiver.
    for line in manifest.splitlines():
        if re.match(rf'^\s*{re.escape(name)}\s*=', line) and "#" in line:
            match = re.search(ISSUE, line.split("#", 1)[1])
            if match:
                return match[0]
    return None


def check_version(name, current, latest, blocker, issue_reader=read_json):
    if current == latest:
        return
    message = f"{name}: resolved {current}, latest {latest}"
    if not blocker:
        raise ValueError(message + "; update the manifest/lockfile or cite an open blocker issue")
    match = re.fullmatch(ISSUE, blocker)
    if not match:
        raise ValueError(message + "; invalid blocker issue URL")
    owner, repo, number = match.groups()
    issue = issue_reader(f"https://api.github.com/repos/{owner}/{repo}/issues/{number}")
    explanation = issue.get("body")
    if issue.get("state") != "open" or "pull_request" in issue or not isinstance(explanation, str) or not explanation.strip():
        raise ValueError(message + "; blocker must be an open issue with an explanation")
    print(f"Allowed blocker: {message} ({blocker})")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rust-root", type=Path, default=Path("rust"))
    args = parser.parse_args()
    manifest_path = args.rust_root / "Cargo.toml"
    text = manifest_path.read_text()
    manifest = tomllib.loads(text)
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--manifest-path", str(manifest_path),
        "--all-features", "--locked", "--format-version", "1"], text=True))
    root = metadata["resolve"]["root"]
    root_node = next(n for n in metadata["resolve"]["nodes"] if n["id"] == root)
    packages = {p["id"]: p for p in metadata["packages"]}
    direct = {packages[id]["name"]: packages[id]["version"] for id in root_node["dependencies"]}
    declared = {}
    sections = [manifest] + list(manifest.get("target", {}).values())
    for section in sections:
        for key in ("dependencies", "dev-dependencies", "build-dependencies"):
            declared.update(section.get(key, {}))
    # Do not silently skip disabled optional dependencies or unresolved aliases.
    names = {key: value.get("package", key) if isinstance(value, dict) else key
             for key, value in declared.items()}
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as executor:
        latest = dict(zip(names, executor.map(latest_version, names.values())))
    failures = []
    for key, name in names.items():
        try:
            check_version(name, direct[name], latest[key], blocker_for(text, key))
            print(f"{name}: {direct[name]} (latest {latest[key]})")
        except (ValueError, KeyError, OSError) as error:
            failures.append(str(error))
    if failures:
        raise ValueError("\n".join(failures))


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        print(f"Dependency freshness failed: {error}", file=sys.stderr)
        sys.exit(1)
