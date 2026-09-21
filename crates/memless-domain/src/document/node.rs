use super::drain::drain_node;
use super::{RawKey, RawScalar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawNode {
    Scalar(RawScalar),
    Sequence(Vec<RawNode>),
    Mapping(Vec<(RawKey, RawNode)>),
    Null,
}

impl Drop for RawNode {
    fn drop(&mut self) {
        drain_node(self);
    }
}
