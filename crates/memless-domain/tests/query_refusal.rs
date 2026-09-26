use memless_domain::refusal::{QueryRefusal, RowLabel};
use memless_domain::{Id, Refusal};

#[test]
fn invalid_sql_names_the_detail() {
    let refusal = QueryRefusal::InvalidSql { detail: "unexpected end of input".to_string() };
    assert_eq!(refusal.to_string(), "invalid SQL: unexpected end of input");
}

#[test]
fn outside_subset_names_the_construct() {
    let refusal = QueryRefusal::OutsideSubset { construct: "GROUP BY".to_string() };
    assert_eq!(refusal.to_string(), "GROUP BY is outside the supported SQL subset");
}

#[test]
fn unknown_table_quotes_the_name() {
    let refusal = QueryRefusal::UnknownTable { table: "ghosts".to_string() };
    assert_eq!(refusal.to_string(), "no table \"ghosts\"");
}

#[test]
fn unknown_column_quotes_both_names() {
    let refusal = QueryRefusal::UnknownColumn { table: "users".to_string(), column: "ghost".to_string() };
    assert_eq!(refusal.to_string(), "no column \"ghost\" in table \"users\"");
}

#[test]
fn join_not_relation_names_the_target_id() {
    let refusal = QueryRefusal::JoinNotRelation {
        table: "wallets".to_string(),
        column: "balance".to_string(),
        target: "users".to_string(),
    };
    assert_eq!(
        refusal.to_string(),
        "\"balance\" of \"wallets\" is not a guessed relation to the id of \"users\""
    );
}

#[test]
fn sum_not_number_names_the_row() {
    let refusal = QueryRefusal::SumNotNumber {
        table: "users".to_string(),
        column: "role".to_string(),
        row: RowLabel::Id(Id::Integer(2)),
    };
    assert_eq!(refusal.to_string(), "cannot sum \"role\" of \"users\" at row 2");
}

#[test]
fn sum_overflow_names_the_column() {
    let refusal = QueryRefusal::SumOverflow { table: "nums".to_string(), column: "v".to_string() };
    assert_eq!(refusal.to_string(), "sum of \"v\" in \"nums\" overflows");
}

#[test]
fn order_mixed_types_names_both_rows() {
    let refusal = QueryRefusal::OrderMixedTypes {
        table: "wallets".to_string(),
        column: "balance".to_string(),
        first: RowLabel::Id(Id::Integer(1)),
        second: RowLabel::Id(Id::Text("w3".to_string())),
    };
    assert_eq!(
        refusal.to_string(),
        "cannot order by \"balance\" of \"wallets\": row 1 and row \"w3\" differ in type"
    );
}

#[test]
fn query_refusal_lifts_into_refusal() {
    let refusal: Refusal = QueryRefusal::UnknownTable { table: "ghosts".to_string() }.into();
    assert_eq!(refusal.to_string(), "no table \"ghosts\"");
}
