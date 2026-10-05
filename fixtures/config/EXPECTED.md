# Config fixture expectations

Files ending `.canonical.json` decode, validate, and re-encode byte-identically
under `spec/config.md`. Files ending `.invalid.json` reject: relative cwd,
unknown workspace field, embedded secret field, and an integer outside the
I-JSON safe range respectively.
Web-search origins also reject IPv4-mapped IPv6 special-use addresses; the
mapped-loopback fixture guards both URL-literal parsing and the shared SSRF
classifier.

`providers-route-identity.invalid.json` is otherwise a legacy-shaped provider
but omits every required exact-route identity field. It rejects as
`invalid_schema`; an implementation must not infer dialect, route owner,
gateway, evidence revision, model profile, or exact SKU from adapter/model
names.
