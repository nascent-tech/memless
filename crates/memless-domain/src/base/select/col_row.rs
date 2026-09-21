use super::candidate::Candidate;
use super::col::Col;
use crate::base::row::Row;

pub(crate) fn col_row<'a>(candidate: &Candidate<'a>, col: &Col) -> Option<&'a Row> {
    match col.joined {
        true => candidate.joined,
        false => Some(candidate.from),
    }
}
