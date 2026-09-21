use memless_domain::QueryRefusal;
use memless_domain::QueryRefusal::OutsideSubset;

pub(crate) fn outside(construct: &str) -> QueryRefusal {
    OutsideSubset { construct: construct.to_string() }
}
