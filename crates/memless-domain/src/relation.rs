pub(crate) fn guessed_target(column: &str) -> Option<String> {
    let stem = column.strip_suffix("_id")?;
    Some(format!("{stem}s"))
}
