# Issue #5 — FormalAI adoption blocker: align provider parity and defaults

Upstream FormalAI issue: <https://github.com/link-assistant/formal-ai/issues/410>

## Summary

FormalAI ([`link-assistant/formal-ai`](https://github.com/link-assistant/formal-ai))
wants to replace its in-repo `src/web_search_core.rs` provider registry with the
`@link-assistant/web-search` component. Before it can, `web-search` has to be a
**superset** of FormalAI's provider IDs and align its **default search
semantics** with FormalAI's DuckDuckGo-first behaviour. This case study captures
the raw evidence, the gap analysis, and the solution that this PR implements.

## Raw data

- [`raw-data/formal-ai/web_search_core.rs`](raw-data/formal-ai/web_search_core.rs) —
  a verbatim copy of FormalAI's `main` provider core
  (`WEB_SEARCH_PROVIDER_REGISTRY`, `WEB_SEARCH_PROVIDERS`, RRF helpers).
- [`formal-ai-compatibility.json`](formal-ai-compatibility.json) — the
  machine-readable compatibility map (FormalAI's 32 providers, the live default
  plan, the documented CORS exceptions, and the `web-search`-only extras). This
  file is consumed by the parity tests in both languages
  (`js/tests/formal-ai-compat.test.js` and `rust/tests/formal_ai_compat.rs`).

## FormalAI provider contract (as of the case study)

FormalAI's `WEB_SEARCH_PROVIDER_REGISTRY` declares **32** providers across the
four categories `web-search` already mirrors (`search`, `knowledge`, `papers`,
`code`). The canonical default per category is the first entry in its bucket:
`duckduckgo` (search), `wikipedia` (knowledge), `arxiv` (papers), `github`
(code).

FormalAI's **live default plan** (`WEB_SEARCH_PROVIDERS`, the CORS-readable
subset fired in the browser worker) is:

```
duckduckgo → internet-archive → wikipedia → wikidata → wiktionary → wikinews
```

## Gap analysis (HEAD before this PR)

| Gap | FormalAI | web-search (before) | Resolution |
| --- | --- | --- | --- |
| Search default | `duckduckgo` | `google` (`defaultForCategory: true`) | Flip default to `duckduckgo`. |
| Default plan | DDG, Internet Archive, Wikipedia, Wikidata, Wiktionary, Wikinews | DDG, Google, Bing, Wikipedia | `getDefaultProviderIds()` now returns FormalAI's live plan. |
| `openalex`/`crossref` category | `knowledge` | `papers` | Re-categorised to `knowledge`; `arxiv` becomes the `papers` default. |
| Missing IDs | 16 provider IDs | absent | Added (see below). |

### Provider IDs added in this PR

- **search**: `yandex`
- **knowledge**: `wiktionary`, `wikinews`, `internet-archive`, `dbpedia`,
  `openlibrary`, `semantic-scholar`, `cambridge-dictionary`, `merriam-webster`,
  `dictionary-com`, `collins-dictionary`
- **papers**: `europepmc`, `doaj`
- **code**: `gitlab`, `codeberg`, `gitee`, `bitbucket`, `gitflic`

After this PR `web-search` registers every FormalAI provider ID plus its own
extras (`searx`, `lite`, `hackernews`, and the `wc:*` web-capture namespace), so
it is a strict superset.

## Documented compatibility exceptions

The acceptance criteria allow "a documented compatibility map for provider IDs
that intentionally differ". There is exactly one metadata difference among the
shared IDs:

- **`duckduckgo` `corsReadable`**: FormalAI marks it `true` (it models the
  DuckDuckGo *Instant Answer* JSON API). `web-search` ships an HTML-SERP scraper
  (`html.duckduckgo.com` / `lite.duckduckgo.com`), which is **not**
  browser-CORS-readable, so `web-search` keeps `corsReadable: false`. The id,
  category, label intent, and search-default flag all match. This is recorded in
  `corsReadableExceptions` in the compatibility map and asserted by the parity
  tests.

## Native search providers vs. web-capture-delegated providers

`web-capture` providers stay in their own `wc:*` namespace with
`access: "component"`. Everything else is a **native** search provider: `api`
(JSON/Atom endpoint), `html` (SERP scrape through the shared anchor-list
parser), or `hybrid` (official API with a scraping fallback — `google`, `bing`).
The registry's `access` field makes the distinction explicit, and the parity
test asserts that every FormalAI ID resolves to a native (non-`wc:`) provider.

## Acceptance criteria → resolution

1. **Superset of provider IDs / documented compatibility map** — ✅ all 32
   FormalAI IDs registered; the one intentional metadata difference is
   documented in `formal-ai-compatibility.json` and asserted by tests.
2. **Align default search semantics (DuckDuckGo-first)** — ✅ `duckduckgo` is the
   `search` default in both languages.
3. **Include FormalAI's live default plan** — ✅ `getDefaultProviderIds()` /
   `get_default_provider_ids()` return DDG, Internet Archive, Wikipedia,
   Wikidata, Wiktionary, Wikinews.
4. **Registry tests in JS and Rust** — ✅ `formal-ai-compat.test.js` and
   `formal_ai_compat.rs` assert IDs, categories, CORS/fetchability metadata, and
   defaults against the shared compatibility map.
5. **Keep the `web-capture` namespace; clarify native vs. delegated** — ✅ the
   `wc:*` namespace is retained with `access: "component"`; the parity test
   distinguishes native providers from delegated ones.
