# secret-store fixtures

These hand-authored bytes freeze the generic-password record format and
resolution classes. `fixture-secret-never-log` may appear only in the active
Keychain-value fixture and private credential-broker lease packets; tests scan
ledger, endpoint, log and carrier outputs for absence.

`generation-floor.canonical.json` freezes the separate durable supervisor
authority. It contains only the highest observed generation and SHA-256 of the
exact Keychain record bytes. The active sentinel and record bytes must never
appear in that authority.
