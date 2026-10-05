# Web search provider v1

This executable contract owns the one production implementation of the fixed
`web_search` tool. It is independent of model-provider adapters. Format 1 has
one closed adapter, `tavily_v1`; no other configured adapter may expose the
tool.

## Configuration and readiness

`config/providers.json` may contain the optional top-level `web_search` value
defined by [config](config.md). Its endpoint is an HTTPS public origin.
The request target is exactly `endpoint + "/search"`; redirects are disabled.
At dispatch time DNS resolution is pinned to one public address and rejects
loopback, private, link-local, multicast, and unspecified addresses. A
redirect response is a terminal protocol failure and is never followed.
Configuration literals, DNS results, result-URL literals, and the generic HTTP
tool use one shared special-use classifier. IPv4-mapped IPv6 is recursively
classified as the mapped IPv4 address, so `::ffff:127.0.0.1` is loopback, not
public.

The supervisor resolves only that value's `credential_key` into the exact
broker scope `(credential_key, "tavily_v1", endpoint_origin, "web_search")`.
Model-provider credentials never create a web-search scope. A worker launch
receives `--web-search-ready` only when the scope is active and the private
broker channel was created. The worker exposes `web_search` only when all of
these are true: workspace/instruction network policy permits network access,
the supported configuration is present, `--web-search-ready` was supplied,
the broker descriptor validated, and the bounded HTTP/search backend was
constructed. Missing, revoked, unavailable, unsupported, or malformed state
fails closed and does not advertise the tool. A readiness transition stops
the old worker so the next immutable launch recomputes the catalog.

## Credential transaction

The durable `tool_call` and approval/effective-execution gates precede every
credential request. The adapter requests:

```text
credential_get {
  request_id: credential_request_id("tool:" || call_id, credential_key,
                                    endpoint_origin),
  attempt: "tool:" || call_id,
  credential_id: credential_key,
  purpose: "web_search",
  adapter: "tavily_v1",
  endpoint_origin
}
```

Each call resolves its material with one `credential_get` keyed by the call
id before dispatch. Worker cancellation or supervisor loss cancels the HTTP
operation. Owned material is zeroed when the call returns. A later call obtains
the current generation; a revoked id fails the next call closed before any
send. Credential
material never appears in events, assets, hooks, errors, URLs, environment,
argv, metrics, or fixtures.

## HTTP request

The method is `POST`; `Content-Type` and `Accept` are `application/json` and
the credential is sent only as `Authorization: Bearer <material>`. The exact
request object has this closed field set:

```text
{
  query: nonempty string,
  search_depth: "basic",
  max_results: integer 1..10,
  topic: "general" | "news",
  include_answer: false,
  include_raw_content: false,
  include_images: false
}
```

Request bytes are RFC-8785 canonical JSON without a trailing LF. The client
uses the bounded HTTP timeout and response-byte limit from `tool-runtime`.
It performs no automatic retry inside one durable tool call.

## Response and errors

A successful response is one JSON object containing required `results` array.
Unknown top-level fields and unknown fields inside a result are ignored. Every
consumed result requires string `title`, `url`, and `content`; `content` maps
to `SearchHit.snippet`. `url` must be an absolute syntactically-public `http`
or `https` URL without userinfo: localhost/`.localhost`/`.local`/`.internal`
names and non-public address literals reject. Result validation performs no
DNS lookup; a later `web_fetch` resolves and revalidates every hop. More than
the requested result count is truncated only after the complete bounded
response validates. A missing field, wrong type,
invalid result URL, non-I-JSON/duplicate-key input, or over-limit body is a
terminal protocol failure.

Status classification is closed:

| status | result |
|---|---|
| `200...299` | parse the successful response |
| `400` | terminal invalid/provider error |
| `401`, `403` | authentication failure |
| `429`, `500...599` | retryable unavailable |
| every other non-2xx, including `3xx` | terminal unavailable |

Transport failure before a response, timeout, and cancellation are typed
unavailable/cancelled outcomes according to the tool runtime; a revoked or
missing credential is typed unavailable before dispatch. They
do not cause an automatic second dispatch under the same call id.

## Oracle

`fixtures/web-search-provider/` freezes the request bytes, successful result
mapping, status classes, invalid response shapes, readiness matrix, and secret
scope. `scripts/check-web-search-provider-fixtures.py` checks canonical bytes,
the closed case inventory, and the absence of credential material.
