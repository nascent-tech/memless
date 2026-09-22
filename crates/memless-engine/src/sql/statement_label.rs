use sqlparser::ast::Statement;

pub(crate) fn statement_label(statement: &Statement) -> &'static str {
    match statement {
        Statement::Insert(_) => "INSERT",
        Statement::Update { .. } => "UPDATE",
        Statement::Delete(_) => "DELETE",
        Statement::CreateTable(_) => "CREATE TABLE",
        Statement::Savepoint { .. } => "SAVEPOINT",
        Statement::ReleaseSavepoint { .. } => "RELEASE",
        _ => "this statement",
    }
}
