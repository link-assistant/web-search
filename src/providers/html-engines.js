/**
 * HTML-scraping search engine descriptors.
 *
 * These engines have no public keyless JSON API, so results are parsed from
 * their HTML SERP best-effort (the same approach used by the Google/Bing/
 * DuckDuckGo providers). Parsing is factored through {@link parseAnchorList}
 * so each engine only declares its result-link pattern, and every parser is
 * unit-tested against recorded HTML fixtures.
 *
 * @module providers/html-engines
 */

import { cleanText } from './html-utils.js';

/**
 * @typedef {import('./base.js').SearchResult} SearchResult
 */

/**
 * Generic HTML result-list parser driven by a per-engine regex.
 *
 * @param {string} html - SERP HTML body
 * @param {Object} config - Parser configuration
 * @param {RegExp} config.itemRegex - Global regex; capture groups feed the fields
 * @param {string} config.source - Provider id used as the result `source`
 * @param {number} config.limit - Max results to return
 * @param {number} config.urlGroup - Capture group index for the URL
 * @param {number} config.titleGroup - Capture group index for the title
 * @param {number} [config.snippetGroup] - Capture group index for the snippet
 * @param {(url: string) => string} [config.urlTransform] - Normalize the raw URL
 * @param {(url: string) => boolean} [config.skip] - Return true to drop a URL
 * @returns {SearchResult[]}
 */
export function parseAnchorList(html, config) {
  const {
    itemRegex,
    source,
    limit,
    urlGroup,
    titleGroup,
    snippetGroup,
    urlTransform,
    skip,
  } = config;
  const results = [];
  const seen = new Set();
  let match;

  while (
    (match = itemRegex.exec(html || '')) !== null &&
    results.length < limit
  ) {
    let url = match[urlGroup];
    if (urlTransform) {
      url = urlTransform(url);
    }
    if (!url || seen.has(url) || (skip && skip(url))) {
      continue;
    }
    seen.add(url);
    results.push({
      title: cleanText(match[titleGroup]) || 'Untitled',
      url,
      snippet: snippetGroup ? cleanText(match[snippetGroup]) : '',
      source,
      rank: results.length + 1,
    });
  }

  return results;
}

/**
 * Decode a Yahoo redirect URL (`.../RU=<encoded>/RK=...`) to its destination.
 *
 * @param {string} href - Raw Yahoo anchor href
 * @returns {string} Resolved destination URL
 */
export function resolveYahooHref(href) {
  if (!href) {
    return '';
  }
  const ru = href.match(/\/RU=([^/]+)\//);
  if (ru) {
    try {
      return decodeURIComponent(ru[1]);
    } catch {
      return ru[1];
    }
  }
  return href;
}

/** Brave Search — HTML SERP. */
export const brave = {
  id: 'brave',
  label: 'Brave Search',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const params = new URLSearchParams({ q: query });
    if (options.safeSearch === true) {
      params.set('safesearch', 'strict');
    } else if (options.safeSearch === false) {
      params.set('safesearch', 'off');
    }
    return `https://search.brave.com/search?${params}`;
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'brave',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      snippetGroup: 3,
      itemRegex:
        /<a[^>]+href="(https?:\/\/[^"]+)"[^>]*class="[^"]*result-header[^"]*"[^>]*>[\s\S]*?<span[^>]*class="[^"]*title[^"]*"[^>]*>([\s\S]*?)<\/span>[\s\S]*?<p[^>]*class="[^"]*snippet[^"]*"[^>]*>([\s\S]*?)<\/p>/g,
      skip: (url) => url.includes('search.brave.com'),
    });
  },
};

/** Mojeek — independent index, HTML SERP. */
export const mojeek = {
  id: 'mojeek',
  label: 'Mojeek',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const params = new URLSearchParams({ q: query });
    if (options.safeSearch === false) {
      params.set('safe', '0');
    }
    return `https://www.mojeek.com/search?${params}`;
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'mojeek',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      snippetGroup: 3,
      itemRegex:
        /<a[^>]+href="(https?:\/\/[^"]+)"[^>]*class="[^"]*ob[^"]*"[^>]*>([\s\S]*?)<\/a>[\s\S]*?<p[^>]*class="[^"]*s[^"]*"[^>]*>([\s\S]*?)<\/p>/g,
      skip: (url) => url.includes('mojeek.com'),
    });
  },
};

/** Ecosia — HTML SERP (Bing-backed index). */
export const ecosia = {
  id: 'ecosia',
  label: 'Ecosia',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const params = new URLSearchParams({ q: query });
    if (options.language) {
      params.set('hl', options.language);
    }
    return `https://www.ecosia.org/search?${params}`;
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'ecosia',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      snippetGroup: 3,
      itemRegex:
        /<a[^>]+class="[^"]*result__link[^"]*"[^>]*href="(https?:\/\/[^"]+)"[^>]*>([\s\S]*?)<\/a>[\s\S]*?<(?:p|div)[^>]*class="[^"]*result__description[^"]*"[^>]*>([\s\S]*?)<\/(?:p|div)>/g,
      skip: (url) => url.includes('ecosia.org'),
    });
  },
};

/** Startpage — Google results without tracking, HTML SERP. */
export const startpage = {
  id: 'startpage',
  label: 'Startpage',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const params = new URLSearchParams({ query });
    if (options.language) {
      params.set('language', options.language);
    }
    return `https://www.startpage.com/sp/search?${params}`;
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'startpage',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      snippetGroup: 3,
      itemRegex:
        /<a[^>]+class="[^"]*result-title[^"]*"[^>]*href="(https?:\/\/[^"]+)"[^>]*>([\s\S]*?)<\/a>[\s\S]*?<p[^>]*class="[^"]*description[^"]*"[^>]*>([\s\S]*?)<\/p>/g,
      skip: (url) => url.includes('startpage.com'),
    });
  },
};

/** Yahoo Search — HTML SERP (URLs wrapped in redirects). */
export const yahoo = {
  id: 'yahoo',
  label: 'Yahoo Search',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  buildUrl(query, options = {}) {
    const params = new URLSearchParams({ p: query });
    if (options.region) {
      params.set('vc', options.region);
    }
    return `https://search.yahoo.com/search?${params}`;
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'yahoo',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      urlTransform: resolveYahooHref,
      itemRegex:
        /<h3[^>]*class="[^"]*title[^"]*"[^>]*>[\s\S]*?<a[^>]+href="([^"]+)"[^>]*>([\s\S]*?)<\/a>/g,
      skip: (url) => !url || url.includes('yahoo.com'),
    });
  },
};

/** DuckDuckGo Lite — lightweight table-based SERP. */
export const lite = {
  id: 'lite',
  label: 'DuckDuckGo Lite',
  category: 'search',
  kind: 'html',
  corsReadable: false,
  method: 'POST',
  buildUrl() {
    return 'https://lite.duckduckgo.com/lite/';
  },
  buildBody(query, options = {}) {
    const params = new URLSearchParams({ q: query });
    if (options.region) {
      params.set('kl', options.region);
    }
    return params.toString();
  },
  parse(html, limit) {
    return parseAnchorList(html, {
      source: 'lite',
      limit,
      urlGroup: 1,
      titleGroup: 2,
      itemRegex:
        /<a[^>]+class="[^"]*result-link[^"]*"[^>]*href="(https?:\/\/[^"]+)"[^>]*>([\s\S]*?)<\/a>/g,
      skip: (url) => url.includes('duckduckgo.com'),
    });
  },
};

/** All HTML-scraping engine descriptors keyed by id. */
export const HTML_ENGINES = {
  brave,
  mojeek,
  ecosia,
  startpage,
  yahoo,
  lite,
};
