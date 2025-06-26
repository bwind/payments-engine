use std::fs::File;

use anyhow::{Context, Result};

use crate::transaction_reader::raw_transaction::RawTransaction;

mod transaction_reader;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("Usage: cargo run -- <transactions.csv>")?;

    let file = File::open(&path).with_context(|| format!("Failed to open input path: {}", path))?;

    let reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(file)
        .into_deserialize::<RawTransaction>();

    for result in reader {
        match result {
            Ok(raw_tx) => {
                println!(
                    "{:?} {} {} {}",
                    raw_tx.tx_type(),
                    raw_tx.client(),
                    raw_tx.tx(),
                    raw_tx.amount().unwrap_or_default()
                );
            }
            Err(e) => eprintln!("Skipping invalid record: {:?}", e),
        }
    }

    Ok(())
}
