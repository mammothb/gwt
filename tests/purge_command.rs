use std::{env::set_current_dir, fs, path::PathBuf};

use assert_cmd::Command;
use git2::Repository;
use rstest::*;
use serial_test::serial;
use tempfile::TempDir;

#[fixture]
fn repository() -> (TempDir, PathBuf) {
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().join("test-repo.git");

    Repository::init_bare(&repo_path).unwrap();

    (repo_dir, repo_path)
}

#[fixture]
fn repository_with_config() -> (TempDir, PathBuf, PathBuf) {
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().join("test-repo.git");
    let config_path = repo_path.join("config");

    let repo = Repository::init_bare(&repo_path).unwrap();
    let mut config = repo.config().unwrap();
    config
        .set_str("branch.main.merge", "refs/heads/main")
        .unwrap();
    config
        .set_str("branch.feature.merge", "refs/heads/feature")
        .unwrap();

    (repo_dir, repo_path, config_path)
}

// #[rstest]
// #[serial]
// fn purge_succeeds_empty_tracked_branches(repository: (TempDir, PathBuf)) {
//     // Arrange
//     let (_repo_dir, repo_path) = repository;
//
//     // Act + Assert
//     Command::cargo_bin("gwt")
//         .unwrap()
//         .current_dir(&repo_path)
//         .arg("purge")
//         .assert()
//         .success();
// }

#[rstest]
#[serial]
fn purge_removes_orphaned_branch_config(repository_with_config: (TempDir, PathBuf, PathBuf)) {
    // Arrange
    let (_repo_dir, repo_path, config_path) = repository_with_config;

    // Act
    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(&repo_path)
        .arg("purge")
        .assert()
        .success();

    // Assert
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(!content.contains("branch \"main\""));
    assert!(!content.contains("branch \"feature\""));
}
