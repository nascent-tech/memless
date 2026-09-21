use super::Scalar;

pub(crate) fn guess_keyword_or_text(lexeme: &str) -> Scalar {
    match lexeme {
        "true" => Scalar::Boolean(true),
        "false" => Scalar::Boolean(false),
        _ => Scalar::Text(lexeme.to_string()),
    }
}
