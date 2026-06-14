/**
 * Unit tests for the web-search CLI argument parser.
 *
 * Focus: the HTTP-server entry point can be started either with the
 * `--serve` flag or the `serve` subcommand (cross-language parity with the
 * Rust CLI, which uses `web-search serve --port <port>`).
 *
 * Works with Node.js, Bun, and Deno.
 */

import { describe, it, expect } from 'test-anywhere';
import { parseArgs } from '../bin/web-search.js';

describe('CLI parseArgs', () => {
  it('parses a plain search query', () => {
    const args = parseArgs(['artificial', 'intelligence']);
    expect(args.query).toBe('artificial intelligence');
    expect(args.serve).toBe(false);
  });

  it('starts the server with the --serve flag', () => {
    const args = parseArgs(['--serve', '--port', '8080']);
    expect(args.serve).toBe(true);
    expect(args.port).toBe(8080);
    expect(args.query).toBe(null);
  });

  it('starts the server with the serve subcommand (Rust parity)', () => {
    const args = parseArgs(['serve', '--port', '3000']);
    expect(args.serve).toBe(true);
    expect(args.port).toBe(3000);
    expect(args.query).toBe(null);
  });

  it('serve subcommand defaults to port 3000', () => {
    const args = parseArgs(['serve']);
    expect(args.serve).toBe(true);
    expect(args.port).toBe(3000);
  });

  it('does not treat a query containing "serve" as a subcommand', () => {
    const args = parseArgs(['how', 'to', 'serve', 'coffee']);
    expect(args.serve).toBe(false);
    expect(args.query).toBe('how to serve coffee');
  });

  it('parses provider and format options', () => {
    const args = parseArgs([
      'machine learning',
      '--providers',
      'google,bing',
      '--format',
      'json',
      '--limit',
      '20',
    ]);
    expect(args.providers).toEqual(['google', 'bing']);
    expect(args.format).toBe('json');
    expect(args.limit).toBe(20);
  });
});
