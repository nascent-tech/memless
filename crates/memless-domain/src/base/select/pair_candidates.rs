use super::candidate::Candidate;
use super::join_plan::JoinPlan;
use super::linked::linked;
use crate::base::row::Row;

pub(crate) fn pair_candidates<'a>(from: &'a Row, join: &JoinPlan<'a>) -> Vec<Candidate<'a>> {
    let matches = join.table.rows.iter().filter(|joined| linked(from, joined, join));
    matches.map(|joined| Candidate { from, joined: Some(joined) }).collect()
}
