use super::delete::delete;
use super::insert::insert;
use super::update::update;
use crate::base::table::Table;
use crate::query::Write;
use crate::refusal::Refusal;

pub(crate) fn apply(tables: &[Table], statement: &Write) -> Result<(Vec<Table>, u64), Refusal> {
    match statement {
        Write::Insert(spec) => insert(tables, spec),
        Write::Update(spec) => update(tables, spec),
        Write::Delete(spec) => delete(tables, spec),
    }
}
