use super::RawStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawScalar {
    pub lexeme: String,
    pub style: RawStyle,
}
