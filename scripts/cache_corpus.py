"""Preserved multi-turn inputs for the real workflow cache measurement."""
import json
from pathlib import Path

_CORPUS = Path(__file__).resolve().parents[1] / 'fixtures/live/cache-corpus/prompts.json'


def prompts(scenario):
    if scenario == 'long-session-cache':
        from long_session_cache import prompts as long_session_prompts
        return long_session_prompts()
    if scenario == 'large-prefix-cache':
        from long_session_cache import large_prefix_prompts
        return large_prefix_prompts()
    corpus = json.loads(_CORPUS.read_text())
    if scenario not in corpus:
        raise ValueError(f'Unknown cache corpus scenario: {scenario!r}')
    return corpus[scenario]


def seed_files(scenario):
    if scenario != 'batch-ledger-bugfix':
        return {}
    directory = _CORPUS.parent / scenario
    return {path.name: path.read_text() for path in sorted(directory.iterdir())
            if path.is_file()}


def require_successful_root_completion(outcome, turn_index):
    # Kernel's durable completed settlement corresponds to the legacy fact
    # completion. Missing, held, interrupted and error outcomes are not success.
    if outcome != 'completed':
        raise ValueError(f'Root turn {turn_index} did not complete successfully: {outcome!r}')
