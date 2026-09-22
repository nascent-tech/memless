use super::blank_control::is_blank_or_control;
use super::flow_null::is_flow_or_null;
use super::indicator::has_leading_indicator;

pub(super) fn needs_syntax_quote(lexeme: &str) -> bool {
    is_blank_or_control(lexeme) || has_leading_indicator(lexeme) || is_flow_or_null(lexeme)
}
