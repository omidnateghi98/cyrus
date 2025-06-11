//! Integration tests for CLI commands

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("cyrus").unwrap();
    cmd.arg("version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cyrus"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("cyrus").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All-in-One Language Management Tool"));
}
