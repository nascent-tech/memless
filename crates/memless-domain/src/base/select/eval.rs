use super::candidate::Candidate;
use super::cell::cell as cell_at;
use super::plan::Plan;
use crate::base::filter::Cells;
use crate::query::ColumnRef;
use crate::scalar::Scalar;

pub(crate) struct Eval<'e, 'a> {
    pub plan: &'e Plan<'a>,
    pub candidate: &'e Candidate<'a>,
}

impl Cells for Eval<'_, '_> {
    fn cell(&self, column: &ColumnRef) -> Option<&Scalar> {
        cell_at(self.plan, self.candidate, column)
    }
}
