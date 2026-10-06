# secret-store fixtures

Production no longer reads the Keychain or keeps a generation floor; provider
secrets come from the launching application's environment
([secret-store](../../spec/secret-store.md)). These fixtures still freeze the
record and floor formats used by the retained tests and embedder paths.

These hand-authored bytes freeze the generic-password record format and
resolution classes. `fixture-secret-never-log` may appear only in the active
Keychain-value fixture and private credential-broker lease packets; tests scan
ledger, endpoint, log and carrier outputs for absence.

`generation-floor.canonical.json` freezes the separate durable supervisor
authority. It contains only the highest observed generation and SHA-256 of the
exact Keychain record bytes. The active sentinel and record bytes must never
appear in that authority.
