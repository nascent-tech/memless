mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::refusal::{Refusal, StructureRefusal};
use memless_domain::{Base, Delete, Insert, Scalar, Write};

fn shop() -> Base {
    Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", quoted("USER"))]),
            ]),
        ),
        field(
            "wallets",
            rows(vec![
                record(vec![field("id", quoted("w1")), field("user_id", plain("1")), field("balance", plain("100"))]),
                record(vec![field("id", quoted("w2")), field("user_id", plain("2")), field("balance", plain("50"))]),
            ]),
        ),
    ]))
    .unwrap()
}

fn text(value: &str) -> Option<Scalar> {
    Some(Scalar::Text(value.to_string()))
}

fn int(value: i64) -> Option<Scalar> {
    Some(Scalar::Integer(value))
}

fn names(columns: &[&str]) -> Vec<String> {
    columns.iter().map(|column| column.to_string()).collect()
}

fn insert(table: &str, columns: &[&str], values: Vec<Option<Scalar>>) -> Write {
    Write::Insert(Insert { table: table.to_string(), columns: names(columns), values })
}

fn drop_users() -> Write {
    Write::Delete(Delete { table: "users".to_string(), filter: None })
}

#[test]
fn apply_write_accepts_a_duplicate_id_without_verifying() {
    let applied = shop().apply_write(&insert("users", &["id", "role"], vec![int(1), text("X")])).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn write_refuses_the_same_duplicate_id() {
    let refusal = shop().write(&insert("users", &["id", "role"], vec![int(1), text("X")])).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::DuplicateId { .. })));
}

#[test]
fn apply_write_accepts_a_broken_relation_without_verifying() {
    let applied = shop().apply_write(&drop_users()).unwrap();
    assert_eq!(applied.affected, 2);
}

#[test]
fn write_refuses_the_same_broken_relation() {
    let refusal = shop().write(&drop_users()).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::BrokenRelation { .. })));
}

#[test]
fn apply_write_still_refuses_an_insert_without_an_id() {
    let refusal = shop().apply_write(&insert("users", &["role"], vec![text("X")])).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::MissingId { .. })));
}

#[test]
fn verify_state_flags_a_duplicate_id() {
    let working = shop().apply_write(&insert("users", &["id", "role"], vec![int(1), text("X")])).unwrap().base;
    assert!(matches!(working.verify_state().unwrap_err(), Refusal::Structure(StructureRefusal::DuplicateId { .. })));
}

#[test]
fn verify_state_flags_a_broken_relation() {
    let working = shop().apply_write(&drop_users()).unwrap().base;
    assert!(matches!(working.verify_state().unwrap_err(), Refusal::Structure(StructureRefusal::BrokenRelation { .. })));
}

#[test]
fn verify_state_accepts_a_healthy_state() {
    assert!(shop().verify_state().is_ok());
}

#[test]
fn write_and_apply_write_agree_on_a_healthy_write() {
    let statement = insert("users", &["id", "role"], vec![int(3), text("GUEST")]);
    let by_write = shop().write(&statement).unwrap();
    let by_apply = shop().apply_write(&statement).unwrap();
    assert_eq!(by_write.affected, by_apply.affected);
    assert!(by_write.base == by_apply.base);
}
