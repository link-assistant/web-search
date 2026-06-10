/**
 * API-based search engine descriptors.
 *
 * These engines expose structured JSON (or Atom XML) endpoints that do not
 * require scraping HTML, so they are the most reliable providers and can be
 * tested deterministically with recorded fixtures. Each descriptor is consumed
 * by {@link module:providers/generic.GenericProvider}.
 *
 * @module providers/api-engines
 */

import { cleanText } from './html-utils.js';

/**
 * @typedef {import('./base.js').SearchResult} SearchResult
 * @typedef {import('./base.js').SearchOptions} SearchOptions
 */

/**
 * Build a normalized {@link SearchResult}.
 *
 * @param {string} source - Provider id
 * @param {{title?: string, url: string, snippet?: string}} item - Raw item
 * @param {number} rank - 1-based rank
 * @returns {SearchResult}
 */
function makeResult(source, item, rank) {
  return {
    title: cleanText(item.title) || 'Untitled',
    url: item.url,
    snippet: cleanText(item.snippet) || '',
    source,
    rank,
  };
}

/**
 * Reconstruct an abstract from OpenAlex's inverted index representation.
 *
 * @param {Object<string, number[]>} [inverted] - Inverted index
 * @returns {string} Reconstructed abstract text
 */
export function reconstructInvertedAbstract(inverted) {
  if (!inverted || typeof inverted !== 'object') {
    return '';
  }
  const slots = [];
  for (const [word, positions] of Object.entries(inverted)) {
    for (const pos of positions) {
      slots[pos] = word;
    }
  }
  return slots.filter((w) => w !== undefined).join(' ');
}

/** Wikipedia REST search — native JSON API, CORS-readable. */
export const wikipedia = {
  id: 'wikipedia',
  label: 'Wikipedia',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
  defaultForCategory: true,
  buildUrl(query, options = {}) {
    const limit = Math.min(options.limit || 10, 100);
    const lang = (options.language || 'en').slice(0, 12);
    return `https://${lang}.wikipedia.org/w/rest.php/v1/search/page?q=${encodeURIComponent(query)}&limit=${limit}`;
  },
  parse(data, limit, options = {}) {
    const lang = (options.language || 'en').slice(0, 12);
    const pages = Array.isArray(data?.pages) ? data.pages : [];
    return pages.slice(0, limit).map((p, i) =>
      makeResult(
        'wikipedia',
        {
          title: p.title,
          url: `https://${lang}.wikipedia.org/wiki/${encodeURIComponent(p.key)}`,
          snippet: p.excerpt || p.description || '',
        },
        i + 1
      )
    );
  },
};

/** Wikidata entity search — native JSON API. */
export const wikidata = {
  id: 'wikidata',
  label: 'Wikidata',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const limit = Math.min(options.limit || 10, 50);
    const lang = (options.language || 'en').slice(0, 12);
    return `https://www.wikidata.org/w/api.php?action=wbsearchentities&format=json&language=${lang}&uselang=${lang}&limit=${limit}&search=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const entries = Array.isArray(data?.search) ? data.search : [];
    return entries.slice(0, limit).map((e, i) =>
      makeResult(
        'wikidata',
        {
          title: e.label || e.id,
          url: e.concepturi || `https://www.wikidata.org/wiki/${e.id || ''}`,
          snippet: e.description || '',
        },
        i + 1
      )
    );
  },
};

/** SearXNG meta-search — JSON output from a public instance. */
export const searx = {
  id: 'searx',
  label: 'SearXNG',
  category: 'search',
  kind: 'json',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const base = options.searxInstance || 'https://searx.be';
    return `${base.replace(/\/$/, '')}/search?format=json&q=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const entries = Array.isArray(data?.results) ? data.results : [];
    return entries
      .slice(0, limit)
      .map((e, i) =>
        makeResult(
          'searx',
          { title: e.title, url: e.url, snippet: e.content || '' },
          i + 1
        )
      );
  },
};

/** Crossref scholarly works — native JSON API. */
export const crossref = {
  id: 'crossref',
  label: 'Crossref',
  category: 'papers',
  kind: 'json',
  corsReadable: true,
  defaultForCategory: true,
  buildUrl(query, options = {}) {
    const rows = Math.min(options.limit || 10, 50);
    return `https://api.crossref.org/works?rows=${rows}&query=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const items = Array.isArray(data?.message?.items) ? data.message.items : [];
    return items
      .slice(0, limit)
      .map((it, i) =>
        makeResult(
          'crossref',
          {
            title: Array.isArray(it.title) ? it.title[0] : it.title,
            url: it.URL || (it.DOI ? `https://doi.org/${it.DOI}` : ''),
            snippet: it.abstract || it['container-title']?.[0] || '',
          },
          i + 1
        )
      )
      .filter((r) => r.url);
  },
};

/** OpenAlex scholarly works — native JSON API. */
export const openalex = {
  id: 'openalex',
  label: 'OpenAlex',
  category: 'papers',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const perPage = Math.min(options.limit || 10, 50);
    return `https://api.openalex.org/works?per-page=${perPage}&search=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const items = Array.isArray(data?.results) ? data.results : [];
    return items
      .slice(0, limit)
      .map((it, i) =>
        makeResult(
          'openalex',
          {
            title: it.title || it.display_name,
            url: it.doi || it.id,
            snippet: reconstructInvertedAbstract(it.abstract_inverted_index),
          },
          i + 1
        )
      )
      .filter((r) => r.url);
  },
};

/** GitHub repository search — native JSON API. */
export const github = {
  id: 'github',
  label: 'GitHub',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  defaultForCategory: true,
  headers() {
    const headers = {
      Accept: 'application/vnd.github+json',
      'X-GitHub-Api-Version': '2022-11-28',
    };
    if (process.env.GITHUB_TOKEN) {
      headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
    }
    return headers;
  },
  buildUrl(query, options = {}) {
    const perPage = Math.min(options.limit || 10, 50);
    return `https://api.github.com/search/repositories?per_page=${perPage}&q=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const items = Array.isArray(data?.items) ? data.items : [];
    return items.slice(0, limit).map((it, i) =>
      makeResult(
        'github',
        {
          title: it.full_name || it.name,
          url: it.html_url,
          snippet: it.description || '',
        },
        i + 1
      )
    );
  },
};

/** Hacker News (Algolia) — native JSON API. */
export const hackernews = {
  id: 'hackernews',
  label: 'Hacker News',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const hits = Math.min(options.limit || 10, 50);
    return `https://hn.algolia.com/api/v1/search?hitsPerPage=${hits}&query=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    const hits = Array.isArray(data?.hits) ? data.hits : [];
    return hits.slice(0, limit).map((h, i) =>
      makeResult(
        'hackernews',
        {
          title: h.title || h.story_title,
          url:
            h.url ||
            h.story_url ||
            `https://news.ycombinator.com/item?id=${h.objectID}`,
          snippet: h.story_text || h.comment_text || '',
        },
        i + 1
      )
    );
  },
};

/**
 * Parse an arXiv Atom feed into normalized results.
 *
 * @param {string} xml - Atom XML body
 * @param {number} limit - Max results
 * @returns {SearchResult[]}
 */
export function parseArxivAtom(xml, limit) {
  const results = [];
  const entryRe = /<entry>([\s\S]*?)<\/entry>/g;
  let m;
  while ((m = entryRe.exec(xml)) !== null && results.length < limit) {
    const entry = m[1];
    const title = entry.match(/<title>([\s\S]*?)<\/title>/)?.[1] || '';
    const id = entry.match(/<id>([\s\S]*?)<\/id>/)?.[1] || '';
    const summary = entry.match(/<summary>([\s\S]*?)<\/summary>/)?.[1] || '';
    if (!id) {
      continue;
    }
    results.push(
      makeResult(
        'arxiv',
        { title, url: id.trim(), snippet: summary },
        results.length + 1
      )
    );
  }
  return results;
}

/** arXiv pre-print search — Atom XML API. */
export const arxiv = {
  id: 'arxiv',
  label: 'arXiv',
  category: 'papers',
  kind: 'text',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const max = Math.min(options.limit || 10, 50);
    return `http://export.arxiv.org/api/query?max_results=${max}&search_query=${encodeURIComponent(`all:${query}`)}`;
  },
  parse(xml, limit) {
    return parseArxivAtom(xml, limit);
  },
};

/** All API-based engine descriptors keyed by id. */
export const API_ENGINES = {
  wikipedia,
  wikidata,
  searx,
  crossref,
  openalex,
  github,
  hackernews,
  arxiv,
};
