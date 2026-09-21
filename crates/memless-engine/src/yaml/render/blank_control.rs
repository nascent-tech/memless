pub(super) fn is_blank_or_control(lexeme: &str) -> bool {
    lexeme.is_empty()
        || lexeme != lexeme.trim()
        || lexeme.contains('\u{FEFF}')
        || lexeme.chars().any(|character| character.is_control())
}
