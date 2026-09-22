pub(crate) fn col_header(table: &str, name: &str, has_join: bool) -> String {
    if has_join {
        return format!("{table}.{name}");
    }
    name.to_string()
}
