/**
 * Resolve and execute a caller-owned transport while preserving response
 * provenance for detailed searches.
 */

/**
 * @param {Function|{fetch: Function}|undefined} transport
 * @returns {Function}
 */
function resolveFetch(transport) {
  if (typeof transport === 'function') {
    return transport;
  }
  if (transport && typeof transport.fetch === 'function') {
    return transport.fetch.bind(transport);
  }
  return globalThis.fetch;
}

/**
 * @param {Function|{fetch: Function}|undefined} defaultTransport
 * @param {string|URL} url
 * @param {RequestInit} init
 * @param {import('./providers/base.js').SearchOptions} options
 * @returns {Promise<Response>}
 */
export async function request(defaultTransport, url, init, options = {}) {
  const fetchImpl = resolveFetch(options.transport || defaultTransport);
  if (typeof fetchImpl !== 'function') {
    throw new Error('No fetch-compatible transport is available');
  }

  const response = await fetchImpl(url, {
    ...init,
    ...(options.signal ? { signal: options.signal } : {}),
  });

  if (typeof options.captureResponse === 'function') {
    let capture = response.captureReceipt;
    if (capture === undefined && typeof response.clone === 'function') {
      const clone = response.clone();
      const body = new Uint8Array(await clone.arrayBuffer());
      capture = {
        url: response.url || String(url),
        status: response.status,
        headers: Object.fromEntries(response.headers?.entries?.() || []),
        body,
      };
    }
    if (capture !== undefined) {
      options.captureResponse(capture);
    }
  }

  return response;
}
