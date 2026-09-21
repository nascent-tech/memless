const ALWAYS: &[char] = &['!', '&', '*', '[', ']', '{', '}', ',', '#', '|', '>', '@', '`', '"', '\'', '%'];

pub(super) fn has_leading_indicator(lexeme: &str) -> bool {
    lexeme.starts_with(ALWAYS)
        || lexeme == "-"
        || lexeme == "?"
        || lexeme == ":"
        || lexeme.starts_with("- ")
        || lexeme.starts_with("? ")
}
