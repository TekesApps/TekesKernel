fn main() {
    println!("cargo:rerun-if-env-changed=TEKES_INSTALLER_TEAM_ID");
    if let Ok(team) = std::env::var("TEKES_INSTALLER_TEAM_ID") {
        assert!(
            team.len() == 10
                && team
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()),
            "invalid installer team identity"
        );
        println!("cargo:rustc-env=TEKES_INSTALLER_TEAM_ID={team}");
    }
}
