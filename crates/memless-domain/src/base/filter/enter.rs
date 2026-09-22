use super::cells::Cells;
use super::enter_branch::enter_branch;
use super::eval_leaf::eval_leaf;
use super::frame::Frame;
use crate::query::Filter;

pub(crate) fn enter<'f, C: Cells>(cells: &C, filter: &'f Filter, work: &mut Vec<Frame<'f>>, results: &mut Vec<bool>) {
    match filter {
        Filter::And(left, right) => enter_branch(work, Frame::CombineAnd, left, right),
        Filter::Or(left, right) => enter_branch(work, Frame::CombineOr, left, right),
        leaf => results.push(eval_leaf(cells, leaf)),
    }
}
