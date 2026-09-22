use super::cell::cell;
use super::eval::Eval;
use super::holds::holds;
use crate::query::Compare;

pub(crate) fn compare_holds(eval: &Eval, compare: &Compare) -> bool {
    match cell(eval.plan, eval.candidate, &compare.column) {
        Some(value) => holds(compare.op, value, &compare.literal),
        None => false,
    }
}
