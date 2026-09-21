use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Select;

pub(crate) fn reject_clustering(select: &Select) -> Result<(), QueryRefusal> {
    if !select.cluster_by.is_empty() || !select.distribute_by.is_empty() {
        return Err(outside("CLUSTER BY"));
    }
    if !select.sort_by.is_empty() {
        return Err(outside("SORT BY"));
    }
    if !select.lateral_views.is_empty() {
        return Err(outside("LATERAL VIEW"));
    }
    Ok(())
}
