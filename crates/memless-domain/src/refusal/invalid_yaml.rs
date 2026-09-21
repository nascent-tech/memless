use super::TextPosition;

pub(crate) fn invalid_yaml_message(path: &str, at: &Option<TextPosition>) -> String {
    match at {
        Some(point) => format!("invalid YAML in {path:?} at line {}, column {}", point.line, point.column),
        None => format!("invalid YAML in {path:?}"),
    }
}
