use rust_decimal::Decimal;

use crate::transaction_reader::raw_transaction::{RawTransaction, RawTransactionType};

#[derive(Default)]
enum TransactionState {
    #[default]
    Normal,
}

pub struct StoredTransaction {
    #[allow(dead_code)]
    client: u16,
    tx: u32,
    amount: Decimal,
    tx_type: RawTransactionType,
    #[allow(dead_code)]
    state: TransactionState,
}

impl From<RawTransaction> for StoredTransaction {
    fn from(raw_tx: RawTransaction) -> Self {
        Self {
            client: raw_tx.client(),
            tx: raw_tx.tx(),
            amount: raw_tx.amount().unwrap_or_default(),
            tx_type: raw_tx.tx_type(),
            state: TransactionState::default(),
        }
    }
}

impl StoredTransaction {
    pub fn tx(&self) -> u32 {
        self.tx
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn tx_type(&self) -> RawTransactionType {
        self.tx_type
    }
}
