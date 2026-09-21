use super::bare_ref::bare_ref;
use super::compound_ref::compound_ref;
use super::outside::outside;
use memless_domain::query::ColumnRef;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn column_ref_of(expr: &Expr, require_qualified: bool) -> Result<ColumnRef, QueryRefusal> {
    match expr {
        Expr::CompoundIdentifier(parts) => compound_ref(parts),
        Expr::Identifier(_) if require_qualified => Err(outside("unqualified column in a join")),
        Expr::Identifier(ident) => Ok(bare_ref(ident)),
        _ => Err(outside("this column expression")),
    }
}
