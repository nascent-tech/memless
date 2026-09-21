use memless_domain::document::{RawScalar, RawStyle};
use memless_domain::Scalar;

fn plain(lexeme: &str) -> RawScalar {
    RawScalar { lexeme: lexeme.to_string(), style: RawStyle::Plain }
}

fn explicit(lexeme: &str) -> RawScalar {
    RawScalar { lexeme: lexeme.to_string(), style: RawStyle::ExplicitText }
}

#[test]
fn guesses_a_quoted_scalar_as_text_without_inspecting_its_content() {
    assert_eq!(Scalar::guess(&explicit("5")), Scalar::Text("5".to_string()));
    assert_eq!(Scalar::guess(&explicit("true")), Scalar::Text("true".to_string()));
}

#[test]
fn guesses_a_block_or_tagged_scalar_as_text() {
    assert_eq!(Scalar::guess(&explicit("a\nb")), Scalar::Text("a\nb".to_string()));
}

#[test]
fn guesses_a_bare_integer_fitting_i64_as_integer() {
    assert_eq!(Scalar::guess(&plain("42")), Scalar::Integer(42));
    assert_eq!(Scalar::guess(&plain("-7")), Scalar::Integer(-7));
    assert_eq!(Scalar::guess(&plain("+3")), Scalar::Integer(3));
}

#[test]
fn guesses_an_integer_overflowing_i64_as_text() {
    let big = "99999999999999999999";
    assert_eq!(Scalar::guess(&plain(big)), Scalar::Text(big.to_string()));
}

#[test]
fn guesses_a_bare_decimal_as_decimal() {
    assert_eq!(Scalar::guess(&plain("1.25")), Scalar::Decimal(1.25));
    assert_eq!(Scalar::guess(&plain("-0.5")), Scalar::Decimal(-0.5));
}

#[test]
fn guesses_a_decimal_overflowing_f64_as_text() {
    let huge = format!("1{}.0", "0".repeat(400));
    assert_eq!(Scalar::guess(&plain(&huge)), Scalar::Text(huge.clone()));
}

#[test]
fn guesses_exponent_infinity_nan_hexadecimal_as_text() {
    assert_eq!(Scalar::guess(&plain("1e3")), Scalar::Text("1e3".to_string()));
    assert_eq!(Scalar::guess(&plain(".inf")), Scalar::Text(".inf".to_string()));
    assert_eq!(Scalar::guess(&plain(".nan")), Scalar::Text(".nan".to_string()));
    assert_eq!(Scalar::guess(&plain("0x1F")), Scalar::Text("0x1F".to_string()));
}

#[test]
fn guesses_exactly_true_or_false_as_boolean() {
    assert_eq!(Scalar::guess(&plain("true")), Scalar::Boolean(true));
    assert_eq!(Scalar::guess(&plain("false")), Scalar::Boolean(false));
    assert_eq!(Scalar::guess(&plain("True")), Scalar::Text("True".to_string()));
    assert_eq!(Scalar::guess(&plain("yes")), Scalar::Text("yes".to_string()));
}

#[test]
fn guesses_an_unquoted_date_as_text() {
    assert_eq!(Scalar::guess(&plain("2026-09-21")), Scalar::Text("2026-09-21".to_string()));
}

#[test]
fn renders_each_variant_for_refusal_messages() {
    assert_eq!(Scalar::Text("abc".to_string()).to_string(), "\"abc\"");
    assert_eq!(Scalar::Integer(5).to_string(), "5");
    assert_eq!(Scalar::Boolean(true).to_string(), "true");
    assert_eq!(Scalar::Decimal(5.0).to_string(), "5.0");
    assert_eq!(Scalar::Decimal(1.25).to_string(), "1.25");
}

#[test]
fn renders_text_and_integer_of_the_same_digits_differently() {
    assert_ne!(Scalar::Text("5".to_string()).to_string(), Scalar::Integer(5).to_string());
}

#[test]
fn treats_different_variants_as_never_equal() {
    assert_ne!(Scalar::Text("5".to_string()), Scalar::Integer(5));
    assert_ne!(Scalar::Integer(5), Scalar::Decimal(5.0));
}

#[test]
fn treats_same_variant_same_value_as_equal() {
    assert_eq!(Scalar::Integer(5), Scalar::Integer(5));
    assert_eq!(Scalar::Text("x".to_string()), Scalar::Text("x".to_string()));
}
