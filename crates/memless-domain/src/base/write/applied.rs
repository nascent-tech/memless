use crate::base::Base;

#[derive(Debug)]
pub struct Applied {
    pub base: Base,
    pub affected: u64,
}
