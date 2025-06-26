use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    domain::stored_transaction::StoredTransaction, errors::TransactionError,
    transaction_reader::raw_transaction::RawTransactionType,
};

#[derive(Default, Serialize)]
pub struct Account {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

impl Account {
    pub fn process_transaction(&mut self, tx: StoredTransaction) -> Result<(), TransactionError> {
        if self.locked() {
            return Err(TransactionError::AccountIsLocked);
        }

        match tx.tx_type() {
            RawTransactionType::Deposit => {
                self.available += tx.amount();
                self.update_total();
            }
            RawTransactionType::Withdrawal => {}
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
