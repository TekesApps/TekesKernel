"""Pinned AgentEval task inputs from TekesAppServer e4df9dc9 Tests/AgentEval/AgentEvalTasks.swift."""
TASKS = {
    "memory-recall-codename": {
        "dimension": "memory",
        "prompts": [
            "Remember this for later: the internal codename for this project is BLUEHERON. Reply with exactly: NOTED",
            "What is the internal codename for this project? Reply with only the codename, nothing else.",
        ],
        "seed_files": {},
    },
    "subagent-summarize-notes": {
        "dimension": "subagent",
        "prompts": ["Use a subagent to read NOTES.md and produce a one-sentence summary, then give me only that sentence."],
        "seed_files": {"NOTES.md": "# Project Notes\nThe release train ships every other Tuesday. The on-call rotation is weekly.\nThe staging bucket is named tekes-staging-eu. Feature flags live in flags.json."},
    },
}
