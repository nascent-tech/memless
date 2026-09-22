use super::apply::apply;
use super::Applied;
use crate::base::Base;
use crate::query::Write;
use crate::refusal::Refusal;

impl Base {
    pub fn write(&self, statement: &Write) -> Result<Applied, Refusal> {
        let (tables, affected) = apply(&self.tables, statement)?;
        let base = Base { tables };
        base.verify_state()?;
        Ok(Applied { base, affected })
    }
}
