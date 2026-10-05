# spec: rewrite publication v1

This contract owns offline thread-folder fork and redact. It is the executable
authority for `staging/`, materialization, publication, retirement, recovery,
and rewrite garbage collection. Docs 01/02/04/10 are subordinate summaries.

## Scope and preconditions

A rewrite operates on an active, unarchived thread folder while no worker
holds its lifecycle lock. The source id, destination id, and every rewritten
file identity are lower-case canonical UUIDs. The destination must not exist
in `threads/`, `archive/`, rewrite trash, or another live operation.

Once `op.json` is durable, the source is **rewrite-owned**: keyed delivery,
archive, delete, and another rewrite reject with `rewrite_in_progress` until
the operation closes. Each recovery step reacquires the source lifecycle lock
when the source still exists and verifies the recorded source-prefix digest.
A mismatch is corruption; recovery never publishes a projection built from a
different source prefix.

Rewrite creation, phase advancement, and rewrite-debris GC serialize on the
root `.rewrite.lock`. Creation and advancement then take
`.thread-catalog.lock` exclusive before the source lifecycle lock; debris GC
does not change membership and needs only `.rewrite.lock`. Create/archive take
the catalog lock without taking the rewrite lock, so no writer acquires the
pair in reverse. Creation checks both source and destination reservations
while holding the two namespace locks, and ordinary create rejects a
destination reserved by a live rewrite, so two sources cannot publish the
same identity. Delivery takes the source lifecycle lock and archive takes the
catalog lock then the lifecycle lock; both recheck the durable operation
inventory. A caller that raced `prepared` therefore rejects instead of
appending to the bound source prefix.

Thread-level v1 rewrite consumes the complete valid prefix of every `*.jsonl`
file in the folder. Every source turn must be terminal and every source child
must have a terminal `child_result`; parked and running prefixes are rejected.
Task-level child creation remains the spawn/seed transaction in doc 07.
Before publishing `prepared`, creation runs the pure materialized projection in
memory. A deterministic unsupported-visible-fact, dropped child-edge half, or
invalid remap rejects without creating an operation record; no permanently
unadvanceable operation may acquire source ownership.

## Operation record

`staging/<op-id>/op.json` is RFC-8785 canonical JSON followed by one LF:

```text
{
  "format": 1,
  "id": str,
  "type": "fork" | "redact",
  "source": uuid,
  "dest": uuid,
  "created_at": timestamp,
  "source_digest": "sha256-" + 64 lowercase hex,
  "phase": "prepared" | "built" | "validated" | "published" |
           "source_retired" | "source_removed",
  "files": [{"source": str, "dest": str, "thread": uuid}],
  "redaction"?: {
    "events": [{"file": str, "seq": int}],
    "assets": [asset-name],
    "fingerprints": ["sha256-" + 64 lowercase hex]
  }
}
```

`files` is canonical `main.jsonl` first, then ASCII source-filename order;
the other arrays are ASCII-lexicographically sorted and duplicate-free, and
`events` sort by `(file, seq)`. `redaction` is required exactly for `type:redact`.
Fingerprints are audit evidence only. The operation record never stores the
forbidden plaintext. `main.jsonl` maps to destination `main.jsonl` and its
`thread` equals `dest`; every source child maps to a new UUID-named file and
new UUID thread identity. The complete map is fixed before `prepared` syncs.

The tree digest hashes, in ASCII filename order, each ledger filename, one NUL,
the complete longest-valid-prefix bytes, and one NUL. Assets are immutable and
are therefore bound by the event/carrier references and their own names.

## Materialized projection

The builder validates every source prefix under event, computes supersede
and compact coverage, and emits a fresh valid ledger per mapped file.

1. A new genesis is seq 1. It carries the mapped identity, source workspace,
   resume/config/instruction policy, and a rewrite origin tuple. A source seed
   snapshot is retained only when its complete recursive carrier closure is
   clean; otherwise redact omits it. Child genesis `parent` points to the
   mapped parent file and mapped spawn seq and keeps the same `spawn_id`.
2. Superseded events, `queue_edit`, `compact`, checkpoints,
   `run_start`, attempts and their dispatch/recovery/error
   satellites, effective-execution records, stop generations, approval
   protocol records, rebuildable `.thread-search.json` caches, and
   runtime-only unknown extensions are not copied.
   Compact-covered facts are represented by the compact summary, never by
   both the covered facts and the marker.
3. Live `input` records retain their content/source but receive a rewrite
   origin tuple targeted at the mapped thread. `turn_open` input seqs are
   remapped. If redact removes a consumed input, the whole source turn is
   removed; no dangling trigger is legal.
4. Provider outputs, reasoning, and tool rows are historical facts, not live
   continuation state. Each retained row becomes `state` subkind
   `rewrite.<source-kind>` whose `payload` contains `source_file`,
   `source_seq`, and the source payload after removing attempt,
   continuation, sealed fragments, execution-only ids, and envelope fields.
   `state` rows remain `state`; live `spawn`/`child_result` pairs are retained
   with mapped child ids so child genesis bindings remain checkable.
5. A retained turn receives one final settle with its original closed
   outcome. Turns and every seq-bearing field are renumbered contiguously.
   A compact summary is emitted as `state {subkind:"rewrite.compact"}` in a
   synthetic, runtime-triggered terminal turn when it cannot attach to an
   existing retained turn. No destination reference may name a dropped seq.
6. The latest source provider profile, when present, becomes one new trailing
   epoch with a fresh id, reason `fork` or `redact`, no pending inventory, and
   the same adapter/model/system/tools/renderer profile. It is authoritative
   for a later ordinary turn but admits nothing by itself.
7. `meta` materializes only the effective title and labels, without source
   origin/dedup, ownership, upgrade, or supersede fields. Effective reader and
   writer minima are folded into the new genesis.

Unknown model-visible source facts or a carrier class the implementation does
not understand make the rewrite fail closed. This is preferable to silently
publishing a history with missing required-visible information.

### Redaction selection and carrier closure

Redact scans source event bytes and every recursively reachable asset for the
caller-supplied forbidden byte strings before writing `prepared`. It records
the exact affected `(file,seq)` and asset names plus only SHA-256 fingerprints.
The projection is then a pure function of the source prefix and this inventory.

Every string-valued `sha256-<64 lowercase hex>` reachable from a retained
event is an asset reference. The scanner additionally understands the declared
carriers: genesis seed snapshots and their lines, config and instruction
snapshots, epoch system assets, spilled values, tool
blocks, `endpoint.jsonl`, D-70 control receipts, session-endpoint §Management authority and recovery rpc
records targeting the source session, and recursively referenced assets. The
endpoint journal is a protocol carrier, never a semantic source row: fork and
redact never copy it or its lock. The destination journal starts empty and
rebuilds deterministically from the materialized semantic prefix under the
destination identity, allocating destination endpoint seqs (and, from that
prefix's holds, derived request rpcIds). Redact scans the source journal for
forbidden bytes and schema-declared asset references before dropping it, so
view bytes cannot retain a poisoned value. Control receipts never enter a materialized
destination; redact scans them for the forbidden bytes and source retirement
removes them. Endpoint rpc records are endpoint-scoped and never copied;
fork/redact retain only the retired idempotency tombstones defined by
session-endpoint §Management authority and recovery. Before source retirement, redact scans every rpc and
operation record targeting the source identity and atomically replaces a
matching record with its content-free retired form: request/response bytes,
`rpc_id`, and operation intent are absent; only their hashes, target identity,
operation, phase, and session-endpoint §Management authority and recovery's exact
`retired_reason:"redact"` (rpc row) or `retired:"redact"` (operation row)
remain; there is no generic `reason` field. A later retry therefore
returns the closed `idempotency-conflict` response and can never recover the
poisoned bytes from an exact-response cache. Fork copies the complete live
closure. Redact drops a poisoned row; for a poisoned seed or implicit profile
carrier it drops/regenerates the carrier or fails closed. Sealed fragments are
always removed before the live-asset scan, so they can never keep a poisoned
blob alive. Destination validation verifies every retained asset's filename,
digest, and recursive closure. After validation, no recorded forbidden event
or asset and no sealed carrier is reachable from the destination.

## Phase machines and crash recovery

`payload/` lives beside `op.json` until publication. Each arrow consists of
the action, required file/directory syncs, then an atomic+synced `op.json`
phase replacement:

```text
fork:   prepared -> built -> validated -> published -> CLOSED
redact: prepared -> built -> validated -> published
        -> source_retired -> source_removed -> CLOSED
```

- `prepared -> built`: rebuild `payload/` from the recorded prefix/map and
  sync every ledger, asset, directory, and the payload root.
- `built -> validated`: replay all ledgers, verify parent/spawn mappings,
  carrier closure, asset digests, and redaction inventory.
- `validated -> published`: exclusively rename `payload/` to `threads/<dest>`
  and sync `threads/`. If recovery sees destination present and payload absent,
  publication completed and it records `published`; any other collision is
  fail-stop.
- Fork `published -> CLOSED`: remove `op.json`, sync the operation directory,
  remove the empty directory, and sync `staging/`. The source is untouched.
- Redact `published -> source_retired`: rename the source to
  `.rewrite-trash/<op-id>-<source>` and sync both parent directories. Recovery
  accepts exactly source-present or tombstone-present, never both/neither.
- `source_retired -> source_removed`: recursively remove the tombstone and
  sync `.rewrite-trash/`, then record `source_removed`.
- `source_removed -> CLOSED`: close as for fork.

Every transition is idempotent across a crash after its action and before its
phase record. Boot/periodic sweep enumerates only `staging/*/op.json`, decodes
the closed record, and advances it. A malformed record is corruption and is
never garbage-collected. Recordless entries in `staging/` are debris and may
be removed; keyed-create staging uses `.create-staging/` and is outside this
namespace. GC never removes a live operation, published destination, source,
or rewrite tombstone.

## Errors and observability

The public result is typed: `rewrite_in_progress`, `source_changed`,
`destination_exists`, `source_not_terminal`, `unsupported_visible_fact`,
`carrier_corrupt`, or `operation_corrupt`. Progress reports the durable phase;
`closed` means no `op.json` remains. Redact reports completion only after the
source tombstone is gone. Fork reports completion only after the operation
record closes and the unchanged source digest still matches.
