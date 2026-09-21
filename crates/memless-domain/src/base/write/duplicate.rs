pub(crate) fn first_duplicate(columns: &[String]) -> Option<String> {
    columns
        .iter()
        .enumerate()
        .find(|(index, column)| columns[..*index].contains(column))
        .map(|(_, column)| column.clone())
}
