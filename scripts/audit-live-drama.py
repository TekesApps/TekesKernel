#!/usr/bin/env python3
"""Check the original drama deliverable contract against saved files and ledger."""
import json
from pathlib import Path
import re
import sys
root = Path(sys.argv[1])
ledger = next((root/'runtime/threads').glob('*/main.jsonl'))
events = [json.loads(line) for line in ledger.read_text().splitlines()]
settles = [e for e in events if e['kind']=='settle']
assert len(settles)==1 and settles[0]['outcome']=='completed'
settle=settles[0]
assert settle['validation']['outcome']=='pass'
output=next(e for e in events if e['seq']==settle['validation']['promoted_output_seq'])
answer=''.join(b.get('text','') for b in output['content'])
labels=['剧名','类型与受众定位','核心卖点','主角设定','主要人物关系','三条压力线','故事总纲','分集大纲','关键爽点与反转','情绪推进曲线','结局与主题落点']
for label in labels: assert label in answer, f'missing final section: {label}'
for forbidden in ['已完成完整 workflow','当前无新增缺失任务','可交付产物已齐备','如需我可以继续整理']:
    assert forbidden not in answer
assert len(answer)<=4200, 'final answer exceeds 4200 characters'
names=['story_brief','episode_map','final_outline']
bodies={name:(root/'workspace/docs/drama'/f'{name}.md').read_text() for name in names}
for name,limit in zip(names,[1200,2400,4200]): assert len(bodies[name])<=limit,(name,len(bodies[name]),limit)
def outline_body(text):
    # The contract is complete prose delivery, not byte-identical Markdown.
    # Ignore only line-edge whitespace and spacing after a bold section label.
    # Never discard headings, punctuation, words, or paragraph boundaries.
    lines=[re.sub(r'(\*\*[^*\n]+[：:]\*\*)[ \t]+',r'\1',line.strip()) for line in text.strip().splitlines()]
    return '\n'.join(lines)
assert outline_body(answer)==outline_body(bodies['final_outline']), 'final answer must deliver complete saved outline'
# Numbered paragraphs may use Arabic episode labels or ordinary Markdown lists.
def episodes(text):
    pattern=r'(?m)^\s*(?:#{1,6}\s*)?(?:第\s*(\d+)\s*集[：:、.．]?|(\d+)[、.．)])\s*'
    matches=list(re.finditer(pattern,text))
    assert len(matches)==12, f'expected 12 numbered episode paragraphs, got {len(matches)}'
    assert [int(m.group(1) or m.group(2)) for m in matches]==list(range(1,13))
    lengths=[]
    for i,m in enumerate(matches):
        body=text[m.end():matches[i+1].start() if i+1<len(matches) else len(text)].strip()
        assert 80<=len(body)<=120,(i+1,len(body))
        lengths.append(len(body))
    return lengths
episode_lengths=episodes(bodies['episode_map'])
final_episodes=bodies['final_outline'].split('分集大纲',1)[1].split('关键爽点与反转',1)[0].strip().rstrip('#').strip()
final_episode_lengths=episodes(final_episodes)
calls=[e for e in events if e['kind']=='tool_call' and e['name']=='task']
assert set(e['args']['task_name'] for e in calls)==set(names)
for index,call in enumerate(calls):
    sources={'story_brief':[],'episode_map':['story_brief'],'final_outline':['story_brief','episode_map']}[call['args']['task_name']]
    assert call['args']['input_sources']==sources
    child_result=next(e for e in events if e['kind']=='child_result' and e.get('call')==call['call'])
    assert child_result['outcome']=='completed'
    if index+1<len(calls): assert child_result['seq']<calls[index+1]['seq'], 'dependent task began before upstream completion'
    child=[json.loads(line) for line in (ledger.parent/child_result['child']).read_text().splitlines()]
    assert any(e['kind']=='settle' and e['outcome']=='completed' for e in child)
    delegation=next(e for e in events if e['kind']=='state' and e.get('subkind')=='delegation' and e['payload']['call']==call['call'])
    if 'resolved_inputs' in delegation['payload']:
        resolved=delegation['payload']['resolved_inputs']
        assert [entry['source'] for entry in resolved]==sources
        for entry in resolved:
            expected_path=(root/'workspace'/entry['path']).resolve()
            assert expected_path==(root/'workspace/docs/drama'/f"{entry['source']}.md").resolve()
            reads=[e for e in child if e['kind']=='tool_call' and e['name']=='read' and (root/'workspace'/e['args']['path']).resolve()==expected_path]
            assert any(e['kind']=='tool_result' and e['outcome']=='ok' and any(read['call']==e['call'] for read in reads) for e in child), 'resolved upstream file was not successfully read'
    writes=[e for e in child if e['kind']=='tool_call' and e['name']=='apply_patch']
    assert any(e['kind']=='tool_result' and e['outcome']=='ok' and any(w['call']==e['call'] for w in writes) for e in child), 'child did not successfully write its artifact'
    assert any(e['kind']=='tool_result' and e['call']==call['call'] and e['outcome']=='ok' for e in events)
skills=[e for e in events if e['kind']=='tool_call' and e['name']=='skill']
assert any(e['kind']=='tool_result' and e['outcome']=='ok' and any(c['call']==e['call'] for c in skills) for e in events)
receipt={'status':'passed','settle_seq':settle['seq'],'episode_lengths':episode_lengths,'final_episode_lengths':final_episode_lengths,'artifact_lengths':{k:len(v) for k,v in bodies.items()},'answer_chars':len(answer),'layout_only_difference':answer.strip()!=bodies['final_outline'].strip()}
(root/'drama-audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt))
