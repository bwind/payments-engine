#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Account is locked")]
    AccountIsLocked,

    #[error("Insufficient funds for transaction")]
    InsufficientFunds,
}
