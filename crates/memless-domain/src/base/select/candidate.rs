use crate::base::row::Row;

pub(crate) struct Candidate<'a> {
    pub from: &'a Row,
    pub joined: Option<&'a Row>,
}
