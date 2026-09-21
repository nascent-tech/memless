use super::cell::cell;
use super::compare_holds::compare_holds;
use super::eval::Eval;
use crate::query::Filter;

pub(crate) fn eval_leaf(eval: &Eval, filter: &Filter) -> bool {
    match filter {
        Filter::Compare(compare) => compare_holds(eval, compare),
        Filter::IsNull(column) => cell(eval.plan, eval.candidate, column).is_none(),
        Filter::IsNotNull(column) => cell(eval.plan, eval.candidate, column).is_some(),
        _ => false,
    }
}
