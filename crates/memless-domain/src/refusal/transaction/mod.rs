use std::fmt;

use super::Refusal;
use message::transaction_message;

mod message;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionRefusal {
    AlreadyOpen,
    NoOpenTransaction,
}

impl fmt::Display for TransactionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", transaction_message(self))
    }
}

impl From<TransactionRefusal> for Refusal {
    fn from(refusal: TransactionRefusal) -> Self {
        Refusal::Transaction(refusal)
    }
}
