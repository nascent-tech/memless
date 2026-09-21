use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Select;

pub(crate) fn reject_positioning(select: &Select) -> Result<(), QueryRefusal> {
    if select.top.is_some() {
        return Err(outside("TOP"));
    }
    if select.into.is_some() {
        return Err(outside("SELECT INTO"));
    }
    if select.prewhere.is_some() {
        return Err(outside("PREWHERE"));
    }
    Ok(())
}
