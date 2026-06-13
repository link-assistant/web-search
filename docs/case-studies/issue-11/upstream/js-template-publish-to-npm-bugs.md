# Upstream report: `publish-to-npm.mjs` retries permanent failures and there is no first-publish bootstrap path

Target repository: `link-foundation/js-ai-driven-development-pipeline-template`

Discovered while fixing `link-assistant/web-search#11`. Two related problems in
the npm release path cause a brand-new package's **first release to fail** and
make the failure slow and hard to diagnose.

---

## Problem 1 — Permanent failures are retried `MAX_RETRIES` times and surfaced as a generic error

### Where

`scripts/publish-to-npm.mjs`

```js
const FAILURE_PATTERNS = [
  'packages failed to publish',
  'error occurred while publishing',
  'npm error code E',
  'npm error 404',
  'npm error 401',
  'npm error 403',
  'Access token expired',
  'ENEEDAUTH',
];
// ...
for (let i = 1; i <= MAX_RETRIES; i++) {
  console.log(`Publish attempt ${i} of ${MAX_RETRIES}...`);
  const { success, error } = await attemptPublish(/* ... */);
  if (success) { /* ... */ return; }
  if (i < MAX_RETRIES) {
    console.log(`Publish failed: ${error.message}, waiting ${RETRY_DELAY / 1000}s before retry...`);
    await sleep(RETRY_DELAY);
  }
}
console.error(`❌ Failed to publish after ${MAX_RETRIES} attempts`);
process.exit(1);
```

Every detected failure is treated as retryable. But `npm error 404` (first
publish of a package that does not exist yet) and `401`/`403`/`ENEEDAUTH`/
`Access token expired` are **permanent** — retrying cannot help.

### Reproducible example

Run the release workflow for a scoped package that has never been published and
without a configured trusted publisher / token. Observed (from
`link-assistant/web-search` run 27448599744):

```
npm error code E404
npm error 404 Not Found - PUT https://registry.npmjs.org/@scope%2fpkg - Not found
The requested resource '@scope/pkg@0.8.1' could not be found or you do not have permission to access it.
...
Publish attempt 1 of 3...
Publish attempt 2 of 3...
Publish attempt 3 of 3...
❌ Failed to publish after 3 attempts
```

~28 seconds are wasted on retries, and the final message hides the real cause.

### Workaround

Set a real `NODE_AUTH_TOKEN` (see Problem 2) so the first publish succeeds; this
sidesteps the retry path but does not fix the misclassification.

### Suggested fix

Classify non-retryable patterns and exit immediately with actionable guidance:

```js
const NON_RETRYABLE_PATTERNS = [
  'npm error 404', 'npm error 401', 'npm error 403',
  'e404', 'e401', 'e403',
  'access token expired', 'eneedauth',
  'you must be logged in', 'unable to authenticate',
];

function isNonRetryableFailure(output) {
  const lower = output.toLowerCase();
  return NON_RETRYABLE_PATTERNS.some((p) => lower.includes(p));
}
```

In the retry loop, when `error?.nonRetryable` is set, print guidance that
explains the first-publish bootstrap and exit `1` without retrying. Reference
implementation: `js/scripts/publish-to-npm.mjs` in
`link-assistant/web-search` PR #12.

---

## Problem 2 — No first-publish bootstrap path (OIDC cannot create a new package)

### Where

`.github/workflows/release.yml` — the `Publish to npm` steps run
`node scripts/publish-to-npm.mjs` with `id-token: write` but **no**
`NODE_AUTH_TOKEN`. They rely solely on npm OIDC trusted publishing.

### Why it fails

npm trusted publishing requires a *trusted publisher* to be configured on
npmjs.com, which can only be done for a package that **already exists**. The
first publish of a brand-new package therefore cannot use OIDC and fails with
E404 (see Problem 1). The template's documented "just push and it releases"
flow is broken for the very first release of any new package.

### Suggested fix

Add an optional token bootstrap fallback on the publish steps:

```yaml
- name: Publish to npm
  id: publish
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
  run: node scripts/publish-to-npm.mjs --should-pull
```

When `NPM_TOKEN` is set, the first publish succeeds; once the package exists and
a trusted publisher is configured, OIDC takes over and the token can be removed.
This keeps OIDC as the steady-state mechanism while unblocking bootstrap.
Document that `NPM_TOKEN` is only needed for the first release.
