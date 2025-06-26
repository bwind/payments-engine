# Payments Engine 🦀

A simple payments engine written in Rust, designed to read a CSV file containing transactions and output the final account balances after processing all transactions.

## Usage

    cargo run -- accounts.csv

## Design

The design of the payments engine is based on a simple state machine for transactions within accounts. The state of each transaction is managed using an enum (`TransactionState`) to represent the different states (`Normal`, `Dispute`, `Resolve`, `Chargeback`), and is enforced through methods on the `StoredTransaction` struct (`dispute`, `resolve`, `chargeback`).

## Further Improvements

Given more time, the following improvements could be made:

- **Encapsulate transaction processing logic** (`deposit`, `withdrawal`, etc.) into separate command objects using the Command pattern. This would allow for easier extension and modification of transaction types without changing the core processing logic.
- **Improve the State Machine implementation.** There's different (and probably better) ways of implementing state machines in Rust, such as using enums with methods for state transitions, or using concrete structs for each state (eg. `Deposit.dispute` -> `Dispute.resolve` -> `Resolve`).

## Testing

The project includes a handful of unit tests to ensure the correctness of the allowed and disallowed transitions for transactions, as well as a few integration tests to validate the CLI functionality. These integration tests treat the system as a black box, ensuring that the output matches the expected results based on input CSV files.

The tests can be run using:

    cargo test

Given more time, benchmark tests could be added using the `criterion` crate to measure performance, especially for large datasets.
