use super::grouped::grouped;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Select;

pub(crate) fn reject_distinct_group(select: &Select) -> Result<(), QueryRefusal> {
    if select.distinct.is_some() {
        return Err(outside("DISTINCT"));
    }
    if select.having.is_some() {
        return Err(outside("HAVING"));
    }
    if grouped(&select.group_by) {
        return Err(outside("GROUP BY"));
    }
    Ok(())
}
