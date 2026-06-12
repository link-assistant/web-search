/**
 * DuckDuckGo search provider
 * Uses DuckDuckGo's HTML search page since they don't have a public API
 */

import { BaseSearchProvider } from './base.js';
import { decodeHtmlEntities, stripHtml } from './html-utils.js';

/**
 * DuckDuckGo search provider implementation
 * @extends BaseSearchProvider
 */
export class DuckDuckGoProvider extends BaseSearchProvider {
  constructor() {
    super('duckduckgo');
    this.baseUrl = 'https://html.duckduckgo.com/html/';
  }

  /**
   * Search DuckDuckGo for results
   * @param {string} query - The search query
   * @param {import('./base.js').SearchOptions} [options] - Search options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string') {
      return [];
    }

    const limit = options.limit || 10;

    try {
      const params = new URLSearchParams({
        q: query,
        kl: options.region || 'wt-wt',
      });

      if (options.safeSearch === false) {
        params.set('kp', '-2');
      } else if (options.safeSearch === true) {
        params.set('kp', '1');
      }

      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
          'User-Agent':
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
        },
        body: params.toString(),
      });

      if (!response.ok) {
        throw new Error(`DuckDuckGo returned status ${response.status}`);
      }

      const html = await response.text();
      return this.parseResults(html, limit);
    } catch (error) {
      console.error(`DuckDuckGo search error: ${error.message}`);
      return [];
    }
  }

  /**
   * Parse DuckDuckGo HTML response to extract search results
   * @param {string} html - The HTML response
   * @param {number} limit - Maximum number of results
   * @returns {import('./base.js').SearchResult[]}
   */
  parseResults(html, limit) {
    const results = [];

    const resultPattern =
      /<a[^>]+class="result__a"[^>]*href="([^"]+)"[^>]*>([^<]+)<\/a>/g;
    const snippetPattern =
      /<a[^>]+class="result__snippet"[^>]*>([^<]*(?:<[^>]+>[^<]*)*)<\/a>/g;

    const urls = [];
    const titles = [];
    const snippets = [];

    let match;
    while ((match = resultPattern.exec(html)) !== null) {
      urls.push(decodeURIComponent(match[1]));
      titles.push(decodeHtmlEntities(match[2].trim()));
    }

    while ((match = snippetPattern.exec(html)) !== null) {
      snippets.push(decodeHtmlEntities(stripHtml(match[1])));
    }

    for (let i = 0; i < Math.min(urls.length, limit); i++) {
      const url = urls[i];
      if (
        !url ||
        url.startsWith('//duckduckgo.com') ||
        url.includes('ad_provider')
      ) {
        continue;
      }

      results.push({
        title: titles[i] || 'Untitled',
        url,
        snippet: snippets[i] || '',
        source: this.name,
        rank: i + 1,
      });
    }

    return results;
  }
}
