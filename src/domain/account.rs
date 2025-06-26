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
                // A deposit always increases the available balance and total balance.
                self.available += tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            RawTransactionType::Withdrawal => {
                // A withdrawal decreases the available balance and total balance if there are
                // sufficient funds.
                if tx.amount() > self.available {
                    return Err(TransactionError::InsufficientFunds);
                }
                self.available -= tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            RawTransactionType::Dispute => {
                // A dispute holds the funds of a transaction, moving them from available to held.
                let Some(disputed_tx) = self.transactions.get_mut(&tx.tx()) else {
                    return Err(TransactionError::TransactionNotFound(tx.tx()));
                };
                if disputed_tx.dispute().is_ok() {
                    self.available -= disputed_tx.amount();
                    self.held += disputed_tx.amount();
                }
            }
            RawTransactionType::Resolve => {
                // A resolve releases the held funds back to available.
                let Some(resolved_tx) = self.transactions.get_mut(&tx.tx()) else {
                    return Err(TransactionError::TransactionNotFound(tx.tx()));
                };
                if resolved_tx.resolve().is_ok() {
                    self.held -= resolved_tx.amount();
                    self.available += resolved_tx.amount();
                }
            }
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
