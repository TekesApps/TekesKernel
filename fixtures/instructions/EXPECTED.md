# Instruction fixture expectations

The `instruction-tree` corpus resolves with user then project0 then project1
precedence. `settings-unknown.invalid.json` rejects the closed settings schema;
`snapshot-bad-index.invalid.json` rejects its dangling effective source index;
`snapshot-wrong-effective.invalid.json` rejects an effective map that is not
the deterministic fold; and `snapshot-wrong-order.invalid.json` rejects
sources outside discovery order. The canonical resolved snapshot fixture is
the byte oracle for that tree.
