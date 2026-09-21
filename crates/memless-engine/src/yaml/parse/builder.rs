use std::collections::HashMap;

use super::frame::Frame;
use memless_domain::document::RawNode;

#[derive(Default)]
pub(super) struct Builder {
    pub stack: Vec<Frame>,
    pub anchors: HashMap<usize, RawNode>,
    pub documents: usize,
    pub root: Option<RawNode>,
}
