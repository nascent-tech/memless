use super::lower_join::lower_join;
use super::lower_order::lower_order;
use super::lower_projection::lower_projection;
use super::lower_selection::lower_selection;
use super::reject_options::reject_options;
use super::single_from::single_from;
use super::table_name::table_name;
use memless_domain::query::Select;
use memless_domain::QueryRefusal;
use sqlparser::ast::{OrderBy, Select as SqlSelect};

pub(crate) fn lower_select(select: &SqlSelect, order_by: &Option<OrderBy>) -> Result<Select, QueryRefusal> {
    reject_options(select)?;
    let from = single_from(&select.from)?;
    let name = table_name(&from.relation)?;
    let join = lower_join(&from.joins, &name)?;
    let has_join = join.is_some();
    let items = lower_projection(&select.projection, has_join)?;
    let filter = lower_selection(&select.selection, has_join)?;
    let order = lower_order(order_by, &items, has_join)?;
    Ok(Select { items, from: name, join, filter, order })
}
