use super::frame::Frame;
use crate::query::Filter;

pub(crate) fn enter_branch<'f>(work: &mut Vec<Frame<'f>>, combine: Frame<'f>, left: &'f Filter, right: &'f Filter) {
    work.push(combine);
    work.push(Frame::Enter(left));
    work.push(Frame::Enter(right));
}
