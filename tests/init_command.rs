use std::env::set_current_dir;

use assert_cmd::Command;
use git2::Repository;
use rstest::*;
use serial_test::serial;
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
#[serial]
fn init_creates_workspace_with_correct_name(repository: (TempDir, String)) {
    let (repo_dir, repo_url) = repository;

    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(&repo_dir)
        .args(["init", &repo_url])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created"));

    assert!(repo_dir.path().join("test-repo-workspace").exists());
}

#[rstest]
#[serial]
fn init_creates_workspace_with_custom_name(repository: (TempDir, String)) {
    let (repo_dir, repo_url) = repository;

    let workspace_name = "my-custom-workspace";
    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(&repo_dir)
        .args(["init", "--name", workspace_name, &repo_url])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created"));

    assert!(repo_dir.path().join(workspace_name).exists());
}
