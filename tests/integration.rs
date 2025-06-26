use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn assert_csv_output(test_case: &str, expect_exact_match: bool) {
    let input_path = format!("tests/data/{}_in.csv", test_case);
    let expected_path = format!("tests/data/{}_out.csv", test_case);

    let expected = fs::read_to_string(&expected_path)
        .unwrap_or_else(|_| panic!("Expected output file not found: {expected_path}"));

    let command = Command::cargo_bin("payments-engine")
        .unwrap()
        .arg(&input_path)
        .assert()
        .success();

    if expect_exact_match {
        command.stdout(predicate::eq(expected));
    } else {
        command.stdout(predicate::str::contains(&expected));
    }
}

#[test]
fn test_simple() {
    // // Verifies that deposits and withdrawals are processed correctly
    assert_csv_output("simple", true);
}

#[test]
fn test_dispute() {
    // Verifies that disputes are processed correctly
    assert_csv_output("dispute", true);
}

#[test]
fn test_resolve() {
    // Verifies that disputes and resolves are processed correctly
    // Note that a resolved dispute produces the exact same output as a transaction that was never
    // disputed.
    assert_csv_output("resolve", true);
}

#[test]
fn test_chargeback() {
    // Verifies that disputes and chargebacks are processed correctly
    assert_csv_output("chargeback", true);
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
    //
    // Notice that only the columns are included in garbage_out.csv, as we are mainly interested in
    // the header and the fact that the program does not panic.
    assert_csv_output("garbage", false);
}
