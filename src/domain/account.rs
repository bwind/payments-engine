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
                let disputed_tx = self.get_transaction_mut(tx.tx())?;
                if disputed_tx.dispute().is_ok() {
                    let amount = disputed_tx.amount();
                    self.available -= amount;
                    self.held += amount;
                }
            }
            RawTransactionType::Resolve => {
                // A resolve releases the held funds back to available.
                let resolved_tx = self.get_transaction_mut(tx.tx())?;
                if resolved_tx.resolve().is_ok() {
                    let amount = resolved_tx.amount();
                    self.held -= amount;
                    self.available += amount;
                }
            }
            RawTransactionType::Chargeback => {
                // A chargeback locks the account and moves held funds to total.
                let chargeback_tx = self.get_transaction_mut(tx.tx())?;
                if chargeback_tx.chargeback().is_ok() {
                    let amount = chargeback_tx.amount();
                    self.locked = true;
                    self.held -= amount;
                    self.update_total();
                }
            }
        }

        Ok(())
    }

    fn get_transaction_mut(
        &mut self,
        tx_id: u32,
    ) -> Result<&mut StoredTransaction, TransactionError> {
        self.transactions
            .get_mut(&tx_id)
            .ok_or(TransactionError::TransactionNotFound(tx_id))
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn update_total(&mut self) {
        self.total = self.available + self.held;
    }
}
