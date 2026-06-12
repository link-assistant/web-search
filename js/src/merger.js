/**
 * Search result merger with reranking support
 * Combines results from multiple search providers and reranks them
 */

/**
 * @typedef {import('./providers/base.js').SearchResult} SearchResult
 */

/**
 * @typedef {Object} MergeOptions
 * @property {'rrf' | 'weighted' | 'interleave'} [strategy] - Merge strategy
 * @property {Object<string, number>} [weights] - Weight for each provider
 * @property {number} [rrfK] - RRF k parameter (default: 60)
 * @property {boolean} [removeDuplicates] - Remove duplicate URLs (default: true)
 */

/**
 * Reciprocal Rank Fusion (RRF) score calculation
 * @param {number} rank - The rank position (1-based)
 * @param {number} k - The k parameter (default: 60)
 * @returns {number}
 */
function rrfScore(rank, k = 60) {
  return 1 / (k + rank);
}

/**
 * Normalize URL for deduplication
 * @param {string} url - The URL to normalize
 * @returns {string}
 */
function normalizeUrl(url) {
  try {
    const parsed = new URL(url);
    let normalized = `${parsed.hostname}${parsed.pathname}`;
    normalized = normalized.replace(/\/+$/, '');
    normalized = normalized.toLowerCase();
    return normalized;
  } catch {
    return url.toLowerCase();
  }
}

/**
 * Merge and rerank search results using Reciprocal Rank Fusion
 * @param {Object<string, SearchResult[]>} resultsByProvider - Results grouped by provider
 * @param {MergeOptions} [options]
 * @returns {SearchResult[]}
 */
export function mergeWithRRF(resultsByProvider, options = {}) {
  const k = options.rrfK || 60;
  const weights = options.weights || {};
  const removeDuplicates = options.removeDuplicates !== false;

  const scoresByUrl = new Map();
  const resultByUrl = new Map();

  for (const [provider, results] of Object.entries(resultsByProvider)) {
    const providerWeight = weights[provider] || 1.0;

    for (const result of results) {
      const normalizedUrl = normalizeUrl(result.url);
      const score = rrfScore(result.rank, k) * providerWeight;

      if (scoresByUrl.has(normalizedUrl)) {
        scoresByUrl.set(normalizedUrl, scoresByUrl.get(normalizedUrl) + score);
        const existing = resultByUrl.get(normalizedUrl);
        if (!existing.sources) {
          existing.sources = [existing.source];
        }
        if (!existing.sources.includes(result.source)) {
          existing.sources.push(result.source);
        }
      } else {
        scoresByUrl.set(normalizedUrl, score);
        resultByUrl.set(normalizedUrl, { ...result });
      }
    }
  }

  const merged = Array.from(scoresByUrl.entries())
    .sort((a, b) => b[1] - a[1])
    .map(([url, score], index) => ({
      ...resultByUrl.get(url),
      score,
      rank: index + 1,
    }));

  return removeDuplicates
    ? merged
    : Array.from(resultByUrl.values()).map((r, i) => ({ ...r, rank: i + 1 }));
}

/**
 * Merge and rerank search results using weighted scoring
 * @param {Object<string, SearchResult[]>} resultsByProvider - Results grouped by provider
 * @param {MergeOptions} [options]
 * @returns {SearchResult[]}
 */
export function mergeWithWeights(resultsByProvider, options = {}) {
  const weights = options.weights || {};
  const maxRank = 100;
  const removeDuplicates = options.removeDuplicates !== false;

  const scoresByUrl = new Map();
  const resultByUrl = new Map();

  for (const [provider, results] of Object.entries(resultsByProvider)) {
    const providerWeight = weights[provider] || 1.0;

    for (const result of results) {
      const normalizedUrl = normalizeUrl(result.url);
      const score = ((maxRank - result.rank + 1) / maxRank) * providerWeight;

      if (scoresByUrl.has(normalizedUrl)) {
        scoresByUrl.set(normalizedUrl, scoresByUrl.get(normalizedUrl) + score);
        const existing = resultByUrl.get(normalizedUrl);
        if (!existing.sources) {
          existing.sources = [existing.source];
        }
        if (!existing.sources.includes(result.source)) {
          existing.sources.push(result.source);
        }
      } else {
        scoresByUrl.set(normalizedUrl, score);
        resultByUrl.set(normalizedUrl, { ...result });
      }
    }
  }

  const merged = Array.from(scoresByUrl.entries())
    .sort((a, b) => b[1] - a[1])
    .map(([url, score], index) => ({
      ...resultByUrl.get(url),
      score,
      rank: index + 1,
    }));

  return removeDuplicates
    ? merged
    : Array.from(resultByUrl.values()).map((r, i) => ({ ...r, rank: i + 1 }));
}

/**
 * Merge results by interleaving (round-robin)
 * @param {Object<string, SearchResult[]>} resultsByProvider - Results grouped by provider
 * @param {MergeOptions} [options]
 * @returns {SearchResult[]}
 */
export function mergeWithInterleave(resultsByProvider, options = {}) {
  const removeDuplicates = options.removeDuplicates !== false;
  const results = [];
  const seenUrls = new Set();

  const providers = Object.entries(resultsByProvider);
  const indices = providers.map(() => 0);
  const maxLength = Math.max(...providers.map(([, r]) => r.length));

  for (let i = 0; i < maxLength; i++) {
    for (let j = 0; j < providers.length; j++) {
      const [, providerResults] = providers[j];
      if (indices[j] < providerResults.length) {
        const result = providerResults[indices[j]];
        indices[j]++;

        if (removeDuplicates) {
          const normalizedUrl = normalizeUrl(result.url);
          if (seenUrls.has(normalizedUrl)) {
            continue;
          }
          seenUrls.add(normalizedUrl);
        }

        results.push({
          ...result,
          rank: results.length + 1,
        });
      }
    }
  }

  return results;
}

/**
 * Merge search results using the specified strategy
 * @param {Object<string, SearchResult[]>} resultsByProvider - Results grouped by provider
 * @param {MergeOptions} [options]
 * @returns {SearchResult[]}
 */
export function mergeResults(resultsByProvider, options = {}) {
  const strategy = options.strategy || 'rrf';

  switch (strategy) {
    case 'weighted':
      return mergeWithWeights(resultsByProvider, options);
    case 'interleave':
      return mergeWithInterleave(resultsByProvider, options);
    case 'rrf':
    default:
      return mergeWithRRF(resultsByProvider, options);
  }
}
