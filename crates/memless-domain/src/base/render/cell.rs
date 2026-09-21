use super::raw_scalar::raw_scalar;
use crate::document::{RawKey, RawNode};
use crate::scalar::Scalar;

pub(crate) fn cell(entry: &(String, Scalar)) -> (RawKey, RawNode) {
    let (name, value) = entry;
    (RawKey::Text(name.clone()), RawNode::Scalar(raw_scalar(value)))
}
