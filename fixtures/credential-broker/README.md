# Credential broker fixtures

`cases.canonical.json` is the closed artifact inventory for the private
credential channel (`spec/credential-broker.md`). Every path listed there must
exist. Packet bodies use only the sentinel secret and are never production
examples.

The channel is request/response only: a `credential_get` keyed by the attempt
(or tool call) answers with `credential` material or a `credential_error`.
Rotation and revocation reach the next `credential_get`; there are no lease ids,
no release messages and no revocation pushes.
