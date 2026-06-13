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
 * Coerce a field that may be a string or an array of strings to its first
 * string value (common in archive.org / DBpedia payloads).
 *
 * @param {string|string[]|undefined} value - Raw field
 * @returns {string}
 */
export function firstOf(value) {
  if (Array.isArray(value)) {
    return typeof value[0] === 'string' ? value[0] : '';
  }
  return typeof value === 'string' ? value : '';
}

/**
 * Project a list of raw items into normalized {@link SearchResult}s.
 *
 * Centralizes the slice/rank/filter loop so each engine parser only declares
 * how to turn one raw item into `{title, url, snippet}`. Items whose `url`
 * projects to a falsy value are dropped.
 *
 * @param {string} source - Provider id
 * @param {*} items - Raw array (or anything; non-arrays yield `[]`)
 * @param {number} limit - Max results
 * @param {(item: *) => {title?: string, url: string, snippet?: string}} project
 * @returns {SearchResult[]}
 */
export function listResults(source, items, limit, project) {
  const arr = Array.isArray(items) ? items : [];
  const out = [];
  for (const item of arr) {
    if (out.length >= limit) {
      break;
    }
    const fields = project(item);
    if (!fields || !fields.url) {
      continue;
    }
    out.push(makeResult(source, fields, out.length + 1));
  }
  return out;
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

/**
 * Build a MediaWiki REST `search/page` engine descriptor.
 *
 * Wikipedia, Wiktionary, and Wikinews all expose the same CORS-readable REST
 * search endpoint and article-URL shape, differing only by host domain, so they
 * share one factory (issue #5 / formal-ai parity).
 *
 * @param {Object} spec - Engine spec
 * @param {string} spec.id - Provider id
 * @param {string} spec.label - Human-readable label
 * @param {string} spec.domain - Host domain (e.g. `wikipedia.org`)
 * @returns {Object} Engine descriptor
 */
export function mediaWikiSearch({ id, label, domain }) {
  return {
    id,
    label,
    category: 'knowledge',
    kind: 'json',
    corsReadable: true,
    defaultForCategory: id === 'wikipedia',
    buildUrl(query, options = {}) {
      const limit = Math.min(options.limit || 10, 100);
      const lang = (options.language || 'en').slice(0, 12);
      return `https://${lang}.${domain}/w/rest.php/v1/search/page?q=${encodeURIComponent(query)}&limit=${limit}`;
    },
    parse(data, limit, options = {}) {
      const lang = (options.language || 'en').slice(0, 12);
      return listResults(id, data?.pages, limit, (p) => ({
        title: p.title,
        url: `https://${lang}.${domain}/wiki/${encodeURIComponent(p.key)}`,
        snippet: p.excerpt || p.description || '',
      }));
    },
  };
}

/** Wikipedia REST search — native JSON API, CORS-readable. */
export const wikipedia = mediaWikiSearch({
  id: 'wikipedia',
  label: 'Wikipedia',
  domain: 'wikipedia.org',
});

/** Wiktionary REST search — native JSON API, CORS-readable. */
export const wiktionary = mediaWikiSearch({
  id: 'wiktionary',
  label: 'Wiktionary',
  domain: 'wiktionary.org',
});

/** Wikinews REST search — native JSON API, CORS-readable. */
export const wikinews = mediaWikiSearch({
  id: 'wikinews',
  label: 'Wikinews',
  domain: 'wikinews.org',
});

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

/** Crossref scholarly works — native JSON API (knowledge per formal-ai). */
export const crossref = {
  id: 'crossref',
  label: 'Crossref',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
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

/** OpenAlex scholarly works — native JSON API (knowledge per formal-ai). */
export const openalex = {
  id: 'openalex',
  label: 'OpenAlex',
  category: 'knowledge',
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
  defaultForCategory: true,
  buildUrl(query, options = {}) {
    const max = Math.min(options.limit || 10, 50);
    return `http://export.arxiv.org/api/query?max_results=${max}&search_query=${encodeURIComponent(`all:${query}`)}`;
  },
  parse(xml, limit) {
    return parseArxivAtom(xml, limit);
  },
};

/** Internet Archive (archive.org) advanced search — native JSON API. */
export const internetArchive = {
  id: 'internet-archive',
  label: 'Internet Archive',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const rows = Math.min(options.limit || 10, 50);
    return `https://archive.org/advancedsearch.php?q=${encodeURIComponent(query)}&fl[]=identifier&fl[]=title&fl[]=description&rows=${rows}&page=1&output=json`;
  },
  parse(data, limit) {
    return listResults(
      'internet-archive',
      data?.response?.docs,
      limit,
      (d) => ({
        title: firstOf(d.title),
        url: d.identifier ? `https://archive.org/details/${d.identifier}` : '',
        snippet: firstOf(d.description),
      })
    );
  },
};

/** DBpedia Lookup — native JSON API (not CORS-readable). */
export const dbpedia = {
  id: 'dbpedia',
  label: 'DBpedia Lookup',
  category: 'knowledge',
  kind: 'json',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const max = Math.min(options.limit || 10, 50);
    return `https://lookup.dbpedia.org/api/search?format=json&maxResults=${max}&query=${encodeURIComponent(query)}`;
  },
  parse(data, limit) {
    return listResults('dbpedia', data?.docs, limit, (d) => ({
      title: firstOf(d.label),
      url: firstOf(d.resource),
      snippet: firstOf(d.comment),
    }));
  },
};

/** Open Library — native JSON search API, CORS-readable. */
export const openlibrary = {
  id: 'openlibrary',
  label: 'Open Library',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const lim = Math.min(options.limit || 10, 50);
    return `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}&limit=${lim}`;
  },
  parse(data, limit) {
    return listResults('openlibrary', data?.docs, limit, (d) => ({
      title: d.title,
      url: d.key ? `https://openlibrary.org${d.key}` : '',
      snippet: Array.isArray(d.author_name) ? d.author_name.join(', ') : '',
    }));
  },
};

/** Semantic Scholar — native JSON graph API, CORS-readable. */
export const semanticScholar = {
  id: 'semantic-scholar',
  label: 'Semantic Scholar',
  category: 'knowledge',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const lim = Math.min(options.limit || 10, 50);
    return `https://api.semanticscholar.org/graph/v1/paper/search?query=${encodeURIComponent(query)}&limit=${lim}&fields=title,abstract,url,year`;
  },
  parse(data, limit) {
    return listResults('semantic-scholar', data?.data, limit, (p) => ({
      title: p.title,
      url:
        p.url ||
        (p.paperId ? `https://www.semanticscholar.org/paper/${p.paperId}` : ''),
      snippet: p.abstract || '',
    }));
  },
};

/** Europe PMC — native JSON REST API, CORS-readable. */
export const europepmc = {
  id: 'europepmc',
  label: 'Europe PMC',
  category: 'papers',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const pageSize = Math.min(options.limit || 10, 50);
    return `https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=${encodeURIComponent(query)}&format=json&pageSize=${pageSize}`;
  },
  parse(data, limit) {
    return listResults('europepmc', data?.resultList?.result, limit, (r) => ({
      title: r.title,
      url: r.doi
        ? `https://doi.org/${r.doi}`
        : r.id && r.source
          ? `https://europepmc.org/article/${r.source}/${r.id}`
          : '',
      snippet: r.abstractText || '',
    }));
  },
};

/** DOAJ (Directory of Open Access Journals) articles — native JSON API. */
export const doaj = {
  id: 'doaj',
  label: 'DOAJ',
  category: 'papers',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const pageSize = Math.min(options.limit || 10, 50);
    return `https://doaj.org/api/search/articles/${encodeURIComponent(query)}?pageSize=${pageSize}`;
  },
  parse(data, limit) {
    return listResults('doaj', data?.results, limit, (r) => {
      const b = r.bibjson || {};
      const links = Array.isArray(b.link) ? b.link : [];
      const link = links.find((l) => l.type === 'fulltext') || links[0];
      return {
        title: b.title,
        url: link?.url || (r.id ? `https://doaj.org/article/${r.id}` : ''),
        snippet: b.abstract || '',
      };
    });
  },
};

/**
 * Normalize a code-host repository list into results.
 *
 * @param {string} source - Provider id
 * @param {*} data - Decoded JSON body
 * @param {number} limit - Max results
 * @param {string|null} container - Key holding the array (`null` = body is the array)
 * @param {string} titleField - Field used for the repository title
 * @param {string} urlField - Field used for the repository URL
 * @returns {SearchResult[]}
 */
export function repoResults(
  source,
  data,
  limit,
  container,
  titleField,
  urlField
) {
  const items = container ? data?.[container] : data;
  return listResults(source, items, limit, (it) => ({
    title: it[titleField] || it.name,
    url: it[urlField],
    snippet: it.description || '',
  }));
}

/** GitLab projects — native JSON API, CORS-readable. */
export const gitlab = {
  id: 'gitlab',
  label: 'GitLab',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const perPage = Math.min(options.limit || 10, 50);
    return `https://gitlab.com/api/v4/projects?search=${encodeURIComponent(query)}&per_page=${perPage}&order_by=star_count&sort=desc`;
  },
  parse(data, limit) {
    return repoResults(
      'gitlab',
      data,
      limit,
      null,
      'path_with_namespace',
      'web_url'
    );
  },
};

/** Codeberg (Gitea) repositories — native JSON API, CORS-readable. */
export const codeberg = {
  id: 'codeberg',
  label: 'Codeberg',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const lim = Math.min(options.limit || 10, 50);
    return `https://codeberg.org/api/v1/repos/search?q=${encodeURIComponent(query)}&limit=${lim}`;
  },
  parse(data, limit) {
    return repoResults(
      'codeberg',
      data,
      limit,
      'data',
      'full_name',
      'html_url'
    );
  },
};

/** Gitee repositories — native JSON API, CORS-readable. */
export const gitee = {
  id: 'gitee',
  label: 'Gitee',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const perPage = Math.min(options.limit || 10, 50);
    return `https://gitee.com/api/v5/search/repositories?q=${encodeURIComponent(query)}&per_page=${perPage}`;
  },
  parse(data, limit) {
    return repoResults('gitee', data, limit, null, 'full_name', 'html_url');
  },
};

/** Bitbucket Cloud repositories — native JSON API, CORS-readable. */
export const bitbucket = {
  id: 'bitbucket',
  label: 'Bitbucket',
  category: 'code',
  kind: 'json',
  corsReadable: true,
  buildUrl(query, options = {}) {
    const pagelen = Math.min(options.limit || 10, 50);
    const q = encodeURIComponent(`name~"${query}"`);
    return `https://api.bitbucket.org/2.0/repositories?q=${q}&pagelen=${pagelen}&fields=values.full_name,values.links.html.href,values.description`;
  },
  parse(data, limit) {
    return listResults('bitbucket', data?.values, limit, (v) => ({
      title: v.full_name,
      url: v.links?.html?.href || '',
      snippet: v.description || '',
    }));
  },
};

/** GitFlic projects — native JSON API (not CORS-readable, best-effort). */
export const gitflic = {
  id: 'gitflic',
  label: 'GitFlic',
  category: 'code',
  kind: 'json',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const size = Math.min(options.limit || 10, 50);
    return `https://api.gitflic.ru/project?query=${encodeURIComponent(query)}&size=${size}`;
  },
  parse(data, limit) {
    const items =
      data?._embedded?.projectList || data?.items || data?.results || data;
    return listResults('gitflic', items, limit, (it) => ({
      title: it.title || it.name || it.alias,
      url:
        it.url ||
        it._links?.self?.href ||
        (it.owner && it.alias
          ? `https://gitflic.ru/project/${it.owner}/${it.alias}`
          : ''),
      snippet: it.description || '',
    }));
  },
};

/** All API-based engine descriptors keyed by id. */
export const API_ENGINES = {
  wikipedia,
  wikidata,
  wiktionary,
  wikinews,
  'internet-archive': internetArchive,
  dbpedia,
  openlibrary,
  'semantic-scholar': semanticScholar,
  openalex,
  crossref,
  searx,
  arxiv,
  europepmc,
  doaj,
  github,
  hackernews,
  gitlab,
  codeberg,
  gitee,
  bitbucket,
  gitflic,
};
