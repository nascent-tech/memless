use crate::document::{RawScalar, RawStyle};
use crate::scalar::Scalar;

pub(crate) fn text_style(text: &str) -> RawStyle {
    let raw = RawScalar { lexeme: text.to_string(), style: RawStyle::Plain };
    if Scalar::guess(&raw) == Scalar::Text(text.to_string()) {
        return RawStyle::Plain;
    }
    RawStyle::ExplicitText
}
