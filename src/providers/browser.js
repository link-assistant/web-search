/**
 * Browser-based search provider
 * Uses browser-commander for direct browser search testing
 * This is primarily used for testing and verification purposes
 */

import { BaseSearchProvider } from './base.js';

/**
 * Browser search provider using browser-commander
 * Performs actual browser searches for testing and verification
 * @extends BaseSearchProvider
 */
export class BrowserSearchProvider extends BaseSearchProvider {
  /**
   * @param {Object} [config]
   * @param {string} [config.engine] - Search engine to use ('google', 'duckduckgo', 'bing')
   * @param {Object} [config.browserCommander] - Browser-commander instance (optional)
   */
  constructor(config = {}) {
    super('browser');
    this.targetEngine = config.engine || 'duckduckgo';
    this.browserCommander = config.browserCommander;
    this.browserOptions = config.browserOptions || {};
  }

  /**
   * Get the search URL for the target engine
   * @param {string} query - Search query
   * @param {string} engine - Target search engine
   * @returns {string} - Search URL
   */
  getSearchUrl(query, engine) {
    const encodedQuery = encodeURIComponent(query);
    const urls = {
      google: `https://www.google.com/search?q=${encodedQuery}`,
      duckduckgo: `https://duckduckgo.com/?q=${encodedQuery}`,
      bing: `https://www.bing.com/search?q=${encodedQuery}`,
    };
    return urls[engine] || urls.duckduckgo;
  }

  /**
   * Parse search results from the page using the browser
   * @param {Object} page - Browser page object
   * @param {string} engine - Target search engine
   * @param {number} limit - Max results
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async parseResultsFromPage(page, engine, limit) {
    const selectors = {
      google: {
        container: '.g',
        titleSelector: 'h3',
        linkSelector: 'a',
        snippetSelector: '.VwiC3b, [data-snf], .s3v9rd',
      },
      duckduckgo: {
        container: '.result, .react-results--main article',
        titleSelector: 'h2 a, a[data-testid="result-title-a"]',
        linkSelector: 'h2 a, a[data-testid="result-title-a"]',
        snippetSelector: '.result__snippet, [data-result="snippet"]',
      },
      bing: {
        container: '.b_algo',
        titleSelector: 'h2 a',
        linkSelector: 'h2 a',
        snippetSelector: 'p',
      },
    };

    const sel = selectors[engine] || selectors.duckduckgo;

    const results = await page.evaluate(
      (containerSel, titleSel, linkSel, snippetSel, maxResults) => {
        const containers = document.querySelectorAll(containerSel);
        const parsed = [];

        for (const container of containers) {
          if (parsed.length >= maxResults) {
            break;
          }

          const titleEl = container.querySelector(titleSel);
          const linkEl = container.querySelector(linkSel);
          const snippetEl = container.querySelector(snippetSel);

          if (!linkEl || !linkEl.href) {
            continue;
          }

          const url = linkEl.href;
          if (
            url.includes('google.com/search') ||
            url.includes('bing.com/search') ||
            url.includes('duckduckgo.com/?q')
          ) {
            continue;
          }

          parsed.push({
            title: titleEl?.textContent?.trim() || 'Untitled',
            url,
            snippet: snippetEl?.textContent?.trim() || '',
          });
        }

        return parsed;
      },
      sel.container,
      sel.titleSelector,
      sel.linkSelector,
      sel.snippetSelector,
      limit
    );

    return results.map((r, i) => ({
      ...r,
      source: `browser-${engine}`,
      rank: i + 1,
    }));
  }

  /**
   * Search using browser automation
   * @param {string} query - The search query
   * @param {import('./base.js').SearchOptions} [options] - Search options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string') {
      return [];
    }

    if (!this.browserCommander) {
      console.warn(
        'BrowserSearchProvider: browser-commander not configured, returning empty results'
      );
      return [];
    }

    const limit = options.limit || 10;
    const engine = this.targetEngine;

    try {
      const searchUrl = this.getSearchUrl(query, engine);

      const results = await this.browserCommander.runAction(
        async (page, signal) => {
          await page.goto(searchUrl, {
            waitUntil: 'networkidle0',
            timeout: 30000,
          });

          if (signal.aborted) {
            return [];
          }

          await new Promise((resolve) => setTimeout(resolve, 2000));

          return this.parseResultsFromPage(page, engine, limit);
        }
      );

      return results;
    } catch (error) {
      console.error(`Browser search error (${engine}): ${error.message}`);
      return [];
    }
  }

  /**
   * Create a browser commander instance with Puppeteer
   * @param {Object} [puppeteerOptions] - Puppeteer launch options
   * @returns {Promise<Object>} - Browser commander instance
   */
  static async createWithPuppeteer(puppeteerOptions = {}) {
    try {
      const browserCommander = await import('browser-commander');
      const puppeteer = await import('puppeteer');

      const browser = await puppeteer.default.launch({
        headless: true,
        args: ['--no-sandbox', '--disable-setuid-sandbox'],
        ...puppeteerOptions,
      });

      const commander = new browserCommander.BrowserCommander({ browser });
      return commander;
    } catch (error) {
      console.error('Failed to create browser commander:', error.message);
      return null;
    }
  }
}

/**
 * Create a browser search provider with browser-commander
 * @param {Object} [config] - Configuration options
 * @returns {BrowserSearchProvider}
 */
export function createBrowserProvider(config = {}) {
  return new BrowserSearchProvider(config);
}
