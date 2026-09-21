use std::collections::HashMap;

use super::row::Row;
use crate::Id;

pub(crate) fn first_duplicate(rows: &[Row]) -> Option<(usize, usize)> {
    let mut seen: HashMap<&Id, usize> = HashMap::with_capacity(rows.len());
    rows.iter()
        .enumerate()
        .find_map(|(index, row)| seen.insert(&row.id, index + 1).map(|first| (first, index + 1)))
}
