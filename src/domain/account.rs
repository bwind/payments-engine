use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    domain::stored_transaction::StoredTransaction, errors::TransactionError,
    transaction_reader::raw_transaction::RawTransactionType,
};

#[derive(Serialize, Default)]
pub struct Account {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,

    #[serde(skip)]
    transactions: BTreeMap<u32, StoredTransaction>,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            ..Default::default()
        }
    }

    pub fn process_transaction(&mut self, tx: StoredTransaction) -> Result<(), TransactionError> {
        if self.locked() {
            return Err(TransactionError::AccountIsLocked);
        }

        match tx.tx_type() {
            RawTransactionType::Deposit => {
                self.available += tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            RawTransactionType::Withdrawal => {
                if tx.amount() > self.available {
                    return Err(TransactionError::InsufficientFunds);
                }
                self.available -= tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            RawTransactionType::Dispute => {}
            RawTransactionType::Resolve => {}
            RawTransactionType::Chargeback => {}
        }

        Ok(())
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn update_total(&mut self) {
        self.total = self.available + self.held;
    }
}
