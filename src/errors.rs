#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Account is locked")]
    AccountIsLocked,
}
