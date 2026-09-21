use memless_domain::query::Write;
use memless_domain::{Applied, Base, Refusal};

pub(crate) fn apply(base: &Base, statement: &Write) -> Result<Applied, Refusal> {
    base.write(statement)
}
