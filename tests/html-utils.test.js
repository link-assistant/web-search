/**
 * Unit tests for shared HTML parsing utilities.
 */

import { describe, it, expect } from 'test-anywhere';
import {
  decodeHtmlEntities,
  stripHtml,
  cleanText,
} from '../src/providers/html-utils.js';

describe('html-utils', () => {
  describe('decodeHtmlEntities', () => {
    it('decodes named entities', () => {
      expect(decodeHtmlEntities('a &amp; b &lt;c&gt; &quot;d&quot;')).toBe(
        'a & b <c> "d"'
      );
    });

    it('decodes apostrophe entities', () => {
      expect(decodeHtmlEntities('it&#39;s &apos;ok&apos;')).toBe("it's 'ok'");
    });

    it('decodes non-breaking spaces', () => {
      expect(decodeHtmlEntities('a&nbsp;b')).toBe('a b');
    });

    it('decodes decimal numeric references', () => {
      expect(decodeHtmlEntities('&#65;&#66;&#67;')).toBe('ABC');
    });

    it('decodes hexadecimal numeric references', () => {
      expect(decodeHtmlEntities('&#x41;&#x42;')).toBe('AB');
    });

    it('returns empty string for falsy input', () => {
      expect(decodeHtmlEntities('')).toBe('');
      expect(decodeHtmlEntities(undefined)).toBe('');
      expect(decodeHtmlEntities(null)).toBe('');
    });
  });

  describe('stripHtml', () => {
    it('removes tags and trims', () => {
      expect(stripHtml('  <b>bold</b> <i>text</i>  ')).toBe('bold text');
    });

    it('handles self-closing and attribute-laden tags', () => {
      expect(stripHtml('<a href="x">link</a><br/>')).toBe('link');
    });

    it('returns empty string for falsy input', () => {
      expect(stripHtml('')).toBe('');
      expect(stripHtml(undefined)).toBe('');
    });
  });

  describe('cleanText', () => {
    it('strips tags, decodes entities, and collapses whitespace', () => {
      expect(cleanText('<b>a</b>   &amp;\n\t  <i>b</i>')).toBe('a & b');
    });

    it('returns empty string for falsy input', () => {
      expect(cleanText('')).toBe('');
      expect(cleanText(undefined)).toBe('');
    });
  });
});
