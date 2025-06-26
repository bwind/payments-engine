use std::fs::File;

use anyhow::{Context, Result};

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
        .from_reader(file)
        .into_records();

    for result in reader {
        match result {
            Ok(record) => {
                println!("{:?}", record);
            }
            Err(e) => eprintln!("Skipping invalid record: {:?}", e),
        }
    }

    Ok(())
}
