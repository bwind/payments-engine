use std::collections::BTreeMap;

use crate::{domain::account::Account, transaction_reader::raw_transaction::RawTransaction};

pub trait Engine {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), anyhow::Error>;
    fn accounts(&self) -> &BTreeMap<u16, Account>;
}

#[derive(Default)]
pub struct InMemoryEngine {
    accounts: BTreeMap<u16, Account>,
}

impl Engine for InMemoryEngine {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), anyhow::Error> {
        self.accounts
            .entry(tx.client())
            .or_default()
            .process_transaction(&tx);
        Ok(())
    }

    fn accounts(&self) -> &BTreeMap<u16, Account> {
        &self.accounts
    }
}
