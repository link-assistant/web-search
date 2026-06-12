#!/usr/bin/env node

/**
 * Detect code changes for CI/CD pipeline
 *
 * This script detects what types of files have changed between two commits
 * and outputs the results for use in GitHub Actions workflow conditions.
 *
 * Key behavior:
 * - For PRs: compares PR head against base branch
 * - For pushes: compares HEAD against HEAD^
 * - Excludes certain folders and file types from "code changes" detection
 *
 * Excluded from code changes (don't require changesets):
 * - Markdown files (*.md) in any folder
 * - .changeset/ folder (changeset metadata)
 * - docs/ folder (documentation)
 * - experiments/ folder (experimental scripts)
 * - examples/ folder (example scripts)
 *
 * Usage:
 *   node js/scripts/detect-code-changes.mjs
 *
 * Environment variables (set by GitHub Actions):
 *   - GITHUB_EVENT_NAME: 'pull_request' or 'push'
 *   - GITHUB_BASE_SHA: Base commit SHA for PR
 *   - GITHUB_HEAD_SHA: Head commit SHA for PR
 *
 * Outputs (written to GITHUB_OUTPUT):
 *   - mjs: 'true' if any .mjs files changed
 *   - js: 'true' if any .js files changed
 *   - package: 'true' if package.json changed
 *   - js-code-changed: 'true' if JavaScript package code/config changed
 *   - rust-code-changed: 'true' if Rust crate code/config changed
 *   - docs: 'true' if any .md files changed
 *   - workflow: 'true' if any .github/workflows/ files changed
 *   - any-code-changed: 'true' if any code files changed (excludes docs, changesets, experiments, examples)
 */

import { execSync } from 'child_process';
import { appendFileSync } from 'fs';

/**
 * Execute a shell command and return trimmed output
 * @param {string} command - The command to execute
 * @returns {string} - The trimmed command output
 */
function exec(command) {
  try {
    return execSync(command, { encoding: 'utf-8' }).trim();
  } catch (error) {
    console.error(`Error executing command: ${command}`);
    console.error(error.message);
    return '';
  }
}

/**
 * Write output to GitHub Actions output file
 * @param {string} name - Output name
 * @param {string} value - Output value
 */
function setOutput(name, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${name}=${value}\n`);
  }
  console.log(`${name}=${value}`);
}

/**
 * Get the list of changed files between two commits
 * @returns {string[]} Array of changed file paths
 */
function getChangedFiles() {
  const eventName = process.env.GITHUB_EVENT_NAME || 'local';

  if (eventName === 'pull_request') {
    const baseSha = process.env.GITHUB_BASE_SHA;
    const headSha = process.env.GITHUB_HEAD_SHA;

    if (baseSha && headSha) {
      console.log(`Comparing PR: ${baseSha}...${headSha}`);
      try {
        // Ensure we have the base commit
        try {
          execSync(`git cat-file -e ${baseSha}`, { stdio: 'ignore' });
        } catch {
          console.log('Base commit not available locally, attempting fetch...');
          execSync(`git fetch origin ${baseSha}`, { stdio: 'inherit' });
        }
        const output = exec(`git diff --name-only ${baseSha} ${headSha}`);
        return output ? output.split('\n').filter(Boolean) : [];
      } catch (error) {
        console.error(`Git diff failed: ${error.message}`);
      }
    }
  }

  // For push events or fallback
  console.log('Comparing HEAD^ to HEAD');
  try {
    const output = exec('git diff --name-only HEAD^ HEAD');
    return output ? output.split('\n').filter(Boolean) : [];
  } catch {
    // If HEAD^ doesn't exist (first commit), list all files in HEAD
    console.log('HEAD^ not available, listing all files in HEAD');
    const output = exec('git ls-tree --name-only -r HEAD');
    return output ? output.split('\n').filter(Boolean) : [];
  }
}

/**
 * Check if a file should be excluded from code changes detection
 * @param {string} filePath - The file path to check
 * @returns {boolean} True if the file should be excluded
 */
function isExcludedFromCodeChanges(filePath) {
  // Exclude markdown files in any folder
  if (filePath.endsWith('.md')) {
    return true;
  }

  // Exclude specific folders from code changes
  const excludedFolders = [
    '.changeset/',
    'js/.changeset/',
    'docs/',
    'experiments/',
    'examples/',
    'js/examples/',
  ];

  for (const folder of excludedFolders) {
    if (filePath.startsWith(folder)) {
      return true;
    }
  }

  return false;
}

function setBooleanOutput(name, condition) {
  setOutput(name, condition ? 'true' : 'false');
}

function logFileList(title, files) {
  console.log(title);
  if (files.length === 0) {
    console.log('  (none)');
  } else {
    files.forEach((file) => console.log(`  ${file}`));
  }
  console.log('');
}

function isJsWorkflowFile(filePath) {
  return (
    filePath === '.github/workflows/js.yml' ||
    filePath === '.github/workflows/parity.yml'
  );
}

function isRustWorkflowFile(filePath) {
  return (
    filePath === '.github/workflows/rust.yml' ||
    filePath === '.github/workflows/parity.yml'
  );
}

function isScriptFile(filePath) {
  return filePath.startsWith('js/scripts/') || filePath.startsWith('scripts/');
}

function isJsCodeFile(filePath) {
  return (
    /^js\/(?:src|tests|bin)\//.test(filePath) ||
    /^js\/.*\.(?:js|mjs|json|toml|yml|yaml)$/.test(filePath)
  );
}

function isRustCodeFile(filePath) {
  return /^rust\/.*\.(?:rs|toml|lock|yml|yaml)$/.test(filePath);
}

function getPackageCodeFiles(changedFiles, packagePrefix) {
  return changedFiles.filter(
    (file) => file.startsWith(packagePrefix) && !isExcludedFromCodeChanges(file)
  );
}

function getCodeChangedFiles(changedFiles) {
  return changedFiles.filter((file) => !isExcludedFromCodeChanges(file));
}

function detectChangeFlags(changedFiles) {
  const jsWorkflowChanged = changedFiles.some(isJsWorkflowFile);
  const rustWorkflowChanged = changedFiles.some(isRustWorkflowFile);
  const scriptsChanged = changedFiles.some(isScriptFile);
  const jsChangedFiles = getPackageCodeFiles(changedFiles, 'js/');
  const rustChangedFiles = getPackageCodeFiles(changedFiles, 'rust/');
  const codeChangedFiles = getCodeChangedFiles(changedFiles);
  const codePattern = /\.(mjs|js|json|yml|yaml)$|\.github\/workflows\//;

  return {
    mjsChanged: changedFiles.some((file) => file.endsWith('.mjs')),
    jsChanged: changedFiles.some((file) => file.endsWith('.js')),
    packageChanged: changedFiles.some(
      (file) => file === 'package.json' || file === 'js/package.json'
    ),
    docsChanged: changedFiles.some((file) => file.endsWith('.md')),
    workflowChanged: changedFiles.some((file) =>
      file.startsWith('.github/workflows/')
    ),
    jsWorkflowChanged,
    rustWorkflowChanged,
    scriptsChanged,
    jsCodeChanged:
      jsWorkflowChanged || scriptsChanged || jsChangedFiles.some(isJsCodeFile),
    rustCodeChanged:
      rustWorkflowChanged || rustChangedFiles.some(isRustCodeFile),
    codeChanged: codeChangedFiles.some((file) => codePattern.test(file)),
    codeChangedFiles,
  };
}

function emitChangeOutputs(flags) {
  setBooleanOutput('mjs-changed', flags.mjsChanged);
  setBooleanOutput('js-changed', flags.jsChanged);
  setBooleanOutput('package-changed', flags.packageChanged);
  setBooleanOutput('docs-changed', flags.docsChanged);
  setBooleanOutput('workflow-changed', flags.workflowChanged);
  setBooleanOutput('js-workflow-changed', flags.jsWorkflowChanged);
  setBooleanOutput('rust-workflow-changed', flags.rustWorkflowChanged);
  setBooleanOutput('scripts-changed', flags.scriptsChanged);
  setBooleanOutput('js-code-changed', flags.jsCodeChanged);
  setBooleanOutput('any-js-code-changed', flags.jsCodeChanged);
  setBooleanOutput('rust-code-changed', flags.rustCodeChanged);
  setBooleanOutput('any-rust-code-changed', flags.rustCodeChanged);
}

/**
 * Main function to detect changes
 */
function detectChanges() {
  console.log('Detecting file changes for CI/CD...\n');

  const changedFiles = getChangedFiles();

  logFileList('Changed files:', changedFiles);

  const flags = detectChangeFlags(changedFiles);
  emitChangeOutputs(flags);

  logFileList('\nFiles considered as code changes:', flags.codeChangedFiles);
  setBooleanOutput('any-code-changed', flags.codeChanged);

  console.log('\nChange detection completed.');
}

// Run the detection
detectChanges();
