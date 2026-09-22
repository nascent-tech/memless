use std::collections::HashSet;

use crate::refusal::QueryRefusal;
use crate::rows::Rows;

pub(crate) fn distinct_headers(rows: &Rows) -> Result<(), QueryRefusal> {
    let mut seen = HashSet::new();
    if rows.columns.iter().all(|header| seen.insert(header)) {
        return Ok(());
    }
    Err(QueryRefusal::OutsideSubset { construct: "duplicate output column".to_string() })
}
