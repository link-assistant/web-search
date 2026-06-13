/**
 * Unit tests for HTML-scraping engine descriptors. Parsers are exercised
 * against small HTML fixtures so the regex extraction is verified offline.
 */

import { describe, it, expect } from 'test-anywhere';
import {
  HTML_ENGINES,
  parseAnchorList,
  resolveYahooHref,
  brave,
  mojeek,
  ecosia,
  startpage,
  yahoo,
  lite,
} from '../src/providers/html-engines.js';

describe('html-engines', () => {
  describe('catalog', () => {
    it('exposes every engine in HTML_ENGINES', () => {
      expect(Object.keys(HTML_ENGINES).sort()).toEqual(
        [
          'brave',
          'cambridge-dictionary',
          'collins-dictionary',
          'dictionary-com',
          'ecosia',
          'lite',
          'merriam-webster',
          'mojeek',
          'startpage',
          'yahoo',
          'yandex',
        ].sort()
      );
    });
  });

  describe('parseAnchorList', () => {
    const html = `
      <a href="https://a.example" class="x">Title A</a><p class="s">Snippet A</p>
      <a href="https://b.example" class="x">Title B</a><p class="s">Snippet B</p>
    `;
    const config = {
      itemRegex:
        /<a[^>]+href="(https?:\/\/[^"]+)"[^>]*class="x"[^>]*>([\s\S]*?)<\/a><p[^>]*class="s"[^>]*>([\s\S]*?)<\/p>/g,
      source: 'test',
      limit: 10,
      urlGroup: 1,
      titleGroup: 2,
      snippetGroup: 3,
    };

    it('extracts ranked results', () => {
      const results = parseAnchorList(html, config);
      expect(results.length).toBe(2);
      expect(results[0].title).toBe('Title A');
      expect(results[0].url).toBe('https://a.example');
      expect(results[0].snippet).toBe('Snippet A');
      expect(results[0].rank).toBe(1);
      expect(results[1].rank).toBe(2);
    });

    it('respects the limit', () => {
      const results = parseAnchorList(html, { ...config, limit: 1 });
      expect(results.length).toBe(1);
    });

    it('deduplicates repeated URLs', () => {
      const dup = html + html;
      const results = parseAnchorList(dup, config);
      expect(results.length).toBe(2);
    });

    it('skips URLs rejected by the skip predicate', () => {
      const results = parseAnchorList(html, {
        ...config,
        skip: (url) => url.includes('a.example'),
      });
      expect(results.length).toBe(1);
      expect(results[0].url).toBe('https://b.example');
    });

    it('applies a urlTransform', () => {
      const results = parseAnchorList(html, {
        ...config,
        urlTransform: (url) => url.replace('https://', 'http://'),
      });
      expect(results[0].url).toBe('http://a.example');
    });

    it('returns [] for empty input', () => {
      expect(parseAnchorList('', config).length).toBe(0);
      expect(parseAnchorList(undefined, config).length).toBe(0);
    });
  });

  describe('resolveYahooHref', () => {
    it('decodes RU redirect URLs', () => {
      const href = '/url/RU=https%3A%2F%2Fdest.example%2Fpage/RK=2/RS=abc';
      expect(resolveYahooHref(href)).toBe('https://dest.example/page');
    });

    it('returns the href unchanged when no redirect present', () => {
      expect(resolveYahooHref('https://plain.example')).toBe(
        'https://plain.example'
      );
    });

    it('returns empty string for falsy input', () => {
      expect(resolveYahooHref('')).toBe('');
    });
  });

  describe('buildUrl variants', () => {
    it('brave honors safeSearch', () => {
      expect(
        brave.buildUrl('x', { safeSearch: true }).includes('safesearch=strict')
      ).toBe(true);
      expect(
        brave.buildUrl('x', { safeSearch: false }).includes('safesearch=off')
      ).toBe(true);
    });

    it('mojeek disables safe search', () => {
      expect(
        mojeek.buildUrl('x', { safeSearch: false }).includes('safe=0')
      ).toBe(true);
    });

    it('ecosia passes the language', () => {
      expect(ecosia.buildUrl('x', { language: 'fr' }).includes('hl=fr')).toBe(
        true
      );
    });

    it('startpage passes the language', () => {
      expect(
        startpage.buildUrl('x', { language: 'es' }).includes('language=es')
      ).toBe(true);
    });

    it('yahoo passes the region', () => {
      expect(yahoo.buildUrl('x', { region: 'uk' }).includes('vc=uk')).toBe(
        true
      );
    });

    it('lite posts to the lite endpoint with a body', () => {
      expect(lite.buildUrl()).toBe('https://lite.duckduckgo.com/lite/');
      expect(
        lite.buildBody('cats', { region: 'us-en' }).includes('q=cats')
      ).toBe(true);
      expect(lite.method).toBe('POST');
    });
  });

  describe('engine parsers', () => {
    it('brave parses a result block', () => {
      const html = `<a href="https://r.example" class="result-header"><span class="title">R Title</span></a><p class="snippet">R snip</p>`;
      const results = brave.parse(html, 10);
      expect(results.length).toBe(1);
      expect(results[0].title).toBe('R Title');
      expect(results[0].source).toBe('brave');
    });

    it('yahoo decodes redirect URLs in results', () => {
      const html = `<h3 class="title"><a href="/url/RU=https%3A%2F%2Fdest.example%2F/RK=1">Dest</a></h3>`;
      const results = yahoo.parse(html, 10);
      expect(results.length).toBe(1);
      expect(results[0].url).toBe('https://dest.example/');
    });

    it('lite parses table result links', () => {
      const html = `<a class="result-link" href="https://lr.example">Lite Result</a>`;
      const results = lite.parse(html, 10);
      expect(results.length).toBe(1);
      expect(results[0].url).toBe('https://lr.example');
    });
  });
});
