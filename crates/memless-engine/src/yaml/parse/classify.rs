use memless_domain::document::{RawKey, RawScalar};
use memless_domain::Scalar;

pub(super) fn classify_key(raw: RawScalar) -> RawKey {
    match Scalar::guess(&raw) {
        Scalar::Text(_) => RawKey::Text(raw.lexeme),
        other => RawKey::NonText { rendered: other.to_string() },
    }
}
