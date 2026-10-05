# Hook oracle

- `pre-request.json` and `post-request.json` are accepted canonical requests.
- `pre-allow.json`, `pre-deny.json`, `pre-mutate.json`, `pre-ask.json`,
  `post-allow.json`, and `post-deny.json` cover every verdict arm.
- `correlation.invalid.json` parses as a response but must be rejected against
  `pre-request.json` because `call` differs.
- `extra-output.invalid.txt` is invalid because a hook emits more than one
  stdout line.
