# deployment-tests

This non-published harness is deliberately separate from Kernel authority
crates. Portable tests validate deployment fixture/disk parity and packaging
rejections. `TEKES_SLICE10_SELECTOR_BIN` and `TEKES_SLICE10_SUPERVISOR_BIN` are
explicit executable seams for the real selector/daemon artifacts; no mock
binary is silently treated as production evidence.

The installer-recovery test copies the four actual built executables, renders
the exact plist, assembles/verifies the real bundle bytes, drives crash recovery
and proves uninstall preservation. It is supporting evidence, not Gate 76.

`scripts/ci-slice10.sh` uses a repository-built signed coordinator app and
requires `TEKES_SLICE10_CLIENT_UAT` to name the separately signed Client app.
The
coordinator owns safe Keychain setup/cleanup and launchctl lifecycle; the
Client actor owns `.tekes` behavior and reboot continuation through the closed
protocol in `packaging/macos/PRODUCTION-UAT.md`. Missing production input is a
hard release failure; portable success never claims launchd, Keychain,
code-signing, provisioning-profile/AMFI admission, notarization, reboot or
Client UAT acceptance.
