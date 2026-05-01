use assert_cmd::Command;
use rstest::*;
use tempfile::TempDir;

#[rstest]
#[case("add")]
#[case("add-feat")]
#[case("add-fix")]
#[case("add-pr")]
fn add_fails_without_branch(#[case] command: &str) {
    Command::cargo_bin("gwt")
        .unwrap()
        .args([command, "some-path"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("-b <BRANCH>"));
}

#[rstest]
fn add_fails_outside_repo() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(dir.path())
        .args(["add", "some-path", "-b", "some-branch"])
        .assert()
        .success()
        .stderr(predicates::str::contains("Git operation failed"));
}
