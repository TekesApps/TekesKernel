# Session Endpoint v2 fixtures

`authority/` is byte-identical to the Tekes-owned v2 corpus pinned by
`authority-lock.canonical.json`. `projection-source.jsonl` is a valid Kernel
ledger and `projection-expected.jsonl` is its exact endpoint journal. The
remaining canonical files cover mutation ownership and Client stitch inputs.

Run `scripts/check-endpoint-authority.py`; pass `--require-source` in the
workspace parity lane. Implementations read these files and never regenerate
them as expected output.
