---
schema_version: 1
name: chinese-short-drama
description: Complete Chinese short-drama outline workflow with required artifacts
---
# Chinese short-drama outline workflow

## Deliverable contract

Produce a complete, readable Chinese short-drama outline for the requested audience and
theme. The workflow has exactly three required artifacts; none is optional:

1. `docs/drama/story_brief.md`: premise, audience, protagonist, antagonist, character
   relationships, and the family/community/workplace pressure lines. Keep the complete
   artifact at or below 1200 Chinese characters.
2. `docs/drama/episode_map.md`: exactly 12 numbered episode outlines. Every episode must
   contain conflict, an emotional escalation, and a hook or reversal. Use exactly one
   80–120 Chinese-character paragraph per episode and keep the whole artifact at or
   below 2400 Chinese characters.
3. `docs/drama/final_outline.md`: integrate the accepted brief and episode map into the
   complete user deliverable. Include all required section labels and all 12 numbered
   episode outlines, but keep the whole artifact at or below 4200 Chinese characters.

## Required execution order

Register exactly these executable task names and input sources with the task tool:

- `story_brief` with `input_sources: []`.
- `episode_map` with `input_sources: ["story_brief"]`.
- `final_outline` with `input_sources: ["story_brief", "episode_map"]`.

These names are literal. Never prefix an input source with `task_`, and never add an
artifact path as another input source. The episode-map task depends on the accepted
`story_brief` result; the final-outline task depends on both accepted earlier results.
Every file task must write its exact contract path through the apply_patch tool and be
accepted before its dependents run. For each apply_patch call use its exact full workspace path: `docs/drama/story_brief.md`,
`docs/drama/episode_map.md`, or `docs/drama/final_outline.md` respectively. Continue until all three required tasks are
completed. Do not create episode scripts; scripts are outside this outline-only
workflow.

Every artifact must be written in one complete `apply_patch` call. Before calling `apply_patch`,
compress the draft to its contract limit. Never send or retry an oversized payload;
concise synthesis is part of the acceptance contract.

Skill discovery offers are session-local. For every child task, copy the complete
artifact-specific requirements above into that task's `goal`, `instructions`, and file
output contract. State explicitly in each child instruction: "The parent already loaded
the workflow and this task contains its complete applicable contract; do not call
skill_explorer or skill." A child must execute from those instructions and its declared
upstream files, never spend its tool budget rediscovering this skill.

After the final artifact is accepted, read `docs/drama/final_outline.md` and return its
complete Chinese body as the final answer. Do not return a status report or artifact
list. The final body must contain these section labels: 剧名、类型与受众定位、核心卖点、
主角设定、主要人物关系、三条压力线设计、故事总纲、分集大纲、关键爽点与反转、
情绪推进曲线、结局与主题落点.
