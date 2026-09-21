pub(crate) fn guess_integer(lexeme: &str) -> Option<i64> {
    let digits = lexeme.strip_prefix(['+', '-']).unwrap_or(lexeme);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    lexeme.parse::<i64>().ok()
}
