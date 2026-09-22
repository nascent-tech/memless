use super::cells::Cells;
use super::frame::Frame;
use super::step::step;
use crate::query::Filter;

pub(crate) fn keeps<'f, C: Cells>(cells: &C, filter: &'f Filter) -> bool {
    let mut work: Vec<Frame<'f>> = vec![Frame::Enter(filter)];
    let mut results: Vec<bool> = Vec::new();
    while let Some(frame) = work.pop() {
        step(cells, frame, &mut work, &mut results);
    }
    results.pop().unwrap_or(false)
}
