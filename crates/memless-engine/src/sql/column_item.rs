use super::column_ref_of::column_ref_of;
use super::item_kind::ItemKind;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn column_item(expr: &Expr, has_join: bool) -> Result<ItemKind, QueryRefusal> {
    Ok(ItemKind::Column(column_ref_of(expr, has_join)?))
}
