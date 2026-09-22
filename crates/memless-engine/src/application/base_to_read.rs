use super::instance::Instance;
use memless_domain::Base;

pub(crate) fn base_to_read(instance: &Instance) -> &Base {
    instance.transaction.as_ref().unwrap_or(&instance.base)
}
