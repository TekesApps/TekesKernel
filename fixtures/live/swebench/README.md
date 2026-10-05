# SWE-bench Verified smoke fixture

`swe-bench-verified-smoke.jsonl` is the legacy `Tests/SWEBench/Fixtures/swe-bench-verified-smoke.json`
(ten SWE-bench Verified rows: eight `psf/requests`, one `pallets/flask`, one
`sphinx-doc/sphinx`), one JSON object per line, unchanged. `fetch_instance.py`
is the legacy helper that appends a Verified instance from the Hugging Face
dataset (`pip install datasets`).

`scripts/run-swebench.py` is the Kernel harness: provision (bare-mirror cache,
checkout `base_commit`), drive one real turn through the spawned host with the
legacy instruction, capture `git diff --cached HEAD` as the prediction, grade
with the official `swebench` Docker harness when `docker` and the `swebench`
module exist (otherwise the mock oracle records the run as ungraded and
exports `predictions.jsonl` for deferred grading), and write the legacy report
row shape (`pass_at_1`, `pass_at_k` per instance) plus per-trial detail.
