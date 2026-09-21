use super::candidate::Candidate;
use super::join_plan::JoinPlan;
use super::pair_candidates::pair_candidates;
use crate::base::table::Table;

pub(crate) fn joined_candidates<'a>(from: &'a Table, join: &JoinPlan<'a>) -> Vec<Candidate<'a>> {
    from.rows.iter().flat_map(|row| pair_candidates(row, join)).collect()
}
