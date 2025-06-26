use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawTransaction {
    #[serde(rename = "type")]
    tx_type: RawTransactionType,
    client: u16,
    tx: u32,
    amount: Option<Decimal>,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RawTransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl RawTransaction {
    pub fn tx_type(&self) -> RawTransactionType {
        self.tx_type
    }

    pub fn client(&self) -> u16 {
        self.client
    }

    pub fn tx(&self) -> u32 {
        self.tx
    }

    pub fn amount(&self) -> Option<Decimal> {
        self.amount
    }
}
