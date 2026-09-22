use super::TransactionRefusal;
use super::TransactionRefusal::{AlreadyOpen, NoOpenTransaction};

pub(crate) fn transaction_message(refusal: &TransactionRefusal) -> &'static str {
    match refusal {
        AlreadyOpen => "a transaction is already open",
        NoOpenTransaction => "no open transaction",
    }
}
