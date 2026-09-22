use super::escape_char::escaped;

pub(super) fn quote_lexeme(lexeme: &str) -> String {
    let inner: String = lexeme.chars().map(escaped).collect();
    format!("\"{inner}\"")
}
