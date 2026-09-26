use std::mem::discriminant;

use super::candidate::Candidate;
use super::held_cells::held_cells;
use super::mixed_types::mixed_types;
use super::plan::Plan;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;

pub(crate) fn one_type(plan: &Plan, candidates: &[Candidate], column: &ColumnRef) -> Result<(), QueryRefusal> {
    let held = held_cells(plan, candidates, column);
    let Some((first_row, first)) = held.first() else {
        return Ok(());
    };
    match held.iter().find(|(_, value)| discriminant(*value) != discriminant(*first)) {
        Some((second_row, _)) => Err(mixed_types(plan, column, first_row, second_row)),
        None => Ok(()),
    }
}
