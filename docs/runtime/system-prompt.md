# System prompt: shared and profile-specific parts

[Runtime · documentation home](../README.md) · [Execution flow](../flows/turn.md) · [Tools](tools.md)

The root system instructions of every non-validator request are owned by the
runtime. Part of the text is shared by all sessions; part depends on the
session's identity profile (`coding` or `general`). This page lists each part,
where it lives, and why it is there. The exact contract is
[builtin-tools §Model-facing tool guidance](../../spec/builtin-tools.md).

## Order

`tools::root_system_instructions` ([guidance.rs](../../crates/tools/src/guidance.rs))
joins the sections with blank lines, in this order:

| Order | Text | Kind | Source |
|---|---|---|---|
| 1 | `You are an AI agent powered by Tekes Kernel.` | Shared | constant `HARNESS_IDENTITY` in `guidance.rs` |
| 2 | Profile text, `{model}` replaced by the request's wire model id | Per profile | [`crates/tools/prompts/coding.md`](../../crates/tools/prompts/coding.md) or [`general.md`](../../crates/tools/prompts/general.md) |
| 3 | `Your working directory is {cwd}.` (only when a cwd is bound) | Shared | constant `WORKING_DIRECTORY` in `guidance.rs` |
| 4 | The instruction snapshot's `AGENTS.md` scopes, trimmed, when nonempty | User and project | workspace files |
| 5 | Subagent delivery sentence, or the active session goal | Appended by the worker when applicable | [worker](../../crates/worker/src/main.rs) |

## Coding profile

`crates/tools/prompts/coding.md`:

```
You are a coding agent powered by the {model} model.

When the requested change is done and the requested tests pass, stop and report. If you find no defect, say so instead of continuing to search for one.

Do not write extra fuzz, randomized, or differential test scripts unless the user asks for them.

Check expected results by writing and running tests, not by working out program output at length in your reasoning.

Be concise. Do not narrate before each tool call.
```

| Sentence | Purpose | Evidence |
|---|---|---|
| Identity line | Tells the model it is a coding agent inside a harness | Carried the whole trajectory-length effect for deepseek-v4-flash ([earlier findings](../benchmarks/deepseek-cost-2026-10.md#earlier-findings)) |
| Rule 1 | Stop once the requested work is done; report "no defect" instead of hunting for one | Cost spent after tests passed fell by 88% (batch-ledger-bugfix) |
| Rule 2 | No unrequested fuzz, randomized, or differential scripts | Ad-hoc verification scripts fell from 27 to 8 (batch-ledger-bugfix) and from 23 to 0 (weighted-ttl), 6 runs each |
| Rule 3 | Run tests to check expectations instead of long reasoning | Long-reasoning responses fell by 59–71% |
| Rule 4 | Be concise; no narration before each tool call | Narration before tool calls fell from 53–59% to 15–28% of responses |

On deepseek-flash the four rules cut mean session cost by 55% (batch-ledger-bugfix)
and 59% (weighted-ttl) without lowering test quality (mutation score 89.3% vs
88.3%). On GPT-5.6 luna they changed cost by −6% (not significant) and did not
lower quality. Details: [cost benchmark](../benchmarks/deepseek-cost-2026-10.md#the-coding-profile-rules).

Rule 4 is not coding-specific, but it was only measured with the coding
profile, so it stays there.

## General profile

`crates/tools/prompts/general.md`:

```
You are a general-purpose agent powered by the {model} model.
```

The coding rules refer to tests and code, so the general profile does not
carry them.

## What is not in the system prompt

| Content | Where it lives | Why |
|---|---|---|
| How each tool works and what it returns | The tool's schema description | Per-tool usage paragraphs in the system prompt did not reduce cost and were removed (2026-09-26 ablation) |
| Project rules | `AGENTS.md` (section 4 above) | Owned by the user and the project |
| Automatic title prompt | `AUTOMATIC_THREAD_TITLE_SYSTEM` in [process_host.rs](../../crates/supervisor/src/process_host.rs) | Separate request; reasoning is always disabled (`reasoning_disabled`) and output is capped (`title_max_output_tokens` = 64) |
| Validator prompt | `JUDGE_SYSTEM` (validation runtime) | Separate request with its own judge prompt |

## Profile selection

New sessions choose `coding` or `general` from the first user request, before
the first provider call; an ambiguous request selects `coding`. The choice is
durable for the session and is inherited by forks and delegated children. See
[identity selection](../../crates/worker/src/identity.rs).

## Byte authority

`fixtures/tools/builtin-tool-guidance.canonical.json` (format 4) pins the
harness identity, both profile files verbatim (including the final LF), and
the working-directory template. Its `digest` is SHA-256 over the RFC-8785
bytes of `{harness_identity, coding_profile, general_profile, working_directory}`.

To change the prompt:

1. Edit the profile file under `crates/tools/prompts/`.
2. Regenerate the fixture so its bytes and digest match.
3. Run `cargo test -p tools`; `canonical_guidance_oracle_matches_registry_bytes`
   fails until the fixture matches.
4. A changed digest opens a `system_change` epoch for existing sessions.
5. Measure the change before relying on it: interleaved runs of each variant
   on a fixed corpus, priced from provider usage (method in the
   [cost benchmark kit](../../scripts/bench/README.md)).
