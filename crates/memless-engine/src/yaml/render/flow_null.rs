const NULLS: &[&str] = &["null", "Null", "NULL", "~"];

pub(super) fn is_flow_or_null(lexeme: &str) -> bool {
    lexeme.ends_with(':') || lexeme.contains(": ") || lexeme.contains(" #") || NULLS.contains(&lexeme)
}
