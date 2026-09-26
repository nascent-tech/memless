mod common;

use std::sync::atomic::{AtomicU64, Ordering};

use common::{read_path, temp_file};
use memless_domain::Scalar;
use memless_engine::{load, parse, query, read, Instance, Refusal};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique(prefix: &str) -> String {
    format!("{prefix}-{}.yaml", COUNTER.fetch_add(1, Ordering::Relaxed))
}

fn shop() -> Instance {
    let yaml = "\
users:
  - id: 1
    role: ADMIN
  - id: 2
    role: USER
wallets:
  - id: w1
    user_id: 1
    balance: 100
  - id: w2
    user_id: 2
    balance: 50
";
    let path = temp_file(&unique("query-shop"), yaml);
    load(read, &read_path(&path)).unwrap()
}

fn trap() -> Instance {
    let yaml = "\
users:
  - id: 5
  - id: \"5\"
wallets:
  - id: w1
    user_id: 5
";
    let path = temp_file(&unique("query-trap"), yaml);
    load(read, &read_path(&path)).unwrap()
}

#[test]
fn filters_under_the_one_comparison_rule() {
    let rows = query(parse, &shop(), "SELECT role FROM users WHERE id = 1").unwrap();
    assert_eq!(rows.rows.len(), 1);
    assert_eq!(rows.rows[0][0], Some(Scalar::Text("ADMIN".to_string())));
}

#[test]
fn a_cross_type_filter_keeps_nothing() {
    let rows = query(parse, &shop(), "SELECT role FROM users WHERE id = '1'").unwrap();
    assert_eq!(rows.rows.len(), 0);
}

#[test]
fn joins_and_qualifies_headers() {
    let rows = query(parse, &shop(), "SELECT * FROM wallets JOIN users ON wallets.user_id = users.id").unwrap();
    assert_eq!(rows.rows.len(), 2);
    assert!(rows.columns.contains(&"users.role".to_string()));
}

#[test]
fn counts_and_sums() {
    let rows = query(parse, &shop(), "SELECT COUNT(*), SUM(balance) FROM wallets").unwrap();
    assert_eq!(rows.rows[0], vec![Some(Scalar::Integer(2)), Some(Scalar::Integer(150))]);
}

#[test]
fn the_trap_joins_the_same_type_id_only() {
    let sql = "SELECT users.id FROM wallets JOIN users ON wallets.user_id = users.id";
    let rows = query(parse, &trap(), sql).unwrap();
    assert_eq!(rows.rows.len(), 1);
    assert_eq!(rows.rows[0][0], Some(Scalar::Integer(5)));
}

#[test]
fn an_unknown_table_is_a_query_refusal() {
    assert!(matches!(query(parse, &shop(), "SELECT * FROM ghosts"), Err(Refusal::Query(_))));
}

#[test]
fn an_out_of_subset_query_is_a_query_refusal() {
    assert!(matches!(query(parse, &shop(), "SELECT * FROM users LIMIT 1"), Err(Refusal::Query(_))));
}

fn first_cells(sql: &str) -> Vec<Option<Scalar>> {
    let rows = query(parse, &shop(), sql).unwrap();
    rows.rows.into_iter().map(|row| row[0].clone()).collect()
}

fn text(value: &str) -> Option<Scalar> {
    Some(Scalar::Text(value.to_string()))
}

#[test]
fn orders_rows_through_sql() {
    assert_eq!(first_cells("SELECT id FROM wallets ORDER BY balance"), [text("w2"), text("w1")]);
    assert_eq!(first_cells("SELECT id FROM wallets ORDER BY balance DESC"), [text("w1"), text("w2")]);
}

#[test]
fn orders_a_join_on_a_qualified_column() {
    let sql = "SELECT users.role FROM wallets JOIN users ON wallets.user_id = users.id ORDER BY users.role DESC";
    assert_eq!(first_cells(sql), [text("USER"), text("ADMIN")]);
}

#[test]
fn an_unknown_order_column_is_refused_by_name() {
    let refusal = query(parse, &shop(), "SELECT * FROM users ORDER BY ghost").unwrap_err();
    assert_eq!(refusal.to_string(), "no column \"ghost\" in table \"users\"");
}

#[test]
fn ordering_mixed_types_is_refused() {
    let refusal = query(parse, &trap(), "SELECT * FROM users ORDER BY id").unwrap_err();
    assert_eq!(refusal.to_string(), "cannot order by \"id\" of \"users\": row 5 and row \"5\" differ in type");
}
