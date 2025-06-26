use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_simple() {
    // Verifies that deposits and withdrawals are processed correctly
    let expected = fs::read_to_string("tests/data/simple_out.csv").unwrap();

    Command::cargo_bin("payments-engine")
        .unwrap()
        .arg("tests/data/simple_in.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(&expected));
}

#[test]
fn test_dispute() {
    // Verifies that disputes are processed correctly
    let expected = fs::read_to_string("tests/data/dispute_out.csv").unwrap();

    Command::cargo_bin("payments-engine")
        .unwrap()
        .arg("tests/data/dispute_in.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(&expected));
}

#[test]
fn test_resolve() {
    // Verifies that disputes and resolves are processed correctly
    // Note that a resolved dispute produces the exact same output as a transaction that was never
    // disputed.
    let expected = fs::read_to_string("tests/data/resolve_out.csv").unwrap();

    Command::cargo_bin("payments-engine")
        .unwrap()
        .arg("tests/data/resolve_in.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(&expected));
}
