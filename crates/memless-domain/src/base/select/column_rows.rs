use super::candidate::Candidate;
use super::col::Col;
use super::row_cells::row_cells;
use crate::rows::Rows;

pub(crate) fn column_rows<'a>(cols: &[Col], candidates: &[Candidate<'a>]) -> Rows {
    let columns = cols.iter().map(|col| col.header.clone()).collect();
    let rows = candidates.iter().map(|candidate| row_cells(cols, candidate)).collect();
    Rows { columns, rows }
}
