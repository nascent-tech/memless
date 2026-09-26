use std::collections::HashSet;

use crate::refusal::QueryRefusal;

pub(crate) fn distinct_headers(headers: &[String]) -> Result<(), QueryRefusal> {
    let mut seen = HashSet::new();
    if headers.iter().all(|header| seen.insert(header)) {
        return Ok(());
    }
    Err(QueryRefusal::OutsideSubset { construct: "duplicate output column".to_string() })
}
