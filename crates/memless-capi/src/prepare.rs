use memless_engine::Rows;

use super::c_text::c_text;
use super::cell::cell_of;
use super::prepared::Prepared;

pub(crate) fn prepare(rows: Rows) -> Prepared {
    let columns = rows.columns.into_iter().map(|name| c_text(&name)).collect();
    let cells = rows.rows.into_iter().map(|row| row.iter().map(cell_of).collect()).collect();
    Prepared { columns, rows: cells }
}
