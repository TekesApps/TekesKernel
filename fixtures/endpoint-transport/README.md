# Endpoint transport fixtures

`cases.canonical.json` is the Slice-9 carrier matrix and fixes the raw request,
response and expected filenames/fields for every case. DTO values come from
`../endpoint/authority/`; transport fixtures wrap those values without changing
them. Every registered case has one request transcript, response transcript and
closed expected result under its family directory.

`COVERAGE.md` maps the 53 cases to Gates 65–70. The `management/` family has
one request/success/error transcript per closed registration, and
`management-requests.canonical.jsonl` freezes the request identities derived
from approval/question holds and all four resolution outcomes.
`management-create-resolution.canonical.json` freezes `Workspace.path`
resolution, `management-recovery.canonical.json` freezes archive-lock/config-
digest/cap recovery, and `management-redact-carriers.canonical.json` proves
both endpoint journals are scanned, dropped, and rebuilt rather than copied.
`management-model-selection.canonical.json` keeps a selected model out of the
current run and its overflow retry, applying it only to a later spawn.
The summary/model-catalog fixtures freeze native inventory derivation;
`management-rpc-carrier.canonical.json` freezes prepared/handed-off/complete
exact-retry recovery; and `management-respond-authoring.canonical.json` freezes
question-answer event bytes and origin fields.
`management-redact-authorities.canonical.json` proves D-40 removes forbidden
response/intent bytes while retaining an unreusable rpc tombstone.
`management-control-races.canonical.json` freezes the shared admission lock
order, queue rejection's zero-event complete-error, cancel priority, and the
stop-active/recovery-needed response paths.
`management-respond-rejections.canonical.json` proves every bare negative
respond receipt is non-authoring and non-caching. The registered attachment
validation case proves a real Client-shaped named PNG is admitted and its name
is durably message-scoped, then exercises first-failure order; the auth/Origin
matrix does the same for real HTTP/WebSocket transcripts.
Every `/api` raw request carries the deterministic valid 32-byte fixture bearer
`AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA` except the explicit
missing/wrong/valid matrices under `security/`; production
installations generate random bytes and never reuse the fixture value.
Run
`scripts/check-endpoint-transport-fixtures.py` from any directory to verify
registry/disk parity, transcript-v1 grammar, canonical JSON, body/header
lengths, status/close agreement, authority wrapping and coverage.
