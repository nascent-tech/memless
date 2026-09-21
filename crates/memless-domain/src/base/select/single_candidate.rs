use super::candidate::Candidate;
use crate::base::row::Row;

pub(crate) fn single_candidate(row: &Row) -> Candidate<'_> {
    Candidate { from: row, joined: None }
}
