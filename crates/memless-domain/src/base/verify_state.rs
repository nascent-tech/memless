use super::verify::verify;
use crate::refusal::Refusal;

impl super::Base {
    pub fn verify_state(&self) -> Result<(), Refusal> {
        verify(&self.tables).map_err(Refusal::Structure)
    }
}
