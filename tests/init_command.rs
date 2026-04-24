use std::env::set_current_dir;

use assert_cmd::Command;
use git2::Repository;
use rstest::*;
use tempfile::TempDir;

#[fixture]
fn repository() -> (TempDir, String) {
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().join("test-repo.git");
    Repository::init_bare(&repo_path).unwrap();
    (repo_dir, format!("file://{}", repo_path.display()))
}

#[rstest]
fn init_fails_without_url() {
    Command::cargo_bin("gwt")
        .unwrap()
        .arg("init")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[rstest]
fn init_creates_workspace_with_correct_name(repository: (TempDir, String)) {
    set_current_dir(&repository.0).unwrap();

    Command::cargo_bin("gwt")
        .unwrap()
        .args(["init", &repository.1])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created"));

    assert!(repository.0.path().join("test-repo-workspace").exists());
}

#[rstest]
fn init_creates_workspace_with_custom_name(repository: (TempDir, String)) {
    set_current_dir(&repository.0).unwrap();

    let workspace_name = "my-custom-workspace";
    Command::cargo_bin("gwt")
        .unwrap()
        .args(["init", "--name", workspace_name, &repository.1])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created"));

    assert!(repository.0.path().join(workspace_name).exists());
}
