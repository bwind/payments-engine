use std::io::Read;

use crate::transaction_reader::raw_transaction::RawTransaction;

pub struct RawTransactionReader<R: Read> {
    inner: csv::DeserializeRecordsIntoIter<R, RawTransaction>,
}

impl<R: Read> RawTransactionReader<R> {
    pub fn new(reader: R) -> Self {
        let inner = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_reader(reader)
            .into_deserialize();
        Self { inner }
    }
}

impl<R: Read> Iterator for RawTransactionReader<R> {
    type Item = Result<RawTransaction, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|result| result.map_err(|_| ()))
    }
}
