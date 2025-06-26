use crate::{
    domain::stored_transaction::StoredTransaction,
    transaction_reader::raw_transaction::{RawTransaction, RawTransactionType},
};

pub enum TransactionCommand {
    Deposit(StoredTransaction),
    Withdrawal(StoredTransaction),
    Dispute { tx_id: u32 },
    Resolve { tx_id: u32 },
    Chargeback { tx_id: u32 },
}

impl From<RawTransaction> for TransactionCommand {
    fn from(tx: RawTransaction) -> Self {
        match tx.tx_type() {
            RawTransactionType::Deposit => TransactionCommand::Deposit(tx.into()),
            RawTransactionType::Withdrawal => TransactionCommand::Withdrawal(tx.into()),
            RawTransactionType::Dispute => TransactionCommand::Dispute { tx_id: tx.tx() },
            RawTransactionType::Resolve => TransactionCommand::Resolve { tx_id: tx.tx() },
            RawTransactionType::Chargeback => TransactionCommand::Chargeback { tx_id: tx.tx() },
        }
    }
}
