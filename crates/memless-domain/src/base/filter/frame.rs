use crate::query::Filter;

pub(crate) enum Frame<'f> {
    Enter(&'f Filter),
    CombineAnd,
    CombineOr,
}
