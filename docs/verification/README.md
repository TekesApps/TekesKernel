# Verification: rules, commands and evidence

[System overview](../README.md) · [Code architecture](../architecture/README.md)

Choose the behavior you need to prove, then find its gate and run results. Rule definitions, test commands and evidence from individual runs are maintained separately.

| What do you need? | Where to read |
|---|---|
| Are Thread semantics implemented? | [A: rules and code review](thread-alignment.md) |
| How do I run checks? | [Test commands](running-tests.md) |
| Which assertions must a behavior satisfy? | [Gates by topic](gates/README.md) |
| What is the feature and provider coverage? | [Feature parity](feature-parity.md), [Provider parity](provider-parity.md) |
| Are document moves, references and the generator correct? | [Documentation reorganization verification](document-reorganization.md) |
| Where is the earlier architecture index validation? | [Initial index record](source-atlas.md) |
| What happened during validation-flow migration? | [Migration audit](audits/validation-migration.md) |
| What is the current crate and outstanding live-test audit? | [Kernel alignment audit, 2026-09-04](audits/kernel-alignment-2026-09-04.md) |

Raw live-run evidence (the live-test inventory, provider failure audits, run
transcripts) is kept by the maintainers outside this repository. The audits
below summarize what it showed.

For historical gate execution order, see [Stage-to-gate mapping](../history/gate-rollout.md).
This page does not present historical success as a result of the current run or infer product UI or release acceptance from partial tests.

- [Crate README semantic audit 2026-09-05](audits/crate-readme-semantic-audit-2026-09-05.md) — supervisor, worker, engine, provider, mcp claims checked against source
