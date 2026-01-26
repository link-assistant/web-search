/**
 * Search providers module
 * Exports all available search providers
 */

export { BaseSearchProvider } from './base.js';
export { GoogleProvider } from './google.js';
export { DuckDuckGoProvider } from './duckduckgo.js';
export { BingProvider } from './bing.js';
export { BrowserSearchProvider, createBrowserProvider } from './browser.js';

/**
 * Get list of available provider names
 * @returns {string[]}
 */
export function getAvailableProviders() {
  return ['google', 'duckduckgo', 'bing'];
}
