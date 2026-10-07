import { describe, it, expect } from 'test-anywhere';
import { checkOutdated } from '../scripts/check-dependency-freshness.mjs';

describe('dependency freshness policy', () => {
  it('accepts an up-to-date package', async () => {
    expect(await checkOutdated({}, {})).toEqual([]);
  });

  it('fails an incompatible latest release as well as a stale lockfile', async () => {
    const errors = await checkOutdated(
      { tool: { current: '1.0.0', wanted: '1.0.0', latest: '2.0.0' } },
      {}
    );
    expect(errors.length).toBe(1);
  });

  it('accepts only an open issue explaining the blocker', async () => {
    const report = { tool: { current: '1.0.0', latest: '2.0.0' } };
    const exceptions = { tool: 'https://github.com/o/r/issues/1' };
    expect(
      await checkOutdated(report, exceptions, async () => ({
        state: 'open',
        body: 'Requires a runtime migration',
      }))
    ).toEqual([]);
    for (const issue of [
      { state: 'closed' },
      { state: 'open', pull_request: {} },
      { state: 'open', body: '' },
      { state: 'open', body: '  ' },
    ]) {
      expect(
        (await checkOutdated(report, exceptions, async () => issue)).length
      ).toBe(1);
    }
  });

  it('fails when the issue cannot be verified', async () => {
    const errors = await checkOutdated(
      { tool: { current: '1.0.0', latest: '2.0.0' } },
      { tool: 'https://github.com/o/r/issues/1' },
      async () => {
        throw new Error('GitHub unavailable');
      }
    );
    expect(errors.length).toBe(1);
  });
});
