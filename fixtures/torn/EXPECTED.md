After repair by the lock holder, each file ends exactly after the last
complete valid event line; readers ignore the torn suffix without repair.
Normative rule: event §Tail framing and repair (R14-9).
`interleaved-loss.jsonl`: a garbage line followed by a valid-looking
line — recovery truncates at the garbage; the trailing valid-looking
line is post-barrier best-effort and is discarded with it (R15-6).
