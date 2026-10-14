# System prompt: shared and profile-specific parts

[Runtime · documentation home](../README.md) · [Execution flow](../flows/turn.md) · [Tools](tools.md)

The root system instructions of every non-validator request are owned by the
runtime. Part of the text is shared by all sessions; part depends on the
session's identity profile (`coding` or `general`). This page lists each part,
where it lives, and why it is there. The exact contract is
[builtin-tools §Model-facing tool guidance](../../spec/builtin-tools.md).

## Order

`ResolvedPreset::render` ([preset.rs](../../crates/tools/src/preset.rs)) joins the
sections with blank lines, in this order. A session without a genesis preset
renders its identity profile as the equivalent preset (general base, plus the
`coding` layer for coding), which is byte-for-byte the same text as
`tools::root_system_instructions`.

| Order | Text | Kind | Source |
|---|---|---|---|
| 1 | `You are an AI agent powered by Tekes Kernel.` | Shared | constant `HARNESS_IDENTITY` in `guidance.rs` |
| 2 | Preset text: the chosen identity line, the base rules, then each layer's duty; `{model}` replaced by the request's wire model id | Per session preset | [`crates/tools/prompts/`](../../crates/tools/prompts/), inline layers from `session.create {preset}` |
| 3 | `Your working directory is {cwd}.` (only when a cwd is bound) | Shared | constant `WORKING_DIRECTORY` in `guidance.rs` |
| 4 | The instruction snapshot's `AGENTS.md` scopes, trimmed, when nonempty | User and project | workspace files |
| 5 | Subagent delivery sentence, or the active session goal | Appended by the worker when applicable | [worker](../../crates/worker/src/main.rs) |

## Profile text

`IdentityProfile::profile_text` joins, with blank lines:

1. the profile's identity line;
2. the general rules, shared by every profile;
3. the profile's own rules, if any.

The identity line stays first because it carried the trajectory-length effect
for deepseek-v4-flash ([earlier findings](../benchmarks/deepseek-cost-2026-10.md#earlier-findings)).

| File | Text |
|---|---|
| `general-identity.md` | `You are a general-purpose agent powered by the {model} model.` |
| `coding-identity.md` | `You are a coding agent powered by the {model} model.` |
| `general-rules.md` | General rules G1 and G2 below |
| `coding-rules.md` | Coding rules C1–C3 below |

`coding` = coding identity + G1–G2 + C1–C3. `general` = general identity + G1–G2.

The coding profile renders as:

```
You are a coding agent powered by the {model} model.

When the requested work is done, stop and report. If you find no problem, say so instead of continuing to search for one.

Be concise. Do not narrate before each tool call.

Treat a change as done when the requested tests pass.

Do not write extra fuzz, randomized, or differential test scripts unless the user asks for them.

Check expected results by writing and running tests, not by working out program output at length in your reasoning.
```

| Rule | Purpose | Evidence (measured as part of the earlier coding profile) |
|---|---|---|
| G1 | Stop once the requested work is done; report "no problem" instead of hunting for one | Cost spent after tests passed fell by 88% (batch-ledger-bugfix) |
| G2 | Be concise; no narration before each tool call | Narration before tool calls fell from 53–59% to 15–28% of responses |
| C1 | For code, "done" means the requested tests pass | Part of the original stop rule |
| C2 | No unrequested fuzz, randomized, or differential scripts | Ad-hoc verification scripts fell from 27 to 8 (batch-ledger-bugfix) and from 23 to 0 (weighted-ttl), 6 runs each |
| C3 | Run tests to check expectations instead of long reasoning | Long-reasoning responses fell by 59–71% |

On deepseek-flash the earlier four coding rules cut mean session cost by 55%
(batch-ledger-bugfix) and 59% (weighted-ttl) without lowering test quality
(mutation score 89.3% vs 88.3%). On GPT-5.6 luna they changed cost by −6% (not
significant) and did not lower quality. Details:
[cost benchmark](../benchmarks/deepseek-cost-2026-10.md#the-coding-profile-rules).

G1 and G2 were moved out of the coding profile in format 5 because they do not
mention code or tests; G1 was reworded from "requested change ... requested
tests pass" to "requested work", with the test condition kept as C1. In an A/B
on deepseek-flash this cut the cost of a non-coding general session by 27%
(p = 0.0065) and left coding cost unchanged
([general rules benchmark](../benchmarks/general-rules-2026-10.md)).

## What is not in the system prompt

| Content | Where it lives | Why |
|---|---|---|
| How each tool works and what it returns | The tool's schema description | Per-tool usage paragraphs in the system prompt did not reduce cost and were removed (2026-09-26 ablation) |
| Project rules | `AGENTS.md` (section 4 above) | Owned by the user and the project |
| Automatic title prompt | `AUTOMATIC_THREAD_TITLE_SYSTEM` in [process_host.rs](../../crates/supervisor/src/process_host.rs) | Separate request; reasoning is always disabled (`reasoning_disabled`) and output is capped (`title_max_output_tokens` = 64) |
| Validator prompt | `JUDGE_SYSTEM` (validation runtime) | Separate request with its own judge prompt |

## Session presets

A client can open a session with an explicit preset: the workspace's base
(`general` or `blank`) plus ordered layers, either Kernel layers such as
`coding` or inline layers with their own identity line and duty text. An
inline identity is the primary role and wins over the coding identity; the base
rules always stay. `session.presets.resolve` returns the exact text a preset
renders to before any session exists. Contract:
[session-endpoint § Session presets](../../spec/session-endpoint.md#session-presets).

## Profile selection

New sessions choose `coding` or `general` from the first user request, before
the first provider call; an ambiguous request selects `coding`. "Script" and
脚本 also name a screenplay, so they count as code only when the request has no
creative-writing signal; such a signal alone selects `general`. The choice is
durable for the session and is inherited by forks and delegated children. See
[identity selection](../../crates/worker/src/identity.rs).

## Byte authority

`fixtures/tools/builtin-tool-guidance.canonical.json` (format 5) pins the
harness identity, the four prompt files verbatim (including the final LF), and
the working-directory template. Its `digest` is SHA-256 over the RFC-8785
bytes of `{harness_identity, general_identity, general_rules, coding_identity,
coding_rules, working_directory}`.

To change the prompt:

1. Edit the prompt file under `crates/tools/prompts/`.
2. Regenerate the fixture so its bytes and digest match.
3. Run `cargo test -p tools`; `canonical_guidance_oracle_matches_registry_bytes`
   fails until the fixture matches.
4. A changed digest opens a `system_change` epoch for existing sessions.
5. Measure the change before relying on it: interleaved runs of each variant
   on a fixed corpus, priced from provider usage (method in the
   [cost benchmark kit](../../scripts/bench/README.md)).
