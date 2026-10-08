"""Download a bounded, indexed snapshot instead of a hard-coded subset of docs."""

import concurrent.futures
import datetime
import hashlib
import json
import pathlib
import re
import sys
import urllib.request


BASE = "https://docs.parallel.ai"


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "web-search-case-study/2.0"})
    with urllib.request.urlopen(request) as response:
        data = response.read()
        return data, response.headers.get("Content-Type", ""), response.url


def main():
    out = pathlib.Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    index, _, _ = fetch(f"{BASE}/llms.txt")
    sources = {"llms.txt": f"{BASE}/llms.txt"}
    for url in re.findall(r"\]\((https://docs\.parallel\.ai/[^)]+\.md)\)", index.decode()):
        name = url.removeprefix(BASE + "/").replace("/", "_")
        sources[name] = url
    sources.update({
        "public-openapi.json": f"{BASE}/public-openapi.json",
        "docs-latest-openapi.json": f"{BASE}/docs-latest-openapi.json",
        "docs-legacy-openapi.json": f"{BASE}/docs-legacy-openapi.json",
        "account-openapi.json": "https://api.parallel.ai/account/service/openapi.json",
        "parallel-agents.md": "https://parallel.ai/agents.md",
    })
    records, errors = [], []

    def download(item):
        name, url = item
        data, content_type, final_url = (index, "text/plain", url) if name == "llms.txt" else fetch(url)
        if "text/html" in content_type:
            if not name.endswith(".md"):
                raise ValueError(f"Expected structured source, received HTML: {url}")
            # The index includes two redirects to HTML (status and connectors).
            # Preserve their actual format rather than silently skipping them.
            name = name.removesuffix(".md") + ".html"
        if name.endswith(".json"):
            json.loads(data)
        (out / name).write_bytes(data)
        return {"file": name, "url": url, "final_url": final_url, "content_type": content_type,
                "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}

    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
        futures = {pool.submit(download, item): item for item in sources.items()}
        for future in concurrent.futures.as_completed(futures):
            try:
                records.append(future.result())
            except Exception as error:
                errors.append({"url": futures[future][1], "error": str(error)})
    manifest = {"retrieved_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                "pages": sum(r["file"].endswith((".md", ".html")) and r["file"] != "parallel-agents.md" for r in records),
                "sources": sorted(records, key=lambda r: r["file"]), "errors": errors}
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"downloaded={len(records)} pages={manifest['pages']} failed={len(errors)}")
    for error in errors:
        print(error)
    return bool(errors)


if __name__ == "__main__":
    sys.exit(main())
