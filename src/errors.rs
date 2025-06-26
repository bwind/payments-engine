#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AppError {
    #[error("Account is locked")]
    AccountIsLocked,

    #[error("Insufficient funds for transaction")]
    InsufficientFunds,

    #[error("Cannot dispute non-deposit transaction")]
    CannotDisputeNonDeposit,

    #[error("Invalid transition for dispute")]
    InvalidDisputeTransition,

    #[error("Transaction not found: {0}")]
    TransactionNotFound(u32),

    #[error("Invalid transition for resolve")]
    InvalidResolveTransition,

    #[error("Invalid transition for chargeback")]
    InvalidChargebackTransition,
}
