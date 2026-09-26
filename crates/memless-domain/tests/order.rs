mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::document::RawNode;
use memless_domain::query::{
    Aggregate, ColumnRef, Compare, Direction, Filter, Items, Join, Op, OrderKey, Select,
};
use memless_domain::refusal::QueryRefusal;
use memless_domain::{Base, Scalar};

fn wallet(id: &str, owner: &str, balance: Option<RawNode>) -> RawNode {
    let mut fields = vec![field("id", quoted(id)), field("owner", quoted(owner))];
    if let Some(value) = balance {
        fields.push(field("balance", value));
    }
    record(fields)
}

fn wallets() -> Base {
    Base::load(document(vec![field(
        "wallets",
        rows(vec![
            wallet("w1", "bob", Some(plain("100"))),
            wallet("w2", "ann", Some(plain("50"))),
            wallet("w3", "bob", None),
            wallet("w4", "ann", Some(plain("75"))),
            wallet("w5", "bob", Some(plain("50"))),
        ]),
    )]))
    .unwrap()
}

fn mixed() -> Base {
    Base::load(document(vec![field(
        "wallets",
        rows(vec![
            record(vec![field("id", plain("1")), field("balance", plain("10"))]),
            record(vec![field("id", plain("2")), field("balance", plain("20"))]),
            record(vec![field("id", plain("3")), field("balance", quoted("30"))]),
            record(vec![field("id", plain("4")), field("balance", plain("2.5"))]),
        ]),
    )]))
    .unwrap()
}

fn joined() -> Base {
    Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", plain("7"))]),
                record(vec![field("id", plain("3")), field("role", quoted("USER"))]),
            ]),
        ),
        field(
            "wallets",
            rows(vec![
                record(vec![field("id", quoted("w1")), field("user_id", plain("3"))]),
                record(vec![field("id", quoted("w2")), field("user_id", plain("1"))]),
                record(vec![field("id", quoted("w3")), field("user_id", plain("2"))]),
            ]),
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

fn asc(column: ColumnRef) -> OrderKey {
    OrderKey { column, direction: Direction::Ascending }
}

fn desc(column: ColumnRef) -> OrderKey {
    OrderKey { column, direction: Direction::Descending }
}

fn ordered(from: &str, order: Vec<OrderKey>) -> Select {
    Select { items: Items::Columns(vec![col("id")]), from: from.to_string(), join: None, filter: None, order }
}

fn ids(base: &Base, query: &Select) -> Vec<String> {
    let rows = base.select(query).unwrap();
    rows.rows.iter().map(|row| row[0].as_ref().map(ToString::to_string).unwrap_or_default()).collect()
}

fn text_ids(list: &[&str]) -> Vec<String> {
    list.iter().map(|id| format!("{id:?}")).collect()
}

fn wallets_join_users(order: Vec<OrderKey>) -> Select {
    Select {
        items: Items::Columns(vec![qual("wallets", "id")]),
        from: "wallets".to_string(),
        join: Some(Join { table: "users".to_string(), left: qual("wallets", "user_id"), right: qual("users", "id") }),
        filter: None,
        order,
    }
}

#[test]
fn orders_ascending_by_default_direction_with_absent_last() {
    let query = ordered("wallets", vec![asc(col("balance"))]);
    assert_eq!(ids(&wallets(), &query), text_ids(&["w2", "w5", "w4", "w1", "w3"]));
}

#[test]
fn orders_descending_and_still_places_absent_last() {
    let query = ordered("wallets", vec![desc(col("balance"))]);
    assert_eq!(ids(&wallets(), &query), text_ids(&["w1", "w4", "w2", "w5", "w3"]));
}

#[test]
fn equal_keys_keep_file_order() {
    let query = ordered("wallets", vec![asc(col("owner"))]);
    assert_eq!(ids(&wallets(), &query), text_ids(&["w2", "w4", "w1", "w3", "w5"]));
}

#[test]
fn a_second_key_breaks_ties_of_the_first() {
    let query = ordered("wallets", vec![asc(col("owner")), desc(col("balance"))]);
    assert_eq!(ids(&wallets(), &query), text_ids(&["w4", "w2", "w1", "w5", "w3"]));
}

#[test]
fn orders_on_a_column_left_out_of_the_projection() {
    let rows = wallets().select(&ordered("wallets", vec![desc(col("owner"))])).unwrap();
    assert_eq!(rows.columns, vec!["id"]);
    assert_eq!(rows.rows.len(), 5);
}

#[test]
fn orders_after_the_filter() {
    let mut query = ordered("wallets", vec![desc(col("balance"))]);
    let owner = Compare { column: col("owner"), op: Op::Eq, literal: Scalar::Text("bob".to_string()) };
    query.filter = Some(Filter::Compare(owner));
    assert_eq!(ids(&wallets(), &query), text_ids(&["w1", "w5", "w3"]));
}

#[test]
fn refuses_mixed_types_naming_the_first_two_rows_of_different_type() {
    let error = mixed().select(&ordered("wallets", vec![asc(col("balance"))])).unwrap_err();
    assert_eq!(error.to_string(), "cannot order by \"balance\" of \"wallets\": row 1 and row 3 differ in type");
}

#[test]
fn refuses_an_integer_next_to_a_decimal() {
    let mut query = ordered("wallets", vec![desc(col("balance"))]);
    query.filter = Some(Filter::Compare(Compare { column: col("id"), op: Op::Ne, literal: Scalar::Integer(3) }));
    let error = mixed().select(&query).unwrap_err();
    assert_eq!(error.to_string(), "cannot order by \"balance\" of \"wallets\": row 1 and row 4 differ in type");
}

#[test]
fn rows_filtered_out_do_not_count_toward_mixed_types() {
    let mut query = ordered("wallets", vec![asc(col("balance"))]);
    query.filter = Some(Filter::Compare(Compare { column: col("id"), op: Op::Lt, literal: Scalar::Integer(3) }));
    assert_eq!(ids(&mixed(), &query), vec!["1", "2"]);
}

#[test]
fn orders_a_join_on_a_qualified_column() {
    let base = joined();
    let mut query = wallets_join_users(vec![desc(qual("users", "id"))]);
    assert_eq!(ids(&base, &query), text_ids(&["w1", "w3", "w2"]));
    query.order = vec![asc(qual("wallets", "user_id"))];
    assert_eq!(ids(&base, &query), text_ids(&["w2", "w3", "w1"]));
}

#[test]
fn mixed_types_in_a_joined_column_name_rows_in_that_table_file_order() {
    let error = joined().select(&wallets_join_users(vec![asc(qual("users", "role"))])).unwrap_err();
    assert_eq!(error.to_string(), "cannot order by \"role\" of \"users\": row 1 and row 2 differ in type");
}

#[test]
fn refuses_an_unknown_order_column_like_an_unknown_filter_column() {
    let error = wallets().select(&ordered("wallets", vec![asc(col("missing"))])).unwrap_err();
    assert_eq!(error, QueryRefusal::UnknownColumn { table: "wallets".to_string(), column: "missing".to_string() });
}

fn one_column(values: Vec<RawNode>) -> Base {
    let records = values.into_iter().enumerate().map(|(index, value)| {
        record(vec![field("id", plain(&(index + 1).to_string())), field("v", value)])
    });
    Base::load(document(vec![field("t", rows(records.collect()))])).unwrap()
}

#[test]
fn every_scalar_type_orders_within_itself() {
    let query = ordered("t", vec![asc(col("v"))]);
    assert_eq!(ids(&one_column(vec![plain("30"), plain("-2"), plain("7")]), &query), ["2", "3", "1"]);
    assert_eq!(ids(&one_column(vec![plain("2.5"), plain("-0.5"), plain("1.25")]), &query), ["2", "3", "1"]);
    assert_eq!(ids(&one_column(vec![quoted("pear"), quoted("Apple"), quoted("apple")]), &query), ["2", "3", "1"]);
    assert_eq!(ids(&one_column(vec![plain("true"), plain("false")]), &query), ["2", "1"]);
}

fn select_t(items: Items, order: Vec<OrderKey>) -> Select {
    Select { items, from: "t".to_string(), join: None, filter: None, order }
}

#[test]
fn a_duplicate_output_column_is_refused_before_mixed_types() {
    let base = one_column(vec![plain("5"), quoted("5")]);
    let query = select_t(Items::Columns(vec![col("v"), col("v")]), vec![asc(col("v"))]);
    let refusal = QueryRefusal::OutsideSubset { construct: "duplicate output column".to_string() };
    assert_eq!(base.select(&query).unwrap_err(), refusal);
}

#[test]
fn refuses_order_by_with_an_aggregate() {
    let base = one_column(vec![plain("5"), plain("6")]);
    let query = select_t(Items::Aggregates(vec![Aggregate::Sum(col("v"))]), vec![asc(col("v"))]);
    let refusal = QueryRefusal::OutsideSubset { construct: "ORDER BY with an aggregate".to_string() };
    assert_eq!(base.select(&query).unwrap_err(), refusal);
}

#[test]
fn refuses_an_unknown_order_qualifier() {
    let error = joined().select(&wallets_join_users(vec![asc(qual("others", "id"))])).unwrap_err();
    assert_eq!(error, QueryRefusal::UnknownTable { table: "others".to_string() });
}
