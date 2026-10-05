#!/usr/bin/env python3
"""Legacy AppServer SQLite corpus replay (Kernel port of the pinned
ValidationLoopReplayLiveTests.legacy_production_database_replays_under_the_loop).

Reads a COPY of a production appserver.sqlite3 read-only and immutable (never the
live file), proves the schema opens intact, that every historical lineage kind
(including the retired task.repair(...) and branch.repair.coordinator kinds)
parses under the legacy grammar, that no reserved validation control candidate
was ever persisted as a durable record, and exports every workspace's root
threads (up to twelve each, the pinned bound) as neutral turn transcripts for
the Kernel import test (crates/endpoint/tests/legacy_replay.rs), which rebuilds
each thread as a Kernel ledger and projects it through the endpoint journal.

Usage: replay-legacy-sqlite.py <appserver.sqlite3 copy> <output directory>
"""
import json, re, sqlite3, sys
from collections import defaultdict
from pathlib import Path

REQUIRED_TABLES = {'workspaces', 'sessions', 'records', 'attributes', 'endpoint_events_v1', 'usage_records'}
LINEAGE_GRAMMAR = [re.compile(p) for p in (r'^root$', r'^branch\.(worker|memory|validator)$', r'^branch\.repair\.coordinator$', r'^task\.repair\([^()]+\)$', r'^task\([^()]+\)$')]
RESERVED_CANDIDATES = ('validation.route_request', 'validation.settlement_candidate')
THREADS_PER_WORKSPACE = 12

def main(source: Path, out: Path):
    out.mkdir(parents=True, exist_ok=False)
    db = sqlite3.connect(f'file:{source.resolve()}?mode=ro&immutable=1', uri=True)
    db.row_factory = sqlite3.Row
    integrity = db.execute('pragma integrity_check').fetchone()[0]
    assert integrity == 'ok', integrity
    tables = {row[0] for row in db.execute("select name from sqlite_master where type='table'")}
    missing = REQUIRED_TABLES - tables
    assert not missing, f'legacy schema lacks {sorted(missing)}'
    counts = {t: db.execute(f'select count(*) from {t}').fetchone()[0] for t in sorted(tables)}

    kinds = {row[0]: row[1] for row in db.execute("select value,count(*) from attributes where path='lineage' and name='kind' group by 1")}
    unparseable = [k for k in kinds if not any(g.match(k) for g in LINEAGE_GRAMMAR)]
    assert not unparseable, f'unparseable legacy lineage kinds: {unparseable}'
    repair_kinds = {k: n for k, n in kinds.items() if k.startswith('task.repair(') or k == 'branch.repair.coordinator'}
    assert repair_kinds, 'corpus carries no historical repair lineage (acceptance #16 needs one)'
    reserved = {name: db.execute('select count(*) from records where name=?', (name,)).fetchone()[0] for name in RESERVED_CANDIDATES}
    assert all(n == 0 for n in reserved.values()), f'reserved candidate persisted: {reserved}'

    workspaces = [dict(row) for row in db.execute('select id,name,root_path,archived from workspaces order by id')]
    assert workspaces
    threads_dir = out / 'threads'; threads_dir.mkdir()
    exported = []
    for workspace in workspaces:
        roots = db.execute('select id,status,created_at from sessions where workspace_id=? and parent_session_id is null order by created_at', (workspace['id'],)).fetchall()
        for session in roots[:THREADS_PER_WORKSPACE]:
            thread = export_thread(db, session['id'], session['status'])
            (threads_dir / f"{session['id']}.json").write_text(json.dumps(thread, ensure_ascii=False, indent=1))
            exported.append({'workspace': workspace['id'], 'session': session['id'], 'turns': len(thread['turns']), 'messages': sum(1 + len(t['final']) for t in thread['turns']), 'tool_calls': sum(len(t['tools']) for t in thread['turns'])})
    assert exported, 'no root thread exported'
    report = {'source': str(source), 'integrity': integrity, 'tables': counts, 'lineage_kinds': kinds, 'repair_kinds': repair_kinds, 'reserved_candidates': reserved, 'workspaces': len(workspaces), 'threads_exported': len(exported), 'threads': exported}
    (out / 'report.json').write_text(json.dumps(report, ensure_ascii=False, indent=1))
    print(json.dumps({k: v for k, v in report.items() if k != 'threads'}, ensure_ascii=False))

def export_thread(db, session_id, status):
    records = [dict(r) for r in db.execute("select id,target,type,name,version,created_at from records where session_id=? and target='digested' order by version", (session_id,))]
    attributes = defaultdict(dict)
    for row in db.execute("select record_id,layer,path,name,value from attributes where session_id=? and target='digested'", (session_id,)):
        attributes[row['record_id']][(row['path'], row['name'])] = row['value']
    turns = []
    current = None
    for record in records:
        attrs = attributes.get(record['id'], {})
        name = record['name']
        if name == 'input.from_user':
            text = attrs.get(('tool.input', 'user_input')) or attrs.get(('.', 'text')) or ''
            if current is not None and (current['final'] or current['tools'] or current['inputs']):
                turns.append(current)
            current = {'inputs': [text], 'tools': [], 'final': [], 'settlement': None, 'version': record['version']}
            continue
        if current is None:
            continue
        if record['type'] in ('result', 'error') and name.startswith('result.') or name == 'result.write':
            tool = attrs.get(('tool', 'name')) or name.split('.', 2)[1]
            if tool in ('brief', 'output'):
                continue
            current['tools'].append({'name': tool, 'arguments': attrs.get(('response.tool_call', 'arguments')), 'outcome': 'error' if record['type'] == 'error' else 'ok', 'summary': (attrs.get(('tool.output', 'summary')) or attrs.get(('.', 'result')) or '')[:400]})
        elif name == 'result.output' and attrs.get(('.', 'response_final_answer')) == 'true':
            current['final'].append(attrs.get(('.', 'response_text')) or '')
        elif name == 'turn.settlement':
            current['settlement'] = attrs.get(('.', 'status')) or attrs.get(('turn', 'status'))
    if current is not None:
        turns.append(current)
    for turn in turns:
        if turn['settlement'] is None:
            turn['settlement'] = 'completed' if turn['final'] else ('interrupted' if status in ('interrupted', 'stopped') else 'open')
    return {'session': session_id, 'status': status, 'records': len(records), 'turns': turns}

if __name__ == '__main__':
    main(Path(sys.argv[1]), Path(sys.argv[2]))
