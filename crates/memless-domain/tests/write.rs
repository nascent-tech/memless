mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::query::{ColumnRef, Compare, Filter, Op};
use memless_domain::refusal::{QueryRefusal, Refusal, StructureRefusal, WriteRefusal};
use memless_domain::{Base, Delete, Insert, Scalar, Update, Write};

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

fn where_id(value: i64) -> Option<Filter> {
    let column = ColumnRef { table: None, column: "id".to_string() };
    Some(Filter::Compare(Compare { column, op: Op::Eq, literal: Scalar::Integer(value) }))
}

fn update(table: &str, assignments: Vec<(String, Option<Scalar>)>, filter: Option<Filter>) -> Write {
    Write::Update(Update { table: table.to_string(), assignments, filter })
}

fn set(column: &str, value: Option<Scalar>) -> (String, Option<Scalar>) {
    (column.to_string(), value)
}

#[test]
fn insert_appends_a_row_to_an_existing_table() {
    let applied = shop().write(&insert("users", &["id", "role"], vec![int(3), text("GUEST")])).unwrap();
    let expected = Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", quoted("USER"))]),
                record(vec![field("id", plain("3")), field("role", quoted("GUEST"))]),
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
    .unwrap();
    assert_eq!(applied.affected, 1);
    assert!(applied.base == expected);
}

#[test]
fn insert_creates_a_new_table_at_the_end() {
    let applied = shop().write(&insert("logs", &["id", "line"], vec![int(1), text("hi")])).unwrap();
    assert_eq!(applied.affected, 1);
    let same = applied.base.write(&Write::Delete(Delete { table: "logs".to_string(), filter: None })).unwrap();
    assert_eq!(same.affected, 1);
}

#[test]
fn insert_may_carry_a_column_no_other_row_holds() {
    let applied = shop().write(&insert("users", &["id", "role", "note"], vec![int(3), text("G"), text("vip")])).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn insert_with_null_omits_the_column() {
    let applied = shop().write(&insert("users", &["id", "role"], vec![int(3), None])).unwrap();
    let expected = Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", quoted("USER"))]),
                record(vec![field("id", plain("3"))]),
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
    .unwrap();
    assert!(applied.base == expected);
}

#[test]
fn update_with_a_filter_touches_one_row() {
    let applied = shop().write(&update("users", vec![set("role", text("LEAD"))], where_id(1))).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn update_without_a_filter_touches_every_row() {
    let applied = shop().write(&update("users", vec![set("role", text("LEAD"))], None)).unwrap();
    assert_eq!(applied.affected, 2);
}

#[test]
fn update_setting_null_drops_the_column() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: None, column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let applied = shop().write(&update("wallets", vec![set("balance", None)], filter)).unwrap();
    let expected = Base::load(document(vec![
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
                record(vec![field("id", quoted("w1")), field("user_id", plain("1"))]),
                record(vec![field("id", quoted("w2")), field("user_id", plain("2")), field("balance", plain("50"))]),
            ]),
        ),
    ]))
    .unwrap();
    assert_eq!(applied.affected, 1);
    assert!(applied.base == expected);
}

#[test]
fn update_can_change_an_id() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: None, column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let applied = shop().write(&update("wallets", vec![set("id", text("w9"))], filter)).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn delete_with_a_filter_removes_one_row() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: None, column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let applied = shop().write(&Write::Delete(Delete { table: "wallets".to_string(), filter })).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn delete_without_a_filter_empties_the_table() {
    let applied = shop().write(&Write::Delete(Delete { table: "wallets".to_string(), filter: None })).unwrap();
    let expected = Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", quoted("USER"))]),
            ]),
        ),
        field("wallets", rows(vec![])),
    ]))
    .unwrap();
    assert_eq!(applied.affected, 2);
    assert!(applied.base == expected);
}

#[test]
fn an_update_that_changes_nothing_leaves_the_base_equal() {
    let start = shop();
    let applied = start.write(&update("users", vec![set("role", text("ADMIN"))], where_id(1))).unwrap();
    assert_eq!(applied.affected, 1);
    assert!(applied.base == start);
}

#[test]
fn refuses_a_column_count_mismatch() {
    let write = insert("users", &["id"], vec![int(3), text("X")]);
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Write(WriteRefusal::ColumnCountMismatch { .. })));
}

#[test]
fn refuses_a_repeated_column() {
    let write = insert("users", &["id", "id"], vec![int(3), int(4)]);
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Write(WriteRefusal::ColumnRepeated { .. })));
}

#[test]
fn refuses_an_insert_without_an_id() {
    let write = insert("users", &["role"], vec![text("X")]);
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::MissingId { .. })));
}

#[test]
fn refuses_an_id_set_to_a_decimal() {
    let write = update("users", vec![set("id", Some(Scalar::Decimal(1.5)))], where_id(1));
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::IdNotTextOrInteger { .. })));
}

#[test]
fn refuses_a_duplicate_id() {
    let write = insert("users", &["id", "role"], vec![int(1), text("X")]);
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::DuplicateId { .. })));
}

#[test]
fn refuses_a_delete_that_breaks_a_relation() {
    let refusal = shop().write(&Write::Delete(Delete { table: "users".to_string(), filter: None })).unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(StructureRefusal::BrokenRelation { .. })));
}

#[test]
fn refuses_an_insert_that_makes_a_table_appear_and_orphans_a_row() {
    let write = insert("accounts", &["id"], vec![int(7)]);
    let base = Base::load(document(vec![
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("account_id", plain("1"))])])),
        field("accounts", rows(vec![record(vec![field("id", plain("1"))])])),
    ]))
    .unwrap();
    let applied = base.write(&write).unwrap();
    assert_eq!(applied.affected, 1);
    let orphan = insert("accounts", &["id"], vec![int(7)]);
    let alone = Base::load(document(vec![
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("account_id", plain("9"))])])),
    ]))
    .unwrap();
    assert!(matches!(alone.write(&orphan).unwrap_err(), Refusal::Structure(StructureRefusal::BrokenRelation { .. })));
}

#[test]
fn refuses_an_update_on_an_unknown_table() {
    let refusal = shop().write(&update("ghosts", vec![set("x", int(1))], None)).unwrap_err();
    assert!(matches!(refusal, Refusal::Query(QueryRefusal::UnknownTable { .. })));
}

#[test]
fn a_repeated_column_beats_an_unknown_table_for_update() {
    let write = update("ghosts", vec![set("c", int(1)), set("c", int(2))], None);
    let refusal = shop().write(&write).unwrap_err();
    assert!(matches!(refusal, Refusal::Write(WriteRefusal::ColumnRepeated { .. })));
}

#[test]
fn refuses_a_where_qualifier_naming_another_table() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: Some("ghosts".to_string()), column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let refusal = shop().write(&Write::Delete(Delete { table: "wallets".to_string(), filter })).unwrap_err();
    assert!(matches!(refusal, Refusal::Query(QueryRefusal::UnknownTable { .. })));
}

#[test]
fn accepts_a_where_qualifier_naming_the_target_table() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: Some("wallets".to_string()), column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let applied = shop().write(&Write::Delete(Delete { table: "wallets".to_string(), filter })).unwrap();
    assert_eq!(applied.affected, 1);
}

#[test]
fn refuses_a_filter_on_an_unknown_column() {
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: None, column: "missing".to_string() },
        op: Op::Eq,
        literal: Scalar::Integer(1),
    }));
    let refusal = shop().write(&update("users", vec![set("role", text("X"))], filter)).unwrap_err();
    assert!(matches!(refusal, Refusal::Query(QueryRefusal::UnknownColumn { .. })));
}

#[test]
fn refuses_a_filter_on_an_emptied_table() {
    let emptied = shop().write(&Write::Delete(Delete { table: "wallets".to_string(), filter: None })).unwrap().base;
    let filter = Some(Filter::Compare(Compare {
        column: ColumnRef { table: None, column: "id".to_string() },
        op: Op::Eq,
        literal: Scalar::Text("w1".to_string()),
    }));
    let refusal = emptied.write(&Write::Delete(Delete { table: "wallets".to_string(), filter })).unwrap_err();
    assert!(matches!(refusal, Refusal::Query(QueryRefusal::UnknownColumn { .. })));
}
