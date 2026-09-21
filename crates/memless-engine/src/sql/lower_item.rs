use super::aggregate_item::aggregate_item;
use super::column_item::column_item;
use super::item_kind::ItemKind;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, SelectItem};

pub(crate) fn lower_item(item: &SelectItem, has_join: bool) -> Result<ItemKind, QueryRefusal> {
    match item {
        SelectItem::UnnamedExpr(Expr::Function(function)) => aggregate_item(function, has_join),
        SelectItem::UnnamedExpr(expr) => column_item(expr, has_join),
        _ => Err(outside("this projection item")),
    }
}
