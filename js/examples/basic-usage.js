/**
 * Basic usage example
 * Demonstrates registry discovery and result merging without network calls.
 *
 * Run with any runtime:
 * - Bun: bun examples/basic-usage.js
 * - Node.js: node examples/basic-usage.js
 * - Deno: deno run examples/basic-usage.js
 */

import { getProviderIds, mergeResults } from '../src/index.js';

console.log('Default search providers:');
console.log(`  ${getProviderIds('search').slice(0, 5).join(', ')} ...`);

const merged = mergeResults(
  {
    duckduckgo: [
      {
        title: 'Example result',
        url: 'https://example.com/result',
        snippet: 'From DuckDuckGo',
        source: 'duckduckgo',
        rank: 1,
      },
    ],
    wikipedia: [
      {
        title: 'Example result',
        url: 'https://example.com/result/',
        snippet: 'From Wikipedia',
        source: 'wikipedia',
        rank: 1,
      },
    ],
  },
  { strategy: 'rrf' }
);

console.log('\nMerged results:');
console.log(JSON.stringify(merged, null, 2));
