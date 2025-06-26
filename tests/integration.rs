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

#[test]
fn test_chargeback() {
    // Verifies that disputes and chargebacks are processed correctly
    let expected = fs::read_to_string("tests/data/chargeback_out.csv").unwrap();

    Command::cargo_bin("payments-engine")
        .unwrap()
        .arg("tests/data/chargeback_in.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(&expected));
}

#[test]
fn test_garbage() {
    // Verifies that garbage input is handled gracefully:
    // - invalid RawTransactionType enum values
    // - non-numeric client IDs
    // - non-numeric amounts
    // - empty transaction IDs
    // - empty client IDs
    // - disputes on non-existing transactions
    // - withdrawals that exceed available funds
    let expected = fs::read_to_string("tests/data/garbage_out.csv").unwrap();

    Command::cargo_bin("payments-engine")
        .unwrap()
        .arg("tests/data/garbage_in.csv")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(&expected));
}
