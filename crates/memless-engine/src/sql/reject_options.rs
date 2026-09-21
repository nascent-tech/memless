use super::reject_clustering::reject_clustering;
use super::reject_distinct_group::reject_distinct_group;
use super::reject_positioning::reject_positioning;
use super::reject_windowing::reject_windowing;
use memless_domain::QueryRefusal;
use sqlparser::ast::Select;

pub(crate) fn reject_options(select: &Select) -> Result<(), QueryRefusal> {
    reject_distinct_group(select)?;
    reject_positioning(select)?;
    reject_clustering(select)?;
    reject_windowing(select)
}
