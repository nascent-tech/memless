use super::cells::Cells;
use super::holds::holds;
use crate::query::Compare;

pub(crate) fn compare_holds<C: Cells>(cells: &C, compare: &Compare) -> bool {
    match cells.cell(&compare.column) {
        Some(value) => holds(compare.op, value, &compare.literal),
        None => false,
    }
}
