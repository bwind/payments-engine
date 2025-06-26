# Payments Engine 🦀

A simple, streaming payments engine written in Rust. It reads a CSV file of transactions and outputs the final balances per client.

## Usage

Run the engine like this:

```
cargo run -- transactions.csv
```

The engine reads the CSV line by line and prints final balances to stdout.

## Design

The core logic is modeled as a **state machine** for transactions. Each `StoredTransaction` tracks its state via an enum (`Normal`, `Disputed`, `Resolved`, `ChargedBack`), and transitions are enforced via explicit methods like `dispute()`, `resolve()`, and `chargeback()`.

All accounts are stored in memory, indexed by client ID, and updated incrementally as transactions are processed.

## Potential Improvements

With more time, the following improvements could be explored:

* **Command pattern**: Extract transaction logic (`deposit`, `withdrawal`, etc.) into separate commands to improve modularity.
* **Stronger state transitions**: Use types to represent transaction states (e.g. `Deposit`, `DisputedDeposit`, etc.), making invalid transitions unrepresentable at compile time.
* **Memory optimization**: Accounts could be flushed from memory immediately after a `chargeback` if their state is final.
* **Better error reporting**: Include CSV line numbers and raw content in error messages for failed rows.
* **Benchmarking**: Use the `criterion` crate to test performance on large datasets.
* **Concurrency**: Process transactions in parallel, for instance by wrapping Engine in `Arc<Mutex<>>` to allow multiple threads to handle transactions concurrently.
* **Total funds calculation**: Use a calculated field for `Account.total` instead of using `update_total()` after every transaction.
* **Money struct instead of Decimal**: Use a custom `Money(u64)` struct with a 10_000 factor for performance and precision, avoiding floating-point arithmetic.

## Testing

This project includes:

* **Unit tests** for transaction state transitions and edge cases.
* **Integration tests** that verify full engine output from real CSV input files (see `tests/data`).

Run all tests with:

```
cargo test
```
