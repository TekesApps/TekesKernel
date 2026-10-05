# Rewrite oracle

- `op-fork-prepared.canonical.json` and
  `op-redact-published.canonical.json` are the exact RFC-8785+LF operation
  records from rewrite-publication.
- The records contain no redaction plaintext. Redaction selection is an exact
  event/asset inventory plus SHA-256 fingerprints.
- Gate 15 derives projected ledgers and assets from a live source folder;
  gates 16/24 exercise every durable phase and staging ownership boundary.
