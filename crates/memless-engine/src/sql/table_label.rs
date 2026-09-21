use sqlparser::ast::TableFactor;

pub(crate) fn table_label(relation: &TableFactor) -> &'static str {
    match relation {
        TableFactor::Table { .. } => "this table reference",
        _ => "this table expression",
    }
}
