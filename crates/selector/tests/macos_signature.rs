#![cfg(target_os = "macos")]

use std::env;
use std::path::Path;

use tekes_selector::{CodeSignatureVerifier, MacOsCodeSignatureVerifier};

#[test]
#[ignore = "requires an explicitly selected, signed and provisioned macOS app"]
fn provisioned_app_is_accepted_by_the_real_macos_verifier() {
    let app = env::var("TEKES_TEST_PROFILED_APP").expect("TEKES_TEST_PROFILED_APP");
    let requirement = env::var("TEKES_TEST_APP_REQUIREMENT").expect("TEKES_TEST_APP_REQUIREMENT");
    let team = env::var("TEKES_TEST_APP_TEAM").expect("TEKES_TEST_APP_TEAM");
    let identifier = env::var("TEKES_TEST_APP_IDENTIFIER").expect("TEKES_TEST_APP_IDENTIFIER");
    let groups = env::var("TEKES_TEST_APP_ACCESS_GROUPS")
        .expect("TEKES_TEST_APP_ACCESS_GROUPS")
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();

    MacOsCodeSignatureVerifier
        .verify_provisioned_app(Path::new(&app), &requirement, &team, &identifier, &groups)
        .expect("real macOS app/profile verification");
}
