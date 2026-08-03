/**
 * Google search provider
 * Uses Google Custom Search JSON API when API key is available
 * Falls back to web scraping for basic functionality
 */

import { BaseSearchProvider } from './base.js';
import { decodeHtmlEntities } from './html-utils.js';
import { request } from '../transport.js';

/**
 * Google search provider implementation
 * @extends BaseSearchProvider
 */
export class GoogleProvider extends BaseSearchProvider {
  /**
   * @param {Object} [config]
   * @param {string} [config.apiKey] - Google Custom Search API key
   * @param {string} [config.searchEngineId] - Google Custom Search Engine ID (cx)
   */
  constructor(config = {}) {
    super('google');
    this.apiKey = config.apiKey || process.env.GOOGLE_API_KEY;
    this.searchEngineId = config.searchEngineId || process.env.GOOGLE_CX;
    this.transport = config.transport || config.fetchImpl;
    this.apiUrl = 'https://www.googleapis.com/customsearch/v1';
    this.webUrl = 'https://www.google.com/search';
  }

  /**
   * Check if API credentials are configured
   * @returns {boolean}
   */
  hasApiCredentials() {
    return Boolean(this.apiKey && this.searchEngineId);
  }

  /**
   * Search Google for results
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
   * Search using Google Custom Search API
   * @param {string} query
   * @param {import('./base.js').SearchOptions} options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async searchWithApi(query, options) {
    const limit = Math.min(options.limit || 10, 10);

    try {
      const params = new URLSearchParams({
        key: this.apiKey,
        cx: this.searchEngineId,
        q: query,
        num: String(limit),
      });

      if (options.language) {
        params.set('lr', `lang_${options.language}`);
      }

      if (options.region) {
        params.set('gl', options.region);
      }

      if (options.safeSearch === true) {
        params.set('safe', 'active');
      } else if (options.safeSearch === false) {
        params.set('safe', 'off');
      }

      const response = await request(
        this.transport,
        `${this.apiUrl}?${params}`,
        {},
        options
      );

      if (!response.ok) {
        const error = await response.text();
        throw new Error(`Google API error: ${response.status} - ${error}`);
      }

      const data = await response.json();
      return this.parseApiResults(data);
    } catch (error) {
      console.error(`Google API search error: ${error.message}`);
      return this.searchWithScraping(query, options);
    }
  }

  /**
   * Parse Google Custom Search API response
   * @param {Object} data - API response data
   * @returns {import('./base.js').SearchResult[]}
   */
  parseApiResults(data) {
    if (!data.items || !Array.isArray(data.items)) {
      return [];
    }

    return data.items.map((item, index) => ({
      title: item.title || 'Untitled',
      url: item.link,
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
        num: String(Math.min(limit, 20)),
      });

      if (options.language) {
        params.set('hl', options.language);
      }

      if (options.region) {
        params.set('gl', options.region);
      }

      if (options.safeSearch === true) {
        params.set('safe', 'active');
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
        throw new Error(`Google returned status ${response.status}`);
      }

      const html = await response.text();
      return this.parseScrapedResults(html, limit);
    } catch (error) {
      if (options.throwOnError) {
        throw error;
      }
      console.error(`Google scraping search error: ${error.message}`);
      return [];
    }
  }

  /**
   * Parse scraped Google HTML results
   * @param {string} html - The HTML response
   * @param {number} limit - Maximum number of results
   * @returns {import('./base.js').SearchResult[]}
   */
  parseScrapedResults(html, limit) {
    const results = [];

    const resultPattern =
      /<a[^>]+href="\/url\?q=([^&"]+)[^"]*"[^>]*>.*?<h3[^>]*>([^<]+)<\/h3>/gs;
    const alternativePattern =
      /<a[^>]+href="(https?:\/\/[^"]+)"[^>]*>.*?<h3[^>]*>([^<]+)<\/h3>/gs;

    let match;
    const seen = new Set();

    for (const pattern of [resultPattern, alternativePattern]) {
      while ((match = pattern.exec(html)) !== null && results.length < limit) {
        const url = decodeURIComponent(match[1]).split('&')[0];
        const title = decodeHtmlEntities(match[2].trim());

        if (
          seen.has(url) ||
          url.includes('google.com') ||
          url.startsWith('/search')
        ) {
          continue;
        }

        seen.add(url);

        results.push({
          title: title || 'Untitled',
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
