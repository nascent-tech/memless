pub(crate) fn signed_text(number: &str, negative: bool) -> String {
    if negative {
        return format!("-{number}");
    }
    number.to_string()
}
