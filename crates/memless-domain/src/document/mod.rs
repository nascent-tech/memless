mod key;
mod node;
mod scalar;
mod style;

pub use key::RawKey;
pub use node::RawNode;
pub use scalar::RawScalar;
pub use style::RawStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDocument {
    pub source: String,
    pub root: RawNode,
}
