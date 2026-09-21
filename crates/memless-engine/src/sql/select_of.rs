use super::body_label::body_label;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Query, Select, SetExpr};

pub(crate) fn select_of(query: Query) -> Result<Box<Select>, QueryRefusal> {
    match *query.body {
        SetExpr::Select(select) => Ok(select),
        other => Err(outside(body_label(&other))),
    }
}
