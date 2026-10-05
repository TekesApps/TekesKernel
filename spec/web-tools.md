# Web tools v1 — public fetch and search closure

This executable contract closes Slice 14E for the fixed `web_fetch` and
`web_search` tools. It owns their provider/extractor chain, network and secret
boundaries, terminal classification, retry/repair/cache policy, and the
disposition of the predecessor Web module. It does not add a Session Endpoint
route; the tools remain ordinary model-facing tool calls.

`builtin-tools` owns argument schemas and catalog availability,
`tool-runtime` owns write-ahead execution and terminal events, and
`web-search-provider` owns the exact `tavily_v1` wire. This contract wins
for the behavior that joins those authorities.

## `web_fetch` production chain

The v1 chain has exactly two in-process stages:

1. **bounded public fetch** — one `GET`, a single 30-second whole-operation
   deadline, a 4 MiB configured response cap (hard maximum 16 MiB), and at
   most five redirects;
2. **deterministic local extraction** — no second network service, model, DOM
   runtime, or process is invoked.

Each hop is `GET` with exactly
`User-Agent: TekesBot/1.0 (+https://tekes.local)` and
`Accept: text/html,application/xhtml+xml,text/plain,application/json;q=0.9,*/*;q=0.8`;
it carries no cookie, authorization, referer, or body.

The effective workspace policy must permit network access and the ordinary
`execute` approval/effective-execution gate must have committed before stage
1. Every initial URL and redirect hop must be absolute HTTP(S), contain no
userinfo, and resolve only to public addresses. DNS is resolved once for the
hop, every answer is classified, any non-public answer rejects the hop, and
the selected public address is pinned into the connection. The shared
classifier rejects loopback, private, link-local, CGNAT, multicast,
unspecified, documentation/test, IPv4-mapped private IPv6, and local host
names. IPv4-compatible and well-known-prefix NAT64 addresses are classified
by their embedded IPv4 address. A failed lookup is unavailable, never
permission to connect by name.

Only `2xx` is a response. A redirect without a valid `Location`, a sixth
redirect, a non-`2xx`, an over-limit `Content-Length` or streamed body, a
deadline, cancellation, and transport failure produce the corresponding
typed tool-runtime terminal. The operation makes no automatic retry; the
kernel performs no response repair and stores no HTTP cache, validator, or
cookie. A user/model retry is a new durable tool call and new network effect.

## Local extraction

Content type is compared case-insensitively. HTML, `text/*`, JSON, XML,
JavaScript, and a missing content type are textual. UTF-8 is decoded first;
other bytes use an exact ISO-8859-1 byte-to-scalar mapping. Other media types
return the literal note
`[non-text content: <lowercase-type>, <bytes> bytes — not rendered]` and never
embed the raw body in the durable tool result.

With an explicit HTML content type, or with no type and an HTML/doctype/body
marker in the first 512 characters, extraction:

- captures the first `title` and restores it as `Title: <title>` when the body
  does not already begin with that title;
- removes comments and all `head`, `script`, `style`, and `noscript` content;
- turns `br`/`hr`, list items, and block closing tags into stable line breaks;
- strips remaining tags; decodes the named entities frozen in
  `fixtures/web-tools/` and decimal/hex scalar entities; and collapses
  horizontal whitespace while retaining at most one blank line.

Plain textual content is trimmed but otherwise preserved. Output is limited
to the first 24,000 Unicode scalar values; truncation appends
`\n\n[truncated at 24000 characters]`. HTML is marked `under_rendered:true`
when its meaningful body has fewer than 160 scalars or contains
`enable javascript`/`requires javascript`. That flag is diagnostic only in
v1: it never authorizes another egress.

The completed result is the closed object
`{final_url,status,content_type,bytes,text,extraction}` where `final_url` and
`text` are strings, `status` and `bytes` are nonnegative integers,
`content_type` is string or JSON null, and `extraction` is
`{kind:"html"|"text"|"non_text",under_rendered,truncated}`.
`bytes` is the received-body length; the received body itself is not returned
or durably duplicated.

## `web_search` production chain

`web_search` is present only when `web-search-provider`'s exact
configuration, network policy, active broker scope, and production backend are
all ready in the immutable worker launch. It resolves one generation-scoped
credential per durable call and holds it only for that call. Worker
cancellation or supervisor loss cancels an in-flight request; rotation and
revocation reach the next call. No
credential or bearer-derived value may reach durable bytes or diagnostics.

The only v1 adapter is `tavily_v1`; its request bytes, response projection,
status classification, DNS pinning, redirect rejection, and result-URL
validation are exactly `web-search-provider`. It performs no automatic
retry, response repair, synthesized-answer request, or cache. `429`/`5xx` are
reported retryable to the durable tool pipeline, but any later dispatch is a
new tool call; one call id is never sent twice by the backend.

## Predecessor disposition

The pinned AppServer Web module is closed as follows:

| predecessor behavior | v1 disposition |
|---|---|
| public GET, per-hop SSRF guard, HTML/plain extraction, 24k rendering cap | retained by the production `web_fetch` chain |
| credential-conditional Tavily search, topic/max-results, ranked title/URL/snippet | retained by `tavily_v1` |
| Jina Reader fallback | **permanently retired** — it discloses the target URL to a second service and had only environment-selected policy |
| Tavily Extract fallback | **permanently retired** — no separate reader credential/purpose or hidden paid backstop exists |
| Tavily synthesized answer, score and published-date output | **permanently retired** — the fixed tool returns source hits only |
| `TEKES_WEB_READER`, `JINA_API_KEY`, `TAVILY_API_KEY` process-environment ownership | **permanently retired** — configuration and credentials use config plus the broker only |

Retirement is executable: format-1 config has no reader-chain fields or
reader credential purpose, the worker contains no Jina/Tavily-extract
dispatcher, and no reader capability is advertised. Reintroducing any row
requires a new versioned contract, explicit privacy/credential policy, and
negative fixtures; it cannot be inferred from `under_rendered:true`.

## Oracle and internal seam

`fixtures/web-tools/` freezes extraction bytes, truncation/under-rendering
boundaries, network negatives, and every predecessor disposition. The
existing `fixtures/web-search-provider/` corpus remains the exact search wire
oracle. Gates 109–112 must execute the production extractor, public-address
classifier, Tavily encoder/parser/status code, credential rotation/revocation,
and the retirement inventory. Static fixture validation alone cannot pass a
gate.

Slice 14E registers no public Client route. Slice 14F may expose only generic
tool/catalog administration already owned by its versioned capability; it
must not create a Web-specific endpoint or silently revive a retired reader.
