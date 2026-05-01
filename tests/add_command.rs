use std::{
    fs,
    path::{Path, PathBuf},
};

use assert_cmd::Command;
use git2::{Repository, Signature, Time};
use rstest::*;
use serial_test::serial;
use tempfile::TempDir;

#[fixture]
fn repo_with_commit() -> (TempDir, PathBuf) {
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().join("test-repo.git");
    let repo = Repository::init_bare(&repo_path).unwrap();

    let sig = Signature::new("test", "test@example.com", &Time::new(0, 0)).unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
        .unwrap();

    (repo_dir, repo_path)
}

#[fixture]
fn repo_with_commits() -> (TempDir, PathBuf, String) {
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().join("test-repo.git");
    let repo = Repository::init(&repo_path).unwrap();

    let sig = Signature::new("test", "test@example.com", &Time::new(0, 0)).unwrap();
    let tree = repo
        .find_tree({
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        })
        .unwrap();

    let first = repo
        .commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
        .unwrap();

    // move HEAD with second commit
    let tree = repo
        .find_tree({
            fs::write(repo_path.join("a.txt"), b"x").unwrap();
            let mut index = repo.index().unwrap();
            index.add_path(Path::new("a.txt")).unwrap();
            index.write().unwrap();
            index.write_tree().unwrap()
        })
        .unwrap();

    let parent = repo.find_commit(first).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "second", &tree, &[&parent])
        .unwrap();

    (repo_dir, repo_path, first.to_string())
}

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
        .failure()
        .stderr(predicates::str::contains("Git operation failed"));
}

#[rstest]
#[serial]
#[case("add", Some("new-worktree"), "new-worktree")]
#[case("add-feat", None, "feat")]
#[case("add-fix", None, "fix")]
#[case("add-pr", None, "pr")]
fn add_creates_worktree(
    #[case] command: &str,
    #[case] path: Option<&str>,
    #[case] expected_path: &str,
    repo_with_commit: (TempDir, PathBuf),
) {
    let (repo_dir, repo_path) = repo_with_commit;

    let mut args = vec![command, "-b", "new-branch"];
    if let Some(p) = path {
        args.push(p);
    }

    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(repo_path)
        .args(args)
        .assert()
        .success();

    assert!(repo_dir.path().join(expected_path).exists());
}

#[rstest]
#[serial]
fn add_with_commit(repo_with_commits: (TempDir, PathBuf, String)) {
    let (_repo_dir, repo_path, first_commit) = repo_with_commits;
    let branch_name = "new-branch";
    let worktree_dir = "new-worktree";

    Command::cargo_bin("gwt")
        .unwrap()
        .current_dir(&repo_path)
        .args(["add", "-b", branch_name, worktree_dir, &first_commit])
        .assert()
        .success();

    let repo = Repository::open(repo_path.join(worktree_dir)).unwrap();
    let head = repo.head().unwrap();

    assert_eq!(head.target().unwrap().to_string(), first_commit);
    assert!(head.is_branch());
    assert_eq!(head.shorthand(), Some(branch_name));
}
