# Client extension v1 oracle

This corpus is the executable `.tekes` Client extension catalog: a
17-method base and 19 capabilities with 63 methods. It was first frozen in
Slice 14F and has been extended since. It is separate from the Session
Endpoint base routes and does not define a new `SessionEvent` family.

- `catalog.canonical.json` is the atomic capability/method/class registry.
- `method-cases.canonical.json` supplies one closed request and success value
  for every retained method.
- `errors.canonical.json` freezes common and capability-specific failures and
  the absent-capability behavior.
- `dispositions.canonical.json` inventories the pinned predecessor routes and
  gives every one a base, implemented replacement, intentional replacement,
  or retired outcome.
- `negative.canonical.json` freezes partial-capability, frozen-name collision,
  unknown-field, fake-empty-success, unproved-provider, ignored-schedule-
  policy, ignored-schedule-model, missing route identity, non-cascading
  provider/profile delete, and TCU-special-case rejection.
- `value-cases.canonical.json` covers every resource/content/tool-source union,
  plugin/provider readiness arm, zero/multiple-profile connection readiness,
  valid workspace-policy relaxation below its ceiling, and the complete
  provider route target.

All canonical JSON files contain one RFC-8785 object plus LF. Values are
secret-free. `scripts/check-client-extension-fixtures.py` compares the corpus
with disk and checks both directions of the catalog/disposition relation.
