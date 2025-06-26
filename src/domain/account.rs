use serde::Serialize;

use crate::transaction_reader::raw_transaction::{RawTransaction, RawTransactionType};

#[derive(Serialize, Default)]
pub struct Account {
    pub client: u16,
    pub available: rust_decimal::Decimal,
    pub held: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub locked: bool,
}

impl Account {
    pub fn process_transaction(&mut self, tx: &RawTransaction) {
        if matches!(tx.tx_type(), RawTransactionType::Deposit) {
            if let Some(amount) = tx.amount() {
                self.available += amount;
                self.total += amount;
            }
        }
    }
}
