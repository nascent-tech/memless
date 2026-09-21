use super::escape::quote_lexeme;
use super::quote::needs_syntax_quote;
use memless_domain::document::{RawScalar, RawStyle};

pub(super) fn render_scalar(scalar: &RawScalar) -> String {
    if scalar.style == RawStyle::ExplicitText || needs_syntax_quote(&scalar.lexeme) {
        return quote_lexeme(&scalar.lexeme);
    }
    scalar.lexeme.clone()
}
