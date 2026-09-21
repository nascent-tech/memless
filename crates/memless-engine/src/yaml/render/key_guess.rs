use memless_domain::document::{RawScalar, RawStyle};
use memless_domain::Scalar;

pub(super) fn guessed_non_text(text: &str) -> bool {
    let raw = RawScalar { lexeme: text.to_string(), style: RawStyle::Plain };
    Scalar::guess(&raw) != Scalar::Text(text.to_string())
}
