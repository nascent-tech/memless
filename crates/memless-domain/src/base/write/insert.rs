use super::build_fields::build_fields;
use super::count::check_count;
use super::next_position::next_position;
use super::place_row::place_row;
use super::unique_columns::check_unique;
use crate::base::build_row::build_row;
use crate::base::table::Table;
use crate::query::Insert;
use crate::refusal::Refusal;

pub(crate) fn insert(tables: &[Table], spec: &Insert) -> Result<(Vec<Table>, u64), Refusal> {
    check_count(spec)?;
    check_unique(&spec.table, &spec.columns)?;
    let fields = build_fields(&spec.columns, &spec.values);
    let row = build_row(&spec.table, next_position(tables, &spec.table), &fields)?;
    let mut next = tables.to_vec();
    place_row(&mut next, &spec.table, row);
    Ok((next, 1))
}
