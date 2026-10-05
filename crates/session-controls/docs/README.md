# session-controls — goals and delegated child inventory

[Crate map](../../../docs/architecture/crates.md) · [Generated source index](generated/index.md)

This library implements `goals.get`, `goals.edit`, `goals.clear`,
`goals.pause`, `goals.resume`, and `subagents.list`. A goal's Client-authored
record is stored per session with a compare-and-set revision. Model-authored
goal events can be folded into the view when their ID matches the host binding.
The current worker also uses an active bound goal to continue across settled
turns, subject to the round limit and stop conditions.

`subagents.list` reads the parent's semantic ledger. Delegated children are
bounded work under that parent, not independently continuable sessions. See
[source](../src/lib.rs) and the [Session Endpoint goal contract](../../../spec/session-endpoint.md#goals)
for exact transitions and lifecycle rules.
