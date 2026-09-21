use apply::apply;
use crate::query::Write;
use crate::refusal::Refusal;
pub use applied::Applied;

mod apply;
mod applied;
mod assign;
mod assign_one;
mod build_fields;
mod check_where;
mod count;
mod cut_rows;
mod delete;
mod duplicate;
mod edit_rows;
mod has_column;
mod insert;
mod matches_row;
mod next_position;
mod place_row;
mod rebuild;
mod remove_column;
mod row_cells;
mod set_column;
mod table_index;
mod targets;
mod unique_columns;
mod update;
mod update_one;
mod where_column;
mod where_qualifier;

impl super::Base {
    pub fn write(&self, statement: &Write) -> Result<Applied, Refusal> {
        let (tables, affected) = apply(&self.tables, statement)?;
        super::verify::verify(&tables)?;
        Ok(Applied { base: super::Base { tables }, affected })
    }
}
