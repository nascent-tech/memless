use super::cells::Cells;
use super::compare_holds::compare_holds;
use crate::query::Filter;

pub(crate) fn eval_leaf<C: Cells>(cells: &C, filter: &Filter) -> bool {
    match filter {
        Filter::Compare(compare) => compare_holds(cells, compare),
        Filter::IsNull(column) => cells.cell(column).is_none(),
        Filter::IsNotNull(column) => cells.cell(column).is_some(),
        _ => false,
    }
}
