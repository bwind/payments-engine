use std::collections::BTreeMap;

use crate::{
    domain::account::Account, errors::AppError, transaction_reader::raw_transaction::RawTransaction,
};

pub trait Engine<A = Account> {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), AppError>;
    fn accounts(&self) -> &BTreeMap<u16, A>;
}

#[derive(Default)]
pub struct InMemoryEngine {
    accounts: BTreeMap<u16, Account>,
}

impl Engine for InMemoryEngine {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), AppError> {
        self.accounts
            .entry(tx.client())
            .or_insert_with(|| Account::new(tx.client()))
            .process_transaction(tx.into())?;
        Ok(())
    }

    fn accounts(&self) -> &BTreeMap<u16, Account> {
        &self.accounts
    }
}
