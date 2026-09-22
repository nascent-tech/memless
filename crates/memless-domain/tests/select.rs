mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::query::{Aggregate, ColumnRef, Compare, Filter, Items, Join, Op, Select};
use memless_domain::refusal::QueryRefusal;
use memless_domain::{Base, Scalar};

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
                record(vec![
                    field("id", quoted("w1")),
                    field("user_id", plain("1")),
                    field("balance", plain("100")),
                ]),
                record(vec![
                    field("id", quoted("w2")),
                    field("user_id", plain("2")),
                    field("balance", plain("50")),
                ]),
            ]),
        ),
    ]))
    .unwrap()
}

fn trap() -> Base {
    Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("5"))]),
                record(vec![field("id", quoted("5"))]),
            ]),
        ),
        field(
            "wallets",
            rows(vec![record(vec![field("id", quoted("w1")), field("user_id", plain("5"))])]),
        ),
    ]))
    .unwrap()
}

fn col(name: &str) -> ColumnRef {
    ColumnRef { table: None, column: name.to_string() }
}

fn qual(table: &str, name: &str) -> ColumnRef {
    ColumnRef { table: Some(table.to_string()), column: name.to_string() }
}

fn all(from: &str) -> Select {
    Select { items: Items::All, from: from.to_string(), join: None, filter: None }
}

fn filtered(from: &str, filter: Filter) -> Select {
    Select { items: Items::All, from: from.to_string(), join: None, filter: Some(filter) }
}

fn compare(column: ColumnRef, op: Op, literal: Scalar) -> Filter {
    Filter::Compare(Compare { column, op, literal })
}

fn wallets_join_users() -> Join {
    Join { table: "users".to_string(), left: qual("wallets", "user_id"), right: qual("users", "id") }
}

#[test]
fn selects_all_columns_in_file_order() {
    let rows = shop().select(&all("users")).unwrap();
    assert_eq!(rows.columns, vec!["id", "role"]);
    assert_eq!(rows.rows.len(), 2);
    assert_eq!(rows.rows[0], vec![Some(Scalar::Integer(1)), Some(Scalar::Text("ADMIN".to_string()))]);
}

#[test]
fn filter_keeps_same_type_match() {
    let query = filtered("users", compare(col("role"), Op::Eq, Scalar::Text("ADMIN".to_string())));
    assert_eq!(shop().select(&query).unwrap().rows.len(), 1);
}

#[test]
fn filter_excludes_cross_type_comparison() {
    let query = filtered("users", compare(col("id"), Op::Eq, Scalar::Text("1".to_string())));
    assert_eq!(shop().select(&query).unwrap().rows.len(), 0);
}

#[test]
fn filter_orders_within_a_type() {
    let query = filtered("wallets", compare(col("balance"), Op::Lt, Scalar::Integer(60)));
    assert_eq!(shop().select(&query).unwrap().rows.len(), 1);
}

#[test]
fn is_not_null_drops_absent_rows() {
    let base = Base::load(document(vec![field(
        "users",
        rows(vec![
            record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
            record(vec![field("id", plain("2"))]),
        ]),
    )]))
    .unwrap();
    let query = filtered("users", Filter::IsNotNull(col("role")));
    assert_eq!(base.select(&query).unwrap().rows.len(), 1);
}

#[test]
fn joins_by_guessed_relation() {
    let query = Select {
        items: Items::All,
        from: "wallets".to_string(),
        join: Some(wallets_join_users()),
        filter: None,
    };
    let rows = shop().select(&query).unwrap();
    assert_eq!(rows.rows.len(), 2);
    assert!(rows.columns.contains(&"users.role".to_string()));
}

#[test]
fn join_matches_the_same_type_id_only() {
    let query = Select {
        items: Items::Columns(vec![qual("users", "id")]),
        from: "wallets".to_string(),
        join: Some(wallets_join_users()),
        filter: None,
    };
    let rows = trap().select(&query).unwrap();
    assert_eq!(rows.rows.len(), 1);
    assert_eq!(rows.rows[0], vec![Some(Scalar::Integer(5))]);
}

#[test]
fn counts_and_sums_over_retained_rows() {
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::CountStar, Aggregate::Sum(col("balance"))]),
        from: "wallets".to_string(),
        join: None,
        filter: None,
    };
    let rows = shop().select(&query).unwrap();
    assert_eq!(rows.rows[0], vec![Some(Scalar::Integer(2)), Some(Scalar::Integer(150))]);
}

#[test]
fn sum_over_no_present_value_is_absent() {
    let base = Base::load(document(vec![field(
        "t",
        rows(vec![
            record(vec![field("id", plain("1")), field("v", plain("10"))]),
            record(vec![field("id", plain("2"))]),
        ]),
    )]))
    .unwrap();
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::Sum(col("v"))]),
        from: "t".to_string(),
        join: None,
        filter: Some(compare(col("id"), Op::Eq, Scalar::Integer(2))),
    };
    assert_eq!(base.select(&query).unwrap().rows[0], vec![None]);
}

#[test]
fn and_or_nest_in_tree_order() {
    let inner = Filter::Or(
        Box::new(compare(col("role"), Op::Eq, Scalar::Text("ADMIN".to_string()))),
        Box::new(compare(col("role"), Op::Eq, Scalar::Text("USER".to_string()))),
    );
    let query = filtered("users", Filter::And(Box::new(inner), Box::new(compare(col("id"), Op::Lt, Scalar::Integer(2)))));
    assert_eq!(shop().select(&query).unwrap().rows.len(), 1);
}

#[test]
fn is_null_keeps_absent_rows() {
    let base = Base::load(document(vec![field(
        "users",
        rows(vec![
            record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
            record(vec![field("id", plain("2"))]),
        ]),
    )]))
    .unwrap();
    let query = filtered("users", Filter::IsNull(col("role")));
    let rows = base.select(&query).unwrap();
    assert_eq!(rows.rows.len(), 1);
    assert_eq!(rows.rows[0][0], Some(Scalar::Integer(2)));
}

#[test]
fn join_from_the_id_side_keeps_joined_file_order() {
    let base = Base::load(document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))])])),
        field(
            "wallets",
            rows(vec![
                record(vec![field("id", quoted("wA")), field("user_id", plain("1"))]),
                record(vec![field("id", quoted("wB")), field("user_id", plain("1"))]),
            ]),
        ),
    ]))
    .unwrap();
    let query = Select {
        items: Items::Columns(vec![qual("wallets", "id")]),
        from: "users".to_string(),
        join: Some(Join { table: "wallets".to_string(), left: qual("wallets", "user_id"), right: qual("users", "id") }),
        filter: None,
    };
    let rows = base.select(&query).unwrap();
    assert_eq!(rows.rows.len(), 2);
    assert_eq!(rows.rows[0][0], Some(Scalar::Text("wA".to_string())));
    assert_eq!(rows.rows[1][0], Some(Scalar::Text("wB".to_string())));
}

#[test]
fn join_drops_rows_with_absent_relation() {
    let base = Base::load(document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))])])),
        field(
            "wallets",
            rows(vec![
                record(vec![field("id", quoted("w1")), field("user_id", plain("1"))]),
                record(vec![field("id", quoted("w2"))]),
            ]),
        ),
    ]))
    .unwrap();
    let query = Select {
        items: Items::All,
        from: "wallets".to_string(),
        join: Some(wallets_join_users()),
        filter: None,
    };
    assert_eq!(base.select(&query).unwrap().rows.len(), 1);
}

#[test]
fn sum_of_mixed_number_types_names_the_second_type_row() {
    let base = Base::load(document(vec![field(
        "t",
        rows(vec![
            record(vec![field("id", plain("1")), field("v", plain("10"))]),
            record(vec![field("id", plain("2")), field("v", plain("1.5"))]),
        ]),
    )]))
    .unwrap();
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::Sum(col("v"))]),
        from: "t".to_string(),
        join: None,
        filter: None,
    };
    let error = base.select(&query).unwrap_err();
    assert_eq!(error.to_string(), "cannot sum \"v\" of \"t\" at row 2");
}

#[test]
fn count_column_ignores_absent_values() {
    let base = Base::load(document(vec![field(
        "users",
        rows(vec![
            record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
            record(vec![field("id", plain("2"))]),
        ]),
    )]))
    .unwrap();
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::Count(col("role"))]),
        from: "users".to_string(),
        join: None,
        filter: None,
    };
    assert_eq!(base.select(&query).unwrap().rows[0], vec![Some(Scalar::Integer(1))]);
}

#[test]
fn refuses_unknown_table() {
    assert!(matches!(shop().select(&all("ghosts")), Err(QueryRefusal::UnknownTable { .. })));
}

#[test]
fn refuses_unknown_qualifier() {
    let query = Select {
        items: Items::Columns(vec![qual("ghosts", "id")]),
        from: "users".to_string(),
        join: None,
        filter: None,
    };
    assert!(matches!(shop().select(&query), Err(QueryRefusal::UnknownTable { .. })));
}

#[test]
fn refuses_unknown_column() {
    let query = Select {
        items: Items::Columns(vec![col("ghost")]),
        from: "users".to_string(),
        join: None,
        filter: None,
    };
    assert!(matches!(shop().select(&query), Err(QueryRefusal::UnknownColumn { .. })));
}

#[test]
fn refuses_join_that_is_not_a_relation() {
    let query = Select {
        items: Items::All,
        from: "wallets".to_string(),
        join: Some(Join {
            table: "users".to_string(),
            left: qual("wallets", "balance"),
            right: qual("users", "id"),
        }),
        filter: None,
    };
    assert!(matches!(shop().select(&query), Err(QueryRefusal::JoinNotRelation { .. })));
}

#[test]
fn refuses_sum_of_text() {
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::Sum(col("role"))]),
        from: "users".to_string(),
        join: None,
        filter: None,
    };
    let error = shop().select(&query).unwrap_err();
    assert_eq!(error.to_string(), "cannot sum \"role\" of \"users\" at row 1");
}

#[test]
fn refuses_sum_overflow() {
    let base = Base::load(document(vec![field(
        "nums",
        rows(vec![
            record(vec![field("id", plain("1")), field("v", plain("9223372036854775807"))]),
            record(vec![field("id", plain("2")), field("v", plain("1"))]),
        ]),
    )]))
    .unwrap();
    let query = Select {
        items: Items::Aggregates(vec![Aggregate::Sum(col("v"))]),
        from: "nums".to_string(),
        join: None,
        filter: None,
    };
    assert!(matches!(base.select(&query), Err(QueryRefusal::SumOverflow { .. })));
}
