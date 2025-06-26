use std::fs::File;

use anyhow::{Context, Result};
use tracing::error;

use crate::{
    account_writer::writer::AccountWriter,
    engine::{Engine, InMemoryEngine},
    transaction_reader::reader::RawTransactionReader,
};

mod account_writer;
mod domain;
mod engine;
mod errors;
mod transaction_reader;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(false)
        .init();

    let path = std::env::args()
        .nth(1)
        .context("Usage: cargo run -- <transactions.csv>")?;

    let file = File::open(&path).with_context(|| format!("Failed to open input path: {path}"))?;
    let transaction_reader = RawTransactionReader::new(file);
    let mut engine = InMemoryEngine::default();

    for result in transaction_reader {
        if let Ok(raw_tx) = result {
            if engine.process_transaction(raw_tx).is_err() {
                // Skip the transaction if processing fails
            }
        } else {
            // Silently skip invalid transactions
        }
    }

    let mut writer = AccountWriter::new(std::io::stdout());
    for account in engine.accounts().values() {
        if let Err(e) = writer.write(account) {
            error!("Failed to write account: {:?}", e);
        }
    }
    writer.flush()?;

    Ok(())
}
