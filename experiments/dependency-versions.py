"""Record latest direct Rust and npm releases from their authoritative registries."""
import concurrent.futures
import json
from pathlib import Path
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def latest(item):
    ecosystem, name, declared = item
    if ecosystem == "rust":
        path = name if len(name) == 1 else f"2/{name}" if len(name) == 2 else f"3/{name[0]}/{name}" if len(name) == 3 else f"{name[:2]}/{name[2:4]}/{name}"
        url = f"https://index.crates.io/{path}"
        with urllib.request.urlopen(url) as response:
            versions = [json.loads(line) for line in response]
        releases = [v for v in versions if not v["yanked"] and "-" not in v["vers"].split("+", 1)[0]]
        version = max(releases, key=lambda v: tuple(map(int, v["vers"].split("+", 1)[0].split("."))))["vers"]
    else:
        url = f"https://registry.npmjs.org/{name}/latest"
        with urllib.request.urlopen(url) as response:
            version = json.load(response)["version"]
    return dict(ecosystem=ecosystem, name=name, declared=declared, latest=version, source=url)


if __name__ == "__main__":
    cargo = tomllib.loads((ROOT / "rust/Cargo.toml").read_text())
    npm = json.loads((ROOT / "js/package.json").read_text())
    items = [("rust", name, value if isinstance(value, str) else value["version"])
             for section in ("dependencies", "dev-dependencies") for name, value in cargo[section].items()]
    items += [("npm", name, value) for section in ("dependencies", "devDependencies")
              for name, value in npm[section].items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as executor:
        print(json.dumps(list(executor.map(latest, items)), indent=2))
