use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin("dscribe")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}
#[test]
fn accept_path() -> Result<()> {
    Command::cargo_bin("dscribe")?
        .args(&["data_examples/onlinefoods.csv", "head"])
        .assert()
        .success();
    Ok(())
}
