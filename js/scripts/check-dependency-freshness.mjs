#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

const ISSUE = /^https:\/\/github\.com\/([\w.-]+)\/([\w.-]+)\/issues\/(\d+)$/;

async function readIssue(url) {
  const response = await fetch(url, {
    headers: {
      'User-Agent': 'web-search-dependency-freshness',
      ...(process.env.GH_TOKEN
        ? { Authorization: `Bearer ${process.env.GH_TOKEN}` }
        : {}),
    },
  });
  if (!response.ok) {
    throw new Error(`GitHub HTTP ${response.status}`);
  }
  return response.json();
}

// package.json cannot contain comments: dependencyFreshnessExceptions records
// the equivalent per-dependency blocker URL without invalidating npm's manifest.
export async function checkOutdated(
  report,
  exceptions,
  issueReader = readIssue
) {
  const errors = [];
  for (const [name, versions] of Object.entries(report)) {
    const message = `${name}: resolved ${versions.current}, latest ${versions.latest}`;
    const match = exceptions[name]?.match(ISSUE);
    if (!match) {
      errors.push(`${message}; update or cite an open blocker issue`);
      continue;
    }
    const [, owner, repo, number] = match;
    try {
      const issue = await issueReader(
        `https://api.github.com/repos/${owner}/${repo}/issues/${number}`
      );
      if (
        issue.state !== 'open' ||
        'pull_request' in issue ||
        typeof issue.body !== 'string' ||
        !issue.body.trim()
      ) {
        throw new Error('blocker must be an open issue with an explanation');
      }
      console.log(`Allowed blocker: ${message} (${exceptions[name]})`);
    } catch (error) {
      errors.push(`${message}; ${error.message}`);
    }
  }
  return errors;
}

async function main() {
  const manifest = JSON.parse(readFileSync('package.json', 'utf8'));
  let output;
  try {
    output = execFileSync('npm', ['outdated', '--json'], { encoding: 'utf8' });
  } catch (error) {
    // npm uses exit 1 for outdated packages; all other failures remain errors.
    if (error.status !== 1 || !error.stdout) {
      throw error;
    }
    output = error.stdout;
  }
  const report = JSON.parse(output || '{}');
  if (report.error) {
    throw new Error(JSON.stringify(report.error));
  }
  const errors = await checkOutdated(
    report,
    manifest.dependencyFreshnessExceptions || {}
  );
  if (errors.length) {
    throw new Error(errors.join('\n'));
  }
  console.log('JavaScript direct dependencies are current');
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  main().catch((error) => {
    console.error(`Dependency freshness failed: ${error.message}`);
    process.exitCode = 1;
  });
}
