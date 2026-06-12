/**
 * Unit tests for API-based engine descriptors. Each parser is exercised against
 * a recorded-shape fixture so the normalization logic is verified without any
 * network access.
 */

import { describe, it, expect } from 'test-anywhere';
import {
  API_ENGINES,
  wikipedia,
  wikidata,
  searx,
  crossref,
  openalex,
  github,
  hackernews,
  arxiv,
  reconstructInvertedAbstract,
  parseArxivAtom,
} from '../src/providers/api-engines.js';

describe('api-engines', () => {
  describe('catalog', () => {
    it('exposes every engine in API_ENGINES', () => {
      expect(Object.keys(API_ENGINES).sort()).toEqual(
        [
          'arxiv',
          'crossref',
          'github',
          'hackernews',
          'openalex',
          'searx',
          'wikidata',
          'wikipedia',
        ].sort()
      );
    });

    it('each descriptor has a stable id and category', () => {
      for (const [key, d] of Object.entries(API_ENGINES)) {
        expect(d.id).toBe(key);
        expect(typeof d.label).toBe('string');
        expect(
          ['search', 'knowledge', 'papers', 'code'].includes(d.category)
        ).toBe(true);
        expect(typeof d.buildUrl).toBe('function');
        expect(typeof d.parse).toBe('function');
      }
    });
  });

  describe('wikipedia', () => {
    it('builds a localized REST search URL', () => {
      const url = wikipedia.buildUrl('quantum', { limit: 5, language: 'de' });
      expect(url.includes('de.wikipedia.org')).toBe(true);
      expect(url.includes('q=quantum')).toBe(true);
      expect(url.includes('limit=5')).toBe(true);
    });

    it('parses pages into results with article URLs', () => {
      const data = {
        pages: [
          {
            key: 'Quantum_mechanics',
            title: 'Quantum mechanics',
            excerpt: 'Q <b>m</b>',
          },
          { key: 'Quantum', title: 'Quantum', description: 'desc' },
        ],
      };
      const results = wikipedia.parse(data, 10, { language: 'en' });
      expect(results.length).toBe(2);
      expect(results[0].url).toBe(
        'https://en.wikipedia.org/wiki/Quantum_mechanics'
      );
      expect(results[0].snippet).toBe('Q m');
      expect(results[0].source).toBe('wikipedia');
      expect(results[0].rank).toBe(1);
    });

    it('returns [] for malformed payloads', () => {
      expect(wikipedia.parse({}, 10).length).toBe(0);
      expect(wikipedia.parse(null, 10).length).toBe(0);
    });
  });

  describe('wikidata', () => {
    it('parses entities and falls back to concepturi', () => {
      const data = {
        search: [
          {
            id: 'Q42',
            label: 'Douglas Adams',
            description: 'author',
            concepturi: 'http://www.wikidata.org/entity/Q42',
          },
          { id: 'Q1', label: 'Universe' },
        ],
      };
      const results = wikidata.parse(data, 10);
      expect(results.length).toBe(2);
      expect(results[0].url).toBe('http://www.wikidata.org/entity/Q42');
      expect(results[1].url).toBe('https://www.wikidata.org/wiki/Q1');
    });
  });

  describe('searx', () => {
    it('honors a custom instance', () => {
      const url = searx.buildUrl('cats', {
        searxInstance: 'https://my.searx/',
      });
      expect(url.startsWith('https://my.searx/search?format=json')).toBe(true);
    });

    it('parses results', () => {
      const data = {
        results: [{ title: 'T', url: 'https://t.example', content: 'c' }],
      };
      const results = searx.parse(data, 10);
      expect(results.length).toBe(1);
      expect(results[0].snippet).toBe('c');
    });
  });

  describe('crossref', () => {
    it('parses works and drops entries without a URL', () => {
      const data = {
        message: {
          items: [
            {
              title: ['A Paper'],
              DOI: '10.1/x',
              'container-title': ['Journal'],
            },
            { title: ['No URL'] },
          ],
        },
      };
      const results = crossref.parse(data, 10);
      expect(results.length).toBe(1);
      expect(results[0].url).toBe('https://doi.org/10.1/x');
      expect(results[0].title).toBe('A Paper');
    });
  });

  describe('openalex', () => {
    it('reconstructs abstracts from the inverted index', () => {
      const data = {
        results: [
          {
            display_name: 'Deep Learning',
            doi: 'https://doi.org/10.2/y',
            abstract_inverted_index: { Deep: [0], learning: [1], works: [2] },
          },
        ],
      };
      const results = openalex.parse(data, 10);
      expect(results.length).toBe(1);
      expect(results[0].snippet).toBe('Deep learning works');
      expect(results[0].url).toBe('https://doi.org/10.2/y');
    });
  });

  describe('github', () => {
    it('parses repositories', () => {
      const data = {
        items: [
          {
            full_name: 'a/b',
            html_url: 'https://github.com/a/b',
            description: 'd',
          },
        ],
      };
      const results = github.parse(data, 10);
      expect(results[0].title).toBe('a/b');
      expect(results[0].url).toBe('https://github.com/a/b');
    });

    it('sets the GitHub API headers', () => {
      const headers = github.headers();
      expect(headers.Accept).toBe('application/vnd.github+json');
    });
  });

  describe('hackernews', () => {
    it('falls back to the HN item URL when story has none', () => {
      const data = {
        hits: [
          { title: 'Show HN', objectID: '123' },
          { title: 'External', url: 'https://ext.example' },
        ],
      };
      const results = hackernews.parse(data, 10);
      expect(results[0].url).toBe('https://news.ycombinator.com/item?id=123');
      expect(results[1].url).toBe('https://ext.example');
    });
  });

  describe('arxiv', () => {
    it('parses an Atom feed', () => {
      const xml = `<feed>
        <entry><title>Paper One</title><id>http://arxiv.org/abs/1</id><summary>Sum one</summary></entry>
        <entry><title>Paper Two</title><id>http://arxiv.org/abs/2</id><summary>Sum two</summary></entry>
      </feed>`;
      const results = arxiv.parse(xml, 10);
      expect(results.length).toBe(2);
      expect(results[0].title).toBe('Paper One');
      expect(results[0].url).toBe('http://arxiv.org/abs/1');
    });

    it('respects the limit', () => {
      const xml = `<feed>
        <entry><title>A</title><id>http://arxiv.org/abs/1</id><summary>x</summary></entry>
        <entry><title>B</title><id>http://arxiv.org/abs/2</id><summary>y</summary></entry>
      </feed>`;
      expect(parseArxivAtom(xml, 1).length).toBe(1);
    });
  });

  describe('reconstructInvertedAbstract', () => {
    it('orders words by their positions', () => {
      expect(reconstructInvertedAbstract({ world: [1], hello: [0] })).toBe(
        'hello world'
      );
    });

    it('returns empty string for falsy input', () => {
      expect(reconstructInvertedAbstract(undefined)).toBe('');
      expect(reconstructInvertedAbstract(null)).toBe('');
    });
  });
});
