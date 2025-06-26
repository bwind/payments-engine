use std::io::Stdout;

use csv::Writer;
use tracing::info;

use crate::transaction_reader::raw_transaction::RawTransaction;

pub trait Engine {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), anyhow::Error>;
    fn flush_accounts(&self, writer: Writer<Stdout>) -> Result<(), anyhow::Error>;
}

#[derive(Default)]
pub struct InMemoryEngine {}

impl Engine for InMemoryEngine {
    fn process_transaction(&mut self, tx: RawTransaction) -> Result<(), anyhow::Error> {
        info!(
            "{:?} {} {} {:?}",
            tx.tx_type(),
            tx.client(),
            tx.tx(),
            tx.amount()
        );
        Ok(())
    }

    fn flush_accounts(&self, _writer: Writer<Stdout>) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
