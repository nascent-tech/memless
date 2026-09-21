use memless_domain::query::{Aggregate, ColumnRef};

pub(crate) enum ItemKind {
    Column(ColumnRef),
    Aggregate(Aggregate),
}
