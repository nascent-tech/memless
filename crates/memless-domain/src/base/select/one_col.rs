use super::col::Col;
use super::col_header::col_header;

pub(crate) fn one_col(table: &str, name: String, joined: bool, has_join: bool) -> Col {
    let header = col_header(table, &name, has_join);
    Col { header, joined, name }
}
