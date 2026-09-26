use super::body_label::body_label;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Select, SetExpr};

pub(crate) fn select_of(body: SetExpr) -> Result<Box<Select>, QueryRefusal> {
    match body {
        SetExpr::Select(select) => Ok(select),
        other => Err(outside(body_label(&other))),
    }
}
