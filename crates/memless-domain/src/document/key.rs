#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawKey {
    Text(String),
    NonText { rendered: String },
}
