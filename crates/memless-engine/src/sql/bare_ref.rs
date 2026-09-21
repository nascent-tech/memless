use memless_domain::query::ColumnRef;
use sqlparser::ast::Ident;

pub(crate) fn bare_ref(ident: &Ident) -> ColumnRef {
    ColumnRef { table: None, column: ident.value.clone() }
}
