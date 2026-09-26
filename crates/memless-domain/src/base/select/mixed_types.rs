use super::holder_table::holder_table;
use super::plan::Plan;
use crate::base::row::Row;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::OrderMixedTypes;
use crate::refusal::RowLabel;

pub(crate) fn mixed_types(plan: &Plan, column: &ColumnRef, first: &Row, second: &Row) -> QueryRefusal {
    OrderMixedTypes {
        table: holder_table(plan, column).name.clone(),
        column: column.column.clone(),
        first: RowLabel::Id(first.id.clone()),
        second: RowLabel::Id(second.id.clone()),
    }
}
