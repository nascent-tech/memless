use memless_domain::QueryRefusal;
use memless_domain::QueryRefusal::InvalidSql;

pub(crate) fn invalid(detail: &str) -> QueryRefusal {
    InvalidSql { detail: detail.to_string() }
}
