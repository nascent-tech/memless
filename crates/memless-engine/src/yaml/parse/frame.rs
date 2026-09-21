use memless_domain::document::{RawKey, RawNode};

pub(super) enum Frame {
    Sequence { items: Vec<RawNode>, anchor: usize },
    Mapping { entries: Vec<(RawKey, RawNode)>, pending: Option<RawKey>, anchor: usize },
}
