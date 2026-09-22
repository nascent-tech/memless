use memless_domain::Base;

pub struct Instance {
    pub path: String,
    pub base: Base,
    pub transaction: Option<Base>,
}
