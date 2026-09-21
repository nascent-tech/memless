use std::fmt;

use crate::document::{RawScalar, RawStyle};

mod decimal;
pub(crate) mod decimal_display;
mod integer;
mod keyword;
pub(crate) mod scalar_order;

use decimal::guess_decimal;
use decimal_display::render_decimal;
use integer::guess_integer;
use keyword::guess_keyword_or_text;

#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    Text(String),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
}

impl Scalar {
    pub fn guess(raw: &RawScalar) -> Scalar {
        if raw.style == RawStyle::ExplicitText {
            return Scalar::Text(raw.lexeme.clone());
        }
        if let Some(number) = guess_integer(&raw.lexeme) {
            return Scalar::Integer(number);
        }
        if let Some(number) = guess_decimal(&raw.lexeme) {
            return Scalar::Decimal(number);
        }
        guess_keyword_or_text(&raw.lexeme)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scalar::Text(text) => write!(formatter, "{text:?}"),
            Scalar::Integer(value) => write!(formatter, "{value}"),
            Scalar::Boolean(value) => write!(formatter, "{value}"),
            Scalar::Decimal(value) => write!(formatter, "{}", render_decimal(*value)),
        }
    }
}
