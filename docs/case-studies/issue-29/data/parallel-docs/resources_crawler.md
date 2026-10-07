> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Crawler

> This documentation provides guidance for webmasters on managing their website's interaction with our crawling system

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

## ShapBot

ShapBot helps discover and index websites for Parallel's web APIs. To maximize your site's visibility in search results, we suggest allowing ShapBot access in your robots.txt configuration and permitting connections from our designated IP ranges.

Full user-agent string: Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko); compatible; ShapBot/0.1.0

For the complete list of ShapBot IPs, see [shapbot.json](https://docs.parallel.ai/resources/shapbot.json).

For more about our use of bots, see [Parallel bots](https://parallel.ai/parallel-web-systems-bots).

## Contact & Support

If you have questions about our crawlers or need assistance, please contact us at [support@parallel.ai](mailto:support@parallel.ai)

## Changes to This Documentation

We may update this documentation periodically to reflect changes in our crawler behavior or policies. Please check back regularly for updates.
