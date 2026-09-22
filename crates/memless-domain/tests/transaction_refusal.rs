use memless_domain::refusal::{Refusal, TransactionRefusal};

#[test]
fn already_open_reads_a_transaction_is_already_open() {
    assert_eq!(TransactionRefusal::AlreadyOpen.to_string(), "a transaction is already open");
}

#[test]
fn no_open_transaction_reads_no_open_transaction() {
    assert_eq!(TransactionRefusal::NoOpenTransaction.to_string(), "no open transaction");
}

#[test]
fn a_transaction_refusal_becomes_a_refusal() {
    let refusal: Refusal = TransactionRefusal::AlreadyOpen.into();
    assert!(matches!(refusal, Refusal::Transaction(TransactionRefusal::AlreadyOpen)));
}

#[test]
fn a_refusal_carries_the_transaction_message() {
    let refusal: Refusal = TransactionRefusal::NoOpenTransaction.into();
    assert_eq!(refusal.to_string(), "no open transaction");
}
