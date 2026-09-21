pub(super) fn escaped(character: char) -> String {
    match character {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        '\n' => "\\n".to_string(),
        control if control.is_control() => format!("\\u{:04X}", control as u32),
        other => other.to_string(),
    }
}
