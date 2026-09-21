use crate::base::row::Row;
use crate::base::table::Table;

pub(crate) fn place_row(tables: &mut Vec<Table>, name: &str, row: Row) {
    if let Some(table) = tables.iter_mut().find(|table| table.name == name) {
        table.rows.push(row);
        return;
    }
    tables.push(Table { name: name.to_string(), rows: vec![row] });
}
