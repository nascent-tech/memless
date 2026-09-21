pub(crate) fn render_decimal(value: f64) -> String {
    let rendered = format!("{value}");
    if rendered.contains('.') {
        rendered
    } else {
        format!("{rendered}.0")
    }
}
