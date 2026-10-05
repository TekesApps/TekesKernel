# Deployment fixtures

`cases.canonical.json` is the Slice-10 macOS acceptance matrix and fixes each
setup/steps/expected artifact plus required step fields. Each case owns
an isolated temporary storage root and explicit fault points. Test teardown
must prove that no worker/helper/job process, listener, or file lock survives.

All 22 cases are materialized under `cases/`. The top-level selector, stable
selector-update manifest, bundle, complete layout, plist, bootstrap-status,
closed authority-registry/profile compatibility, endpoint credential/discovery,
install identity/installer recovery, failure-attribution, readiness,
publication and observability oracles freeze shared bytes used
across the matrix. `support-bundle/` is the byte-checked three-file
redaction/digest oracle. The canonical plist enters through resident
`tekes-selector serve`; first-activation/publication power-loss cases prove that
this launchd path recovers before spawn rather than relying on a later installer
run. `selector-update-power-loss` proves the stable launcher pathname is always
one complete verified old/new executable.
`provisioning-profiles.canonical.json` freezes the three entitlement-bearing
application containers, identifiers, access groups and runtime-admission
requirement. A successful `codesign --verify` without a matching embedded
profile and executable probe is deliberately insufficient.
`COVERAGE.md` maps cases to Gates 71–76.

Run `scripts/check-deployment-fixtures.py` from any directory to validate
canonical bytes, closed schemas, registry/disk parity and cross-artifact state.
