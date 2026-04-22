use assert_cmd::Command;
use rstest::*;

#[rstest]
fn init_fails_without_url() {
    Command::cargo_bin("gwt")
        .unwrap()
        .arg("init")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}
