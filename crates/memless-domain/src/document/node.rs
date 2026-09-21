use super::{RawKey, RawScalar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawNode {
    Scalar(RawScalar),
    Sequence(Vec<RawNode>),
    Mapping(Vec<(RawKey, RawNode)>),
    Null,
}
