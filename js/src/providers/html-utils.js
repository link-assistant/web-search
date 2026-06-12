/**
 * Shared HTML parsing utilities for search providers.
 *
 * Centralizes the text-cleaning helpers that every HTML-scraping provider
 * needs (entity decoding, tag stripping, whitespace normalization) so the
 * logic lives in exactly one place instead of being copied into each provider.
 *
 * @module providers/html-utils
 */

/**
 * Named HTML entities decoded by {@link decodeHtmlEntities}, plus numeric
 * (decimal and hexadecimal) character references.
 */
const NAMED_ENTITIES = {
  '&amp;': '&',
  '&lt;': '<',
  '&gt;': '>',
  '&quot;': '"',
  '&#39;': "'",
  '&apos;': "'",
  '&nbsp;': ' ',
  '&hellip;': '…',
  '&mdash;': '—',
  '&ndash;': '–',
};

/**
 * Decode the common HTML entities (named + numeric) found in search results.
 *
 * @param {string} text - Raw text that may contain HTML entities
 * @returns {string} Text with entities decoded
 */
export function decodeHtmlEntities(text) {
  if (!text) {
    return '';
  }
  return String(text)
    .replace(
      /&(?:amp|lt|gt|quot|#39|apos|nbsp|hellip|mdash|ndash);/g,
      (m) => NAMED_ENTITIES[m]
    )
    .replace(/&#(\d+);/g, (_, code) => String.fromCodePoint(Number(code)))
    .replace(/&#x([0-9a-fA-F]+);/g, (_, code) =>
      String.fromCodePoint(parseInt(code, 16))
    );
}

/**
 * Strip HTML tags from a string and trim the result.
 *
 * @param {string} html - Markup to strip
 * @returns {string} Plain text
 */
export function stripHtml(html) {
  if (!html) {
    return '';
  }
  return String(html)
    .replace(/<[^>]+>/g, '')
    .trim();
}

/**
 * Normalize a snippet/title: strip tags, decode entities, and collapse
 * runs of whitespace into a single space.
 *
 * @param {string} text - Raw text/markup
 * @returns {string} Cleaned single-line text
 */
export function cleanText(text) {
  if (!text) {
    return '';
  }
  return decodeHtmlEntities(stripHtml(text)).replace(/\s+/g, ' ').trim();
}
