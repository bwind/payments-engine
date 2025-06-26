use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    domain::{stored_transaction::StoredTransaction, transaction_command::TransactionCommand},
    errors::AppError,
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

    pub fn process_transaction(&mut self, command: TransactionCommand) -> Result<(), AppError> {
        if self.is_locked() {
            return Err(AppError::AccountIsLocked);
        }

        match command {
            TransactionCommand::Deposit(tx) => {
                // A deposit always increases the available balance and total balance.
                self.available += tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            TransactionCommand::Withdrawal(tx) => {
                // A withdrawal decreases the available balance and total balance if there are
                // sufficient funds.
                if tx.amount() > self.available {
                    return Err(AppError::InsufficientFunds);
                }
                self.available -= tx.amount();
                self.update_total();
                self.transactions.insert(tx.tx(), tx);
            }
            TransactionCommand::Dispute { tx_id } => {
                // A dispute holds the funds of a transaction, moving them from available to held.
                let tx = self.get_transaction_mut(tx_id)?;
                tx.dispute()?;
                let amount = tx.amount();
                self.available -= amount;
                self.held += amount;
            }
            TransactionCommand::Resolve { tx_id } => {
                // A resolve releases the held funds back to available.
                let tx = self.get_transaction_mut(tx_id)?;
                tx.resolve()?;
                let amount = tx.amount();
                self.held -= amount;
                self.available += amount;
            }
            TransactionCommand::Chargeback { tx_id } => {
                // A chargeback locks the account and moves held funds to total.
                let tx = self.get_transaction_mut(tx_id)?;
                tx.chargeback()?;
                let amount = tx.amount();
                self.held -= amount;
                self.locked = true;
                self.update_total();
            }
        }
        Ok(())
    }

    fn get_transaction_mut(&mut self, tx_id: u32) -> Result<&mut StoredTransaction, AppError> {
        self.transactions
            .get_mut(&tx_id)
            .ok_or(AppError::TransactionNotFound(tx_id))
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn update_total(&mut self) {
        self.total = self.available + self.held;
    }
}
