# @link-assistant/web-search

[![npm package](https://img.shields.io/npm/v/@link-assistant/web-search?label=npm)](https://www.npmjs.com/package/@link-assistant/web-search)
[![npm downloads](https://img.shields.io/npm/dm/@link-assistant/web-search?label=downloads)](https://www.npmjs.com/package/@link-assistant/web-search)
[![JavaScript Checks and Release](https://github.com/link-assistant/web-search/actions/workflows/js.yml/badge.svg)](https://github.com/link-assistant/web-search/actions/workflows/js.yml)
[![JS release tag](https://img.shields.io/badge/GitHub%20release-js--v0.8.0-blue)](https://github.com/link-assistant/web-search/releases?q=js-v)

JavaScript implementation of the `web-search` library, CLI, and HTTP service.
It mirrors the Rust crate in `../rust` with the same 22-provider catalog, four
provider categories, merge strategies, and discovery surface.

## Install

```bash
npm install @link-assistant/web-search
bun add @link-assistant/web-search
yarn add @link-assistant/web-search
```

The package publishes only runtime assets (`src`, `bin`, `examples`, README,
and changelog). It is a public scoped package and uses the repository directory
metadata required by npm for packages stored below the repository root.

## Library

```javascript
import {
  createSearchEngine,
  buildProviders,
  getProviderIds,
} from '@link-assistant/web-search';

const engine = createSearchEngine();

const results = await engine.search('graph neural networks', {
  limit: 10,
  providers: ['arxiv', 'crossref', 'openalex'],
  strategy: 'rrf',
});

const github = buildProviders().get('github');
const codeResults = await github.search('web search cli', { limit: 5 });

console.log(getProviderIds('papers'));
console.log(results.map((result) => result.url));
console.log(codeResults.length);
```

## CLI

```bash
npx web-search "rust async search" --limit 10
npx web-search "transformer architecture" --providers arxiv,crossref --format json
npx web-search --list-providers
```

## HTTP Service

```bash
npx web-search serve --port 3000

curl "http://localhost:3000/search?q=rust+programming&limit=10"
curl "http://localhost:3000/providers?category=papers"
curl "http://localhost:3000/categories"
```

## Providers

The live registry has 22 providers in four categories:

| Category    | Provider ids                                                                                               |
| ----------- | ---------------------------------------------------------------------------------------------------------- |
| `search`    | `google`, `bing`, `duckduckgo`, `searx`, `brave`, `mojeek`, `ecosia`, `startpage`, `yahoo`, `lite`, `wc:*` |
| `knowledge` | `wikipedia`, `wikidata`                                                                                    |
| `papers`    | `crossref`, `openalex`, `arxiv`                                                                            |
| `code`      | `github`, `hackernews`                                                                                     |

`google` and `bing` use official APIs when credentials are configured and fall
back to HTML parsing otherwise. `GITHUB_TOKEN` is optional and raises the GitHub
search rate limit.

## web-capture

`wc:wikipedia`, `wc:duckduckgo`, `wc:google`, `wc:bing`, and `wc:brave` delegate
to `@link-assistant/web-capture` when it is installed. The dependency is loaded
lazily; without it, the provider warns once and returns an empty result set so
the rest of aggregation can continue.

```bash
npm install @link-assistant/web-capture
```

```javascript
import { createWebCaptureProvider } from '@link-assistant/web-search';

const provider = createWebCaptureProvider({ engine: 'wikipedia' });
const results = await provider.search('OpenAI', { limit: 5 });
```

## Caller-owned transport and detailed outcomes

`transport` accepts a fetch-compatible function or an object with a `fetch`
method. It can be configured on the engine or supplied per call. `signal` is
forwarded to every provider request.

```javascript
const { results, outcomes } = await engine.searchDetailed('OpenAI', {
  providers: ['wikipedia', 'github'],
  transport: cachedTransport,
  signal: abortController.signal,
});
```

Each outcome reports `success`, `error`, or `unavailable`, retains the
provider's unmerged results, and includes response receipts. Native `Response`
objects are cloned into an exact byte capture; a custom transport may instead
attach an opaque `captureReceipt` property to its response.

## Release

The JavaScript release workflow publishes through npm trusted publishing and
creates GitHub releases tagged as `js-v<version>`. Version changes are managed
with changesets; pull requests that touch JavaScript package code should add a
changeset in `.changeset/`.

```bash
npm run changeset
npm run changeset:status
npm publish --dry-run --json
```

## Development

```bash
npm install
npm test
npm run check
node bin/web-search.js --list-providers
```

Cross-language parity is checked from the repository root:

```bash
node js/scripts/check-js-rust-parity.mjs
```
