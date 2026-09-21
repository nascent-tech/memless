use super::combine_and::combine_and;
use super::combine_or::combine_or;
use super::enter::enter;
use super::eval::Eval;
use super::frame::Frame;

pub(crate) fn step<'f>(eval: &Eval, frame: Frame<'f>, work: &mut Vec<Frame<'f>>, results: &mut Vec<bool>) {
    match frame {
        Frame::Enter(filter) => enter(eval, filter, work, results),
        Frame::CombineAnd => combine_and(results),
        Frame::CombineOr => combine_or(results),
    }
}
