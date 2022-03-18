use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("meowsay")
        .expect("binary exists")
        .assert()
        .stdout(predicate::str::contains("Created by Taufiq Rahman"))
        .success();

    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("meowsay")
        .expect("binary exists")
        .args(&["-f", "non_existing_file.txt"])
        .assert()
        .failure();
    Ok(())
}
