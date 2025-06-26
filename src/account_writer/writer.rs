use std::io::Write;

use crate::domain::account::Account;

pub struct AccountWriter<W: Write> {
    inner: csv::Writer<W>,
}

impl<W: Write> AccountWriter<W> {
    pub fn new(writer: W) -> Self {
        let inner = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(writer);
        Self { inner }
    }

    pub fn write(&mut self, account: &Account) -> Result<(), csv::Error> {
        self.inner.serialize(account)
    }
}
