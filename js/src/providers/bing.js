/**
 * Bing search provider
 * Uses Bing Web Search API when API key is available
 * Falls back to web scraping for basic functionality
 */

import { BaseSearchProvider } from './base.js';
import { decodeHtmlEntities, stripHtml } from './html-utils.js';
import { request } from '../transport.js';

/**
 * Bing search provider implementation
 * @extends BaseSearchProvider
 */
export class BingProvider extends BaseSearchProvider {
  /**
   * @param {Object} [config]
   * @param {string} [config.apiKey] - Bing Search API key
   */
  constructor(config = {}) {
    super('bing');
    this.apiKey = config.apiKey || process.env.BING_API_KEY;
    this.transport = config.transport || config.fetchImpl;
    this.apiUrl = 'https://api.bing.microsoft.com/v7.0/search';
    this.webUrl = 'https://www.bing.com/search';
  }

  /**
   * Check if API credentials are configured
   * @returns {boolean}
   */
  hasApiCredentials() {
    return Boolean(this.apiKey);
  }

  /**
   * Search Bing for results
   * @param {string} query - The search query
   * @param {import('./base.js').SearchOptions} [options] - Search options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string') {
      return [];
    }

    if (this.hasApiCredentials()) {
      return await this.searchWithApi(query, options);
    }

    return await this.searchWithScraping(query, options);
  }

  /**
   * Search using Bing Web Search API
   * @param {string} query
   * @param {import('./base.js').SearchOptions} options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async searchWithApi(query, options) {
    const limit = Math.min(options.limit || 10, 50);

    try {
      const params = new URLSearchParams({
        q: query,
        count: String(limit),
        responseFilter: 'Webpages',
      });

      if (options.region) {
        params.set(
          'mkt',
          `${options.language || 'en'}-${options.region.toUpperCase()}`
        );
      }

      if (options.safeSearch === true) {
        params.set('safeSearch', 'Strict');
      } else if (options.safeSearch === false) {
        params.set('safeSearch', 'Off');
      } else {
        params.set('safeSearch', 'Moderate');
      }

      const response = await request(
        this.transport,
        `${this.apiUrl}?${params}`,
        {
          headers: {
            'Ocp-Apim-Subscription-Key': this.apiKey,
          },
        },
        options
      );

      if (!response.ok) {
        const error = await response.text();
        throw new Error(`Bing API error: ${response.status} - ${error}`);
      }

      const data = await response.json();
      return this.parseApiResults(data);
    } catch (error) {
      console.error(`Bing API search error: ${error.message}`);
      return this.searchWithScraping(query, options);
    }
  }

  /**
   * Parse Bing Web Search API response
   * @param {Object} data - API response data
   * @returns {import('./base.js').SearchResult[]}
   */
  parseApiResults(data) {
    if (!data.webPages || !data.webPages.value) {
      return [];
    }

    return data.webPages.value.map((item, index) => ({
      title: item.name || 'Untitled',
      url: item.url,
      snippet: item.snippet || '',
      source: this.name,
      rank: index + 1,
    }));
  }

  /**
   * Search using web scraping (fallback)
   * @param {string} query
   * @param {import('./base.js').SearchOptions} options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async searchWithScraping(query, options) {
    const limit = options.limit || 10;

    try {
      const params = new URLSearchParams({
        q: query,
        count: String(Math.min(limit, 30)),
      });

      if (options.region) {
        params.set('cc', options.region.toUpperCase());
      }

      if (options.safeSearch === true) {
        params.set('safeSearch', 'Strict');
      } else if (options.safeSearch === false) {
        params.set('safeSearch', 'Off');
      }

      const response = await request(
        this.transport,
        `${this.webUrl}?${params}`,
        {
          headers: {
            'User-Agent':
              'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            Accept:
              'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
            'Accept-Language': 'en-US,en;q=0.5',
          },
        },
        options
      );

      if (!response.ok) {
        throw new Error(`Bing returned status ${response.status}`);
      }

      const html = await response.text();
      return this.parseScrapedResults(html, limit);
    } catch (error) {
      if (options.throwOnError) {
        throw error;
      }
      console.error(`Bing scraping search error: ${error.message}`);
      return [];
    }
  }

  /**
   * Parse scraped Bing HTML results
   * @param {string} html - The HTML response
   * @param {number} limit - Maximum number of results
   * @returns {import('./base.js').SearchResult[]}
   */
  parseScrapedResults(html, limit) {
    const results = [];

    const resultPattern =
      /<li[^>]+class="b_algo"[^>]*>.*?<a[^>]+href="([^"]+)"[^>]*>([^<]*(?:<[^>]+>[^<]*)*)<\/a>.*?<p[^>]*>([^<]*(?:<[^>]+>[^<]*)*)<\/p>/gs;

    let match;
    while (
      (match = resultPattern.exec(html)) !== null &&
      results.length < limit
    ) {
      const url = match[1];
      const title = stripHtml(match[2]);
      const snippet = stripHtml(match[3]);

      if (url.includes('bing.com') || url.startsWith('/')) {
        continue;
      }

      results.push({
        title: decodeHtmlEntities(title) || 'Untitled',
        url,
        snippet: decodeHtmlEntities(snippet) || '',
        source: this.name,
        rank: results.length + 1,
      });
    }

    if (results.length === 0) {
      const simplePattern =
        /<a[^>]+href="(https?:\/\/[^"]+)"[^>]*>.*?<h2[^>]*>([^<]+)<\/h2>/gs;
      while (
        (match = simplePattern.exec(html)) !== null &&
        results.length < limit
      ) {
        const url = match[1];
        const title = match[2];

        if (url.includes('bing.com') || url.includes('microsoft.com/maps')) {
          continue;
        }

        results.push({
          title: decodeHtmlEntities(title) || 'Untitled',
          url,
          snippet: '',
          source: this.name,
          rank: results.length + 1,
        });
      }
    }

    return results;
  }
}
