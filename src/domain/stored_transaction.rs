use rust_decimal::Decimal;

use crate::{
    errors::TransactionError,
    transaction_reader::raw_transaction::{RawTransaction, RawTransactionType},
};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
enum TransactionState {
    #[default]
    Normal,
    Disputed,
    Resolved,
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

    pub fn dispute(&mut self) -> Result<(), TransactionError> {
        match self.tx_type {
            RawTransactionType::Deposit => {}
            _ => return Err(TransactionError::CannotDisputeNonDeposit),
        }

        match self.state {
            TransactionState::Normal => {
                self.state = TransactionState::Disputed;
                Ok(())
            }
            _ => Err(TransactionError::InvalidDisputeTransition),
        }
    }

    pub fn resolve(&mut self) -> Result<(), TransactionError> {
        match self.state {
            TransactionState::Disputed => {
                self.state = TransactionState::Resolved;
                Ok(())
            }
            _ => Err(TransactionError::InvalidResolveTransition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TransitionCase {
        from: TransactionState,
        action: RawTransactionType,
        expected: Result<TransactionState, TransactionError>,
    }

    #[test]
    fn test_transaction_state_transitions() {
        let cases = vec![
            // Valid transitions
            TransitionCase {
                from: TransactionState::Normal,
                action: RawTransactionType::Dispute,
                expected: Ok(TransactionState::Disputed),
            },
            TransitionCase {
                from: TransactionState::Disputed,
                action: RawTransactionType::Resolve,
                expected: Ok(TransactionState::Resolved),
            },
            // Invalid transitions
            TransitionCase {
                from: TransactionState::Normal,
                action: RawTransactionType::Resolve,
                expected: Err(TransactionError::InvalidResolveTransition),
            },
        ];

        for case in cases {
            let mut tx = StoredTransaction {
                client: 1,
                tx: 1,
                tx_type: RawTransactionType::Deposit,
                amount: Decimal::from(1),
                state: case.from,
            };

            let result = match case.action {
                RawTransactionType::Dispute => tx.dispute().map(|_| tx.state),
                RawTransactionType::Resolve => tx.resolve().map(|_| tx.state),
                _ => panic!("Unexpected transition type: {:?}", case.action),
            };

            match (&result, &case.expected) {
                (Ok(actual), Ok(expected)) => assert_eq!(actual, expected, "case: {:?}", case),
                (Err(e), Err(expected_err)) => {
                    assert_eq!(e, expected_err, "case: {:?}", case)
                }
                _ => panic!("Mismatched result for case: {:?} → got {:?}", case, result),
            }
        }
    }
}
