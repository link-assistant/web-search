#!/usr/bin/env bun

/**
 * Create a changeset file for manual releases
 * Usage: node scripts/create-manual-changeset.mjs --bump-type <major|minor|patch> [--description <description>]
 *
 * IMPORTANT: Update the PACKAGE_NAME constant below to match your package.json
 *
 * Uses link-foundation libraries:
 * - use-m: Dynamic package loading without package.json dependencies
 * - command-stream: Modern shell command execution with streaming support
 * - lino-arguments: Unified configuration from CLI args, env vars, and .lenv files
 */

import { mkdirSync, writeFileSync } from 'fs';
import { randomBytes } from 'crypto';
import { join } from 'path';

import {
  getChangesetDir,
  getJsRoot,
  needsCd,
  parseJsRootConfig,
} from './js-paths.mjs';

const PACKAGE_NAME = '@link-assistant/web-search';

// Load use-m dynamically
const { use } = eval(
  await (await fetch('https://unpkg.com/use-m/use.js')).text()
);

// Import link-foundation libraries
const { $ } = await use('command-stream');
const { makeConfig } = await use('lino-arguments');

// Parse CLI arguments using lino-arguments
const config = makeConfig({
  yargs: ({ yargs, getenv }) =>
    yargs
      .option('bump-type', {
        type: 'string',
        default: getenv('BUMP_TYPE', ''),
        describe: 'Version bump type: major, minor, or patch',
        choices: ['major', 'minor', 'patch'],
      })
      .option('description', {
        type: 'string',
        default: getenv('DESCRIPTION', ''),
        describe: 'Description for the changeset',
      })
      .option('js-root', {
        type: 'string',
        default: getenv('JS_ROOT', ''),
        describe:
          'JavaScript package root directory (auto-detected if not specified)',
      }),
});

try {
  const { bumpType, description: descriptionArg, jsRoot: jsRootArg } = config;
  const jsRootConfig = jsRootArg || parseJsRootConfig();
  const jsRoot = getJsRoot({ jsRoot: jsRootConfig, verbose: true });
  const changesetDir = getChangesetDir({ jsRoot });

  // Use provided description or default based on bump type
  const description = descriptionArg || `Manual ${bumpType} release`;

  if (!bumpType || !['major', 'minor', 'patch'].includes(bumpType)) {
    console.error(
      'Usage: node scripts/create-manual-changeset.mjs --bump-type <major|minor|patch> [--description <description>]'
    );
    process.exit(1);
  }

  // Generate a random changeset ID
  const changesetId = randomBytes(4).toString('hex');
  const changesetFileName = `manual-release-${changesetId}.md`;
  const changesetFile = join(changesetDir, changesetFileName);

  // Create the changeset file with single quotes to match Prettier config
  const content = `---
'${PACKAGE_NAME}': ${bumpType}
---

${description}
`;

  mkdirSync(changesetDir, { recursive: true });
  writeFileSync(changesetFile, content, 'utf-8');

  console.log(`Created changeset: ${changesetFile}`);
  console.log('Content:');
  console.log(content);

  // Format with Prettier
  console.log('\nFormatting with Prettier...');
  if (needsCd({ jsRoot })) {
    await $`cd ${jsRoot} && npx prettier --write ".changeset/${changesetFileName}"`;
  } else {
    await $`npx prettier --write "${changesetFile}"`;
  }

  console.log('\n✅ Changeset created and formatted successfully');
} catch (error) {
  console.error('Error creating changeset:', error.message);
  if (process.env.DEBUG) {
    console.error('Stack trace:', error.stack);
  }
  process.exit(1);
}
