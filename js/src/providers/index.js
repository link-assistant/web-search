/**
 * Search providers module
 * Exports all available search providers, the descriptor-driven generic
 * provider, the web-capture component-library provider, and the typed
 * registry that ties every engine together.
 */

export { BaseSearchProvider } from './base.js';
export { GoogleProvider } from './google.js';
export { DuckDuckGoProvider } from './duckduckgo.js';
export { BingProvider } from './bing.js';
export { BrowserSearchProvider, createBrowserProvider } from './browser.js';
export { GenericProvider, createGenericProvider } from './generic.js';
export { WebCaptureProvider, createWebCaptureProvider } from './web-capture.js';

export { decodeHtmlEntities, stripHtml, cleanText } from './html-utils.js';

export { API_ENGINES } from './api-engines.js';
export { HTML_ENGINES, parseAnchorList } from './html-engines.js';

export {
  CATEGORIES,
  getRegistry,
  getProviderIds,
  getDefaultProviderIds,
  buildProviders,
} from './registry.js';

import { getProviderIds } from './registry.js';

/**
 * Get list of all available provider ids (every registered engine).
 * @returns {string[]}
 */
export function getAvailableProviders() {
  return getProviderIds();
}
