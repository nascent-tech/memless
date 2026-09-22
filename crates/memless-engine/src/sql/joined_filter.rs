use memless_domain::query::Filter;

pub(crate) fn joined_filter(is_and: bool, left: Filter, right: Filter) -> Filter {
    if is_and {
        return Filter::And(Box::new(left), Box::new(right));
    }
    Filter::Or(Box::new(left), Box::new(right))
}
