use super::explicit::text_style;
use crate::document::{RawScalar, RawStyle};
use crate::scalar::decimal_display::render_decimal;
use crate::scalar::Scalar;

pub(crate) fn raw_scalar(value: &Scalar) -> RawScalar {
    match value {
        Scalar::Text(text) => RawScalar { lexeme: text.clone(), style: text_style(text) },
        Scalar::Integer(number) => RawScalar { lexeme: number.to_string(), style: RawStyle::Plain },
        Scalar::Decimal(number) => RawScalar { lexeme: render_decimal(*number), style: RawStyle::Plain },
        Scalar::Boolean(flag) => RawScalar { lexeme: flag.to_string(), style: RawStyle::Plain },
    }
}
