#!/usr/bin/env python3
"""Hidden acceptance checks for the research-brief corpus.

usage: research_brief_acceptance.py <run>/workspace/brief.md

The agent never sees these checks. They test that the final brief carries the
source facts the three prompts ask for, stays within the word limit, and states
no wrong number or date.
"""
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text()
lower = text.lower()
failures = []


def check(name, ok):
    if not ok:
        failures.append(name)


def has(*alternatives):
    return any(re.search(pattern, text, re.I) for pattern in alternatives)


for section in ('decisions', 'metrics', 'open questions', 'risks'):
    check(f'section {section}', re.search(rf'(?m)^\s*(#+\s*|\*\*)?{section}\b', lower))
check('launch 18 November', has(r'\b18\s*nov', r'\bnov(ember)?\.?\s*18\b'))
check('Pro price $24', has(r'\$\s*24\b'))
check('Team price $12', has(r'\$\s*12\b'))
check('Windows beta dropped', has(r'windows'))
check('weekly active teams 1,240', has(r'\b1,?240\b'))
check('churn 3.1%', has(r'\b3\.1\s*%'))
check('NPS 41', has(r'\bnps\b[^\n]{0,40}\b41\b', r'\b41\b[^\n]{0,40}\bnps\b'))
check('trial-to-paid 18%', has(r'\b18\s*%'))
check('PDF export risk', has(r'\bpdf\b'))
check('PDF export from three customers', has(r'\b(three|3)\s+customers?\b'))
check('50 MB threshold', has(r'\b50\s*mb\b'))
check('SSO risk', has(r'\bsso\b'))
check('SSO from two tickets', has(r'\b(two|2)\s+(support\s+)?tickets?\b'))
check('support briefing open question', has(r'support[^\n]{0,80}brief', r'brief[^\n]{0,80}support'))
check('no wrong Pro price', not has(r'pro[^\n]{0,30}\$\s*(12|20|25)\b'))
check('no wrong churn', not has(r'churn[^\n]{0,20}\b(2\.9|3\.3)\s*%'))
words = len(re.findall(r"[A-Za-z0-9$%][\w$%.,'/-]*", text))
check(f'at most 300 words (got {words})', words <= 300)

if failures:
    print('research-brief acceptance FAILED: ' + '; '.join(failures))
    sys.exit(1)
print(f'research-brief acceptance passed ({words} words)')
