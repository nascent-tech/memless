use memless_domain::query::{Compare, Filter, Items, Join, Op, Select, Statement, Write};
use memless_domain::{QueryRefusal, Scalar};
use memless_engine::parse;

fn select(sql: &str) -> Select {
    match parse(sql).unwrap() {
        Statement::Select(select) => select,
        other => panic!("expected a select for {sql:?}, got {other:?}"),
    }
}

fn write_of(sql: &str) -> Write {
    match parse(sql).unwrap() {
        Statement::Write(write) => write,
        other => panic!("expected a write for {sql:?}, got {other:?}"),
    }
}

#[test]
fn lowers_an_insert_of_one_row() {
    let Write::Insert(insert) = write_of("INSERT INTO users (id, role) VALUES (3, 'GUEST')") else {
        panic!("expected an insert");
    };
    assert_eq!(insert.table, "users");
    assert_eq!(insert.columns, ["id", "role"]);
    assert_eq!(insert.values, [Some(Scalar::Integer(3)), Some(Scalar::Text("GUEST".to_string()))]);
}

#[test]
fn lowers_an_insert_with_a_null_value() {
    let Write::Insert(insert) = write_of("INSERT INTO users (id, role) VALUES (3, NULL)") else {
        panic!("expected an insert");
    };
    assert_eq!(insert.values, [Some(Scalar::Integer(3)), None]);
}

#[test]
fn lowers_an_update_with_a_filter() {
    let Write::Update(update) = write_of("UPDATE users SET role = 'LEAD' WHERE id = 1") else {
        panic!("expected an update");
    };
    assert_eq!(update.table, "users");
    assert_eq!(update.assignments, [("role".to_string(), Some(Scalar::Text("LEAD".to_string())))]);
    assert!(update.filter.is_some());
}

#[test]
fn lowers_an_update_setting_null() {
    let Write::Update(update) = write_of("UPDATE users SET role = NULL") else {
        panic!("expected an update");
    };
    assert_eq!(update.assignments, [("role".to_string(), None)]);
    assert!(update.filter.is_none());
}

#[test]
fn lowers_a_delete_without_a_filter() {
    let Write::Delete(delete) = write_of("DELETE FROM users") else {
        panic!("expected a delete");
    };
    assert_eq!(delete.table, "users");
    assert!(delete.filter.is_none());
}

#[test]
fn refuses_an_insert_without_a_column_list() {
    assert_eq!(construct("INSERT INTO users VALUES (1)"), "INSERT without a column list");
}

#[test]
fn refuses_a_multi_row_insert() {
    assert_eq!(construct("INSERT INTO users (id) VALUES (1), (2)"), "multi-row VALUES");
}

#[test]
fn refuses_an_insert_from_a_select() {
    assert_eq!(construct("INSERT INTO users (id) SELECT id FROM others"), "INSERT ... SELECT");
}

#[test]
fn refuses_arithmetic_in_a_value() {
    assert_eq!(construct("INSERT INTO users (id) VALUES (1 + 1)"), "this literal");
}

#[test]
fn refuses_a_tail_clause_on_an_insert() {
    assert_eq!(construct("INSERT INTO users (id) VALUES (1) LIMIT 1"), "LIMIT");
    assert_eq!(construct("INSERT INTO users (id) VALUES (1) ORDER BY id"), "ORDER BY");
}

#[test]
fn refuses_a_conflict_modifier_on_an_insert() {
    assert_eq!(construct("INSERT OR REPLACE INTO users (id) VALUES (1)"), "INSERT with a conflict modifier");
    assert_eq!(construct("INSERT OR IGNORE INTO users (id) VALUES (1)"), "INSERT with a conflict modifier");
}

#[test]
fn refuses_a_conflict_modifier_on_an_update() {
    assert_eq!(construct("UPDATE OR REPLACE users SET role = 'x'"), "UPDATE with a conflict modifier");
}

#[test]
fn refuses_a_qualified_assignment_target_as_a_column() {
    assert_eq!(construct("UPDATE users SET users.role = 'x'"), "qualified column");
}

fn construct(sql: &str) -> String {
    match parse(sql) {
        Err(QueryRefusal::OutsideSubset { construct }) => construct,
        other => panic!("expected OutsideSubset for {sql:?}, got {other:?}"),
    }
}

fn invalid(sql: &str) -> String {
    match parse(sql) {
        Err(QueryRefusal::InvalidSql { detail }) => detail,
        other => panic!("expected InvalidSql for {sql:?}, got {other:?}"),
    }
}

#[test]
fn lowers_a_star_select() {
    let select = select("SELECT * FROM users");
    assert_eq!(select.from, "users");
    assert!(matches!(select.items, Items::All));
    assert!(select.join.is_none());
}

#[test]
fn lowers_a_guessed_relation_join() {
    let select = select("SELECT * FROM wallets JOIN users ON wallets.user_id = users.id");
    let Some(Join { table, left, right }) = select.join else {
        panic!("expected a join");
    };
    assert_eq!(table, "users");
    assert_eq!(left.column, "user_id");
    assert_eq!(right.column, "id");
}

#[test]
fn keeps_a_quoted_name_verbatim_and_case_sensitive() {
    let select = select("SELECT \"first name\" FROM users");
    match select.items {
        Items::Columns(columns) => assert_eq!(columns[0].column, "first name"),
        other => panic!("expected columns, got {other:?}"),
    }
}

#[test]
fn refuses_broken_sql_as_invalid() {
    assert!(!invalid("SELECT * FRM users").is_empty());
}

#[test]
fn refuses_empty_text_as_invalid() {
    assert!(matches!(parse("   "), Err(QueryRefusal::InvalidSql { .. })));
}

#[test]
fn refuses_each_out_of_subset_construct() {
    assert_eq!(construct("SELECT * FROM users GROUP BY id"), "GROUP BY");
    assert_eq!(construct("SELECT * FROM users HAVING id > 0"), "HAVING");
    assert_eq!(construct("SELECT * FROM users LIMIT 1"), "LIMIT");
    assert_eq!(construct("SELECT * FROM users OFFSET 1 ROWS"), "OFFSET");
    assert_eq!(construct("SELECT * FROM users ORDER BY id"), "ORDER BY");
    assert_eq!(construct("SELECT DISTINCT id FROM users"), "DISTINCT");
    assert_eq!(construct("SELECT * FROM a LEFT JOIN b ON a.b_id = b.id"), "LEFT JOIN");
    assert_eq!(construct("SELECT * FROM a RIGHT JOIN b ON a.b_id = b.id"), "RIGHT JOIN");
    assert_eq!(construct("SELECT * FROM a CROSS JOIN b"), "CROSS JOIN");
    assert_eq!(construct("SELECT * FROM a JOIN b ON a.b_id = b.id JOIN c ON c.a_id = a.id"), "multiple joins");
    assert_eq!(construct("SELECT * FROM users JOIN users ON users.boss_id = users.id"), "self join");
    assert_eq!(construct("SELECT * FROM a, b"), "comma join");
    assert_eq!(construct("SELECT * FROM users u"), "this table reference");
    assert_eq!(construct("SELECT id AS x FROM users"), "this projection item");
    assert_eq!(construct("CREATE TABLE users (id INT)"), "CREATE TABLE");
    assert_eq!(construct("SELECT id, COUNT(*) FROM users"), "a column next to an aggregate");
    assert_eq!(construct("SELECT * FROM users; SELECT * FROM users"), "multiple statements");
    assert_eq!(construct("SELECT id FROM wallets JOIN users ON wallets.user_id = users.id"), "unqualified column in a join");
    assert_eq!(construct("SELECT AVG(balance) FROM wallets"), "this aggregate");
    assert_eq!(construct("SELECT COUNT(DISTINCT id) FROM users"), "DISTINCT in an aggregate");
    assert_eq!(construct("SELECT COUNT(*) OVER () FROM users"), "window function");
    assert_eq!(construct("WITH x AS (SELECT 1) SELECT COUNT(*) FROM users"), "WITH");
    assert_eq!(construct("SELECT * FROM users WHERE role NOT IN ('A')"), "this condition");
    assert_eq!(construct("SELECT * FROM users WHERE balance = 1 + 1"), "this literal");
    assert_eq!(construct("SELECT 1 FROM users"), "this column expression");
    assert_eq!(construct("SELECT * FROM users WHERE role = NULL"), "NULL literal");
}

#[test]
fn refuses_backticks_as_invalid_sql() {
    assert!(matches!(parse("SELECT `role` FROM users"), Err(QueryRefusal::InvalidSql { .. })));
}

#[test]
fn refuses_brackets() {
    assert!(parse("SELECT [role] FROM users").is_err());
}

#[test]
fn keeps_parentheses_in_the_where_tree() {
    let select = select("SELECT * FROM users WHERE (id = 1 OR id = 2) AND id = 3");
    assert!(matches!(select.filter, Some(Filter::And(_, _))));
}

#[test]
fn builds_the_and_tree_left_to_right() {
    let select = select("SELECT * FROM users WHERE id = 1 AND role = 'A'");
    let Some(Filter::And(left, right)) = select.filter else {
        panic!("expected an AND");
    };
    assert_eq!(*left, Filter::Compare(Compare { column: col_ref("id"), op: Op::Eq, literal: Scalar::Integer(1) }));
    assert_eq!(*right, Filter::Compare(Compare { column: col_ref("role"), op: Op::Eq, literal: Scalar::Text("A".to_string()) }));
}

#[test]
fn lowers_literals_of_each_type() {
    assert_eq!(literal_in("WHERE balance = -1"), Scalar::Integer(-1));
    assert_eq!(literal_in("WHERE balance = 1.5"), Scalar::Decimal(1.5));
    assert_eq!(literal_in("WHERE active = true"), Scalar::Boolean(true));
    assert_eq!(literal_in("WHERE balance = -9223372036854775808"), Scalar::Integer(i64::MIN));
    assert!(matches!(parse("SELECT * FROM users WHERE balance = 9223372036854775808"), Err(QueryRefusal::InvalidSql { .. })));
}

fn col_ref(name: &str) -> memless_domain::query::ColumnRef {
    memless_domain::query::ColumnRef { table: None, column: name.to_string() }
}

fn literal_in(clause: &str) -> Scalar {
    let select = select(&format!("SELECT * FROM users {clause}"));
    match select.filter {
        Some(Filter::Compare(compare)) => compare.literal,
        other => panic!("expected a comparison, got {other:?}"),
    }
}
